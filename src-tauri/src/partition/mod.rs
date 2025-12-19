// Partition management module
//
// This module provides functionality for reading and manipulating disk partitions.
// It supports multiple partition table formats (MBR, GPT) and filesystems (NTFS, ext4, FAT32).

pub mod types;
pub mod info;
pub mod platform;
pub mod resize;

// Re-export commonly used types
pub use types::*;
pub use info::*;
pub use resize::*;
