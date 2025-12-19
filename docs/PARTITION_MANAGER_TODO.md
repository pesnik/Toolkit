# Partition Manager - Todo Tracking

**Project**: IT Toolkit - Partition Manager Feature
**Start Date**: 2025-12-19
**Target Completion**: 10 weeks (Phase 1-5)
**Current Phase**: Phase 1 - Foundation

---

## Progress Overview

| Phase | Status | Start Date | End Date | Completion |
|-------|--------|------------|----------|------------|
| Phase 1: Foundation | 🔵 In Progress | 2025-12-19 | TBD | 0% |
| Phase 2: Safe Expansion | ⚪ Not Started | - | - | 0% |
| Phase 3: NTFS Shrink | ⚪ Not Started | - | - | 0% |
| Phase 4: ext4 Resize | ⚪ Not Started | - | - | 0% |
| Phase 5: Polish & Testing | ⚪ Not Started | - | - | 0% |

**Overall Progress**: 0/150+ tasks completed (0%)

---

## Phase 1: Foundation (Weeks 1-2) - IN PROGRESS

**Goal**: Read-only partition information viewer
**Target**: 2 weeks
**Started**: 2025-12-19

### Environment Setup

- [ ] **Install development tools**
  - [ ] Install ntfs-3g tools on dev machine
  - [ ] Install e2fsprogs (resize2fs, e2fsck)
  - [ ] Verify Rust toolchain is up to date
  - [ ] Install platform-specific disk tools

- [ ] **Create test environment**
  - [ ] Create test VHD files
    - [ ] 10GB test disk
    - [ ] 50GB test disk
    - [ ] 100GB test disk
  - [ ] Set up Windows 11 test VM
  - [ ] Set up Ubuntu 24.04 test VM
  - [ ] Configure VM snapshots for rollback testing
  - [ ] Create test partitions (NTFS, ext4, FAT32)

### Backend Development (Rust)

- [ ] **Add dependencies to Cargo.toml**
  - [ ] Add `libparted = "0.4"` (if available, else use FFI)
  - [ ] Add `gptman = "1.0"`
  - [ ] Add `mbrman = "0.5"`
  - [ ] Add `thiserror = "2.0"`
  - [ ] Add `env_logger = "0.11"`
  - [ ] Verify all dependencies compile

- [ ] **Create module structure**
  - [ ] Create `src-tauri/src/partition/` directory
  - [ ] Create `mod.rs` - Module exports
  - [ ] Create `types.rs` - Type definitions
  - [ ] Create `info.rs` - Partition information reader
  - [ ] Create `platform.rs` - Platform-specific code
  - [ ] Update `src-tauri/src/lib.rs` to include partition module

- [ ] **Define core types (types.rs)**
  - [ ] `DiskInfo` struct
  - [ ] `PartitionInfo` struct
  - [ ] `FilesystemType` enum
  - [ ] `PartitionType` enum
  - [ ] `PartitionTableType` enum (MBR/GPT)
  - [ ] `DiskStatus` struct
  - [ ] Add serde derives for JSON serialization

- [ ] **Implement disk detection (info.rs)**
  - [ ] Function: `get_all_disks() -> Result<Vec<DiskInfo>>`
  - [ ] Function: `get_disk_by_path(path: &str) -> Result<DiskInfo>`
  - [ ] Detect physical disk model and size
  - [ ] Read disk serial number
  - [ ] Check disk health status
  - [ ] Handle permission errors gracefully

- [ ] **Implement partition table reading**
  - [ ] Function: `get_partitions(disk_path: &str) -> Result<Vec<PartitionInfo>>`
  - [ ] Read MBR partition tables
  - [ ] Read GPT partition tables
  - [ ] Detect partition table type
  - [ ] Handle corrupted partition tables

