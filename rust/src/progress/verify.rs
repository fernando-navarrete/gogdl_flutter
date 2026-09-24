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
        }
    }

    fn on_tick(&mut self) -> Vec<VerifyDownloadProgress> {
        self.flush()
    }

    fn finish(&mut self) -> Vec<VerifyDownloadProgress> {
        self.flush()
    }
}
