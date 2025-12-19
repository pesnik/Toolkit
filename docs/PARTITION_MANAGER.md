# Partition Manager - Implementation Plan

> **Feature**: Custom Partition Resize Implementation for IT Toolkit
>
> **Status**: Planning Phase
>
> **Platforms**: Windows (Primary), Linux (Secondary), macOS (Future)

---

## Table of Contents

1. [Overview](#overview)
2. [Requirements](#requirements)
3. [Technical Architecture](#technical-architecture)
4. [Implementation Roadmap](#implementation-roadmap)
5. [Safety & Risk Management](#safety--risk-management)
6. [Testing Strategy](#testing-strategy)
7. [Todo Checklist](#todo-checklist)

---

## Overview

The Partition Manager feature enables IT support teams to safely resize disk partitions without data loss. This is a critical tool for disk management, allowing expansion and shrinking of partitions across different filesystems.

### Target Filesystems

| Filesystem | Platform | Priority | Status |
|------------|----------|----------|--------|
| NTFS | Windows | High | Planned |
| ext2/3/4 | Linux | High | Planned |
| FAT32 | Cross-platform | Medium | Future |
| APFS | macOS | Low | Future |

---

## Requirements

### Functional Requirements

#### Core Operations

1. **Partition Information Display** (Read-Only)
   - List all disks and partitions
   - Show partition type (Primary, Logical, Extended)
   - Display filesystem type
   - Show total size, used space, free space
   - Display partition table format (MBR/GPT)
   - Show mount points and drive letters

2. **Resize Partition** (Primary Requirement)
   - Expand partition into adjacent unallocated space
   - Shrink partition while preserving data
   - Support both online and offline resize (where applicable)
   - Real-time progress monitoring
   - Automatic filesystem resize after partition resize

3. **Safety Features**
   - Pre-flight validation checks
   - Backup verification requirement
   - Dry-run mode (preview changes)
   - Rollback capability on failure
   - Comprehensive error reporting
   - Audit logging

#### Nice-to-Have Operations (Phase 2+)

4. **Create/Delete Partitions**
   - Create new partition in unallocated space
   - Delete existing partition
   - Format partition with specified filesystem

5. **Partition Management**
   - Change drive letter (Windows)
   - Set active partition
   - View partition properties

### Non-Functional Requirements

1. **Safety**
   - No data loss under normal operation
   - Multiple confirmation layers for destructive operations
   - Automatic integrity checks before and after operations
   - Transaction-based operations with rollback support

2. **Performance**
   - Progress updates at least every 2 seconds
   - Responsive UI during long operations
   - Ability to cancel operations (where safe)

3. **Usability**
   - Clear error messages with actionable guidance
   - Visual representation of disk layout
   - Intuitive resize interface with drag-and-drop
   - Platform-specific best practices guidance

4. **Reliability**
   - Handle unexpected errors gracefully
   - Comprehensive logging for troubleshooting
   - Recovery from partial operations
   - SMART data monitoring during operations

---

## Technical Architecture

### System Architecture

```
┌─────────────────────────────────────────────────────────┐
│                   Frontend (React/Next.js)              │
│  ┌────────────────────────────────────────────────────┐ │
│  │          Partition Manager UI Component            │ │
│  │  - Disk visualization                              │ │
│  │  - Resize controls                                 │ │
│  │  - Progress monitoring                             │ │
│  │  - Safety confirmations                            │ │
│  └───────────────────┬────────────────────────────────┘ │
└────────────────────────┼────────────────────────────────┘
                         │ Tauri IPC
┌────────────────────────┼────────────────────────────────┐
│                   Backend (Rust/Tauri)                  │
│  ┌────────────────────┴───────────────────────────────┐ │
│  │       Resize Orchestration Engine (Rust)           │ │
│  │                                                     │ │
│  │  Components:                                        │ │
│  │  - Pre-flight validator                            │ │
│  │  - Size calculator                                 │ │
│  │  - Transaction manager                             │ │
│  │  - Progress tracker                                │ │
│  │  - Audit logger                                    │ │
│  └────────────────────┬───────────────────────────────┘ │
│                       │                                  │
│  ┌────────────────────┴───────────────────────────────┐ │
│  │      Platform Abstraction Layer                    │ │
│  │                                                     │ │
│  │  - Windows: IOCTL + ntfsresize wrapper             │ │
│  │  - Linux: libparted + resize2fs wrapper            │ │
│  │  - macOS: diskutil wrapper                         │ │
│  └─────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────┘
```

### Tech Stack

#### Rust Dependencies

```toml
[dependencies]
# Core (already present)
tauri = "2.9.5"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"
tokio = { version = "1", features = ["full"] }
log = "0.4"

# Partition management
libparted = "0.4"        # Linux partition operations
gptman = "1.0"           # GPT partition table
mbrman = "0.5"           # MBR partition table
sysinfo = "0.30"         # System information (already present)

# Error handling
thiserror = "2.0"        # Custom error types

# Logging
env_logger = "0.11"      # Environment-based logging
chrono = "0.4"           # Timestamps (already present)
```

#### Frontend Dependencies

```json
{
  "dependencies": {
    "@fluentui/react-components": "^9.72.8",  // Already present
    "@fluentui/react-icons": "^2.0.316",      // Already present
    "react": "19.2.1",                        // Already present
    "react-dom": "19.2.1"                     // Already present
  }
}
```

### Core Algorithm: Resize Operation

```rust
pub struct ResizeOperation {
    partition_id: String,
    device_path: String,
    current_size: u64,
    target_size: u64,
    filesystem_type: FilesystemType,
}

impl ResizeOperation {
    pub async fn execute(&self) -> Result<ResizeResult> {
        // Phase 1: Pre-flight Safety Checks (5%)
        self.validate_preconditions().await?;
        emit_progress(5, "Validation complete");

        // Phase 2: Calculate Safe Size (10%)
        let safe_size = self.calculate_safe_resize_size().await?;
        emit_progress(10, "Size calculation complete");

        // Phase 3: Backup Verification (15%)
        self.require_backup_confirmation().await?;
        emit_progress(15, "Backup confirmed");

        // Phase 4: Create Transaction Snapshot (20%)
        let transaction = Transaction::begin(self)?;
        emit_progress(20, "Transaction started");

        // Phase 5: Filesystem Resize - Step 1 (30-60%)
        if self.is_shrinking() {
            // CRITICAL: Resize filesystem BEFORE partition when shrinking
            self.resize_filesystem(safe_size).await
                .map_err(|e| transaction.rollback())?;
            emit_progress(60, "Filesystem shrunk");
        }

        // Phase 6: Partition Table Update (60-70%)
        self.resize_partition_table(safe_size).await
            .map_err(|e| transaction.rollback())?;
        emit_progress(70, "Partition table updated");

        // Phase 7: Filesystem Resize - Step 2 (70-90%)
        if self.is_growing() {
            // CRITICAL: Resize filesystem AFTER partition when growing
            self.resize_filesystem(safe_size).await
                .map_err(|e| transaction.rollback())?;
            emit_progress(90, "Filesystem expanded");
        }

        // Phase 8: Verification (90-95%)
        self.verify_integrity().await
            .map_err(|e| transaction.rollback())?;
        emit_progress(95, "Integrity verified");

        // Phase 9: Commit Transaction (100%)
        transaction.commit()?;
        emit_progress(100, "Resize complete");

        Ok(ResizeResult::success())
    }
}
```

### Platform-Specific Implementations

#### Windows (NTFS)

**Tool**: `ntfsresize` from ntfs-3g package

**Implementation**:

```rust
#[cfg(target_os = "windows")]
pub async fn resize_ntfs(device: &str, new_size: u64) -> Result<()> {
    use std::process::Command;

    // Step 1: Dry run to validate
    let dry_run = Command::new("ntfsresize")
        .arg("--no-action")
        .arg("--force")
        .arg("--size")
        .arg(format!("{}M", new_size / 1024 / 1024))
        .arg(device)
        .output()
        .await?;

    if !dry_run.status.success() {
        return Err(anyhow!("Dry run failed: {}",
            String::from_utf8_lossy(&dry_run.stderr)));
    }

    // Step 2: Actual resize
    let result = Command::new("ntfsresize")
        .arg("--force")
        .arg("--size")
        .arg(format!("{}M", new_size / 1024 / 1024))
        .arg(device)
        .output()
        .await?;

    if !result.status.success() {
        return Err(anyhow!("Resize failed: {}",
            String::from_utf8_lossy(&result.stderr)));
    }

    // Step 3: Verify with chkdsk
    verify_ntfs(device).await?;

    Ok(())
}

async fn verify_ntfs(device: &str) -> Result<()> {
    let output = Command::new("chkdsk")
        .arg(device)
        .arg("/F")
        .output()
        .await?;

    if !output.status.success() {
        return Err(anyhow!("NTFS verification failed"));
    }

    Ok(())
}
```

#### Linux (ext4)

**Tool**: `resize2fs` from e2fsprogs

**Implementation**:

```rust
#[cfg(target_os = "linux")]
pub async fn resize_ext4(
    device: &str,
    new_size: u64,
    is_mounted: bool
) -> Result<()> {
    use std::process::Command;

    // Growing can be done online (mounted)
    if self.is_growing() && is_mounted {
        let result = Command::new("resize2fs")
            .arg(device)
            .arg(format!("{}K", new_size / 1024))
            .output()
            .await?;

        if !result.status.success() {
            return Err(anyhow!("Online resize failed"));
        }

        return Ok(());
    }

    // Shrinking requires unmount + fsck first
    if !is_mounted {
        unmount_filesystem(device).await?;
    }

    // Step 1: Force filesystem check
    let fsck = Command::new("e2fsck")
        .arg("-f")
        .arg("-y")
        .arg(device)
        .output()
        .await?;

    if !fsck.status.success() {
        return Err(anyhow!("Filesystem check failed"));
    }

    // Step 2: Resize
    let result = Command::new("resize2fs")
        .arg(device)
        .arg(format!("{}K", new_size / 1024))
        .output()
        .await?;

    if !result.status.success() {
        return Err(anyhow!("Resize failed"));
    }

    Ok(())
}
```

---

## Implementation Roadmap

### Phase 1: Foundation (Weeks 1-2)

**Goal**: Read-only partition information viewer

**Tasks**:
- [ ] Set up development environment
  - [ ] Install ntfs-3g on development machine
  - [ ] Install e2fsprogs tools
  - [ ] Create test disk images (VHD/VMDK)
  - [ ] Set up test VMs (Windows, Linux)
- [ ] Implement partition info reader
  - [ ] Detect all disks and partitions
  - [ ] Read partition table (MBR/GPT)
  - [ ] Detect filesystem types
  - [ ] Calculate used/free space
- [ ] Create UI components
  - [ ] Disk list view
  - [ ] Partition table visualization
  - [ ] Partition properties panel
- [ ] Add platform badges to existing tools

**Deliverables**:
- Functional partition viewer
- Visual disk layout display
- Platform-aware toolshed badges

### Phase 2: Safe Expansion (Weeks 3-4)

**Goal**: Expand partition into unallocated space

**Tasks**:
- [ ] Implement partition table expansion
  - [ ] Update GPT partition entries
  - [ ] Update MBR partition entries
- [ ] Implement filesystem expansion
  - [ ] NTFS expansion (Windows)
  - [ ] ext4 expansion (Linux)
- [ ] Add validation checks
  - [ ] Verify adjacent unallocated space
  - [ ] Check filesystem integrity
  - [ ] Validate partition alignment
- [ ] Build resize UI
  - [ ] Size selector with visual preview
  - [ ] Validation feedback
  - [ ] Progress monitoring

**Deliverables**:
- Working partition expansion
- Visual resize interface
- Progress tracking

### Phase 3: NTFS Shrink (Weeks 5-6)

**Goal**: Safely shrink NTFS partitions

**Tasks**:
- [ ] Integrate ntfsresize tool
  - [ ] Implement command wrapper
  - [ ] Parse output for progress
  - [ ] Handle errors gracefully
- [ ] Add shrink-specific validations
  - [ ] Check minimum size requirements
  - [ ] Verify used space calculations
  - [ ] Detect unmovable files
- [ ] Implement safety mechanisms
  - [ ] Dry-run preview
  - [ ] Backup verification UI
  - [ ] Multiple confirmation dialogs
- [ ] Add transaction support
  - [ ] Create partition snapshots
  - [ ] Implement rollback logic
  - [ ] Test recovery scenarios

**Deliverables**:
- Safe NTFS shrink operation
- Transaction rollback capability
- Comprehensive error handling

### Phase 4: ext4 Resize (Weeks 7-8)

**Goal**: Resize ext4 filesystems (grow & shrink)

**Tasks**:
- [ ] Integrate resize2fs tool
  - [ ] Implement command wrapper
  - [ ] Handle online/offline resize
  - [ ] Monitor progress output
- [ ] Implement filesystem checks
  - [ ] Integrate e2fsck
  - [ ] Validate filesystem health
  - [ ] Check for errors
- [ ] Handle mount/unmount
  - [ ] Detect mount status
  - [ ] Safe unmount procedures
  - [ ] Remount after resize
- [ ] Cross-platform testing
  - [ ] Test on various Linux distros
  - [ ] Validate different ext4 configurations

**Deliverables**:
- Working ext4 resize (both directions)
- Online resize for expansion
- Offline resize for shrinking

### Phase 5: Polish & Testing (Weeks 9-10)

**Goal**: Production-ready feature

**Tasks**:
- [ ] Comprehensive testing
  - [ ] Unit tests for core logic
  - [ ] Integration tests with real disks
  - [ ] Edge case testing
  - [ ] Failure scenario testing
- [ ] User experience improvements
  - [ ] Refine UI/UX based on testing
  - [ ] Add helpful tooltips
  - [ ] Improve error messages
  - [ ] Add success confirmations
- [ ] Documentation
  - [ ] User guide
  - [ ] Safety best practices
  - [ ] Troubleshooting guide
  - [ ] API documentation
- [ ] Performance optimization
  - [ ] Optimize progress updates
  - [ ] Reduce memory footprint
  - [ ] Improve responsiveness

**Deliverables**:
- Production-ready Partition Manager
- Complete documentation
- Test suite with >80% coverage

---

## Safety & Risk Management

### Pre-Flight Safety Checks

**Mandatory Validations** (Must all pass before proceeding):

1. **Partition State Checks**
   ```rust
   ✓ Partition exists and is accessible
   ✓ Partition is not system/boot partition (warn user)
   ✓ Partition is not currently mounted (for shrink)
   ✓ No active processes using the partition
   ```

2. **Filesystem Health Checks**
   ```rust
   ✓ Filesystem has no errors (run fsck/chkdsk)
   ✓ No bad sectors detected
   ✓ SMART data shows healthy disk
   ✓ Filesystem metadata is consistent
   ```

3. **Space Calculations**
   ```rust
   ✓ Target size > used space (for shrink)
   ✓ Adjacent unallocated space available (for grow)
   ✓ Partition alignment requirements met
   ✓ Minimum filesystem size respected
   ```

4. **Backup Verification**
   ```rust
   ✓ User confirms backup exists
   ✓ Backup is recent (< 24 hours old)
   ✓ User has verified backup integrity
   ✓ User accepts data loss risk
   ```

5. **Permission Checks**
   ```rust
   ✓ Administrator/root privileges active
   ✓ Write access to partition table
   ✓ No disk write protection
   ✓ No BitLocker/LUKS encryption active
   ```

### Risk Levels by Operation

| Operation | Risk | Mitigation | Confirmation Level |
|-----------|------|------------|-------------------|
| View partitions | 🟢 None | Read-only | None |
| Expand into free space | 🟡 Low | Pre-flight checks | 1 confirmation |
| Shrink NTFS | 🟠 Medium | Dry-run + checks | 2 confirmations |
| Shrink ext4 | 🟠 Medium | Fsck + checks | 2 confirmations |
| Resize system partition | 🔴 High | All checks + recovery media | 3 confirmations |
| Move partition | 🔴 Critical | **NOT IMPLEMENTED** | N/A |

### User Confirmation Flow

**For Medium-Risk Operations (Shrink)**:

```
Step 1: Pre-flight check results
┌────────────────────────────────────────┐
│ Pre-Flight Check Results               │
│                                        │
│ ✓ Partition health: OK                │
│ ✓ Filesystem check: Passed            │
│ ✓ SMART status: Healthy                │
│ ✓ No bad sectors detected              │
│                                        │
│ ⚠ This operation will shrink the      │
│   partition from 500 GB to 250 GB     │
│                                        │
│ [Cancel] [Continue to Backup Check]   │
└────────────────────────────────────────┘

Step 2: Backup verification
┌────────────────────────────────────────┐
│ Backup Verification Required           │
│                                        │
│ Before proceeding, you MUST confirm:   │
│                                        │
│ ☐ I have backed up all important data │
│ ☐ I have verified the backup works    │
│ ☐ The backup was made in last 24 hrs  │
│ ☐ I understand this cannot be undone  │
│                                        │
│ [Cancel] [I Have Backed Up - Preview] │
└────────────────────────────────────────┘

Step 3: Dry-run preview
┌────────────────────────────────────────┐
│ Resize Preview (Dry Run)               │
│                                        │
│ Current: [████████████░░░░] 500 GB    │
│ New:     [██████░░░░░░░░░░] 250 GB    │
│                                        │
│ Changes to be made:                    │
│ • Partition size: 500 GB → 250 GB     │
│ • Unallocated space created: 250 GB   │
│ • Data to relocate: 15.3 GB           │
│                                        │
│ Estimated time: 25-35 minutes          │
│                                        │
│ [Cancel] [Proceed with Resize]        │
└────────────────────────────────────────┘

Step 4: Final confirmation
┌────────────────────────────────────────┐
│ ⚠️  FINAL CONFIRMATION                 │
│                                        │
│ You are about to resize partition D:\  │
│                                        │
│ THIS OPERATION IS RISKY AND CANNOT    │
│ BE UNDONE. Data loss may occur if:    │
│ • Power is lost during operation      │
│ • Process is interrupted              │
│ • Disk has hardware failure           │
│                                        │
│ Type "RESIZE" to confirm:              │
│ [________________]                     │
│                                        │
│ [Cancel] [Start Resize]                │
└────────────────────────────────────────┘
```

### Audit Logging

**Log Format**:

```json
{
  "timestamp": "2025-12-19T10:30:45Z",
  "user": "admin@company.com",
  "hostname": "IT-SUPPORT-01",
  "operation": "resize_partition",
  "partition": {
    "device": "/dev/sda2",
    "filesystem": "ntfs",
    "label": "Data",
    "old_size_bytes": 536870912000,
    "new_size_bytes": 268435456000
  },
  "pre_checks": {
    "filesystem_health": "pass",
    "smart_status": "pass",
    "backup_confirmed": true,
    "admin_privileges": true
  },
  "result": {
    "status": "success",
    "duration_seconds": 1847,
    "errors": [],
    "warnings": ["Filesystem was marked dirty, ran chkdsk"]
  }
}
```

**Log Location**: `~/.ittoolkit/logs/partition_operations.jsonl`

---

## Testing Strategy

### Test Environments

1. **Virtual Disk Images** (Primary Testing)
   - Create VHD/VMDK files for testing
   - Various sizes (1GB, 10GB, 100GB)
   - Different partition tables (MBR, GPT)
   - Multiple filesystems (NTFS, ext4, FAT32)

2. **Virtual Machines** (Integration Testing)
   - Windows 10/11 VM
   - Ubuntu 22.04/24.04 VM
   - Debian VM
   - Fedora VM

3. **Physical Hardware** (Final Validation)
   - Non-production machines only
   - External USB drives
   - Secondary internal disks (never boot disk)

### Test Cases

#### Expand Partition Tests

```
Test 1: Expand NTFS into adjacent unallocated space
  Given: 100GB NTFS partition with 50GB unallocated space after it
  When: User expands partition by 25GB
  Then: Partition size is 125GB and filesystem is accessible

Test 2: Expand to maximum available space
  Given: 100GB partition with 150GB unallocated space
  When: User clicks "Expand to Max"
  Then: Partition size is 250GB

Test 3: Expand with no adjacent space
  Given: Partition with no unallocated space after it
  When: User attempts to expand
  Then: Error message shown, operation prevented
```

#### Shrink Partition Tests

```
Test 4: Shrink NTFS partition with plenty of free space
  Given: 500GB NTFS partition with 50GB used
  When: User shrinks to 200GB
  Then: Partition is 200GB, all data intact

Test 5: Shrink below used space (should fail)
  Given: 500GB partition with 400GB used
  When: User attempts to shrink to 300GB
  Then: Error shown, operation prevented

Test 6: Shrink with unmovable files
  Given: NTFS partition with pagefile.sys at end
  When: User attempts to shrink
  Then: Warning shown about unmovable files, suggest safe size
```

#### Error Recovery Tests

```
Test 7: Power loss during resize (simulated)
  Given: Resize operation in progress
  When: VM is force-powered off
  Then: On restart, filesystem is marked for check, data intact

Test 8: Filesystem errors detected
  Given: Partition with filesystem errors
  When: User attempts resize
  Then: Operation blocked, user directed to run fsck/chkdsk

Test 9: Insufficient privileges
  Given: User without admin rights
  When: User attempts resize
  Then: Clear error message requesting elevation
```

### Automated Test Suite

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_calculate_safe_shrink_size() {
        let partition = create_test_partition(
            total_size: 100_000_000_000,  // 100GB
            used_space: 40_000_000_000,   // 40GB
        );

        let safe_size = partition.calculate_safe_shrink_size().await.unwrap();

        // Safe size should be used_space + buffer (20%)
        assert!(safe_size >= 48_000_000_000);  // 40GB + 20% = 48GB
    }

    #[tokio::test]
    async fn test_shrink_below_used_space_fails() {
        let partition = create_test_partition(
            total_size: 100_000_000_000,
            used_space: 60_000_000_000,
        );

        let result = partition.validate_shrink_to(50_000_000_000).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), ErrorKind::InsufficientSpace);
    }
}
```

---

## Todo Checklist

### Phase 1: Foundation (Weeks 1-2)

**Environment Setup**
- [ ] Install ntfs-3g tools on dev machine
- [ ] Install e2fsprogs (resize2fs, e2fsck)
- [ ] Create test VHD files (10GB, 50GB, 100GB)
- [ ] Set up Windows 11 test VM
- [ ] Set up Ubuntu 24.04 test VM
- [ ] Configure VM snapshots for rollback testing

**Backend Development**
- [ ] Add Rust dependencies to Cargo.toml
  - [ ] libparted = "0.4"
  - [ ] gptman = "1.0"
  - [ ] mbrman = "0.5"
  - [ ] thiserror = "2.0"
  - [ ] env_logger = "0.11"
- [ ] Create partition info module (`src-tauri/src/partition/`)
  - [ ] `mod.rs` - Module exports
  - [ ] `info.rs` - Partition information reader
  - [ ] `types.rs` - Type definitions
  - [ ] `platform.rs` - Platform-specific code
- [ ] Implement disk detection
  - [ ] List all physical disks
  - [ ] Read partition tables
  - [ ] Detect filesystem types
- [ ] Implement space calculation
  - [ ] Calculate total partition size
  - [ ] Calculate used space
  - [ ] Calculate free space
  - [ ] Detect unallocated regions
- [ ] Create Tauri commands
  - [ ] `get_disks` - List all disks
  - [ ] `get_partitions` - Get partitions for a disk
  - [ ] `get_partition_info` - Detailed partition info

**Frontend Development**
- [ ] Create PartitionManager component
  - [ ] `src/components/tools/PartitionManager.tsx`
- [ ] Implement disk list view
  - [ ] Show all disks with icons
  - [ ] Display disk size and model
- [ ] Implement partition list view
  - [ ] Tabular view of partitions
  - [ ] Show size, type, filesystem
- [ ] Create visual disk layout
  - [ ] Horizontal bar representation
  - [ ] Color-coded by filesystem
  - [ ] Show unallocated space
- [ ] Add partition properties panel
  - [ ] Display all partition metadata
  - [ ] Show mount points/drive letters
- [ ] Add platform badges to ToolshedPanel
  - [ ] Create Badge component
  - [ ] Add platform detection logic
  - [ ] Update tool definitions with platform info

**Testing**
- [ ] Write unit tests for partition info
- [ ] Test disk detection on Windows
- [ ] Test disk detection on Linux
- [ ] Verify UI displays correctly

### Phase 2: Safe Expansion (Weeks 3-4)

**Backend Development**
- [ ] Create resize module (`src-tauri/src/partition/resize/`)
  - [ ] `mod.rs`
  - [ ] `expand.rs` - Expansion logic
  - [ ] `validation.rs` - Pre-flight checks
  - [ ] `progress.rs` - Progress tracking
- [ ] Implement expand validation
  - [ ] Check for adjacent unallocated space
  - [ ] Verify partition alignment
  - [ ] Check filesystem health
- [ ] Implement partition table expansion
  - [ ] GPT partition expansion
  - [ ] MBR partition expansion
- [ ] Implement filesystem expansion
  - [ ] NTFS expansion wrapper
  - [ ] ext4 expansion wrapper
- [ ] Add progress monitoring
  - [ ] Emit Tauri events for progress
  - [ ] Calculate operation percentage
- [ ] Create Tauri commands
  - [ ] `validate_expand` - Pre-check
  - [ ] `expand_partition` - Execute expansion

**Frontend Development**
- [ ] Create resize UI component
  - [ ] Size selector slider
  - [ ] Visual preview of changes
  - [ ] Validation feedback
- [ ] Implement progress dialog
  - [ ] Progress bar
  - [ ] Status messages
  - [ ] Cancel button (if safe)
- [ ] Add resize confirmation dialog
  - [ ] Show before/after sizes
  - [ ] Display warnings
  - [ ] Require user confirmation

**Testing**
- [ ] Test expand on NTFS partition
- [ ] Test expand on ext4 partition
- [ ] Test expand with various sizes
- [ ] Test error handling (no space, etc.)
- [ ] Verify data integrity after expand

### Phase 3: NTFS Shrink (Weeks 5-6)

**Backend Development**
- [ ] Create shrink module (`resize/shrink.rs`)
- [ ] Implement ntfsresize wrapper
  - [ ] Dry-run functionality
  - [ ] Actual resize execution
  - [ ] Error parsing
- [ ] Add shrink-specific validation
  - [ ] Check used space vs target size
  - [ ] Detect unmovable files
  - [ ] Calculate minimum safe size
- [ ] Implement transaction support
  - [ ] Create partition snapshots
  - [ ] Rollback logic
  - [ ] Recovery procedures
- [ ] Add comprehensive logging
  - [ ] Operation audit log
  - [ ] Error logging
  - [ ] Performance metrics
- [ ] Create Tauri commands
  - [ ] `validate_shrink` - Pre-check with dry-run
  - [ ] `shrink_partition` - Execute shrink

**Frontend Development**
- [ ] Create backup verification UI
  - [ ] Checklist of backup requirements
  - [ ] Confirmation checkboxes
- [ ] Implement dry-run preview
  - [ ] Show proposed changes
  - [ ] Display warnings
  - [ ] Estimate time
- [ ] Add multi-step confirmation
  - [ ] Pre-flight results
  - [ ] Backup confirmation
  - [ ] Preview approval
  - [ ] Final "type to confirm"
- [ ] Improve error messages
  - [ ] User-friendly explanations
  - [ ] Actionable suggestions
  - [ ] Links to documentation

**Testing**
- [ ] Test shrink on various NTFS partitions
- [ ] Test shrink with different used/free ratios
- [ ] Test error cases (shrink too small)
- [ ] Test transaction rollback
- [ ] Simulate power loss (VM snapshot restore)
- [ ] Verify data integrity after shrink

### Phase 4: ext4 Resize (Weeks 7-8)

**Backend Development**
- [ ] Implement resize2fs wrapper
  - [ ] Online resize (mounted)
  - [ ] Offline resize (unmounted)
  - [ ] Progress parsing
- [ ] Implement e2fsck wrapper
  - [ ] Force filesystem check
  - [ ] Parse check results
  - [ ] Handle check failures
- [ ] Add mount/unmount handling
  - [ ] Detect mount status
  - [ ] Safe unmount procedures
  - [ ] Remount after resize
- [ ] Create Tauri commands
  - [ ] `resize_ext4` - Main resize function

**Frontend Development**
- [ ] Add filesystem-specific UI hints
  - [ ] Show if online resize available
  - [ ] Warn about unmount requirement
- [ ] Update progress UI for ext4
  - [ ] Show fsck progress
  - [ ] Show resize progress

**Testing**
- [ ] Test ext4 expand (online)
- [ ] Test ext4 shrink (offline)
- [ ] Test on different Linux distros
- [ ] Verify fsck integration
- [ ] Test mount/unmount handling

### Phase 5: Polish & Testing (Weeks 9-10)

**Testing & QA**
- [ ] Comprehensive edge case testing
  - [ ] Very small partitions (<1GB)
  - [ ] Very large partitions (>1TB)
  - [ ] Nearly full partitions
  - [ ] Nearly empty partitions
- [ ] Failure scenario testing
  - [ ] Disk errors during resize
  - [ ] Power loss simulation
  - [ ] Process termination
  - [ ] Out of space conditions
- [ ] Performance testing
  - [ ] Measure resize times
  - [ ] Optimize progress updates
  - [ ] Reduce memory usage
- [ ] User acceptance testing
  - [ ] Have IT team test workflows
  - [ ] Gather feedback
  - [ ] Iterate on UX

**Documentation**
- [ ] Write user guide
  - [ ] Getting started
  - [ ] Step-by-step resize tutorial
  - [ ] Troubleshooting section
- [ ] Document safety best practices
  - [ ] When to resize
  - [ ] Backup procedures
  - [ ] Risk mitigation
- [ ] Create API documentation
  - [ ] Rust API docs
  - [ ] Tauri command reference
  - [ ] Code examples
- [ ] Write troubleshooting guide
  - [ ] Common errors
  - [ ] Recovery procedures
  - [ ] Support contacts

**Polish**
- [ ] Refine UI/UX
  - [ ] Improve visual design
  - [ ] Add helpful tooltips
  - [ ] Enhance error messages
- [ ] Add accessibility features
  - [ ] Keyboard navigation
  - [ ] Screen reader support
  - [ ] High contrast mode
- [ ] Performance optimization
  - [ ] Optimize progress updates
  - [ ] Reduce UI latency
  - [ ] Improve responsiveness
- [ ] Final code review
  - [ ] Security review
  - [ ] Code quality check
  - [ ] Remove debug code

**Release Preparation**
- [ ] Create release notes
- [ ] Update README.md
- [ ] Tag version in git
- [ ] Build release binaries
- [ ] Create installation guide

---

## Success Criteria

### Minimum Viable Product (MVP)

The Partition Manager feature will be considered MVP-complete when:

1. ✅ **Windows NTFS Support**
   - Can expand NTFS partitions into unallocated space
   - Can safely shrink NTFS partitions
   - Includes comprehensive safety checks
   - Has multi-step user confirmation
   - Provides real-time progress monitoring

2. ✅ **Linux ext4 Support**
   - Can expand ext4 partitions (online and offline)
   - Can shrink ext4 partitions (offline)
   - Integrates e2fsck for filesystem checks
   - Handles mount/unmount automatically

3. ✅ **Safety & Reliability**
   - All pre-flight checks implemented
   - Transaction support with rollback
   - Comprehensive audit logging
   - Error recovery mechanisms
   - No data loss in testing

4. ✅ **User Experience**
   - Intuitive resize interface
   - Clear progress indication
   - Helpful error messages
   - Visual disk layout
   - Platform-aware toolshed

5. ✅ **Documentation**
   - User guide complete
   - API documentation available
   - Troubleshooting guide published

### Post-MVP Enhancements

Future versions may include:

- FAT32 resize support
- macOS APFS resize support
- Partition creation/deletion
- Drive letter changes (Windows)
- Partition cloning
- Disk imaging

---

## Resources & References

### Documentation
- [ntfsresize manual](https://manpages.ubuntu.com/manpages/focal/man8/ntfsresize.8.html)
- [resize2fs manual](https://www.mankier.com/8/resize2fs)
- [libparted API](https://www.gnu.org/software/parted/api/)
- [Tauri Documentation](https://v2.tauri.app/)

### Tools & Libraries
- [ntfs-3g GitHub](https://github.com/tuxera/ntfs-3g)
- [e2fsprogs](https://github.com/tytso/e2fsprogs)
- [GParted](https://gparted.org/)
- [Rust libparted bindings](https://docs.rs/libparted/latest/libparted/)

### Research
- [GNU Parted FAQ](https://www.gnu.org/software/parted/faq.shtml)
- [Resize Without Data Loss](https://www.easeus.com/partition-master/resize-ntfs-partition.html)
- [MBR to GPT Conversion](https://techcommunity.microsoft.com/discussions/windows11/how-do-i-convert-mbr-to-gpt-without-losing-data/4133172)

---

**Document Version**: 1.0
**Last Updated**: 2025-12-19
**Status**: Planning Phase
**Next Review**: Start of Phase 1
