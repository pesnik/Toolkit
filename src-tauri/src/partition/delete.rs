// Partition delete operations
//
// This module implements safe partition deletion with platform-specific implementations.
// DANGEROUS: Deleting partitions destroys all data - use with extreme caution!

use crate::partition::types::*;
use anyhow::{anyhow, Result};
use std::process::Command;

/// Delete a partition (platform-specific)
/// WARNING: This will destroy all data on the partition!
#[cfg(target_os = "windows")]
pub fn delete_partition(partition: &PartitionInfo) -> Result<()> {
    delete_windows(partition)
}

#[cfg(target_os = "macos")]
pub fn delete_partition(partition: &PartitionInfo) -> Result<()> {
    delete_macos(partition)
}

#[cfg(target_os = "linux")]
pub fn delete_partition(partition: &PartitionInfo) -> Result<()> {
    delete_linux(partition)
}

/// Windows partition deletion using diskpart
#[cfg(target_os = "windows")]
fn delete_windows(partition: &PartitionInfo) -> Result<()> {
    use std::fs;
    use std::io::Write;

    // Get drive letter or use partition number
    let delete_command = if let Some(mount_point) = &partition.mount_point {
        // If partition is mounted, select by volume letter
        let drive_letter = mount_point.chars().next()
            .ok_or_else(|| anyhow!("Invalid mount point format"))?;
        format!("select volume {}\ndelete volume\n", drive_letter)
    } else {
        // If unmounted, we need to select by disk and partition number
        // Parse device_path to get disk number and partition number
        // Example: \\?\Volume{GUID}\ or we use WMI data

        // For now, require the partition to be mounted or provide better identification
        return Err(anyhow!(
            "Cannot delete unmounted partition on Windows. Please assign a drive letter first or use Disk Management."
        ));
    };

    let script_path = std::env::temp_dir().join("delete_partition.txt");
    let mut file = fs::File::create(&script_path)?;
    file.write_all(delete_command.as_bytes())?;
    drop(file);

    // Execute diskpart
    let output = Command::new("diskpart")
        .arg("/s")
        .arg(&script_path)
        .output()?;

    // Clean up script file
    let _ = fs::remove_file(&script_path);

    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!(
            "Diskpart delete failed.\nStdout: {}\nStderr: {}",
            stdout,
            stderr
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    if stdout.contains("successfully") || stdout.contains("deleted") || stdout.contains("removed") {
        Ok(())
    } else if stdout.contains("error") || stdout.contains("failed") {
        Err(anyhow!("Delete operation failed. Output: {}", stdout))
    } else {
        // Even if we're not sure, if status.success() we'll accept it
        Ok(())
    }
}

/// macOS partition deletion using diskutil
#[cfg(target_os = "macos")]
fn delete_macos(partition: &PartitionInfo) -> Result<()> {
    let output = Command::new("diskutil")
        .arg("eraseVolume")
        .arg("free")
        .arg("free")
        .arg(&partition.device_path)
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("diskutil erase failed: {}", error));
    }

    Ok(())
}

/// Linux partition deletion using parted
#[cfg(target_os = "linux")]
fn delete_linux(partition: &PartitionInfo) -> Result<()> {
    // Extract partition number from device path (e.g., /dev/sda1 -> 1)
    let partition_num = partition.device_path
        .chars()
        .rev()
        .take_while(|c| c.is_numeric())
        .collect::<String>()
        .chars()
        .rev()
        .collect::<String>();

    // Get disk device (e.g., /dev/sda1 -> /dev/sda)
    let disk_device = partition.device_path
        .trim_end_matches(&partition_num);

    let output = Command::new("parted")
        .arg(disk_device)
        .arg("--script")
        .arg("rm")
        .arg(&partition_num)
        .output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("parted delete failed: {}", error));
    }

    Ok(())
}

/// Validate that a partition can be safely deleted
pub fn validate_delete(partition: &PartitionInfo) -> Result<Vec<String>> {
    let mut warnings = Vec::new();

    // Check if it's a system/boot partition
    if partition.flags.contains(&PartitionFlag::Boot) {
        warnings.push("⚠️ CRITICAL: This is a BOOT partition! Deleting it will make your system UNBOOTABLE!".to_string());
    }

    if partition.flags.contains(&PartitionFlag::System) {
        warnings.push("⚠️ CRITICAL: This is a SYSTEM partition! Deleting it may make your system UNBOOTABLE!".to_string());
    }

    if partition.flags.contains(&PartitionFlag::EFI) {
        warnings.push("⚠️ CRITICAL: This is an EFI partition! Deleting it will make your system UNBOOTABLE!".to_string());
    }

    // Check if partition has data
    if let Some(used_space) = partition.used_space {
        if used_space > 0 {
            let gb = used_space as f64 / (1024.0 * 1024.0 * 1024.0);
            warnings.push(format!(
                "⚠️ This partition contains {:.2} GB of data. ALL DATA WILL BE LOST!",
                gb
            ));
        }
    }

    // Check if mounted
    if partition.is_mounted {
        if let Some(mount) = &partition.mount_point {
            warnings.push(format!(
                "⚠️ Partition is currently mounted at {}. It will be unmounted during deletion.",
                mount
            ));
        }
    }

    Ok(warnings)
}
