use std::collections::HashSet;

use gogdl_lib::VerificationEvent;

use super::Translator;
use crate::api::progress_reporting::VerifyDownloadProgress;

#[derive(Default)]
pub(crate) struct VerifyTranslator {
    checked: u64,
    dirty: bool,
    // Each broken file reports every one of its chunks; only forward the
    // first failure per path so the UI sees one event per file.
    reported: HashSet<String>,
}

impl VerifyTranslator {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    fn flush(&mut self) -> Vec<VerifyDownloadProgress> {
        if !std::mem::take(&mut self.dirty) {
            return Vec::new();
        }
        vec![VerifyDownloadProgress::Progress(self.checked)]
    }

    fn failure(
        &mut self,
        path: String,
        size: u64,
        make: fn(String, u64) -> VerifyDownloadProgress,
    ) -> Vec<VerifyDownloadProgress> {
        self.checked += size;
        self.dirty = true;
        if self.reported.insert(path.clone()) {
            // one-shot signal, flush immediately regardless of ticker
            vec![make(path, size)]
        } else {
            Vec::new()
        }
    }
}

impl Translator for VerifyTranslator {
    type Event = VerificationEvent;
    type Progress = VerifyDownloadProgress;

    fn on_event(&mut self, event: VerificationEvent) -> Vec<VerifyDownloadProgress> {
        match event {
            VerificationEvent::Verified(_, size) => {
                self.checked += size;
                self.dirty = true;
                Vec::new()
            }
            VerificationEvent::CouldNotResolvePath(path, size) => {
                self.failure(path, size, VerifyDownloadProgress::CouldNotResolvePath)
            }
            VerificationEvent::FileNotFound(path, size) => {
                self.failure(path, size, VerifyDownloadProgress::FileNotFound)
            }
            VerificationEvent::ChecksumMismatch(path, size) => {
                self.failure(path, size, VerifyDownloadProgress::ChecksumMismatch)
            }
            // a variant added upstream later has nothing to report yet
            _ => Vec::new(),
        }
    }

    fn on_tick(&mut self) -> Vec<VerifyDownloadProgress> {
        self.flush()
    }

    fn finish(&mut self) -> Vec<VerifyDownloadProgress> {
        self.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn verified(size: u64) -> VerificationEvent {
        VerificationEvent::Verified("a".into(), size)
    }

    #[test]
    fn counters_coalesce_until_a_tick() {
        let mut t = VerifyTranslator::new();
        assert!(t.on_event(verified(10)).is_empty());
        assert!(t.on_event(verified(5)).is_empty());
        assert_eq!(t.on_tick(), vec![VerifyDownloadProgress::Progress(15)]);
        assert!(t.on_tick().is_empty());
    }

    #[test]
    fn failures_count_bytes_and_report_once_per_path() {
        let mut t = VerifyTranslator::new();
        assert_eq!(
            t.on_event(VerificationEvent::ChecksumMismatch("f".into(), 4)),
            vec![VerifyDownloadProgress::ChecksumMismatch("f".into(), 4)]
        );
        // every chunk of a broken file reports; only the first is forwarded
        assert!(
            t.on_event(VerificationEvent::ChecksumMismatch("f".into(), 6))
                .is_empty()
        );
        assert_eq!(
            t.on_event(VerificationEvent::FileNotFound("g".into(), 1)),
            vec![VerifyDownloadProgress::FileNotFound("g".into(), 1)]
        );
        assert_eq!(
            t.on_event(VerificationEvent::CouldNotResolvePath("h".into(), 2)),
            vec![VerifyDownloadProgress::CouldNotResolvePath("h".into(), 2)]
        );
        assert_eq!(t.on_tick(), vec![VerifyDownloadProgress::Progress(13)]);
    }

    #[test]
    fn finish_flushes_only_when_dirty() {
        let mut t = VerifyTranslator::new();
        assert!(t.finish().is_empty());
        t.on_event(verified(3));
        assert_eq!(t.finish(), vec![VerifyDownloadProgress::Progress(3)]);
        assert!(t.finish().is_empty());
    }
}
