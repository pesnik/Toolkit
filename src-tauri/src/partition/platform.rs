// Platform-specific implementations for partition operations

#[cfg(target_os = "windows")]
pub mod windows {
    use super::super::types::*;
    use anyhow::{anyhow, Result};
    use std::collections::HashMap;
    use std::fs;
    use std::path::Path;
    use wmi::{COMLibrary, Variant, WMIConnection};

    /// Get all disks on Windows using WMI
    pub fn get_disks() -> Result<Vec<DiskInfo>> {
        let com_con = COMLibrary::new()?;
        let wmi_con = WMIConnection::new(com_con)?;

        // Query physical disks
        let disks: Vec<HashMap<String, Variant>> = wmi_con
            .raw_query("SELECT * FROM Win32_DiskDrive")
            .map_err(|e| anyhow!("Failed to query disks: {}", e))?;

        let mut result = Vec::new();

        for (index, disk_data) in disks.iter().enumerate() {
            let device_id = get_string_property(disk_data, "DeviceID")
                .unwrap_or_else(|| format!("\\\\.\\PhysicalDrive{}", index));

            let model = get_string_property(disk_data, "Model")
                .unwrap_or_else(|| "Unknown Disk".to_string());

            let size = get_u64_property(disk_data, "Size").unwrap_or(0);

            let serial = get_string_property(disk_data, "SerialNumber");

            // Get partitions for this disk
            let partitions = get_partitions_for_disk(&wmi_con, &device_id, index as u32)?;

            // Determine partition table type
            let table_type = detect_partition_table_type(&device_id);

            let disk_info = DiskInfo {
                id: format!("disk-{}", index),
                device_path: device_id.clone(),
                model,
                total_size: size,
                table_type,
                partitions,
                serial_number: serial,
                status: DiskStatus {
                    is_online: true,
                    has_errors: false,
                    smart_status: None, // TODO: Add SMART status
                },
            };

            result.push(disk_info);
        }

        Ok(result)
    }

    /// Get partitions for a specific disk
    fn get_partitions_for_disk(
        wmi_con: &WMIConnection,
        disk_device_id: &str,
        disk_index: u32,
    ) -> Result<Vec<PartitionInfo>> {
        // Query disk partitions
        let query = format!(
            "SELECT * FROM Win32_DiskPartition WHERE DiskIndex = {}",
            disk_index
        );

        let partitions: Vec<HashMap<String, Variant>> = wmi_con
            .raw_query(&query)
            .map_err(|e| anyhow!("Failed to query partitions: {}", e))?;

        let mut result = Vec::new();

        for partition_data in partitions {
            let partition_number = get_u32_property(&partition_data, "Index").unwrap_or(0) + 1;
            let device_id = get_string_property(&partition_data, "DeviceID")
                .unwrap_or_else(|| format!("Partition {}", partition_number));

            let size = get_u64_property(&partition_data, "Size").unwrap_or(0);
            let start_offset = get_u64_property(&partition_data, "StartingOffset").unwrap_or(0);
            let is_boot = get_bool_property(&partition_data, "BootPartition").unwrap_or(false);
            let is_primary = get_bool_property(&partition_data, "PrimaryPartition").unwrap_or(false);

            // Get associated logical disk (drive letter)
            let (drive_letter, filesystem, used_space, label) =
                get_logical_disk_info(wmi_con, &device_id)?;

            let mut flags = Vec::new();
            if is_boot {
                flags.push(PartitionFlag::Boot);
            }

            let partition_type = if is_primary {
                PartitionType::Primary
            } else {
                PartitionType::Logical
            };

            let partition_info = PartitionInfo {
                id: format!("partition-{}-{}", disk_index, partition_number),
                number: partition_number,
                device_path: drive_letter.clone().unwrap_or(device_id),
                label,
                start_offset,
                total_size: size,
                used_space,
                partition_type,
                filesystem: parse_filesystem_type(&filesystem),
                mount_point: drive_letter.clone(),
                is_mounted: drive_letter.is_some(),
                flags,
            };

            result.push(partition_info);
        }

        Ok(result)
    }

    /// Get logical disk information (drive letter, filesystem, etc.)
    fn get_logical_disk_info(
        wmi_con: &WMIConnection,
        partition_device_id: &str,
    ) -> Result<(Option<String>, String, Option<u64>, Option<String>)> {
        // Query the association between partition and logical disk
        let query = format!(
            "ASSOCIATORS OF {{Win32_DiskPartition.DeviceID='{}'}} WHERE AssocClass = Win32_LogicalDiskToPartition",
            partition_device_id.replace("\\", "\\\\")
        );

        let logical_disks: Vec<HashMap<String, Variant>> = wmi_con
            .raw_query(&query)
            .unwrap_or_default();

        if let Some(logical_disk) = logical_disks.first() {
            let drive_letter = get_string_property(logical_disk, "DeviceID");
            let filesystem = get_string_property(logical_disk, "FileSystem")
                .unwrap_or_else(|| "Unknown".to_string());
            let size = get_u64_property(logical_disk, "Size");
            let free_space = get_u64_property(logical_disk, "FreeSpace");
            let volume_name = get_string_property(logical_disk, "VolumeName");

            let used_space = if let (Some(total), Some(free)) = (size, free_space) {
                Some(total - free)
            } else {
                None
            };

            Ok((drive_letter, filesystem, used_space, volume_name))
        } else {
            Ok((None, "Unknown".to_string(), None, None))
        }
    }

