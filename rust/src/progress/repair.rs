use std::collections::{HashMap, HashSet};

use gogdl_lib::{DownloadEvent, DownloadStageEvent, VerificationEvent};

use super::Translator;
use super::install::{Install, Stage};
use crate::api::progress_reporting::RepairGameProgress;

pub(crate) struct RepairTranslator {
    install: Install,

    // Only the chunks that fail verification get re-downloaded, and the
    // download stage only reports compressed bytes off the wire. This is how
    // each failing (path, uncompressed size) is resolved back to the
    // compressed size to sum for `DownloadStarted`'s total.
    compressed_sizes: HashMap<(String, u64), Vec<u64>>,
    download_total_bytes: u64,

    // Only forward the first verification failure per path so the UI sees one
    // event per file; kept separate from the allocation set so a path that
    // failed allocation doesn't silence its later verification event.
    reported_verification: HashSet<String>,
}

impl RepairTranslator {
    pub(crate) fn new(compressed_sizes: HashMap<(String, u64), Vec<u64>>) -> Self {
        Self {
            install: Install::new(),
            compressed_sizes,
            download_total_bytes: 0,
            reported_verification: HashSet::new(),
        }
    }

    fn on_verification(&mut self, event: VerificationEvent) -> Vec<RepairGameProgress> {
        use VerificationEvent as E;
        let mut out = Vec::new();
        if self.install.stage != Stage::Verification {
            // final flush of whichever pre-verification stage just ended
            out.extend(self.install.flush_pre_verification());
            self.install.stage = Stage::Verification;
        }
        let (path, size, make): (_, _, fn(String, u64) -> RepairGameProgress) = match event {
            E::Verified(_, size) => {
                self.install.checked_bytes += size;
                self.install.dirty = true;
                return out;
            }
            E::CouldNotResolvePath(path, size) => {
                (path, size, RepairGameProgress::CouldNotResolvePath)
            }
            E::FileNotFound(path, size) => (path, size, RepairGameProgress::FileNotFound),
            E::ChecksumMismatch(path, size) => (path, size, RepairGameProgress::ChecksumMismatch),
        };
        self.install.checked_bytes += size;
        self.install.dirty = true;
        if let Some(compressed) = self
            .compressed_sizes
            .get_mut(&(path.clone(), size))
            .and_then(|sizes| sizes.pop())
        {
            self.download_total_bytes += compressed;
        }
        if self.reported_verification.insert(path.clone()) {
            // one-shot signal, flush immediately regardless of ticker
            out.push(make(path, size));
        }
        out
    }
}

impl Translator for RepairTranslator {
    type Event = DownloadStageEvent;
    type Progress = RepairGameProgress;

    fn on_event(&mut self, event: DownloadStageEvent) -> Vec<RepairGameProgress> {
        match event {
            DownloadStageEvent::FileSizeVerificationStage(event) => {
                self.install.on_size_verification(event);
                Vec::new()
            }
            DownloadStageEvent::FileAllocationStage(event) => self.install.on_allocation(event),
            DownloadStageEvent::VerificationStage(event) => self.on_verification(event),
            // `repair_future` is about to resolve with
            // `DownloadError::FileAllocationError`, which surfaces to Dart as
            // a stream error; no progress event needed here.
            DownloadStageEvent::FileAllocationError() => Vec::new(),
            DownloadStageEvent::DownloadStage(DownloadEvent::Preparing) => {
                self.install.stage = Stage::Download;
                self.install.dirty = false;
                vec![
                    // final flush of the verification stage that just ended
                    RepairGameProgress::Verification {
                        checked_bytes: self.install.checked_bytes,
                    },
                    // one-shot signal, flush immediately regardless of ticker
                    RepairGameProgress::DownloadStarted {
                        total_bytes: self.download_total_bytes,
                    },
                ]
            }
            DownloadStageEvent::DownloadStage(DownloadEvent::Prepared)
            | DownloadStageEvent::DownloadStage(DownloadEvent::Downloading) => Vec::new(),
            DownloadStageEvent::DownloadStage(DownloadEvent::Progress(delta)) => {
                self.install.on_download_progress(delta as u64);
                Vec::new()
            }
            DownloadStageEvent::DownloadStage(DownloadEvent::ProgressRegression(delta)) => {
                self.install.on_download_regression(delta as u64);
                Vec::new()
            }
        }
    }

    fn on_tick(&mut self) -> Vec<RepairGameProgress> {
        self.install.flush()
    }

    fn finish(&mut self) -> Vec<RepairGameProgress> {
        self.install.flush()
    }
}

#[cfg(test)]
mod tests {
    use gogdl_lib::{FileAllocationEvent, FileSizeVerificationEvent};

    use super::*;

    type P = RepairGameProgress;

    fn sizes(entries: &[(&str, u64, &[u64])]) -> HashMap<(String, u64), Vec<u64>> {
        entries
            .iter()
            .map(|(path, size, compressed)| ((path.to_string(), *size), compressed.to_vec()))
            .collect()
    }

    fn verify(event: VerificationEvent) -> DownloadStageEvent {
        DownloadStageEvent::VerificationStage(event)
    }

    fn verified(size: u64) -> DownloadStageEvent {
        verify(VerificationEvent::Verified("ok".into(), size))
    }

    fn mismatch(path: &str, size: u64) -> DownloadStageEvent {
        verify(VerificationEvent::ChecksumMismatch(path.into(), size))
    }