- [ ] **Implement filesystem detection**
  - [ ] Detect NTFS filesystems
  - [ ] Detect ext2/3/4 filesystems
  - [ ] Detect FAT32 filesystems
  - [ ] Detect exFAT filesystems
  - [ ] Detect APFS filesystems (macOS)
  - [ ] Handle unknown filesystems

- [ ] **Implement space calculation**
  - [ ] Calculate total partition size
  - [ ] Calculate used space
  - [ ] Calculate free space
  - [ ] Detect unallocated regions
  - [ ] Handle mounted vs unmounted partitions

- [ ] **Platform-specific implementations (platform.rs)**
  - [ ] Windows: Use WMI/Win32 APIs for disk info
  - [ ] Linux: Parse /proc/partitions
  - [ ] macOS: Use diskutil wrapper
  - [ ] Add platform detection (#[cfg(target_os = "...")])

- [ ] **Create Tauri commands**
  - [ ] `#[tauri::command] get_disks() -> Result<Vec<DiskInfo>>`
  - [ ] `#[tauri::command] get_partitions(disk_id: String) -> Result<Vec<PartitionInfo>>`
  - [ ] `#[tauri::command] get_partition_info(partition_id: String) -> Result<PartitionInfo>`
  - [ ] `#[tauri::command] get_filesystem_info(partition_id: String) -> Result<FilesystemInfo>`
  - [ ] Register commands in Tauri builder

- [ ] **Error handling**
  - [ ] Define custom error types
  - [ ] Implement error conversions
  - [ ] Add user-friendly error messages
  - [ ] Test error scenarios

### Frontend Development (React)

- [ ] **Create component structure**
  - [ ] Create `src/components/tools/PartitionManager.tsx`
  - [ ] Create `src/components/tools/partition/` directory
  - [ ] Create `DiskList.tsx` component
  - [ ] Create `PartitionList.tsx` component
  - [ ] Create `DiskVisualization.tsx` component
  - [ ] Create `PartitionProperties.tsx` component

- [ ] **Implement main PartitionManager component**
  - [ ] Set up component state
  - [ ] Add Tauri command invocations
  - [ ] Implement disk selection logic
  - [ ] Add loading states
  - [ ] Add error handling UI
  - [ ] Style with Fluent UI 2

- [ ] **Implement DiskList component**
  - [ ] Display all disks in a list
  - [ ] Show disk icon, model, size
  - [ ] Add disk selection
  - [ ] Show disk health status
  - [ ] Add refresh button
  - [ ] Handle empty state

- [ ] **Implement PartitionList component**
  - [ ] Display partitions in a table
  - [ ] Columns: Name, Size, Type, Filesystem, Status
  - [ ] Add partition selection
  - [ ] Show mount points/drive letters
  - [ ] Color-code by filesystem type
  - [ ] Add sorting capabilities

- [ ] **Implement DiskVisualization component**
  - [ ] Horizontal bar representation
  - [ ] Color-coded segments for partitions
  - [ ] Show unallocated space
  - [ ] Add tooltips on hover
  - [ ] Make it responsive
  - [ ] Add zoom/pan for large disks

- [ ] **Implement PartitionProperties panel**
  - [ ] Display all partition metadata
  - [ ] Show filesystem details
  - [ ] Display partition flags
  - [ ] Show mount information
  - [ ] Format sizes human-readable
  - [ ] Add copy-to-clipboard buttons

- [ ] **Add to ToolshedPanel**
  - [ ] Import PartitionManager component
  - [ ] Add to tools array
  - [ ] Set icon (use existing or add new)
  - [ ] Set category: "Storage & Cleanup"
  - [ ] Set platforms: ['windows', 'linux']
  - [ ] Add description

### Testing

- [ ] **Unit tests (Rust)**
  - [ ] Test disk detection on mock data
  - [ ] Test partition table parsing
  - [ ] Test filesystem detection
  - [ ] Test space calculations
  - [ ] Test error handling

- [ ] **Integration tests**
  - [ ] Test on Windows with real disk
  - [ ] Test on Linux with real disk
  - [ ] Test with various partition schemes
  - [ ] Test error scenarios (no permissions, etc.)

- [ ] **UI tests**
  - [ ] Verify disk list displays correctly
  - [ ] Verify partition list shows data
  - [ ] Verify visualization renders
  - [ ] Test error states in UI
  - [ ] Test loading states

### Documentation

- [ ] **Code documentation**
  - [ ] Add rustdoc comments to all public functions
  - [ ] Document struct fields
  - [ ] Add usage examples
  - [ ] Document error types

- [ ] **User documentation**
  - [ ] Update README with Partition Manager feature
  - [ ] Add screenshots to docs
  - [ ] Document limitations

**Phase 1 Completion Criteria**:
- ✅ Can view all disks and partitions
- ✅ Shows accurate size information
- ✅ Displays partition types correctly
- ✅ Visual disk layout works
- ✅ No crashes or errors in normal use

---

## Phase 2: Safe Expansion (Weeks 3-4) - NOT STARTED

**Goal**: Expand partition into unallocated space
**Target**: 2 weeks

### Backend Development

- [ ] **Create resize module structure**
  - [ ] Create `src-tauri/src/partition/resize/` directory
  - [ ] Create `resize/mod.rs`
  - [ ] Create `resize/expand.rs`
  - [ ] Create `resize/validation.rs`
  - [ ] Create `resize/progress.rs`

- [ ] **Implement validation logic (validation.rs)**
  - [ ] Check for adjacent unallocated space
  - [ ] Verify partition alignment
  - [ ] Check filesystem health
  - [ ] Validate new size bounds
  - [ ] Check for locks on partition

- [ ] **Implement partition table expansion (expand.rs)**
  - [ ] Function: `expand_gpt_partition()`
  - [ ] Function: `expand_mbr_partition()`
  - [ ] Update partition end sector
  - [ ] Recalculate CRC (GPT)
  - [ ] Handle partition alignment

- [ ] **Implement filesystem expansion**
  - [ ] Windows: NTFS expansion wrapper
    - [ ] Call diskpart or ntfsresize
    - [ ] Parse output
    - [ ] Handle errors
  - [ ] Linux: ext4 expansion wrapper
    - [ ] Call resize2fs
    - [ ] Support online resize
    - [ ] Parse output

- [ ] **Implement progress monitoring (progress.rs)**
  - [ ] Define progress events
  - [ ] Emit Tauri events
  - [ ] Calculate operation percentage
  - [ ] Track operation stages

- [ ] **Create Tauri commands**
  - [ ] `validate_expand(partition_id, new_size) -> Result<ValidationResult>`
  - [ ] `expand_partition(partition_id, new_size) -> Result<()>`
  - [ ] Set up event emitters for progress

### Frontend Development

- [ ] **Create resize UI components**
  - [ ] Create `ResizeDialog.tsx`
  - [ ] Create `SizeSelector.tsx`
  - [ ] Create `ResizePreview.tsx`
  - [ ] Create `ProgressDialog.tsx`

- [ ] **Implement SizeSelector**
  - [ ] Slider for size selection
  - [ ] Text input for manual entry
  - [ ] Show min/max bounds
  - [ ] Display validation errors
  - [ ] Format sizes (MB/GB/TB)

- [ ] **Implement ResizePreview**
  - [ ] Visual before/after comparison
  - [ ] Show partition size changes
  - [ ] Highlight affected areas
  - [ ] Display warnings

- [ ] **Implement ProgressDialog**
  - [ ] Progress bar
  - [ ] Status messages
  - [ ] Current operation display
  - [ ] Cancel button (if applicable)
  - [ ] Error display

- [ ] **Add resize confirmation dialog**
  - [ ] Show operation summary
  - [ ] Display risks/warnings
  - [ ] Require user confirmation
  - [ ] Prevent accidental clicks

- [ ] **Add resize button to UI**
  - [ ] Add to partition context menu
  - [ ] Enable only when valid
  - [ ] Show tooltip with requirements

### Testing

- [ ] **Test expand on NTFS**
  - [ ] Small expansion (10GB → 20GB)
  - [ ] Large expansion (100GB → 500GB)
  - [ ] Expand to maximum
  - [ ] Verify data integrity

- [ ] **Test expand on ext4**
  - [ ] Online expansion (mounted)
  - [ ] Offline expansion (unmounted)
  - [ ] Various sizes
  - [ ] Verify filesystem consistency

- [ ] **Error handling tests**
  - [ ] Expand with no adjacent space
  - [ ] Expand beyond disk capacity
  - [ ] Insufficient permissions
  - [ ] Partition in use

- [ ] **UI/UX tests**
  - [ ] Progress updates smoothly
  - [ ] Error messages are clear
  - [ ] Confirmation flow is intuitive

**Phase 2 Completion Criteria**:
- ✅ Can safely expand NTFS partitions
- ✅ Can safely expand ext4 partitions
- ✅ Progress monitoring works
- ✅ All validation checks pass
- ✅ No data loss in any test

---

## Phase 3: NTFS Shrink (Weeks 5-6) - NOT STARTED

**Goal**: Safely shrink NTFS partitions
**Target**: 2 weeks

### Backend Development

- [ ] **Create shrink module**
  - [ ] Create `resize/shrink.rs`
  - [ ] Create `resize/transaction.rs`
  - [ ] Create `resize/audit.rs`

- [ ] **Implement ntfsresize wrapper (shrink.rs)**
  - [ ] Function: `ntfsresize_dry_run()`
  - [ ] Function: `ntfsresize_execute()`
  - [ ] Parse ntfsresize output
  - [ ] Extract minimum size
  - [ ] Handle unmovable files

- [ ] **Implement shrink-specific validation**
  - [ ] Check used space vs target size
  - [ ] Detect unmovable files
  - [ ] Calculate minimum safe size
  - [ ] Check filesystem errors

- [ ] **Implement transaction support (transaction.rs)**
  - [ ] Create partition snapshots (metadata)
  - [ ] Implement rollback logic
  - [ ] Recovery procedures
  - [ ] Transaction state management

- [ ] **Implement audit logging (audit.rs)**
  - [ ] Define log format (JSON)
  - [ ] Log pre-flight checks
  - [ ] Log operation steps
  - [ ] Log results and errors
  - [ ] Store logs persistently

- [ ] **Create Tauri commands**
  - [ ] `validate_shrink(partition_id, new_size) -> Result<ShrinkValidation>`
  - [ ] `shrink_partition(partition_id, new_size) -> Result<()>`
  - [ ] `get_minimum_shrink_size(partition_id) -> Result<u64>`

### Frontend Development

- [ ] **Create backup verification UI**
  - [ ] Checklist component
  - [ ] Backup confirmation checkboxes
  - [ ] Warning messages
  - [ ] Prevent bypass

- [ ] **Implement dry-run preview**
  - [ ] Show proposed changes
  - [ ] Display warnings/limitations
  - [ ] Show unmovable files (if any)
  - [ ] Estimate operation time

- [ ] **Add multi-step confirmation flow**
  - [ ] Step 1: Pre-flight results
  - [ ] Step 2: Backup verification
  - [ ] Step 3: Dry-run preview
  - [ ] Step 4: Final "type to confirm"

- [ ] **Improve error messages**
  - [ ] User-friendly explanations
  - [ ] Actionable suggestions
  - [ ] Link to documentation
  - [ ] Show error codes for support

- [ ] **Add audit log viewer**
  - [ ] Display operation history
  - [ ] Filter by date/status
  - [ ] Export logs
  - [ ] Search functionality

### Testing

- [ ] **Test various NTFS scenarios**
  - [ ] Nearly empty partition
  - [ ] Nearly full partition
  - [ ] Partition with fragmentation
  - [ ] Partition with system files

- [ ] **Test error cases**
  - [ ] Shrink below used space
  - [ ] Filesystem errors present
  - [ ] Unmovable files blocking
  - [ ] Insufficient permissions

- [ ] **Test transaction rollback**
  - [ ] Simulate failures at various stages
  - [ ] Verify rollback works
  - [ ] Check filesystem integrity after rollback

- [ ] **Simulate power loss (VM)**
  - [ ] Power off during shrink
  - [ ] Verify recovery on restart
  - [ ] Check data integrity

**Phase 3 Completion Criteria**:
- ✅ Can safely shrink NTFS partitions
- ✅ Dry-run always executes first
- ✅ Multi-step confirmation works
- ✅ Transaction rollback functional
- ✅ Audit logging complete
- ✅ No data loss in stress tests

---

## Phase 4: ext4 Resize (Weeks 7-8) - NOT STARTED

**Goal**: Resize ext4 filesystems (grow & shrink)
**Target**: 2 weeks

### Backend Development

- [ ] **Implement resize2fs wrapper**
  - [ ] Function: `resize2fs_online()`
  - [ ] Function: `resize2fs_offline()`
  - [ ] Parse resize2fs output
  - [ ] Extract progress information

- [ ] **Implement e2fsck wrapper**
  - [ ] Function: `e2fsck_force()`
  - [ ] Parse e2fsck results
  - [ ] Handle filesystem errors
  - [ ] Automatic repair (with user consent)

- [ ] **Add mount/unmount handling**
  - [ ] Detect mount status
  - [ ] Function: `safe_unmount()`
  - [ ] Function: `remount_after_resize()`
  - [ ] Handle busy filesystem

- [ ] **Extend validation for ext4**
  - [ ] Check if online resize available
  - [ ] Validate unmount is safe
  - [ ] Check filesystem features

- [ ] **Create Tauri commands**
  - [ ] `resize_ext4(partition_id, new_size) -> Result<()>`
  - [ ] `check_ext4_health(partition_id) -> Result<HealthStatus>`

### Frontend Development

- [ ] **Add filesystem-specific UI hints**
  - [ ] Show if online resize available
  - [ ] Warn about unmount requirement
  - [ ] Display filesystem features

- [ ] **Update progress UI for ext4**
  - [ ] Show fsck progress
  - [ ] Show resize progress
  - [ ] Display multiple stages

- [ ] **Add unmount warnings**
  - [ ] List apps using filesystem
  - [ ] Suggest closing apps
  - [ ] Force unmount option

### Testing

- [ ] **Test ext4 expand (online)**
  - [ ] Mounted partition
  - [ ] Various sizes
  - [ ] Verify filesystem consistency

- [ ] **Test ext4 shrink (offline)**
  - [ ] Unmounted partition
  - [ ] Force fsck before resize
  - [ ] Verify data integrity

- [ ] **Test on different Linux distros**
  - [ ] Ubuntu 24.04
  - [ ] Debian 12
  - [ ] Fedora 40
  - [ ] Arch Linux

- [ ] **Test mount/unmount handling**
  - [ ] Busy filesystem scenarios
  - [ ] Apps with open files
  - [ ] Network mounts

**Phase 4 Completion Criteria**:
- ✅ ext4 expand works (online & offline)
- ✅ ext4 shrink works (offline)
- ✅ e2fsck integration complete
- ✅ Mount/unmount handling robust
- ✅ Tested on multiple distros

---

## Phase 5: Polish & Testing (Weeks 9-10) - NOT STARTED

**Goal**: Production-ready feature
**Target**: 2 weeks

### Comprehensive Testing

- [ ] **Edge case testing**
  - [ ] Very small partitions (<1GB)
  - [ ] Very large partitions (>1TB)
  - [ ] Nearly full partitions (>95%)
  - [ ] Nearly empty partitions (<1% used)
  - [ ] Partitions with special characters in labels

- [ ] **Failure scenario testing**
  - [ ] Disk errors during resize
  - [ ] Power loss simulation (VM snapshots)
  - [ ] Process termination (kill -9)
  - [ ] Out of space conditions
  - [ ] Network interruption (if applicable)

- [ ] **Performance testing**
  - [ ] Measure resize times
    - [ ] Small (10GB → 20GB)
    - [ ] Medium (100GB → 200GB)
    - [ ] Large (500GB → 1TB)
  - [ ] Optimize progress updates
  - [ ] Profile memory usage
  - [ ] Reduce CPU overhead

- [ ] **User acceptance testing**
  - [ ] Have IT team test workflows
  - [ ] Gather feedback on UX
  - [ ] Test on real hardware
  - [ ] Document pain points
  - [ ] Iterate based on feedback

- [ ] **Security testing**
  - [ ] Verify permission checks
  - [ ] Test privilege escalation
  - [ ] Validate input sanitization
  - [ ] Check for race conditions

### Documentation

- [ ] **Write user guide**
  - [ ] Getting started section
  - [ ] Step-by-step resize tutorial (with screenshots)
  - [ ] Best practices
  - [ ] Safety guidelines
  - [ ] Troubleshooting section
  - [ ] FAQ

- [ ] **Document safety best practices**
  - [ ] When to resize partitions
  - [ ] Backup procedures
  - [ ] Risk mitigation strategies
  - [ ] Recovery procedures

- [ ] **Create API documentation**
  - [ ] Generate rustdoc
  - [ ] Tauri command reference
  - [ ] Code examples
  - [ ] Architecture diagrams

- [ ] **Write troubleshooting guide**
  - [ ] Common errors and solutions
  - [ ] Recovery procedures
  - [ ] Support escalation path
  - [ ] Log collection instructions

### Polish

- [ ] **Refine UI/UX**
  - [ ] Improve visual design consistency
  - [ ] Add helpful tooltips everywhere
  - [ ] Enhance error messages
  - [ ] Add success animations
  - [ ] Improve color scheme

- [ ] **Add accessibility features**
  - [ ] Keyboard navigation for all dialogs
  - [ ] Screen reader support (ARIA labels)
  - [ ] High contrast mode support
  - [ ] Focus indicators
  - [ ] Reduced motion option

- [ ] **Performance optimization**
  - [ ] Debounce progress updates
  - [ ] Reduce UI re-renders
  - [ ] Lazy load components
  - [ ] Optimize bundle size

- [ ] **Code quality**
  - [ ] Run clippy (Rust linter)
  - [ ] Run ESLint
  - [ ] Format with rustfmt/prettier
  - [ ] Remove unused code
  - [ ] Add missing type annotations

- [ ] **Final code review**
  - [ ] Security review
  - [ ] Performance review
  - [ ] Code quality check
  - [ ] Remove debug code
  - [ ] Update dependencies

### Release Preparation

- [ ] **Create release notes**
  - [ ] List all features
  - [ ] Document limitations
  - [ ] Known issues
  - [ ] Migration guide (if applicable)

- [ ] **Update README.md**
  - [ ] Add Partition Manager section
  - [ ] Add screenshots
  - [ ] Update feature list
  - [ ] Add badges

- [ ] **Version and tag**
  - [ ] Bump version in Cargo.toml
  - [ ] Bump version in package.json
  - [ ] Update tauri.conf.json version
  - [ ] Create git tag

- [ ] **Build release binaries**
  - [ ] Windows installer (NSIS)
  - [ ] Linux package (AppImage/deb)
  - [ ] macOS bundle (if supported)
  - [ ] Test installers

- [ ] **Create installation guide**
  - [ ] Windows installation steps
  - [ ] Linux installation steps
  - [ ] macOS installation steps
  - [ ] Post-install verification

**Phase 5 Completion Criteria**:
- ✅ All tests passing (unit + integration)
- ✅ No known critical bugs
- ✅ Documentation complete
- ✅ Performance acceptable
- ✅ Accessibility standards met
- ✅ Release artifacts built and tested

---

## Success Metrics

### Minimum Viable Product (MVP) Criteria

1. **Functionality**
   - ✅ View all disks and partitions
   - ✅ Expand NTFS partitions
   - ✅ Shrink NTFS partitions
   - ✅ Resize ext4 partitions
   - ✅ Real-time progress monitoring

2. **Safety**
   - ✅ All pre-flight checks implemented
   - ✅ Multi-step confirmation for risky ops
   - ✅ Transaction rollback functional
   - ✅ Audit logging complete
   - ✅ Zero data loss in testing

3. **User Experience**
   - ✅ Intuitive UI
   - ✅ Clear error messages
   - ✅ Responsive design
   - ✅ Loading states
   - ✅ Visual feedback

4. **Platform Support**
   - ✅ Windows 10/11 fully supported
   - ✅ Linux (Ubuntu/Debian) fully supported
   - ⚠️  macOS: read-only (future enhancement)

5. **Documentation**
   - ✅ User guide published
   - ✅ API docs generated
   - ✅ Troubleshooting guide available

### Post-MVP Features (Future)

- [ ] FAT32 resize support
- [ ] macOS APFS resize support
- [ ] Partition creation/deletion
- [ ] Drive letter management (Windows)
- [ ] Partition cloning
- [ ] Disk imaging/restore
- [ ] Scheduled operations
- [ ] Multi-disk operations

---

## Notes & Decisions

### 2025-12-19
- **Decision**: Started with Phase 1 implementation
- **Note**: Created comprehensive documentation in `PARTITION_MANAGER.md`
- **Note**: Added platform badges to ToolshedPanel
- **Decision**: Using hybrid architecture (wrapper approach) instead of pure Rust implementation for safety

### Technical Decisions

1. **Why wrapper approach?**
   - Leverage battle-tested tools (ntfsresize, resize2fs)
   - Faster development
   - More reliable than reimplementing complex algorithms
   - Easier to maintain

2. **Why not libparted directly for resize?**
   - Limited resize capabilities in libparted
   - Better to use filesystem-specific tools
   - More control over error handling

3. **Platform priority: Windows first, then Linux**
   - Majority of IT support teams use Windows
   - NTFS is most common in enterprise
   - Linux support important but secondary

---

## Risk Register

| Risk | Impact | Probability | Mitigation |
|------|--------|-------------|------------|
| Data loss during resize | Critical | Low | Mandatory backups, extensive testing, dry-run mode |
| Performance issues with large disks | High | Medium | Optimize progress updates, async operations |
| Platform-specific bugs | Medium | High | Test on multiple OSes, VMs, real hardware |
| Dependency conflicts | Medium | Low | Pin dependency versions, test upgrades |
| User bypasses safety checks | High | Low | Multiple confirmation layers, audit logging |
| Insufficient error handling | High | Medium | Comprehensive error types, user-friendly messages |

---

## Team & Resources

### Required Skills
- Rust programming
- React/TypeScript
- Tauri framework
- Filesystem knowledge (NTFS, ext4)
- Disk partitioning concepts
- Testing & QA

### External Dependencies
- ntfs-3g (ntfsresize)
- e2fsprogs (resize2fs, e2fsck)
- libparted (optional)
- Tauri v2.9+

### Documentation References
- [PARTITION_MANAGER.md](./PARTITION_MANAGER.md) - Main implementation guide
- [ntfsresize manual](https://manpages.ubuntu.com/manpages/focal/man8/ntfsresize.8.html)
- [resize2fs manual](https://www.mankier.com/8/resize2fs)
- [Tauri Docs](https://v2.tauri.app/)

---

**Last Updated**: 2025-12-19
**Next Review**: End of Phase 1 (Week 2)
