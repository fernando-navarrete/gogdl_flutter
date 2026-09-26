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
            // a variant added upstream later has nothing to report yet
            _ => Vec::new(),
        }
    }

    fn on_tick(&mut self) -> Vec<ProtonDownloadProgress> {
        self.flush()
    }

    fn finish(&mut self) -> Vec<ProtonDownloadProgress> {
        self.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn started_is_immediate() {
        let mut t = ProtonTranslator::new();
        assert_eq!(
            t.on_event(ProtonDownloadEvent::Downloading { total_bytes: 100 }),
            vec![ProtonDownloadProgress::Started(100)]
        );
    }

    #[test]
    fn progress_sums_and_coalesces() {
        let mut t = ProtonTranslator::new();
        assert!(t.on_event(ProtonDownloadEvent::Progress(10)).is_empty());
        assert!(t.on_event(ProtonDownloadEvent::Progress(5)).is_empty());
        assert_eq!(t.on_tick(), vec![ProtonDownloadProgress::Progress(15)]);
        assert!(t.on_tick().is_empty());
    }

    #[test]
    fn only_the_latest_extracted_path_is_reported_after_progress() {
        let mut t = ProtonTranslator::new();
        t.on_event(ProtonDownloadEvent::Extracted("a".into()));
        t.on_event(ProtonDownloadEvent::Extracted("b".into()));
        assert_eq!(
            t.on_tick(),
            vec![
                ProtonDownloadProgress::Progress(0),
                ProtonDownloadProgress::Extracted("b".into()),
            ]
        );
        assert!(t.on_tick().is_empty());
    }

    #[test]
    fn finish_flushes_the_tail() {
        let mut t = ProtonTranslator::new();
        assert!(t.finish().is_empty());
        t.on_event(ProtonDownloadEvent::Progress(7));
        assert_eq!(t.finish(), vec![ProtonDownloadProgress::Progress(7)]);
    }
}
