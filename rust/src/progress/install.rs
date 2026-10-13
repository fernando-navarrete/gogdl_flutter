//! Stage handling shared by `download_game` and `repair_game`.
//!
//! Both run their stages strictly in sequence (see `DownloadStageEvent`'s
//! docs), so events never interleave across stages. Download goes size
//! verification, allocation, download; repair inserts a chunk verification
//! stage before the download.

use std::collections::HashSet;

use gogdl_lib::{FileAllocationEvent, FileSizeVerificationEvent};

use crate::api::progress_reporting::{DownloadGameProgress, RepairGameProgress};

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum Stage {
    SizeVerification,
    Allocation,
    Verification,
    Download,
}

/// The progress events the shared stages emit, so one implementation can
/// build either job's enum.
pub(crate) trait InstallProgress {
    fn file_size_verification(checked_files: u64) -> Self;
    fn file_allocation_started(total_files: u64, total_bytes: u64) -> Self;
    fn file_allocation(allocated_files: u64, allocated_bytes: u64) -> Self;
    fn allocation_error(path: String) -> Self;
    fn download_progress(downloaded_bytes: u64) -> Self;
    /// `None` for jobs that have no chunk verification stage.
    fn verification(_checked_bytes: u64) -> Option<Self>
    where
        Self: Sized,
    {
        None
    }
}

impl InstallProgress for DownloadGameProgress {
    fn file_size_verification(checked_files: u64) -> Self {
        Self::FileSizeVerification { checked_files }
    }
    fn file_allocation_started(total_files: u64, total_bytes: u64) -> Self {
        Self::FileAllocationStarted {
            total_files,
            total_bytes,
        }
    }
    fn file_allocation(allocated_files: u64, allocated_bytes: u64) -> Self {
        Self::FileAllocation {
            allocated_files,
            allocated_bytes,
        }
    }
    fn allocation_error(path: String) -> Self {
        Self::AllocationError(path)
    }
    fn download_progress(downloaded_bytes: u64) -> Self {
        Self::DownloadProgress { downloaded_bytes }
    }
}

impl InstallProgress for RepairGameProgress {
    fn file_size_verification(checked_files: u64) -> Self {
        Self::FileSizeVerification { checked_files }
    }
    fn file_allocation_started(total_files: u64, total_bytes: u64) -> Self {
        Self::FileAllocationStarted {
            total_files,
            total_bytes,
        }
    }
    fn file_allocation(allocated_files: u64, allocated_bytes: u64) -> Self {
        Self::FileAllocation {
            allocated_files,
            allocated_bytes,
        }
    }
    fn allocation_error(path: String) -> Self {
        Self::AllocationError(path)
    }
    fn download_progress(downloaded_bytes: u64) -> Self {
        Self::DownloadProgress { downloaded_bytes }
    }
    fn verification(checked_bytes: u64) -> Option<Self> {
        Some(Self::Verification { checked_bytes })
    }
}

pub(crate) struct Install {
    pub(crate) stage: Stage,
    pub(crate) dirty: bool,

    pub(crate) checked_files: u64,

    // Only files that fail size verification go on to allocation, so these
    // totals are discovered incrementally during that stage and only known
    // for certain once it ends.
    allocation_total_files: u64,
    allocation_total_bytes: u64,
    allocated_files: u64,
    allocated_bytes: u64,

    pub(crate) checked_bytes: u64,
    downloaded: u64,

    // Only forward the first allocation failure per path so the UI sees one
    // event per file.
    reported_allocation: HashSet<String>,
}

impl Install {
    pub(crate) fn new() -> Self {
        Self {
            stage: Stage::SizeVerification,
            dirty: false,
            checked_files: 0,
            allocation_total_files: 0,
            allocation_total_bytes: 0,
            allocated_files: 0,
            allocated_bytes: 0,
            checked_bytes: 0,
            downloaded: 0,
            reported_allocation: HashSet::new(),
        }
    }

