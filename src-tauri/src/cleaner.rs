use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::fs;
use std::time::SystemTime;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JunkItem {
    pub path: String,
    pub name: String,
    pub size: u64,
    pub description: String,
    pub age_days: Option<u32>, // Age in days since last modified
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JunkCategory {
    pub id: String,
    pub name: String,
    pub description: String,
    pub items: Vec<JunkItem>,
    pub total_size: u64,
    pub icon: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CleaningOptions {
    pub min_age_days: Option<u32>, // Only delete files older than this
    pub dry_run: bool, // If true, don't actually delete, just return what would be deleted
    pub skip_errors: bool, // If true, continue on errors instead of stopping
}

impl Default for CleaningOptions {
    fn default() -> Self {
        Self {
            min_age_days: None,
            dry_run: false,
            skip_errors: true,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeletionResult {
    pub deleted_count: usize,
    pub deleted_size: u64,
    pub failed_count: usize,
    pub errors: Vec<String>,
    pub skipped_count: usize, // Files skipped due to age filter
}

#[derive(Debug, Clone)]
struct CleaningPath {
    category_id: &'static str,
    category_name: &'static str,
    path_template: &'static str,
    description: &'static str,
    supports_wildcards: bool,
}

// macOS cleaning paths
#[cfg(target_os = "macos")]
fn get_cleaning_paths() -> Vec<CleaningPath> {
    vec![
        // System Caches
        CleaningPath {
            category_id: "system_cache",
            category_name: "System Caches",
            path_template: "~/Library/Caches",
            description: "User application caches",
            supports_wildcards: false,
        },
        CleaningPath {
            category_id: "system_cache",
            category_name: "System Caches",
            path_template: "/Library/Caches",
            description: "System-wide application caches",
            supports_wildcards: false,
        },
        
        // System Logs
        CleaningPath {
            category_id: "system_logs",
            category_name: "System Logs",
            path_template: "~/Library/Logs",
            description: "User application logs",
            supports_wildcards: false,
        },
        CleaningPath {
            category_id: "system_logs",
            category_name: "System Logs",
            path_template: "/Library/Logs",
            description: "System application logs",
            supports_wildcards: false,
        },
        CleaningPath {
            category_id: "system_logs",
            category_name: "System Logs",
            path_template: "/private/var/log",
            description: "System logs",
            supports_wildcards: false,
        },
        
        // Temporary Files
        CleaningPath {
            category_id: "temp_files",
            category_name: "Temporary Files",
            path_template: "/tmp",
            description: "Temporary files",
            supports_wildcards: false,
        },
        CleaningPath {
            category_id: "temp_files",
            category_name: "Temporary Files",
            path_template: "/var/tmp",
            description: "Persistent temporary files",
            supports_wildcards: false,
        },
        CleaningPath {
            category_id: "temp_files",
            category_name: "Temporary Files",
            path_template: "~/Library/Saved Application State",
            description: "Application state files",
            supports_wildcards: false,
        },
        
        // Browser Caches
        CleaningPath {
            category_id: "browser_cache",
            category_name: "Browser Caches",
            path_template: "~/Library/Caches/Google/Chrome/Default",
            description: "Chrome browser cache",
            supports_wildcards: false,
        },
        CleaningPath {
            category_id: "browser_cache",
            category_name: "Browser Caches",
            path_template: "~/Library/Caches/Firefox/Profiles",
            description: "Firefox browser cache",
            supports_wildcards: false,
        },
        CleaningPath {
            category_id: "browser_cache",
            category_name: "Browser Caches",
            path_template: "~/Library/Containers/com.apple.Safari/Data/Library/Caches/com.apple.Safari/WebKitCache",
            description: "Safari browser cache",
            supports_wildcards: false,
        },
        CleaningPath {
            category_id: "browser_cache",
            category_name: "Browser Caches",
            path_template: "~/Library/Caches/Microsoft Edge/Default/Cache",
            description: "Edge browser cache",
            supports_wildcards: false,
        },
        
        // Developer Tools
        CleaningPath {
            category_id: "developer_cache",
            category_name: "Developer Caches",
            path_template: "~/Library/Developer/Xcode/DerivedData",
            description: "Xcode build artifacts",
            supports_wildcards: false,
        },
        CleaningPath {
            category_id: "developer_cache",
            category_name: "Developer Caches",
            path_template: "~/Library/Developer/Xcode/Archives",
            description: "Xcode archives",
            supports_wildcards: false,
        },
        CleaningPath {
            category_id: "developer_cache",
            category_name: "Developer Caches",
            path_template: "~/Library/Developer/CoreSimulator/Caches",
            description: "iOS Simulator caches",
            supports_wildcards: false,
        },
        CleaningPath {
            category_id: "developer_cache",
            category_name: "Developer Caches",
            path_template: "~/.gradle/caches",
            description: "Gradle build cache",
            supports_wildcards: false,
        },
        CleaningPath {
            category_id: "developer_cache",
            category_name: "Developer Caches",
            path_template: "~/.npm",
            description: "npm package cache",
            supports_wildcards: false,
        },
        CleaningPath {
            category_id: "developer_cache",
            category_name: "Developer Caches",
            path_template: "~/.yarn/cache",
            description: "Yarn package cache",
            supports_wildcards: false,
        },
        CleaningPath {
            category_id: "developer_cache",
            category_name: "Developer Caches",
            path_template: "~/.cache/yarn",
            description: "Yarn cache (alternative)",
            supports_wildcards: false,
        },
        CleaningPath {
            category_id: "developer_cache",
            category_name: "Developer Caches",
            path_template: "~/Library/Caches/com.apple.dt.Xcode",
            description: "Xcode caches",
            supports_wildcards: false,
        },
        CleaningPath {
            category_id: "developer_cache",
            category_name: "Developer Caches",
            path_template: "~/.android/build-cache",
            description: "Android build cache",
            supports_wildcards: false,
        },
        CleaningPath {
            category_id: "developer_cache",
            category_name: "Developer Caches",
            path_template: "~/.vscode/extensions",
            description: "VS Code extensions cache",
            supports_wildcards: false,
        },
        CleaningPath {
            category_id: "developer_cache",
            category_name: "Developer Caches",
            path_template: "~/.cargo/registry",
            description: "Rust cargo cache",
            supports_wildcards: false,
        },
    ]
}

// Windows cleaning paths
#[cfg(target_os = "windows")]
fn get_cleaning_paths() -> Vec<CleaningPath> {
    vec![
        // Temporary Files
        CleaningPath {
            category_id: "temp_files",
            category_name: "Temporary Files",
            path_template: "%TEMP%",
            description: "User temporary files",
            supports_wildcards: false,
        },
        CleaningPath {
            category_id: "temp_files",
            category_name: "Temporary Files",
            path_template: "%LOCALAPPDATA%\\Temp",
            description: "Local AppData temp files",
            supports_wildcards: false,
        },
        CleaningPath {
            category_id: "temp_files",
            category_name: "Temporary Files",
            path_template: "C:\\Windows\\Temp",
            description: "Windows system temporary files",
            supports_wildcards: false,
        },
        
        // System Caches
        CleaningPath {
            category_id: "system_cache",
            category_name: "System Caches",
            path_template: "C:\\Windows\\Prefetch",
            description: "Prefetch files",
            supports_wildcards: false,
        },
        CleaningPath {
            category_id: "system_cache",
            category_name: "System Caches",
            path_template: "C:\\Windows\\SoftwareDistribution\\Download",
            description: "Windows Update cache",
            supports_wildcards: false,
        },
        CleaningPath {
            category_id: "system_cache",
            category_name: "System Caches",
            path_template: "%LOCALAPPDATA%\\Microsoft\\Windows\\Explorer\\ThumbCacheToDelete",
            description: "Thumbnail cache",
            supports_wildcards: false,
        },
        CleaningPath {
            category_id: "system_cache",
            category_name: "System Caches",
            path_template: "%LOCALAPPDATA%\\Microsoft\\Windows\\INetCache",
            description: "Internet Explorer cache",
            supports_wildcards: false,
        },
        CleaningPath {
            category_id: "system_cache",
            category_name: "System Caches",
            path_template: "%LOCALAPPDATA%\\CrashDumps",
            description: "Crash dump files",
            supports_wildcards: false,
        },
        
        // System Logs
        CleaningPath {
            category_id: "system_logs",
            category_name: "System Logs",
            path_template: "C:\\Windows\\Logs",
            description: "Windows logs",
            supports_wildcards: false,
        },
        CleaningPath {
            category_id: "system_logs",
            category_name: "System Logs",
            path_template: "C:\\Windows\\Panther",
            description: "Windows installation logs",
            supports_wildcards: false,
        },
        
        // Browser Caches
        CleaningPath {
            category_id: "browser_cache",
            category_name: "Browser Caches",
            path_template: "%LOCALAPPDATA%\\Google\\Chrome\\User Data\\Default\\Cache",
            description: "Chrome browser cache",
            supports_wildcards: false,
        },
        CleaningPath {
            category_id: "browser_cache",
            category_name: "Browser Caches",
            path_template: "%APPDATA%\\Mozilla\\Firefox\\Profiles",
            description: "Firefox browser cache",
            supports_wildcards: false,
        },
        CleaningPath {
            category_id: "browser_cache",
            category_name: "Browser Caches",
            path_template: "%LOCALAPPDATA%\\Microsoft\\Edge\\User Data\\Default\\Cache",
            description: "Edge browser cache",
            supports_wildcards: false,
        },
        
        // Developer Tools
        CleaningPath {
            category_id: "developer_cache",
            category_name: "Developer Caches",
            path_template: "%USERPROFILE%\\.gradle\\caches",
            description: "Gradle build cache",
            supports_wildcards: false,
        },
        CleaningPath {
            category_id: "developer_cache",
            category_name: "Developer Caches",
            path_template: "%LOCALAPPDATA%\\npm-cache",
            description: "npm package cache",
            supports_wildcards: false,
        },
        CleaningPath {
            category_id: "developer_cache",
            category_name: "Developer Caches",
            path_template: "%LOCALAPPDATA%\\Yarn\\cache",
            description: "Yarn package cache",
            supports_wildcards: false,
        },
        CleaningPath {
            category_id: "developer_cache",
            category_name: "Developer Caches",
            path_template: "%APPDATA%\\Code\\Cache",
            description: "VS Code cache",
            supports_wildcards: false,
        },
        CleaningPath {
            category_id: "developer_cache",
            category_name: "Developer Caches",
            path_template: "%APPDATA%\\Code\\CachedExtensionVSIXs",
            description: "VS Code extensions cache",
            supports_wildcards: false,
        },
        CleaningPath {
            category_id: "developer_cache",
            category_name: "Developer Caches",
            path_template: "%LOCALAPPDATA%\\Android\\build-cache",
            description: "Android build cache",
            supports_wildcards: false,
        },
        CleaningPath {
            category_id: "developer_cache",
            category_name: "Developer Caches",
            path_template: "%USERPROFILE%\\.cargo\\registry",
            description: "Rust cargo cache",
            supports_wildcards: false,
        },
    ]
}

// Linux cleaning paths
#[cfg(target_os = "linux")]
fn get_cleaning_paths() -> Vec<CleaningPath> {
    vec![
        // System Caches
        CleaningPath {
            category_id: "system_cache",
            category_name: "System Caches",
            path_template: "~/.cache",
            description: "User application caches",
            supports_wildcards: false,
        },
        
        // Temporary Files
        CleaningPath {
            category_id: "temp_files",
            category_name: "Temporary Files",
            path_template: "/tmp",
            description: "Temporary files",
            supports_wildcards: false,
        },
        CleaningPath {
            category_id: "temp_files",
            category_name: "Temporary Files",
            path_template: "/var/tmp",
            description: "Persistent temporary files",
            supports_wildcards: false,
        },
        
        // System Logs
        CleaningPath {
            category_id: "system_logs",
            category_name: "System Logs",
            path_template: "/var/log",
            description: "System logs",
            supports_wildcards: false,
        },
        CleaningPath {
            category_id: "system_logs",
            category_name: "System Logs",
            path_template: "~/.xsession-errors",
            description: "X session errors",
            supports_wildcards: false,
        },
        
        // Trash
        CleaningPath {
            category_id: "trash",
            category_name: "Trash",
            path_template: "~/.local/share/Trash",
            description: "User trash",
            supports_wildcards: false,
        },
        
        // Browser Caches
        CleaningPath {
            category_id: "browser_cache",
            category_name: "Browser Caches",
            path_template: "~/.cache/google-chrome/Default/Cache",
            description: "Chrome browser cache",
            supports_wildcards: false,
        },
        CleaningPath {
            category_id: "browser_cache",
            category_name: "Browser Caches",
            path_template: "~/.cache/mozilla/firefox",
            description: "Firefox browser cache",
            supports_wildcards: false,
        },
        CleaningPath {
            category_id: "browser_cache",
            category_name: "Browser Caches",
            path_template: "~/.cache/chromium/Default/Cache",
            description: "Chromium browser cache",
            supports_wildcards: false,
        },
        
        // Package Manager Caches
        CleaningPath {
            category_id: "package_cache",
            category_name: "Package Manager Caches",
            path_template: "/var/cache/apt/archives",
            description: "APT package cache",
            supports_wildcards: false,
        },
        CleaningPath {
            category_id: "package_cache",
            category_name: "Package Manager Caches",
            path_template: "/var/cache/dnf",
            description: "DNF package cache",
            supports_wildcards: false,
        },
        CleaningPath {
            category_id: "package_cache",
            category_name: "Package Manager Caches",
            path_template: "/var/cache/yum",
            description: "YUM package cache",
            supports_wildcards: false,
        },
        
        // Developer Tools
        CleaningPath {
            category_id: "developer_cache",
            category_name: "Developer Caches",
            path_template: "~/.gradle/caches",
            description: "Gradle build cache",
            supports_wildcards: false,
        },
        CleaningPath {
            category_id: "developer_cache",
            category_name: "Developer Caches",
            path_template: "~/.npm",
            description: "npm package cache",
            supports_wildcards: false,
        },
        CleaningPath {
            category_id: "developer_cache",
            category_name: "Developer Caches",
            path_template: "~/.cache/yarn",
            description: "Yarn package cache",
            supports_wildcards: false,
        },
        CleaningPath {
            category_id: "developer_cache",
            category_name: "Developer Caches",
            path_template: "~/.cache/pip",
            description: "Python pip cache",
            supports_wildcards: false,
        },
        CleaningPath {
            category_id: "developer_cache",
            category_name: "Developer Caches",
            path_template: "~/.cargo/registry",
            description: "Rust cargo cache",
            supports_wildcards: false,
        },
        CleaningPath {
            category_id: "developer_cache",
            category_name: "Developer Caches",
            path_template: "~/.m2/repository",
            description: "Maven repository cache",
            supports_wildcards: false,
        },
        CleaningPath {
            category_id: "developer_cache",
            category_name: "Developer Caches",
            path_template: "~/.vscode/extensions",
            description: "VS Code extensions",
            supports_wildcards: false,
        },
        CleaningPath {
            category_id: "developer_cache",
            category_name: "Developer Caches",
            path_template: "~/.config/Code/CachedData",
            description: "VS Code cache",
            supports_wildcards: false,
        },
    ]
}

fn expand_path(path: &str) -> Option<PathBuf> {
    use std::env;
    
    let mut expanded = path.to_string();
    
    // Handle tilde expansion
    if expanded.starts_with('~') {
        if let Some(home_dir) = dirs::home_dir() {
            if expanded == "~" {
                return Some(home_dir);
            }
            expanded = expanded.replacen("~", &home_dir.to_string_lossy(), 1);
        }
    }
    
    // Handle environment variables
    #[cfg(target_os = "windows")]
    {
        // Windows environment variable expansion
        let env_vars = vec![
            ("TEMP", env::var("TEMP").or_else(|_| env::var("TMP")).ok()),
            ("LOCALAPPDATA", env::var("LOCALAPPDATA").ok()),
            ("APPDATA", env::var("APPDATA").ok()),
            ("USERPROFILE", env::var("USERPROFILE").ok()),
            ("PROGRAMDATA", env::var("PROGRAMDATA").ok()),
            ("PUBLIC", env::var("PUBLIC").ok()),
        ];
        
        for (var_name, var_value) in env_vars {
            if let Some(value) = var_value {
                let pattern = format!("%{}%", var_name);
                expanded = expanded.replace(&pattern, &value);
            }
        }
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        // Unix-like environment variable expansion
        if let Some(tmpdir) = env::var("TMPDIR").ok() {
            expanded = expanded.replace("$TMPDIR", &tmpdir);
        }
        if let Some(user) = env::var("USER").ok() {
            expanded = expanded.replace("$USER", &user);
        }
    }
    
    let path_buf = PathBuf::from(expanded);
    if path_buf.exists() {
        Some(path_buf)
    } else {
        None
    }
}

fn calculate_dir_size(path: &Path) -> u64 {
    match fs_extra::dir::get_size(path) {
        Ok(size) => size,
        Err(_) => 0,
    }
}

fn get_file_age_days(metadata: &fs::Metadata) -> Option<u32> {
    metadata
        .modified()
        .ok()
        .and_then(|modified| {
            SystemTime::now()
                .duration_since(modified)
                .ok()
        })
        .map(|duration| (duration.as_secs() / 86400) as u32)
}

pub fn scan_junk_items() -> Vec<JunkCategory> {
    scan_junk_items_with_options(CleaningOptions::default())
}

pub fn scan_junk_items_with_options(options: CleaningOptions) -> Vec<JunkCategory> {
    let mut categories: Vec<JunkCategory> = Vec::new();
    let cleaning_paths = get_cleaning_paths();
    
    for cleaning_path in cleaning_paths {
        if let Some(path) = expand_path(cleaning_path.path_template) {
            if !path.exists() {
                continue;
            }
            
            let mut items = Vec::new();
            let mut total_size = 0;
            
            // Scan directory contents
            if let Ok(read_dir) = fs::read_dir(&path) {
                for entry in read_dir.flatten() {
                    if let Ok(meta) = entry.metadata() {
                        // Calculate age
                        let age_days = get_file_age_days(&meta);
                        
                        // Apply age filter if specified
                        if let Some(min_age) = options.min_age_days {
                            if let Some(age) = age_days {
                                if age < min_age {
                                    continue; // Skip files that are too new
                                }
                            } else {
                                continue; // Skip if we can't determine age
                            }
                        }
                        
                        let size = if meta.is_dir() {
                            calculate_dir_size(&entry.path())
                        } else {
                            meta.len()
                        };
                        
                        total_size += size;
                        
                        items.push(JunkItem {
                            path: entry.path().to_string_lossy().to_string(),
                            name: entry.file_name().to_string_lossy().to_string(),
                            size,
                            description: cleaning_path.description.to_string(),
                            age_days,
                        });
                    }
                }
            }
            
            if !items.is_empty() {
                // Check if category already exists
                if let Some(cat) = categories.iter_mut().find(|c| c.id == cleaning_path.category_id) {
                    cat.items.extend(items);
                    cat.total_size += total_size;
                } else {
                    categories.push(JunkCategory {
                        id: cleaning_path.category_id.to_string(),
                        name: cleaning_path.category_name.to_string(),
                        description: format!("Files in {}", cleaning_path.category_name),
                        items,
                        total_size,
                        icon: cleaning_path.category_id.to_string(),
                    });
                }
            }
        }
    }
    
    categories
}

pub fn delete_junk_items(paths: Vec<String>) -> Result<(), String> {
    let result = delete_junk_items_with_options(paths, CleaningOptions::default())?;
    
    if result.failed_count > 0 {
        Err(result.errors.join("\n"))
    } else {
        Ok(())
    }
}

pub fn delete_junk_items_with_options(
    paths: Vec<String>,
    options: CleaningOptions,
) -> Result<DeletionResult, String> {
    let mut deleted_count = 0;
    let mut deleted_size = 0;
    let mut failed_count = 0;
    let mut skipped_count = 0;
    let mut errors = Vec::new();
    
    for path in paths {
        let p = Path::new(&path);
        
        if !p.exists() {
            if !options.skip_errors {
                return Err(format!("Path does not exist: {}", path));
            }
            errors.push(format!("Path does not exist: {}", path));
            failed_count += 1;
            continue;
        }
        
        // Get metadata for age check and size
        let metadata = match fs::metadata(p) {
            Ok(m) => m,
            Err(e) => {
                if !options.skip_errors {
                    return Err(format!("Failed to get metadata for {}: {}", path, e));
                }
                errors.push(format!("Failed to get metadata for {}: {}", path, e));
                failed_count += 1;
                continue;
            }
        };
        
        // Apply age filter if specified
        if let Some(min_age) = options.min_age_days {
            if let Some(age) = get_file_age_days(&metadata) {
                if age < min_age {
                    skipped_count += 1;
                    continue; // Skip files that are too new
                }
            } else {
                skipped_count += 1;
                continue; // Skip if we can't determine age
            }
        }
        
        // Calculate size before deletion
        let size = if metadata.is_dir() {
            calculate_dir_size(p)
        } else {
            metadata.len()
        };
        
        // Dry run mode - don't actually delete
        if options.dry_run {
            deleted_count += 1;
            deleted_size += size;
            continue;
        }
        
        // Perform actual deletion
        let result = if p.is_file() {
            fs::remove_file(p)
        } else if p.is_dir() {
            fs::remove_dir_all(p)
        } else {
            skipped_count += 1;
            continue;
        };
        
        match result {
            Ok(_) => {
                deleted_count += 1;
                deleted_size += size;
            }
            Err(e) => {
                if !options.skip_errors {
                    return Err(format!("Failed to delete {}: {}", path, e));
                }
                errors.push(format!("Failed to delete {}: {}", path, e));
                failed_count += 1;
            }
        }
    }
    
    Ok(DeletionResult {
        deleted_count,
        deleted_size,
        failed_count,
        errors,
        skipped_count,
    })
}
