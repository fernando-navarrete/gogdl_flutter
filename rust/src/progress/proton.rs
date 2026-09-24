use gogdl_lib::ProtonDownloadEvent;

use super::Translator;
use crate::api::progress_reporting::ProtonDownloadProgress;

#[derive(Default)]
pub(crate) struct ProtonTranslator {
    downloaded: u64,
    last_extracted: Option<String>,
    dirty: bool,
}

impl ProtonTranslator {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    fn flush(&mut self) -> Vec<ProtonDownloadProgress> {
        if !std::mem::take(&mut self.dirty) {
            return Vec::new();
        }
        let mut out = vec![ProtonDownloadProgress::Progress(self.downloaded)];
        // Only the latest extracted path since the last flush is reported.
        out.extend(
            self.last_extracted
                .take()
                .map(ProtonDownloadProgress::Extracted),
        );
        out
    }
}

impl Translator for ProtonTranslator {
    type Event = ProtonDownloadEvent;
    type Progress = ProtonDownloadProgress;

    fn on_event(&mut self, event: ProtonDownloadEvent) -> Vec<ProtonDownloadProgress> {
        match event {
            // one-shot signal, flush immediately regardless of ticker
            ProtonDownloadEvent::Downloading { total_bytes } => {
                vec![ProtonDownloadProgress::Started(total_bytes)]
            }
            ProtonDownloadEvent::Progress(delta) => {
                self.downloaded += delta as u64;
                self.dirty = true;
                Vec::new()
            }
            ProtonDownloadEvent::Extracted(path) => {
                self.last_extracted = Some(path);
                self.dirty = true;
                Vec::new()
            }
        }
    }

    fn on_tick(&mut self) -> Vec<ProtonDownloadProgress> {
        self.flush()
    }

    fn finish(&mut self) -> Vec<ProtonDownloadProgress> {
        self.flush()
    }
}
