// Partition expansion functionality

use crate::partition::types::*;
use anyhow::{anyhow, Result};
use std::process::Command;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

/// Expand a partition to the specified size
pub async fn expand_partition(
    partition: &PartitionInfo,
    target_size: u64,
) -> Result<()> {
    // Step 1: Expand the partition table entry
    expand_partition_table(partition, target_size).await?;

    // Step 2: Expand the filesystem
    expand_filesystem(partition, target_size).await?;

    Ok(())
}

/// Expand the partition table entry
async fn expand_partition_table(
    partition: &PartitionInfo,
    target_size: u64,
) -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        expand_partition_table_windows(partition, target_size).await
    }

    #[cfg(not(target_os = "windows"))]
    {
        Err(anyhow!("Partition table expansion not yet implemented for this platform"))
    }
}

/// Expand partition table on Windows using diskpart
#[cfg(target_os = "windows")]
async fn expand_partition_table_windows(
    partition: &PartitionInfo,
    target_size: u64,
) -> Result<()> {
    // Extract drive letter
    let drive_letter = partition.mount_point.as_ref()
        .and_then(|m| m.chars().next())
        .ok_or_else(|| anyhow!("No drive letter found for partition"))?;

    let size_mb = target_size / (1024 * 1024);

    // Create diskpart script
    let script = format!(
        "select volume {}\nextend size={}\n",
        drive_letter,
        size_mb
    );

    // Write script to temp file
    let script_path = std::env::temp_dir().join("diskpart_expand.txt");
    std::fs::write(&script_path, script)?;

    // Execute diskpart
    const CREATE_NO_WINDOW: u32 = 0x08000000;

    let output = Command::new("diskpart")
        .arg("/s")
        .arg(&script_path)
        .creation_flags(CREATE_NO_WINDOW)
        .output()?;

    // Clean up temp file
    let _ = std::fs::remove_file(&script_path);

    if !output.status.success() {
        return Err(anyhow!(
            "Diskpart failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(())
}

/// Expand the filesystem to fill the partition
async fn expand_filesystem(
    partition: &PartitionInfo,
    target_size: u64,
) -> Result<()> {
    match partition.filesystem {
        FilesystemType::NTFS => expand_ntfs(partition, target_size).await,
        FilesystemType::Ext2 | FilesystemType::Ext3 | FilesystemType::Ext4 => {
            expand_ext4(partition, target_size).await
        }
        _ => Err(anyhow!(
            "Filesystem expansion not supported for {}",
            partition.filesystem.display_name()
        )),
    }
}

/// Expand NTFS filesystem
async fn expand_ntfs(
    partition: &PartitionInfo,
    _target_size: u64,
) -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        // On Windows, NTFS expansion happens automatically with diskpart extend
        // No additional action needed
        Ok(())
    }

    #[cfg(not(target_os = "windows"))]
    {
        // On Linux/macOS, use ntfsresize
        let device = &partition.device_path;

        let output = Command::new("ntfsresize")
            .arg("--force")
            .arg("--no-action")  // Dry run first
            .arg(device)
            .output()?;

        if !output.status.success() {
            return Err(anyhow!(
                "NTFS dry-run failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        // Actual resize
        let output = Command::new("ntfsresize")
            .arg("--force")
            .arg(device)
            .output()?;

        if !output.status.success() {
            return Err(anyhow!(
                "NTFS resize failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        Ok(())
    }
}

/// Expand ext4 filesystem
async fn expand_ext4(
    partition: &PartitionInfo,
    _target_size: u64,
) -> Result<()> {
    #[cfg(target_os = "linux")]
    {
        let device = &partition.device_path;

        // resize2fs can expand online (while mounted) or offline
        let output = Command::new("resize2fs")
            .arg(device)
            .output()?;

        if !output.status.success() {
            return Err(anyhow!(
                "resize2fs failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        Ok(())
    }

    #[cfg(not(target_os = "linux"))]
    {
        Err(anyhow!("ext4 resize is only supported on Linux"))
    }
}
