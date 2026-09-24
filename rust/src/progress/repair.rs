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
