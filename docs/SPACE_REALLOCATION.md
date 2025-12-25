# Space Reallocation Feature

## Overview

This feature allows you to reallocate disk space from one partition to another, solving the common problem: **"C: drive is full, but E: drive has free space."**

## The Challenge

Due to how partitions work on disks, you cannot directly "take" space from one partition and "give" it to another. Partitions are contiguous blocks of space on the disk, and they can only expand into **unallocated space directly adjacent** to them.

## Example Scenario

```
Disk layout:
[C: 50GB FULL] [E: 20GB, 5GB used] [F: 30GB]

Problem: C: needs 15GB more space, E: has 15GB free space
```

## Solutions Implemented

### Solution 1: Simple Expansion (Already Working)

If there's already unallocated space directly after the partition:
```
[C: 50GB] [FREE: 20GB] [F: 30GB]
         ↑ C: can expand here
```

**Usage:**
1. Select partition C:
2. Click "Expand"
3. Choose new size
4. Done!

**This already works with the bug fix we made today.**

---

### Solution 2: Space Reallocation Wizard (New Feature)

When partitions are in the way:
```
Before: [C: 50GB FULL] [E: 20GB] [F: 30GB]
After:  [C: 65GB]                [F: 30GB]
```

**How it works:**
1. User calls: `create_space_reallocation_plan(partition_c_id, 15GB)`
2. App analyzes disk layout
3. App creates a step-by-step plan:
   - ⚠️ Step 1: **BACKUP E: drive data**
   - Step 2: Delete E: partition
   - Step 3: Expand C: into freed space

**Usage from Frontend:**

```typescript
// TypeScript
import { invoke } from '@tauri-apps/api/core';

// Create a reallocation plan
const plan = await invoke('create_space_reallocation_plan', {
  targetPartitionId: 'partition-0-1', // C: drive ID
  desiredAdditionalSpace: 15 * 1024 * 1024 * 1024, // 15GB in bytes
});

console.log(plan);
// {
//   target_partition_id: "partition-0-1",
//   source_partitions: [
//     {
//       partition_id: "partition-0-2",
//       partition_label: "E:",
//       current_size: 21474836480,
//       used_space: 5368709120,
//       action: { DeleteEntirely: {} }
//     }
//   ],
//   total_space_freed: 21474836480,
//   target_new_size: 75161927680,
//   steps: [
//     {
//       step_number: 1,
//       title: "⚠️ BACKUP YOUR DATA",
//       description: "The following partitions will be deleted: E:...",
//       action_type: "UserManual",
//       can_automate: false
//     },
//     {
//       step_number: 2,
//       title: "Delete partition E:",
//       description: "Delete E: (frees 20.00 GB of space)",
//       action_type: "AppAssistedManual",
//       can_automate: true
//     },
//     {
//       step_number: 3,
//       title: "Expand C:",
//       description: "Expand C: from 50.00 GB to 65.00 GB (+15.00 GB)",
//       action_type: "AppAutomated",
//       can_automate: true
//     }
//   ],
//   warnings: [
//     "Partition E: (Data) contains 5.00 GB of data. YOU MUST BACKUP THIS DATA before proceeding!"
//   ]
// }
```

---

### Solution 3: Full Partition Moving (Advanced - Partially Implemented)

This would physically move partition data to a new location:
```
Before: [C: 50GB] [E: 20GB] [F: 30GB]
Step 1: Backup E: data
Step 2: Delete E:
Step 3: Create E: at end of disk
Step 4: Restore E: data
Result: [C: 50GB] [FREE: 20GB] [E: 20GB at end]
```

**Status:**
- ✅ Module structure created
- ✅ Backup/restore logic implemented
- ✅ Delete partition implemented
- ⚠️ Create partition at specific offset - **NOT YET IMPLEMENTED**
  (Requires low-level disk API access, very platform-specific)

