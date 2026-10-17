use gogdl_lib::{DownloadEvent, DownloadStageEvent};

use super::Translator;
use super::install::Install;
use crate::api::progress_reporting::DownloadGameProgress;

pub(crate) struct DownloadTranslator {
    install: Install,
    /// `DownloadGameProgress::Started.total_bytes`: the compressed size of every chunk.
    total_bytes: u64,
}

impl DownloadTranslator {
    pub(crate) fn new(total_bytes: u64) -> Self {
        Self {
            install: Install::new(),
            total_bytes,
        }
    }
}

impl Translator for DownloadTranslator {
    type Event = DownloadStageEvent;
    type Progress = DownloadGameProgress;

    fn on_event(&mut self, event: DownloadStageEvent) -> Vec<DownloadGameProgress> {
        match event {
            DownloadStageEvent::FileSizeVerificationStage(event) => {
                self.install.on_size_verification(event);
                Vec::new()
            }
            DownloadStageEvent::FileAllocationStage(event) => self.install.on_allocation(event),
            // `download_game` skips the chunks that verify, but `DownloadGameProgress` has no
            // verification variant: they count as downloaded from `Started` on.
            DownloadStageEvent::VerificationStage(_) => Vec::new(),
            // `download_future` is about to resolve with
            // `DownloadError::FileAllocationError`, which surfaces to Dart as
            // a stream error; no progress event needed here.
            DownloadStageEvent::FileAllocationError() => Vec::new(),
            DownloadStageEvent::DownloadStage(DownloadEvent::Started { compressed_total }) => {
                // final flush of whichever pre-transfer stage just ended
                let out = self.install.flush_pre_verification().into_iter().collect();
                // `compressed_total` covers only the chunks still to transfer
                self.install
                    .start_download(self.total_bytes.saturating_sub(compressed_total));
                out
            }
            DownloadStageEvent::DownloadStage(DownloadEvent::Progress(delta)) => {
                self.install.on_download_progress(delta as u64);
                Vec::new()
            }
            DownloadStageEvent::DownloadStage(DownloadEvent::ProgressRegression(delta)) => {
                self.install.on_download_regression(delta as u64);
                Vec::new()
            }
            // `Downloading` and anything upstream adds later.
            _ => Vec::new(),
        }
    }

    fn on_tick(&mut self) -> Vec<DownloadGameProgress> {
        self.install.flush()
    }

    fn finish(&mut self) -> Vec<DownloadGameProgress> {
        self.install.flush()
    }
}

#[cfg(test)]
mod tests {
    use gogdl_lib::{FileAllocationEvent, FileSizeVerificationEvent};

    use super::*;

    type P = DownloadGameProgress;

    fn size_ok(path: &str) -> DownloadStageEvent {
        DownloadStageEvent::FileSizeVerificationStage(
            FileSizeVerificationEvent::FileSizeVerificationSuccess(path.into(), 1),
        )
    }

    fn size_bad(path: &str, size: u64) -> DownloadStageEvent {
        DownloadStageEvent::FileSizeVerificationStage(FileSizeVerificationEvent::FileSizeMismatch(
            path.into(),
            size,
        ))
    }

    fn allocated(path: &str, size: u64) -> DownloadStageEvent {
        DownloadStageEvent::FileAllocationStage(FileAllocationEvent::FileAllocationSuccess(
            path.into(),
            size,
        ))
    }

    fn unresolved(path: &str) -> DownloadStageEvent {
        DownloadStageEvent::FileAllocationStage(FileAllocationEvent::CouldNotResolvePath(
            path.into(),
            1,
        ))
    }

    fn started() -> DownloadStageEvent {
        DownloadStageEvent::DownloadStage(DownloadEvent::Started {
            compressed_total: 0,
        })
    }

    fn bytes(delta: usize) -> DownloadStageEvent {
        DownloadStageEvent::DownloadStage(DownloadEvent::Progress(delta))
    }

    fn regress(delta: usize) -> DownloadStageEvent {
        DownloadStageEvent::DownloadStage(DownloadEvent::ProgressRegression(delta))
    }

    #[test]
    fn size_verification_coalesces_on_tick() {
        let mut t = DownloadTranslator::new(0);
        assert!(t.on_event(size_ok("a")).is_empty());
        assert!(t.on_event(size_ok("b")).is_empty());
        assert_eq!(
            t.on_tick(),
            vec![P::FileSizeVerification { checked_files: 2 }]
        );
        assert!(t.on_tick().is_empty());
    }

    #[test]
    fn first_allocation_flushes_size_verification_then_announces_allocation() {
        let mut t = DownloadTranslator::new(0);
        t.on_event(size_ok("a"));
        t.on_event(size_bad("b", 100));
        t.on_event(size_bad("c", 50));
        assert_eq!(
            t.on_event(allocated("b", 100)),
            vec![
                P::FileSizeVerification { checked_files: 3 },
                P::FileAllocationStarted {
                    total_files: 2,
                    total_bytes: 150,
                },
            ]
        );
        // later allocation events don't repeat the stage change
        assert!(t.on_event(allocated("c", 50)).is_empty());
        assert_eq!(
            t.on_tick(),
            vec![P::FileAllocation {
                allocated_files: 2,
                allocated_bytes: 150,
            }]
        );
    }