    /// Detect partition table type (MBR or GPT)
    fn detect_partition_table_type(device_path: &str) -> PartitionTableType {
        // Try to read the first sector to detect partition table type
        // For now, use WMI query
        let com_con = match COMLibrary::new() {
            Ok(c) => c,
            Err(_) => return PartitionTableType::Unknown,
        };

        let wmi_con = match WMIConnection::new(com_con) {
            Ok(w) => w,
            Err(_) => return PartitionTableType::Unknown,
        };

        // Extract disk index from device path
        let disk_index: u32 = device_path
            .trim_start_matches("\\\\.\\PhysicalDrive")
            .parse()
            .unwrap_or(0);

        let query = format!(
            "SELECT * FROM Win32_DiskPartition WHERE DiskIndex = {}",
            disk_index
        );

        let partitions: Vec<HashMap<String, Variant>> = wmi_con
            .raw_query(&query)
            .unwrap_or_default();

        // If we have partitions, check the type
        if let Some(partition) = partitions.first() {
            let partition_type = get_string_property(partition, "Type")
                .unwrap_or_default();

            if partition_type.contains("GPT") {
                PartitionTableType::GPT
            } else if partition_type.contains("MBR") || partition_type.contains("Installable File System") {
                PartitionTableType::MBR
            } else {
                PartitionTableType::Unknown
            }
        } else {
            PartitionTableType::Unknown
        }
    }

    /// Parse filesystem type string to enum
    fn parse_filesystem_type(fs_str: &str) -> FilesystemType {
        match fs_str.to_uppercase().as_str() {
            "NTFS" => FilesystemType::NTFS,
            "FAT32" => FilesystemType::FAT32,
            "EXFAT" => FilesystemType::ExFAT,
            "FAT" => FilesystemType::FAT32,
            "RAW" => FilesystemType::RAW,
            "" => FilesystemType::Unknown,
            _ => FilesystemType::Unknown,
        }
    }

    // Helper functions to extract WMI properties

    fn get_string_property(data: &HashMap<String, Variant>, key: &str) -> Option<String> {
        data.get(key).and_then(|v| match v {
            Variant::String(s) => Some(s.clone()),
            _ => None,
        })
    }

    fn get_u64_property(data: &HashMap<String, Variant>, key: &str) -> Option<u64> {
        data.get(key).and_then(|v| match v {
            Variant::UI8(n) => Some(*n as u64),
            Variant::I4(n) => Some(*n as u64),
            Variant::UI4(n) => Some(*n as u64),
            Variant::String(s) => s.parse().ok(),
            _ => None,
        })
    }

    fn get_u32_property(data: &HashMap<String, Variant>, key: &str) -> Option<u32> {
        data.get(key).and_then(|v| match v {
            Variant::UI8(n) => Some(*n as u32),
            Variant::I4(n) => Some(*n as u32),
            Variant::UI4(n) => Some(*n),
            Variant::String(s) => s.parse().ok(),
            _ => None,
        })
    }

    fn get_bool_property(data: &HashMap<String, Variant>, key: &str) -> Option<bool> {
        data.get(key).and_then(|v| match v {
            Variant::Bool(b) => Some(*b),
            _ => None,
        })
    }
}

#[cfg(target_os = "linux")]
pub mod linux {
    use super::super::types::*;
    use anyhow::Result;
    use sysinfo::{Disks};

    pub fn get_disks() -> Result<Vec<DiskInfo>> {
        let mut result = Vec::new();
        let disks = Disks::new_with_refreshed_list();

        for (index, disk) in disks.iter().enumerate() {
            let disk_name = disk.name().to_string_lossy().to_string();

            // Create a basic DiskInfo entry
            // TODO: Enhance with actual partition detection
            let disk_info = DiskInfo {
                id: format!("disk-{}", index),
                device_path: disk_name.clone(),
                model: disk_name,
                total_size: disk.total_space(),
                table_type: PartitionTableType::Unknown,
                partitions: vec![],
                serial_number: None,
                status: DiskStatus {
                    is_online: true,
                    has_errors: false,
                    smart_status: None,
                },
            };

            result.push(disk_info);
        }

        Ok(result)
    }
}

#[cfg(target_os = "macos")]
pub mod macos {
    use super::super::types::*;
    use anyhow::Result;

    pub fn get_disks() -> Result<Vec<DiskInfo>> {
        // TODO: Implement macOS disk detection using diskutil
        // For now, return an empty list
        Ok(vec![])
    }
}
