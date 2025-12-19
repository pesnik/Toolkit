// Progress tracking for resize operations

use serde::{Deserialize, Serialize};

/// Progress update for a resize operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResizeProgress {
    /// Current operation phase
    pub phase: ResizePhase,

    /// Overall progress percentage (0-100)
    pub percent: f32,

    /// Current status message
    pub message: String,

    /// Whether the operation can be cancelled at this point
    pub can_cancel: bool,
}

/// Phases of a resize operation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ResizePhase {
    /// Validating resize parameters
    Validating,

    /// Checking filesystem integrity
    CheckingFilesystem,

    /// Creating backup/snapshot
    CreatingBackup,

    /// Resizing filesystem (if shrinking)
    ResizingFilesystem,

    /// Updating partition table
    UpdatingPartitionTable,

    /// Expanding filesystem (if growing)
    ExpandingFilesystem,

    /// Verifying result
    Verifying,

    /// Operation complete
    Complete,

    /// Operation failed
    Error,
}

impl ResizeProgress {
    /// Create a new progress update
    pub fn new(phase: ResizePhase, percent: f32, message: String) -> Self {
        let can_cancel = matches!(phase, ResizePhase::Validating | ResizePhase::CheckingFilesystem);

        Self {
            phase,
            percent,
            message,
            can_cancel,
        }
    }

    /// Create a validation progress update
    pub fn validating(message: impl Into<String>) -> Self {
        Self::new(ResizePhase::Validating, 5.0, message.into())
    }

    /// Create a filesystem check progress update
    pub fn checking_filesystem(message: impl Into<String>) -> Self {
        Self::new(ResizePhase::CheckingFilesystem, 15.0, message.into())
    }

    /// Create a backup progress update
    pub fn creating_backup(message: impl Into<String>) -> Self {
        Self::new(ResizePhase::CreatingBackup, 25.0, message.into())
    }

    /// Create a filesystem resize progress update
    pub fn resizing_filesystem(percent: f32, message: impl Into<String>) -> Self {
        Self::new(ResizePhase::ResizingFilesystem, 30.0 + (percent * 0.3), message.into())
    }

    /// Create a partition table update progress
    pub fn updating_partition_table(message: impl Into<String>) -> Self {
        Self::new(ResizePhase::UpdatingPartitionTable, 70.0, message.into())
    }

    /// Create an expand filesystem progress update
    pub fn expanding_filesystem(percent: f32, message: impl Into<String>) -> Self {
        Self::new(ResizePhase::ExpandingFilesystem, 70.0 + (percent * 0.2), message.into())
    }

    /// Create a verification progress update
    pub fn verifying(message: impl Into<String>) -> Self {
        Self::new(ResizePhase::Verifying, 95.0, message.into())
    }

    /// Create a completion progress update
    pub fn complete(message: impl Into<String>) -> Self {
        Self::new(ResizePhase::Complete, 100.0, message.into())
    }

    /// Create an error progress update
    pub fn error(message: impl Into<String>) -> Self {
        Self::new(ResizePhase::Error, 0.0, message.into())
    }
}