    pub(crate) fn on_size_verification(&mut self, event: FileSizeVerificationEvent) {
        use FileSizeVerificationEvent as E;
        self.checked_files += 1;
        self.dirty = true;
        match event {
            E::FileSizeVerificationSuccess(_, _) => {}
            E::FileWithNoChunks(_, size)
            | E::CouldNotResolvePath(_, size)
            | E::FileNotFound(_, size)
            | E::FileSizeVerificationFailed(_, size)
            | E::FileSizeMismatch(_, size) => {
                self.allocation_total_files += 1;
                self.allocation_total_bytes += size;
            }
            // a variant added upstream later has nothing to report yet
            _ => {}
        }
    }

    pub(crate) fn on_allocation<P: InstallProgress>(
        &mut self,
        event: FileAllocationEvent,
    ) -> Vec<P> {
        use FileAllocationEvent as E;
        let mut out = Vec::new();
        if self.stage == Stage::SizeVerification {
            // final flush so the size-verification stage doesn't sit at a
            // stale count once allocation starts
            out.push(P::file_size_verification(self.checked_files));
            if self.allocation_total_files > 0 {
                out.push(P::file_allocation_started(
                    self.allocation_total_files,
                    self.allocation_total_bytes,
                ));
            }
            self.stage = Stage::Allocation;
        }
        match event {
            E::FileAllocationSuccess(_, size) => {
                self.allocated_files += 1;
                self.allocated_bytes += size;
                self.dirty = true;
            }
            E::FileWithNoChunks(_, _) => {
                self.allocated_files += 1;
                self.dirty = true;
            }
            E::CouldNotResolvePath(path, _) => {
                self.allocated_files += 1;
                self.dirty = true;
                if self.reported_allocation.insert(path.clone()) {
                    // one-shot signal, flush immediately regardless of ticker
                    out.push(P::allocation_error(path));
                }
            }
            // a variant added upstream later has nothing to report yet
            _ => {}
        }
        out
    }

    /// Starts the download stage with `already` bytes counted as done: the compressed bytes of
    /// chunks that verified and won't be transferred, so the counter still ends at the job's total.
    pub(crate) fn start_download(&mut self, already: u64) {
        self.stage = Stage::Download;
        self.downloaded = already;
        self.dirty = already > 0;
    }

    pub(crate) fn on_download_progress(&mut self, delta: u64) {
        self.downloaded += delta;
        self.dirty = true;
    }

    pub(crate) fn on_download_regression(&mut self, delta: u64) {
        // a delta to subtract, not a replacement value; see `DownloadEvent`'s docs
        self.downloaded = self.downloaded.saturating_sub(delta);
        self.dirty = true;
    }

    /// The event for the current stage's counters, regardless of `dirty`.
    pub(crate) fn current<P: InstallProgress>(&self) -> Option<P> {
        match self.stage {
            Stage::SizeVerification => Some(P::file_size_verification(self.checked_files)),
            Stage::Allocation => Some(P::file_allocation(
                self.allocated_files,
                self.allocated_bytes,
            )),
            Stage::Verification => P::verification(self.checked_bytes),
            Stage::Download => Some(P::download_progress(self.downloaded)),
        }
    }

    /// Flush of a stage that ended before verification or the download
    /// started; nothing for the later stages.
    pub(crate) fn flush_pre_verification<P: InstallProgress>(&self) -> Option<P> {
        match self.stage {
            Stage::SizeVerification | Stage::Allocation => self.current(),
            Stage::Verification | Stage::Download => None,
        }
    }

    /// Coalesced emit for a tick or the final flush: the current stage's
    /// counters, only when something changed.
    pub(crate) fn flush<P: InstallProgress>(&mut self) -> Vec<P> {
        if !std::mem::take(&mut self.dirty) {
            return Vec::new();
        }
        self.current().into_iter().collect()
    }
}
