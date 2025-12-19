// Type definitions for partition management

use serde::{Deserialize, Serialize};

/// Represents a physical disk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskInfo {
    /// Unique identifier for the disk
    pub id: String,

    /// Device path (e.g., "/dev/sda" on Linux, "\\\\.\\PhysicalDrive0" on Windows)
    pub device_path: String,

    /// Disk model name
    pub model: String,

    /// Total size in bytes
    pub total_size: u64,

    /// Partition table type
    pub table_type: PartitionTableType,

    /// List of partitions on this disk
    pub partitions: Vec<PartitionInfo>,

    /// Disk serial number (if available)
    pub serial_number: Option<String>,

    /// Health status
    pub status: DiskStatus,
}

/// Represents a partition on a disk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartitionInfo {
    /// Unique identifier for the partition
    pub id: String,

    /// Partition number (1-based)
    pub number: u32,

    /// Device path (e.g., "/dev/sda1", "D:")
    pub device_path: String,

    /// Partition label/name (if any)
    pub label: Option<String>,

    /// Start offset in bytes
    pub start_offset: u64,

    /// Total size in bytes
    pub total_size: u64,

    /// Used space in bytes (if available)
    pub used_space: Option<u64>,

    /// Partition type
    pub partition_type: PartitionType,

    /// Filesystem type
    pub filesystem: FilesystemType,

    /// Mount point (e.g., "/" on Linux, "C:\" on Windows)
    pub mount_point: Option<String>,

    /// Whether the partition is mounted
    pub is_mounted: bool,

    /// Partition flags
    pub flags: Vec<PartitionFlag>,
}

/// Type of partition table
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PartitionTableType {
    /// Master Boot Record (legacy, max 2TB)
    MBR,

    /// GUID Partition Table (modern, supports >2TB)
    GPT,

    /// Unknown or unsupported
    Unknown,
}

/// Type of partition
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PartitionType {
    /// Primary partition (MBR)
    Primary,

    /// Extended partition (MBR)
    Extended,

    /// Logical partition (MBR, inside extended)
    Logical,

    /// GPT partition (all partitions in GPT are "primary-like")
    Normal,

    /// Unknown type
    Unknown,
}

/// Filesystem type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FilesystemType {
    /// NTFS (Windows)
    NTFS,

    /// ext2 filesystem (Linux)
    Ext2,

    /// ext3 filesystem (Linux)
    Ext3,

    /// ext4 filesystem (Linux)
    Ext4,

    /// FAT32 filesystem
    FAT32,

    /// exFAT filesystem
    ExFAT,

    /// APFS (macOS)
    APFS,

    /// HFS+ (older macOS)
    HFSPlus,

    /// Unformatted/RAW
    RAW,

    /// Unknown filesystem
    Unknown,
}

/// Partition flags
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PartitionFlag {
    /// Boot/Active partition
    Boot,

    /// Hidden partition
    Hidden,

    /// System partition (ESP)
    System,

    /// Read-only
    ReadOnly,
}

/// Disk health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskStatus {
    /// Whether the disk is online and accessible
    pub is_online: bool,

    /// Whether the disk has any errors
    pub has_errors: bool,

    /// SMART status (if available)
    pub smart_status: Option<SmartStatus>,
}

/// SMART status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartStatus {
    /// Overall health assessment
    pub health: HealthStatus,

    /// Temperature in Celsius (if available)
    pub temperature: Option<f32>,

    /// Power-on hours (if available)
    pub power_on_hours: Option<u64>,
}

/// Health status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthStatus {
    /// Healthy, no issues
    Good,

    /// Warning, some issues detected
    Warning,

    /// Critical, imminent failure
    Critical,

    /// Unknown status
    Unknown,
}

impl FilesystemType {
    /// Get a human-readable name for the filesystem
    pub fn display_name(&self) -> &'static str {
        match self {
            FilesystemType::NTFS => "NTFS",
            FilesystemType::Ext2 => "ext2",
            FilesystemType::Ext3 => "ext3",
            FilesystemType::Ext4 => "ext4",
            FilesystemType::FAT32 => "FAT32",
            FilesystemType::ExFAT => "exFAT",
            FilesystemType::APFS => "APFS",
            FilesystemType::HFSPlus => "HFS+",
            FilesystemType::RAW => "Unformatted",
            FilesystemType::Unknown => "Unknown",
        }
    }

    /// Check if this filesystem supports resize operations
    pub fn supports_resize(&self) -> bool {
        matches!(self, FilesystemType::NTFS | FilesystemType::Ext2 | FilesystemType::Ext3 | FilesystemType::Ext4)
    }
}

impl PartitionTableType {
    /// Get a human-readable name for the partition table type
    pub fn display_name(&self) -> &'static str {
        match self {
            PartitionTableType::MBR => "MBR",
            PartitionTableType::GPT => "GPT",
            PartitionTableType::Unknown => "Unknown",
        }
    }
}
