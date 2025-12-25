// Tauri commands for partition management

use crate::partition::{self, DiskInfo, PartitionInfo, ValidationResult, ResizeProgress, ReallocationPlan};
use tauri::{command, AppHandle, Emitter};

/// Get all disks available on the system
#[command]
pub async fn get_disks() -> Result<Vec<DiskInfo>, String> {
    partition::get_all_disks().map_err(|e| e.to_string())
}

/// Get all partitions for a specific disk
#[command]
pub async fn get_partitions(disk_path: String) -> Result<Vec<PartitionInfo>, String> {
    partition::get_partitions(&disk_path).map_err(|e| e.to_string())
}

/// Get detailed information about a specific partition
#[command]
pub async fn get_partition_info(partition_id: String) -> Result<PartitionInfo, String> {
    partition::get_partition_info(&partition_id).map_err(|e| e.to_string())
}

/// Validate a partition expand request
#[command]
pub async fn validate_expand_partition(
    partition_id: String,
    target_size: u64,
) -> Result<ValidationResult, String> {
    let partition = partition::get_partition_info(&partition_id)
        .map_err(|e| e.to_string())?;

    // Find the disk containing this partition
    let disks = partition::get_all_disks().map_err(|e| e.to_string())?;
    let disk = disks
        .iter()
        .find(|d| d.partitions.iter().any(|p| p.id == partition_id))
        .ok_or_else(|| "Disk not found for partition".to_string())?;

    partition::validation::validate_expand(&partition, disk, target_size)
        .map_err(|e| e.to_string())
}

/// Validate a partition shrink request
#[command]
pub async fn validate_shrink_partition(
    partition_id: String,
    target_size: u64,
) -> Result<ValidationResult, String> {
    let partition = partition::get_partition_info(&partition_id)
        .map_err(|e| e.to_string())?;

    partition::validation::validate_shrink(&partition, target_size)
        .map_err(|e| e.to_string())
}

/// Expand a partition to the specified size
#[command]
pub async fn expand_partition(
    app: AppHandle,
    partition_id: String,
    target_size: u64,
) -> Result<(), String> {
    // Emit progress: Validating
    let _ = app.emit("resize-progress", ResizeProgress::validating("Starting validation..."));

    // Get partition info
    let partition = partition::get_partition_info(&partition_id)
        .map_err(|e| e.to_string())?;

    // Emit progress: Expanding
    let _ = app.emit("resize-progress", ResizeProgress::expanding_filesystem(
        0.0,
        format!("Expanding partition {} to {}...", partition.device_path, format_size(target_size))
    ));

    // Perform expansion
    partition::expand::expand_partition(&partition, target_size)
        .await
        .map_err(|e| e.to_string())?;

    // Emit progress: Complete
    let _ = app.emit("resize-progress", ResizeProgress::complete("Partition expanded successfully!"));

    Ok(())
}

/// Shrink a partition to the specified size
#[command]
pub async fn shrink_partition(
    app: AppHandle,
    partition_id: String,
    target_size: u64,
) -> Result<(), String> {
    // Emit progress: Validating
    let _ = app.emit("resize-progress", ResizeProgress::validating("Starting validation..."));

    // Get partition info
    let partition = partition::get_partition_info(&partition_id)
        .map_err(|e| e.to_string())?;

    // Emit progress: Checking filesystem
    let _ = app.emit("resize-progress", ResizeProgress::checking_filesystem(
        "Checking filesystem integrity..."
    ));

    // Emit progress: Shrinking
    let _ = app.emit("resize-progress", ResizeProgress::resizing_filesystem(
        0.0,
        format!("Shrinking partition {} to {}...", partition.device_path, format_size(target_size))
    ));

    // Perform shrink
    partition::shrink::shrink_partition(&partition, target_size)
        .await
        .map_err(|e| e.to_string())?;

    // Emit progress: Complete
    let _ = app.emit("resize-progress", ResizeProgress::complete("Partition shrunk successfully!"));

    Ok(())
}