    #[test]
    fn allocation_is_skipped_when_every_file_has_the_right_size() {
        let mut t = DownloadTranslator::new(0);
        t.on_event(size_ok("a"));
        // an allocation event without failed sizes still flushes the count
        // but has nothing to announce
        assert_eq!(
            t.on_event(allocated("a", 0)),
            vec![P::FileSizeVerification { checked_files: 1 }]
        );
    }

    #[test]
    fn allocation_error_is_reported_once_per_path() {
        let mut t = DownloadTranslator::new(0);
        t.on_event(size_bad("a", 1));
        assert_eq!(
            t.on_event(unresolved("a")),
            vec![
                P::FileSizeVerification { checked_files: 1 },
                P::FileAllocationStarted {
                    total_files: 1,
                    total_bytes: 1,
                },
                P::AllocationError("a".into()),
            ]
        );
        assert!(t.on_event(unresolved("a")).is_empty());
        assert_eq!(
            t.on_event(unresolved("b")),
            vec![P::AllocationError("b".into())]
        );
    }

    #[test]
    fn started_flushes_the_pre_download_stage_and_clears_dirty() {
        let mut t = DownloadTranslator::new(0);
        t.on_event(size_bad("a", 10));
        t.on_event(allocated("a", 10));
        assert_eq!(
            t.on_event(started()),
            vec![P::FileAllocation {
                allocated_files: 1,
                allocated_bytes: 10,
            }]
        );
        assert!(t.on_tick().is_empty());
    }

    #[test]
    fn started_straight_from_size_verification_flushes_that_stage() {
        let mut t = DownloadTranslator::new(0);
        t.on_event(size_ok("a"));
        assert_eq!(
            t.on_event(started()),
            vec![P::FileSizeVerification { checked_files: 1 }]
        );
    }

    #[test]
    fn download_bytes_sum_and_regressions_subtract() {
        let mut t = DownloadTranslator::new(0);
        t.on_event(started());
        t.on_event(bytes(10));
        t.on_event(bytes(20));
        t.on_event(regress(5));
        assert_eq!(
            t.on_tick(),
            vec![P::DownloadProgress {
                downloaded_bytes: 25
            }]
        );
        assert!(t.on_tick().is_empty());
    }

    #[test]
    fn skipped_chunks_count_as_downloaded_so_the_total_is_reached() {
        let mut t = DownloadTranslator::new(100);
        t.on_event(size_ok("a"));
        t.on_event(DownloadStageEvent::DownloadStage(DownloadEvent::Started {
            compressed_total: 30,
        }));
        // the 70 skipped bytes show up on the next tick
        assert_eq!(
            t.on_tick(),
            vec![P::DownloadProgress {
                downloaded_bytes: 70
            }]
        );
        t.on_event(bytes(30));
        assert_eq!(
            t.finish(),
            vec![P::DownloadProgress {
                downloaded_bytes: 100
            }]
        );
    }

    #[test]
    fn a_complete_install_has_no_download_stage() {
        let mut t = DownloadTranslator::new(100);
        t.on_event(size_ok("a"));
        t.on_event(allocated("a", 0));
        t.on_event(DownloadStageEvent::VerificationStage(
            gogdl_lib::VerificationEvent::Verified("a".into(), 5),
        ));
        // no `Started`: the last stage reported is flushed, nothing is invented
        assert_eq!(
            t.finish(),
            vec![P::FileAllocation {
                allocated_files: 1,
                allocated_bytes: 0,
            }]
        );
    }

    #[test]
    fn regression_saturates_at_zero() {
        let mut t = DownloadTranslator::new(0);
        t.on_event(started());
        t.on_event(bytes(3));
        t.on_event(regress(100));
        assert_eq!(
            t.on_tick(),
            vec![P::DownloadProgress {
                downloaded_bytes: 0
            }]
        );
        t.on_event(regress(1));
        assert_eq!(
            t.on_tick(),
            vec![P::DownloadProgress {
                downloaded_bytes: 0
            }]
        );
    }

    #[test]
    fn events_download_ignores_produce_nothing() {
        let mut t = DownloadTranslator::new(0);
        assert!(
            t.on_event(DownloadStageEvent::FileAllocationError())
                .is_empty()
        );
        assert!(
            t.on_event(DownloadStageEvent::VerificationStage(
                gogdl_lib::VerificationEvent::Verified("a".into(), 5)
            ))
            .is_empty()
        );
        assert!(
            t.on_event(DownloadStageEvent::DownloadStage(
                DownloadEvent::Downloading
            ))
            .is_empty()
        );
        assert!(t.on_tick().is_empty());
    }

    #[test]
    fn finish_flushes_the_download_tail_only_when_dirty() {
        let mut t = DownloadTranslator::new(0);
        t.on_event(started());
        assert!(t.finish().is_empty());
        t.on_event(bytes(9));
        assert_eq!(
            t.finish(),
            vec![P::DownloadProgress {
                downloaded_bytes: 9
            }]
        );
        assert!(t.finish().is_empty());
    }
}
