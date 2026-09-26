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
            // a variant added upstream later has nothing to report yet
            _ => Vec::new(),
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
            // a variant added upstream later has nothing to report yet
            _ => Vec::new(),
        }
    }

    fn on_tick(&mut self) -> Vec<UploadSavesProgress> {
        self.flush()
    }

    fn finish(&mut self) -> Vec<UploadSavesProgress> {
        self.flush()
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    fn dl_progress(total: u64, file: u64) -> DownloadSavesProgress {
        DownloadSavesProgress::Progress {
            downloaded_bytes: total,
            file_downloaded_bytes: file,
        }
    }

    fn ul_progress(total: u64, file: u64) -> UploadSavesProgress {
        UploadSavesProgress::Progress {
            uploaded_bytes: total,
            file_uploaded_bytes: file,
        }
    }

    fn dl_file_started(name: &str) -> SavesDownloadEvent {
        SavesDownloadEvent::FileStarted {
            name: name.into(),
            destination: PathBuf::from("/saves").join(name),
            total_bytes: 10,
        }
    }

    #[test]
    fn download_one_shot_signals_are_immediate() {
        let mut t = SavesDownloadTranslator::new();
        assert_eq!(
            t.on_event(SavesDownloadEvent::Preparing {
                total_files: 2,
                total_bytes: 30,
            }),
            vec![DownloadSavesProgress::Started {
                total_files: 2,
                total_bytes: 30,
            }]
        );
        assert_eq!(
            t.on_event(dl_file_started("a")),
            vec![DownloadSavesProgress::FileStarted {
                name: "a".into(),
                destination: "/saves/a".into(),
                total_bytes: 10,
            }]
        );
    }

    #[test]
    fn download_file_counter_resets_but_total_keeps_going() {
        let mut t = SavesDownloadTranslator::new();
        t.on_event(dl_file_started("a"));
        t.on_event(SavesDownloadEvent::Progress(4));
        t.on_event(SavesDownloadEvent::Progress(6));
        assert_eq!(t.on_tick(), vec![dl_progress(10, 10)]);
        t.on_event(dl_file_started("b"));
        t.on_event(SavesDownloadEvent::Progress(3));
        assert_eq!(t.on_tick(), vec![dl_progress(13, 3)]);
        assert!(t.on_tick().is_empty());
    }

    #[test]
    fn download_file_finished_flushes_pending_progress_first() {
        let mut t = SavesDownloadTranslator::new();
        t.on_event(dl_file_started("a"));
        t.on_event(SavesDownloadEvent::Progress(10));
        assert_eq!(
            t.on_event(SavesDownloadEvent::FileFinished { name: "a".into() }),
            vec![
                dl_progress(10, 10),
                DownloadSavesProgress::FileFinished { name: "a".into() },
            ]
        );
        // nothing pending: only the finish event
        assert_eq!(
            t.on_event(SavesDownloadEvent::FileFinished { name: "a".into() }),
            vec![DownloadSavesProgress::FileFinished { name: "a".into() }]
        );
    }

    #[test]
    fn download_finish_flushes_only_when_dirty() {
        let mut t = SavesDownloadTranslator::new();
        assert!(t.finish().is_empty());
        t.on_event(SavesDownloadEvent::Progress(2));
        assert_eq!(t.finish(), vec![dl_progress(2, 2)]);
    }

    #[test]
    fn upload_one_shot_signals_are_immediate() {
        let mut t = SavesUploadTranslator::new();
        assert_eq!(
            t.on_event(SavesUploadEvent::Preparing { total_files: 3 }),
            vec![UploadSavesProgress::Started { total_files: 3 }]
        );
        assert_eq!(
            t.on_event(SavesUploadEvent::FileStarted {
                name: "a".into(),
                source: PathBuf::from("/saves/a"),
                total_bytes: 10,
            }),
            vec![UploadSavesProgress::FileStarted {
                name: "a".into(),
                source: "/saves/a".into(),
                total_bytes: 10,
            }]
        );
    }

    #[test]
    fn upload_counters_and_file_finished_flush() {
        let mut t = SavesUploadTranslator::new();
        t.on_event(SavesUploadEvent::Progress(4));
        assert_eq!(t.on_tick(), vec![ul_progress(4, 4)]);
        t.on_event(SavesUploadEvent::FileStarted {
            name: "b".into(),
            source: PathBuf::from("/saves/b"),
            total_bytes: 5,
        });
        t.on_event(SavesUploadEvent::Progress(1));
        assert_eq!(
            t.on_event(SavesUploadEvent::FileFinished { name: "b".into() }),
            vec![
                ul_progress(5, 1),
                UploadSavesProgress::FileFinished { name: "b".into() },
            ]
        );
        assert!(t.finish().is_empty());
    }
}