/// Create a space reallocation plan
/// This analyzes how to give more space to a partition by shrinking/deleting others
#[command]
pub async fn create_space_reallocation_plan(
    target_partition_id: String,
    desired_additional_space: u64,
) -> Result<ReallocationPlan, String> {
    // Get all disks
    let disks = partition::get_all_disks().map_err(|e| e.to_string())?;

    // Find the disk containing the target partition
    let disk = disks
        .iter()
        .find(|d| d.partitions.iter().any(|p| p.id == target_partition_id))
        .ok_or_else(|| "Disk not found for partition".to_string())?;

    // Create reallocation plan
    partition::reallocation_wizard::create_reallocation_plan(
        disk,
        &target_partition_id,
        desired_additional_space,
    )
    .map_err(|e| e.to_string())
}

/// Unmount a partition
#[command]
pub async fn unmount_partition(partition_id: String) -> Result<(), String> {
    let partition = partition::get_partition_info(&partition_id)
        .map_err(|e| e.to_string())?;

    partition::unmount_partition(&partition)
        .map_err(|e| e.to_string())
}

/// Mount a partition
#[command]
pub async fn mount_partition(partition_id: String) -> Result<(), String> {
    let partition = partition::get_partition_info(&partition_id)
        .map_err(|e| e.to_string())?;

    partition::mount_partition(&partition)
        .map_err(|e| e.to_string())
}

/// Validate that a partition can be safely deleted
#[command]
pub async fn validate_delete_partition(partition_id: String) -> Result<Vec<String>, String> {
    let partition = partition::get_partition_info(&partition_id)
        .map_err(|e| e.to_string())?;

    partition::validate_delete(&partition)
        .map_err(|e| e.to_string())
}

/// Delete a partition
/// WARNING: This destroys all data on the partition!
#[command]
pub async fn delete_partition(partition_id: String) -> Result<(), String> {
    let partition = partition::get_partition_info(&partition_id)
        .map_err(|e| e.to_string())?;

    partition::delete_partition(&partition)
        .map_err(|e| e.to_string())
}

/// Execute partition reorganization (move partitions)
/// Returns instructions for using MiniTool to complete the operation
#[command]
pub async fn execute_partition_moves(
    move_operations: Vec<partition::MoveOperation>,
) -> Result<String, String> {
    let mut instructions = String::from("To safely reorganize your partitions:\n\n");
    instructions.push_str("RECOMMENDED: Use MiniTool Partition Wizard (Free)\n");
    instructions.push_str("https://www.partitionwizard.com/\n\n");
    instructions.push_str("Steps:\n");
    instructions.push_str("1. Download and install MiniTool Partition Wizard\n");
    instructions.push_str("2. Open the program and select your disk\n");

    for (i, op) in move_operations.iter().enumerate() {
        instructions.push_str(&format!(
            "3.{} Drag partition (ID: {}) to the end of the disk\n",
            i + 1,
            &op.partition_id[..8.min(op.partition_id.len())]
        ));
    }

    let est_time = move_operations.len() * 20;
    instructions.push_str(&format!(
        "\n4. Click 'Apply' and wait for completion ({} partition(s) to move)\n",
        move_operations.len()
    ));
    instructions.push_str("5. Once complete, return to this app and click 'Manage Space' on C: to expand it\n\n");
    instructions.push_str("⚠️ IMPORTANT: Backup your data before proceeding!\n");
    instructions.push_str(&format!("⏱️ Estimated time: {} minutes\n", est_time));

    Ok(instructions)
}

/// Format bytes to human-readable size
fn format_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    if bytes == 0 {
        return "0 B".to_string();
    }

    let base = 1024_f64;
    let exp = (bytes as f64).log(base).floor() as usize;
    let exp = exp.min(UNITS.len() - 1);
    let value = bytes as f64 / base.powi(exp as i32);

    format!("{:.2} {}", value, UNITS[exp])
}
