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
pub(crate) async fn drive<T: Translator>(
    mut translator: T,
    mut rx: UnboundedReceiver<T::Event>,
    mut emit: impl FnMut(T::Progress),
) {
    let mut ticker = interval(TICK);
    ticker.tick().await; // first tick fires immediately, discard it

    loop {
        tokio::select! {
            event = rx.recv() => match event {
                Some(event) => translator.on_event(event).into_iter().for_each(&mut emit),
                None => break, // sender dropped, the job is done
            },
            _ = ticker.tick() => translator.on_tick().into_iter().for_each(&mut emit),
        }
    }

    translator.finish().into_iter().for_each(&mut emit);
}

#[cfg(test)]
mod tests {
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
        tokio::task::JoinHandle<()>,
    ) {
        let (tx, rx) = unbounded_channel();
        let out = Arc::new(Mutex::new(Vec::new()));
        let sink = out.clone();
        let handle = tokio::spawn(drive(Sum::default(), rx, move |p| {
            sink.lock().unwrap().push(p)
        }));
        (tx, out, handle)
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
        handle.await.unwrap();
        assert_eq!(take(&out), vec![Out::Event(4), Out::Finish(4)]);
    }
}
