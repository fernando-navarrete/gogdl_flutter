//! Translation of `gogdl-lib` events into the progress events Dart receives.
//!
//! Each job has a [`Translator`]: a plain state machine with no I/O. [`drive`]
//! runs the tick loop shared by all of them. Lives outside `api/` so
//! flutter_rust_bridge doesn't pick it up for codegen.

use std::time::Duration;

use tokio::sync::mpsc::UnboundedReceiver;
use tokio::time::interval;

pub(crate) mod download;
pub(crate) mod install;
pub(crate) mod proton;
pub(crate) mod repair;
pub(crate) mod saves;
pub(crate) mod verify;

/// How often coalesced progress (byte and file counters) is emitted.
const TICK: Duration = Duration::from_millis(150);

pub(crate) trait Translator {
    type Event;
    type Progress;

    /// Emits that go out immediately (stage changes, per-path events).
    fn on_event(&mut self, event: Self::Event) -> Vec<Self::Progress>;

    /// Coalesced emits (counters), only when something changed since the last one.
    fn on_tick(&mut self) -> Vec<Self::Progress>;

    /// Final flush once the event channel closes, so the UI doesn't sit at a
    /// stale % between the last tick and the end of the job.
    fn finish(&mut self) -> Vec<Self::Progress>;
}

/// Feeds `rx` through `translator` until the sender is dropped, coalescing on
/// a [`TICK`] interval, and hands everything it produces to `emit`.
///
/// `emit` returns `false` when the listener is gone (a failed `sink.add`).
/// `drive` then stops at once, without the final flush, and returns `false`.
/// It returns `true` once the sender is dropped and everything was emitted.
pub(crate) async fn drive<T: Translator>(
    mut translator: T,
    mut rx: UnboundedReceiver<T::Event>,
    mut emit: impl FnMut(T::Progress) -> bool,
) -> bool {
    let mut ticker = interval(TICK);
    ticker.tick().await; // first tick fires immediately, discard it

    loop {
        let emits = tokio::select! {
            event = rx.recv() => match event {
                Some(event) => translator.on_event(event),
                None => break, // sender dropped, the job is done
            },
            _ = ticker.tick() => translator.on_tick(),
        };
        if !emits.into_iter().all(&mut emit) {
            return false;
        }
    }

    translator.finish().into_iter().all(&mut emit)
}

