// Space Reallocation Wizard
//
// This module provides a guided workflow for reallocating space between partitions
// without the complexity of full partition moving.
//
// Workflow: C: is full, E: has free space
// 1. Shrink E: to minimum safe size (frees up space at end of E:)
// 2. User backs up E:'s data
// 3. Delete E: entirely
// 4. Expand C: into the freed space
// 5. Optionally recreate E: at the end with remaining space

use crate::partition::types::*;
use anyhow::{anyhow, Result};

/// Plan for reallocating space from one partition to another
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReallocationPlan {
    /// The partition that needs more space (e.g., C:)
    pub target_partition_id: String,

    /// The partition(s) that will be shrunk/deleted to free space
    pub source_partitions: Vec<SourcePartitionPlan>,

    /// Total space that will be freed (bytes)
    pub total_space_freed: u64,

    /// New size for target partition after reallocation (bytes)
    pub target_new_size: u64,

    /// Steps the user must follow
    pub steps: Vec<ReallocationStep>,

    /// Warnings about this operation
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SourcePartitionPlan {
    pub partition_id: String,
    pub partition_label: String,
    pub current_size: u64,
    pub used_space: Option<u64>,
    pub action: SourcePartitionAction,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum SourcePartitionAction {
    /// Shrink partition to this size, then delete
    ShrinkAndDelete { shrink_to: u64 },
    /// Delete partition entirely
    DeleteEntirely,
    /// Keep partition but shrink it
    ShrinkOnly { new_size: u64 },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReallocationStep {
    pub step_number: usize,
    pub title: String,
    pub description: String,
    pub action_type: StepActionType,
    pub can_automate: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum StepActionType {
    UserManual,       // User must do this manually
    AppAutomated,     // App can do this automatically
    AppAssistedManual, // App guides but user confirms each action
}

/// Analyze disk layout and create a space reallocation plan
pub fn create_reallocation_plan(
    disk: &DiskInfo,
    target_partition_id: &str,
    desired_additional_space: u64,
) -> Result<ReallocationPlan> {
    // Find the target partition (e.g., C:)
    let target_partition = disk
        .partitions
        .iter()
        .find(|p| p.id == target_partition_id)
        .ok_or_else(|| anyhow!("Target partition not found"))?;

    // Find partitions that are blocking expansion (between target and free space)
    let target_end = target_partition.start_offset + target_partition.total_size;

    // Get all partitions after the target, sorted by offset
    let mut partitions_after: Vec<_> = disk
        .partitions
        .iter()
        .filter(|p| p.start_offset >= target_end)
        .collect();
    partitions_after.sort_by_key(|p| p.start_offset);

    if partitions_after.is_empty() {
        // Target partition is at the end of disk
        let available_space = disk.total_size.saturating_sub(target_end);
        if available_space < desired_additional_space {
            return Err(anyhow!(
                "Not enough free space at end of disk. Available: {} bytes, Requested: {} bytes",
                available_space,
                desired_additional_space
            ));
        }

        // Simple case: just expand into unallocated space
        return Ok(ReallocationPlan {
            target_partition_id: target_partition_id.to_string(),
            source_partitions: vec![],
            total_space_freed: available_space,
            target_new_size: target_partition.total_size + desired_additional_space,
            steps: vec![
                ReallocationStep {
                    step_number: 1,
                    title: "Expand partition".to_string(),
                    description: format!(
                        "Expand {} from {} to {}",
                        target_partition.device_path,
                        format_bytes(target_partition.total_size),
                        format_bytes(target_partition.total_size + desired_additional_space)
                    ),
                    action_type: StepActionType::AppAutomated,
                    can_automate: true,
                },
            ],
            warnings: vec![],
        });
    }

    // Complex case: need to deal with partitions in the way
    let mut source_partitions = Vec::new();
    let mut total_freed = 0u64;
    let mut warnings = Vec::new();

    // Strategy: Delete partitions until we have enough space
    for partition in &partitions_after {
        if total_freed >= desired_additional_space {
            break;
        }

        let has_data = partition.used_space.map(|used| used > 0).unwrap_or(false);

        if has_data {
            warnings.push(format!(
                "Partition {} ({}) contains {} of data. YOU MUST BACKUP THIS DATA before proceeding!",
                partition.device_path,
                partition.label.as_ref().unwrap_or(&"Unlabeled".to_string()),
                format_bytes(partition.used_space.unwrap_or(0))
            ));
        }

        source_partitions.push(SourcePartitionPlan {
            partition_id: partition.id.clone(),
            partition_label: partition.label.clone().unwrap_or_else(|| partition.device_path.clone()),
            current_size: partition.total_size,
            used_space: partition.used_space,
            action: SourcePartitionAction::DeleteEntirely,
        });

        total_freed += partition.total_size;
    }

    if total_freed < desired_additional_space {
        return Err(anyhow!(
            "Cannot free enough space. Need {} bytes, can free {} bytes by deleting {} partition(s)",
            desired_additional_space,
            total_freed,
            source_partitions.len()
        ));
    }

    // Build step-by-step plan
    let mut steps = vec![];
    let mut step_num = 1;

    // Warning step
    if !warnings.is_empty() {
        steps.push(ReallocationStep {
            step_number: step_num,
            title: "⚠️ BACKUP YOUR DATA".to_string(),
            description: format!(
                "The following partitions will be deleted: {}. Back up any important data NOW!",
                source_partitions
                    .iter()
                    .map(|p| p.partition_label.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            action_type: StepActionType::UserManual,
            can_automate: false,
        });
        step_num += 1;
    }

    // Delete partitions
    for source in &source_partitions {
        steps.push(ReallocationStep {
            step_number: step_num,
            title: format!("Delete partition {}", source.partition_label),
            description: format!(
                "Delete {} (frees {} of space)",
                source.partition_label,
                format_bytes(source.current_size)
            ),
            action_type: StepActionType::AppAssistedManual,
            can_automate: true,
        });
        step_num += 1;
    }

    // Expand target partition
    steps.push(ReallocationStep {
        step_number: step_num,
        title: format!("Expand {}", target_partition.device_path),
        description: format!(
            "Expand {} from {} to {} (+{})",
            target_partition.device_path,
            format_bytes(target_partition.total_size),
            format_bytes(target_partition.total_size + desired_additional_space),
            format_bytes(desired_additional_space)
        ),
        action_type: StepActionType::AppAutomated,
        can_automate: true,
    });

    Ok(ReallocationPlan {
        target_partition_id: target_partition_id.to_string(),
        source_partitions,
        total_space_freed: total_freed,
        target_new_size: target_partition.total_size + desired_additional_space,
        steps,
        warnings,
    })
}

/// Format bytes to human-readable string
fn format_bytes(bytes: u64) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_reallocation_plan() {
        // Test case: C: is full, E: can be deleted
        // [C: 50GB FULL] [E: 20GB empty] [F: 30GB]
        // Want to give C: 15GB more space

        let disk = DiskInfo {
            id: "disk-0".to_string(),
            device_path: "\\\\.\\PhysicalDrive0".to_string(),
            model: "Test Disk".to_string(),
            total_size: 100 * 1024 * 1024 * 1024, // 100GB
            table_type: PartitionTableType::GPT,
            partitions: vec![
                PartitionInfo {
                    id: "part-c".to_string(),
                    number: 1,
                    device_path: "C:".to_string(),
                    label: Some("System".to_string()),
                    start_offset: 1024 * 1024,
                    total_size: 50 * 1024 * 1024 * 1024, // 50GB
                    used_space: Some(50 * 1024 * 1024 * 1024), // FULL
                    partition_type: PartitionType::Primary,
                    filesystem: FilesystemType::NTFS,
                    mount_point: Some("C:".to_string()),
                    is_mounted: true,
                    flags: vec![PartitionFlag::Boot, PartitionFlag::System],
                },
                PartitionInfo {
                    id: "part-e".to_string(),
                    number: 2,
                    device_path: "E:".to_string(),
                    label: Some("Data".to_string()),
                    start_offset: 50 * 1024 * 1024 * 1024 + 1024 * 1024,
                    total_size: 20 * 1024 * 1024 * 1024, // 20GB
                    used_space: Some(1 * 1024 * 1024 * 1024), // 1GB used
                    partition_type: PartitionType::Primary,
                    filesystem: FilesystemType::NTFS,
                    mount_point: Some("E:".to_string()),
                    is_mounted: true,
                    flags: vec![],
                },
            ],
            serial_number: None,
            status: DiskStatus {
                is_online: true,
                has_errors: false,
                smart_status: None,
            },
        };

        let plan = create_reallocation_plan(&disk, "part-c", 15 * 1024 * 1024 * 1024).unwrap();

        assert_eq!(plan.source_partitions.len(), 1);
        assert_eq!(plan.source_partitions[0].partition_id, "part-e");
        assert!(plan.warnings.len() > 0); // Should warn about data on E:
        assert!(plan.steps.len() >= 3); // Backup warning + delete + expand
    }
}
