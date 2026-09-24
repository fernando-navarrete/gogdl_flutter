use gogdl_lib::{DownloadEvent, DownloadStageEvent};

use super::Translator;
use super::install::{Install, Stage};
use crate::api::progress_reporting::DownloadGameProgress;

pub(crate) struct DownloadTranslator {
    install: Install,
}

impl DownloadTranslator {
    pub(crate) fn new() -> Self {
        Self {
            install: Install::new(),
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
            // `download_game` has no chunk verification stage.
            DownloadStageEvent::VerificationStage(_) => Vec::new(),
            // `download_future` is about to resolve with
            // `DownloadError::FileAllocationError`, which surfaces to Dart as
            // a stream error; no progress event needed here.
            DownloadStageEvent::FileAllocationError() => Vec::new(),
            DownloadStageEvent::DownloadStage(DownloadEvent::Preparing) => {
                // final flush of whichever pre-transfer stage just ended
                let out = self.install.flush_pre_verification().into_iter().collect();
                self.install.stage = Stage::Download;
                self.install.dirty = false;
                out
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

    fn on_tick(&mut self) -> Vec<DownloadGameProgress> {
        self.install.flush()
    }

    fn finish(&mut self) -> Vec<DownloadGameProgress> {
        self.install.flush()
    }
}