/// Runs `job` alongside [`drive`] and returns its output once both are done,
/// so every event is flushed before the caller sends `Finished`.
///
/// Returns `None` if the listener goes away while the job is still running.
/// The job future is then dropped, which stops it (see each `gogdl-lib`
/// job's `# Cancelling` rustdoc). That's only noticed on the next emit, so a
/// job that reports nothing (a stalled request) runs until it does.
pub(crate) async fn run<J: Future, T: Translator>(
    job: J,
    translator: T,
    rx: UnboundedReceiver<T::Event>,
    emit: impl FnMut(T::Progress) -> bool,
) -> Option<J::Output> {
    let drive = drive(translator, rx, emit);
    tokio::pin!(job, drive);

    let mut output = None;
    let mut driven = false;
    while output.is_none() || !driven {
        tokio::select! {
            out = &mut job, if output.is_none() => output = Some(out),
            listening = &mut drive, if !driven => {
                // Once the job is done, a failed final flush only loses the
                // last counters; the job's own result still stands.
                if !listening && output.is_none() {
                    return None;
                }
                driven = true;
            }
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use std::future::pending;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Arc, Mutex};

    use tokio::sync::mpsc::unbounded_channel;
    use tokio::task::yield_now;
    use tokio::time::advance;

    use super::*;

    /// Immediate emits `Event(n)`, coalesces the running sum into `Tick(sum)`
    /// and closes with `Finish(sum)`.
    #[derive(Default)]
    struct Sum {
        sum: u32,
        dirty: bool,
    }

    #[derive(Debug, PartialEq)]
    enum Out {
        Event(u32),
        Tick(u32),
        Finish(u32),
    }

    impl Translator for Sum {
        type Event = u32;
        type Progress = Out;

        fn on_event(&mut self, event: u32) -> Vec<Out> {
            self.sum += event;
            self.dirty = true;
            vec![Out::Event(event)]
        }

        fn on_tick(&mut self) -> Vec<Out> {
            if std::mem::take(&mut self.dirty) {
                vec![Out::Tick(self.sum)]
            } else {
                Vec::new()
            }
        }

        fn finish(&mut self) -> Vec<Out> {
            vec![Out::Finish(self.sum)]
        }
    }

    /// Lets the spawned `drive` task run until it is waiting again.
    async fn settle() {
        for _ in 0..5 {
            yield_now().await;
        }
    }

    fn start() -> (
        tokio::sync::mpsc::UnboundedSender<u32>,
        Arc<Mutex<Vec<Out>>>,
        tokio::task::JoinHandle<bool>,
    ) {
        let (tx, rx) = unbounded_channel();
        let out = Arc::new(Mutex::new(Vec::new()));
        let handle = tokio::spawn(drive(Sum::default(), rx, recorder(&out)));
        (tx, out, handle)
    }

    /// An emit that records everything and never loses its listener.
    fn recorder(out: &Arc<Mutex<Vec<Out>>>) -> impl FnMut(Out) -> bool + use<> {
        let out = out.clone();
        move |p| {
            out.lock().unwrap().push(p);
            true
        }
    }

    /// Sets its flag when dropped, to see a job future being dropped.
    struct DropGuard(Arc<AtomicBool>);

    impl Drop for DropGuard {
        fn drop(&mut self) {
            self.0.store(true, Ordering::SeqCst);
        }
    }

    fn take(out: &Mutex<Vec<Out>>) -> Vec<Out> {
        std::mem::take(&mut *out.lock().unwrap())
    }

    #[tokio::test(start_paused = true)]
    async fn immediate_emits_go_out_before_the_tick() {
        let (tx, out, _handle) = start();
        settle().await;
        tx.send(1).unwrap();
        settle().await;
        assert_eq!(take(&out), vec![Out::Event(1)]);
    }

    #[tokio::test(start_paused = true)]
    async fn events_within_a_tick_coalesce_into_one_emit() {
        let (tx, out, _handle) = start();
        settle().await;
        tx.send(1).unwrap();
        tx.send(2).unwrap();
        tx.send(3).unwrap();
        settle().await;
        take(&out);
        advance(TICK).await;
        settle().await;
        assert_eq!(take(&out), vec![Out::Tick(6)]);
    }

    #[tokio::test(start_paused = true)]
    async fn idle_ticks_emit_nothing() {
        let (tx, out, _handle) = start();
        settle().await;
        tx.send(1).unwrap();
        settle().await;
        advance(TICK).await;
        settle().await;
        take(&out);
        advance(TICK * 3).await;
        settle().await;
        assert!(take(&out).is_empty());
    }

    #[tokio::test(start_paused = true)]
    async fn nothing_ticks_before_the_interval_elapses() {
        let (tx, out, _handle) = start();
        settle().await;
        tx.send(1).unwrap();
        settle().await;
        take(&out);
        advance(TICK / 2).await;
        settle().await;
        assert!(take(&out).is_empty());
    }

    #[tokio::test(start_paused = true)]
    async fn dropping_the_sender_ends_the_loop_with_the_final_flush_last() {
        let (tx, out, handle) = start();
        settle().await;
        tx.send(4).unwrap();
        drop(tx);
        // the loop must finish on its own, without another tick
        assert!(handle.await.unwrap());
        assert_eq!(take(&out), vec![Out::Event(4), Out::Finish(4)]);
    }

    #[tokio::test(start_paused = true)]
    async fn a_failed_emit_stops_the_loop_without_the_final_flush() {
        let (tx, rx) = unbounded_channel();
        let out = Arc::new(Mutex::new(Vec::new()));
        let sink = out.clone();
        let handle = tokio::spawn(drive(Sum::default(), rx, move |p| {
            sink.lock().unwrap().push(p);
            false
        }));
        settle().await;
        tx.send(1).unwrap();
        // the loop must finish while the sender is still alive
        assert!(!handle.await.unwrap());
        assert_eq!(take(&out), vec![Out::Event(1)]);
        drop(tx);
    }

    #[tokio::test(start_paused = true)]
    async fn run_drops_the_job_when_the_listener_goes_away() {
        let (tx, rx) = unbounded_channel();
        let dropped = Arc::new(AtomicBool::new(false));
        let guard = DropGuard(dropped.clone());
        let job = async move {
            let _guard = guard;
            tx.send(1).unwrap();
            pending::<()>().await;
        };
        let out = Arc::new(Mutex::new(Vec::new()));
        let sink = out.clone();
        let result = run(job, Sum::default(), rx, move |p| {
            sink.lock().unwrap().push(p);
            false
        })
        .await;
        assert_eq!(result, None);
        assert!(dropped.load(Ordering::SeqCst));
        assert_eq!(take(&out), vec![Out::Event(1)]);
    }

    #[tokio::test(start_paused = true)]
    async fn run_flushes_every_event_before_returning_the_output() {
        let (tx, rx) = unbounded_channel();
        let job = async move {
            tx.send(1).unwrap();
            tokio::time::sleep(TICK).await;
            tx.send(2).unwrap();
            "done"
        };
        let out = Arc::new(Mutex::new(Vec::new()));
        let result = run(job, Sum::default(), rx, recorder(&out)).await;
        assert_eq!(result, Some("done"));
        assert_eq!(
            take(&out),
            vec![Out::Event(1), Out::Tick(1), Out::Event(2), Out::Finish(3)]
        );
    }

    #[tokio::test(start_paused = true)]
    async fn run_keeps_the_output_when_only_the_final_flush_fails() {
        let (tx, rx) = unbounded_channel();
        let job = async move {
            tx.send(1).unwrap();
            "done"
        };
        let out = Arc::new(Mutex::new(Vec::new()));
        let sink = out.clone();
        let result = run(job, Sum::default(), rx, move |p| {
            let finish = matches!(p, Out::Finish(_));
            sink.lock().unwrap().push(p);
            !finish
        })
        .await;
        assert_eq!(result, Some("done"));
        assert_eq!(take(&out), vec![Out::Event(1), Out::Finish(1)]);
    }
}
