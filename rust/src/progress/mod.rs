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