**Why not fully implemented?**
Creating a partition at a specific disk offset requires:
- Windows: Direct disk I/O using DeviceIoControl with IOCTL_DISK_CREATE_DISK
- Linux: Advanced parted scripting or direct /dev manipulation
- macOS: diskutil with specific sector calculations

This is risky and complex. Most partition tools (GParted, EaseUS, etc.) took years to perfect this.

---

## Recommended Workflow for Users

### Scenario: C: is full, E: has space

**Option A: E: has important data**
1. Manually backup E: to external drive
2. Use app's reallocation wizard to see plan
3. Follow the plan (delete E:, expand C:)
4. Optionally recreate E: at end of disk if needed

**Option B: E: is empty or has unimportant data**
1. Use app's reallocation wizard
2. Follow the plan to delete E: and expand C:

**Option C: Want to keep E: and its data**
- Currently: Not supported (would need full partition moving)
- Workaround: Use commercial tools like EaseUS Partition Master or GParted

---

## Testing Instructions

### Test Simple Expansion (F: drive)

Since F: is at the end and empty:

```powershell
# In your VM

# 1. Shrink F: to create free space
$script = @"
select volume F
shrink desired=10000
"@
$script | Out-File "$env:TEMP\test.txt" -Encoding ASCII
diskpart /s "$env:TEMP\test.txt"

# 2. Use your app to expand F: back to full size
# Should work perfectly!
```

### Test Reallocation Wizard

```typescript
// In your app frontend
const plan = await invoke('create_space_reallocation_plan', {
  targetPartitionId: 'partition-0-1', // C: drive
  desiredAdditionalSpace: 10 * 1024 * 1024 * 1024, // 10GB
});

// Display the plan to user
console.log(plan.steps);
plan.warnings.forEach(w => console.warn(w));
```

---

## API Reference

### Tauri Commands

```rust
/// Create a space reallocation plan
#[command]
pub async fn create_space_reallocation_plan(
    target_partition_id: String,
    desired_additional_space: u64,
) -> Result<ReallocationPlan, String>
```

### Types

```rust
pub struct ReallocationPlan {
    pub target_partition_id: String,
    pub source_partitions: Vec<SourcePartitionPlan>,
    pub total_space_freed: u64,
    pub target_new_size: u64,
    pub steps: Vec<ReallocationStep>,
    pub warnings: Vec<String>,
}

pub struct ReallocationStep {
    pub step_number: usize,
    pub title: String,
    pub description: String,
    pub action_type: StepActionType,
    pub can_automate: bool,
}

pub enum StepActionType {
    UserManual,        // User must do manually
    AppAutomated,      // App can do automatically
    AppAssistedManual, // App guides, user confirms
}
```

---

## Summary of What Was Built Today

### ✅ Fixed Bugs
1. **Partition detection bug** - Fixed `find_next_partition` to use `>=` instead of `>` (was skipping adjacent partitions)
2. **Diskpart command** - Fixed to use size increase instead of absolute size
3. **Error reporting** - Now shows full diskpart output for debugging

### ✅ New Features
1. **Space Reallocation Wizard** - Analyzes disk and creates step-by-step plan
2. **Partition Move Module** (partial) - Foundation for advanced partition moving
3. **Better validation** - Correctly detects adjacent space

### 📝 Documentation
- This guide
- Code comments throughout
- Test cases in reallocation_wizard.rs

---

## Next Steps (Future Work)

1. **UI for Reallocation Wizard** - Build a friendly UI to show the plan and guide users
2. **Partition Deletion** - Add safe partition deletion feature
3. **Advanced Partition Moving** - Complete the low-level partition creation at offset
4. **Progress Tracking** - Real-time progress for long operations
5. **Rollback Support** - Automatic rollback if something fails

---

## Questions?

- Simple expansion: Already works! Test with F: drive
- Space reallocation: Use the new wizard API
- Full partition moving: Partially implemented, needs more work for production use

Remember: **Always backup important data before partition operations!**