    fn preparing() -> DownloadStageEvent {
        DownloadStageEvent::DownloadStage(DownloadEvent::Preparing)
    }

    #[test]
    fn first_verification_flushes_the_pre_verification_stage() {
        let mut t = RepairTranslator::new(HashMap::new());
        t.on_event(DownloadStageEvent::FileSizeVerificationStage(
            FileSizeVerificationEvent::FileSizeVerificationSuccess("a".into(), 1),
        ));
        assert_eq!(
            t.on_event(verified(5)),
            vec![P::FileSizeVerification { checked_files: 1 }]
        );
        // already in the verification stage: no repeat
        assert!(t.on_event(verified(5)).is_empty());
    }

    #[test]
    fn verification_flushes_the_allocation_stage_when_one_ran() {
        let mut t = RepairTranslator::new(HashMap::new());
        t.on_event(DownloadStageEvent::FileSizeVerificationStage(
            FileSizeVerificationEvent::FileNotFound("a".into(), 7),
        ));
        t.on_event(DownloadStageEvent::FileAllocationStage(
            FileAllocationEvent::FileAllocationSuccess("a".into(), 7),
        ));
        assert_eq!(
            t.on_event(verified(1)),
            vec![P::FileAllocation {
                allocated_files: 1,
                allocated_bytes: 7,
            }]
        );
    }

    #[test]
    fn verification_counts_verified_and_failed_bytes_alike() {
        let mut t = RepairTranslator::new(HashMap::new());
        t.on_event(verified(10));
        t.on_event(mismatch("a", 5));
        // drain the stage flush before checking the counters
        assert_eq!(t.on_tick(), vec![P::Verification { checked_bytes: 15 }]);
        assert!(t.on_tick().is_empty());
    }

    #[test]
    fn failures_are_reported_once_per_path() {
        let mut t = RepairTranslator::new(HashMap::new());
        t.on_event(verified(0)); // enter the verification stage
        assert_eq!(
            t.on_event(mismatch("a", 5)),
            vec![P::ChecksumMismatch("a".into(), 5)]
        );
        assert!(t.on_event(mismatch("a", 6)).is_empty());
        assert_eq!(
            t.on_event(verify(VerificationEvent::FileNotFound("b".into(), 1))),
            vec![P::FileNotFound("b".into(), 1)]
        );
        assert_eq!(
            t.on_event(verify(VerificationEvent::CouldNotResolvePath(
                "c".into(),
                2
            ))),
            vec![P::CouldNotResolvePath("c".into(), 2)]
        );
    }

    #[test]
    fn allocation_failure_does_not_silence_the_verification_event() {
        let mut t = RepairTranslator::new(HashMap::new());
        t.on_event(DownloadStageEvent::FileSizeVerificationStage(
            FileSizeVerificationEvent::FileNotFound("a".into(), 1),
        ));
        let out = t.on_event(DownloadStageEvent::FileAllocationStage(
            FileAllocationEvent::CouldNotResolvePath("a".into(), 1),
        ));
        assert!(out.contains(&P::AllocationError("a".into())));
        let out = t.on_event(mismatch("a", 1));
        assert!(out.contains(&P::ChecksumMismatch("a".into(), 1)));
    }

    #[test]
    fn download_started_sums_compressed_sizes_of_failing_chunks() {
        let mut t = RepairTranslator::new(sizes(&[
            ("a", 100, &[40]),
            ("b", 100, &[30, 20]),
            ("untouched", 100, &[999]),
        ]));
        t.on_event(verified(100)); // not a failure: no lookup
        t.on_event(mismatch("a", 100));
        t.on_event(mismatch("b", 100));
        t.on_event(mismatch("b", 100)); // second chunk of b, second entry
        t.on_event(mismatch("b", 100)); // entries exhausted: adds nothing
        t.on_event(mismatch("unknown", 1)); // no entry at all: adds nothing
        assert_eq!(
            t.on_event(preparing()),
            vec![
                P::Verification { checked_bytes: 501 },
                P::DownloadStarted { total_bytes: 90 },
            ]
        );
    }

    #[test]
    fn preparing_clears_dirty_so_the_next_tick_is_silent() {
        let mut t = RepairTranslator::new(HashMap::new());
        t.on_event(verified(1));
        t.on_event(preparing());
        assert!(t.on_tick().is_empty());
    }

    #[test]
    fn download_bytes_sum_and_regressions_saturate() {
        let mut t = RepairTranslator::new(HashMap::new());
        t.on_event(preparing());
        t.on_event(DownloadStageEvent::DownloadStage(DownloadEvent::Progress(
            10,
        )));
        t.on_event(DownloadStageEvent::DownloadStage(
            DownloadEvent::ProgressRegression(4),
        ));
        assert_eq!(
            t.on_tick(),
            vec![P::DownloadProgress {
                downloaded_bytes: 6
            }]
        );
        t.on_event(DownloadStageEvent::DownloadStage(
            DownloadEvent::ProgressRegression(100),
        ));
        assert_eq!(
            t.finish(),
            vec![P::DownloadProgress {
                downloaded_bytes: 0
            }]
        );
        assert!(t.finish().is_empty());
    }

    #[test]
    fn allocation_error_stage_event_produces_nothing() {
        let mut t = RepairTranslator::new(HashMap::new());
        assert!(
            t.on_event(DownloadStageEvent::FileAllocationError())
                .is_empty()
        );
    }
}
