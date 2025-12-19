// Partition information reading functionality

use super::types::*;
use anyhow::{anyhow, Result};

#[cfg(target_os = "windows")]
use super::platform::windows;

#[cfg(target_os = "linux")]
use super::platform::linux;

#[cfg(target_os = "macos")]
use super::platform::macos;

/// Get all disks available on the system
pub fn get_all_disks() -> Result<Vec<DiskInfo>> {
    #[cfg(target_os = "windows")]
    {
        windows::get_disks()
    }

    #[cfg(target_os = "linux")]
    {
        linux::get_disks()
    }

    #[cfg(target_os = "macos")]
    {
        macos::get_disks()
    }

    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    {
        Err(anyhow!("Unsupported operating system"))
    }
}

/// Get a specific disk by its device path
pub fn get_disk_by_path(path: &str) -> Result<DiskInfo> {
    let disks = get_all_disks()?;

    disks
        .into_iter()
        .find(|d| d.device_path == path)
        .ok_or_else(|| anyhow!("Disk not found: {}", path))
}

/// Get all partitions for a specific disk
pub fn get_partitions(disk_path: &str) -> Result<Vec<PartitionInfo>> {
    let disk = get_disk_by_path(disk_path)?;
    Ok(disk.partitions)
}

/// Get detailed information about a specific partition
pub fn get_partition_info(partition_id: &str) -> Result<PartitionInfo> {
    let disks = get_all_disks()?;

    for disk in disks {
        if let Some(partition) = disk.partitions.into_iter().find(|p| p.id == partition_id) {
            return Ok(partition);
        }
    }

    Err(anyhow!("Partition not found: {}", partition_id))
}
