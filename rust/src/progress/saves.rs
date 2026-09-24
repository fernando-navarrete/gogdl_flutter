use gogdl_lib::{SavesDownloadEvent, SavesUploadEvent};

use super::Translator;
use crate::api::progress_reporting::{DownloadSavesProgress, UploadSavesProgress};

/// Byte counters for a saves transfer. Files go one at a time, so `Progress`
/// always belongs to the file opened by the last `FileStarted`.
#[derive(Default)]
struct Counters {
    total: u64,
    file: u64,
    dirty: bool,
}

impl Counters {
    fn start_file(&mut self) {
        self.file = 0;
    }

    fn add(&mut self, delta: u64) {
        self.total += delta;
        self.file += delta;
        self.dirty = true;
    }

    /// `(total, file)` if anything changed since the last flush.
    fn take(&mut self) -> Option<(u64, u64)> {
        std::mem::take(&mut self.dirty).then_some((self.total, self.file))
    }
}

#[derive(Default)]
pub(crate) struct SavesDownloadTranslator {
    counters: Counters,
}

impl SavesDownloadTranslator {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    fn flush(&mut self) -> Vec<DownloadSavesProgress> {
        self.counters
            .take()
            .map(
                |(downloaded_bytes, file_downloaded_bytes)| DownloadSavesProgress::Progress {
                    downloaded_bytes,
                    file_downloaded_bytes,
                },
            )
            .into_iter()
            .collect()
    }
}

impl Translator for SavesDownloadTranslator {
    type Event = SavesDownloadEvent;
    type Progress = DownloadSavesProgress;

    fn on_event(&mut self, event: SavesDownloadEvent) -> Vec<DownloadSavesProgress> {
        match event {
            // one-shot signals below flush immediately regardless of ticker
            SavesDownloadEvent::Preparing {
                total_files,
                total_bytes,
            } => vec![DownloadSavesProgress::Started {
                total_files: total_files as u64,
                total_bytes,
            }],
            SavesDownloadEvent::FileStarted {
                name,
                destination,
                total_bytes,
            } => {
                self.counters.start_file();
                vec![DownloadSavesProgress::FileStarted {
                    name,
                    destination: destination.to_string_lossy().into_owned(),
                    total_bytes,
                }]
            }
            SavesDownloadEvent::Progress(delta) => {
                self.counters.add(delta as u64);
                Vec::new()
            }
            SavesDownloadEvent::FileFinished { name } => {
                // final flush of the file that just ended, so it doesn't sit
                // at a stale % once the next one opens
                let mut out = self.flush();
                out.push(DownloadSavesProgress::FileFinished { name });
                out
            }
        }
    }

    fn on_tick(&mut self) -> Vec<DownloadSavesProgress> {
        self.flush()
    }

    fn finish(&mut self) -> Vec<DownloadSavesProgress> {
        self.flush()
    }
}

#[derive(Default)]
pub(crate) struct SavesUploadTranslator {
    counters: Counters,
}

impl SavesUploadTranslator {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    fn flush(&mut self) -> Vec<UploadSavesProgress> {
        self.counters
            .take()
            .map(
                |(uploaded_bytes, file_uploaded_bytes)| UploadSavesProgress::Progress {
                    uploaded_bytes,
                    file_uploaded_bytes,
                },
            )
            .into_iter()
            .collect()
    }
}

impl Translator for SavesUploadTranslator {
    type Event = SavesUploadEvent;
    type Progress = UploadSavesProgress;

    fn on_event(&mut self, event: SavesUploadEvent) -> Vec<UploadSavesProgress> {
        match event {
            // one-shot signals below flush immediately regardless of ticker
            SavesUploadEvent::Preparing { total_files } => vec![UploadSavesProgress::Started {
                total_files: total_files as u64,
            }],
            SavesUploadEvent::FileStarted {
                name,
                source,
                total_bytes,
            } => {
                self.counters.start_file();
                vec![UploadSavesProgress::FileStarted {
                    name,
                    source: source.to_string_lossy().into_owned(),
                    total_bytes,
                }]
            }
            SavesUploadEvent::Progress(delta) => {
                self.counters.add(delta as u64);
                Vec::new()
            }
            SavesUploadEvent::FileFinished { name } => {
                // final flush of the file that just ended, so it doesn't sit
                // at a stale % once the next one opens
                let mut out = self.flush();
                out.push(UploadSavesProgress::FileFinished { name });
                out
            }
        }
    }

    fn on_tick(&mut self) -> Vec<UploadSavesProgress> {
        self.flush()
    }

    fn finish(&mut self) -> Vec<UploadSavesProgress> {
        self.flush()
    }
}
