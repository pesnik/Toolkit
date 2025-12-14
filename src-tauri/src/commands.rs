use tauri::command;
use crate::scanner::{scan_directory, FileNode};
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::SystemTime;
use lazy_static::lazy_static;
use std::path::Path;

struct CacheEntry {
    node: FileNode,
    timestamp: SystemTime,
}

lazy_static! {
    static ref SCAN_CACHE: Mutex<HashMap<String, CacheEntry>> = Mutex::new(HashMap::new());
}

const CACHE_TTL: u64 = 60 * 60; 

fn normalize_path(path: &str) -> String {
    // Basic normalization: use forward slashes for internal key comparison if needed?
    // Actually, OS specifics matter.
    // On Windows C:\Users and C:/Users should be same.
    // Let's use std::path::Path to canonicalize? Canonicalize resolves symlinks which might be slow or unwanted.
    // Let's just standardise separators.
    
    // Use the string representation provided by OS but maybe trim usage of mixed slashes?
    // For cache keys, exact string match is used.
    // If the frontend sends standardized paths, we are good.
    // The previous app used `str(Path(p))` which standardizes to OS native.
    // Let's rely on that or just use the input string if we trust frontend.
    // But frontend sends what it gets from backend.
    
    // Issue: "C:\" vs "C:" ?
    // Let's strip trailing slash unless root.
    let mut s = path.to_string();
    if s.len() > 1 && (s.ends_with('/') || s.ends_with('\\')) {
         // check if it's root (e.g. C:\ or /)
         // On windows C:\ is root.
         let is_root = s.len() == 3 && s.chars().nth(1) == Some(':');
         if !is_root && s != "/" {
             s.pop();
         }
    }
    s
}

#[command]
pub async fn scan_dir(path: String) -> Result<FileNode, String> {
    scan_dir_internal(path, false).await
}

#[command]
pub async fn refresh_scan(path: String) -> Result<FileNode, String> {
    scan_dir_internal(path, true).await
}

async fn scan_dir_internal(path: String, force_refresh: bool) -> Result<FileNode, String> {
    // Normalize path for cache key
    let key = normalize_path(&path);

    // Check cache
    if !force_refresh {
        let cache = SCAN_CACHE.lock().map_err(|e| e.to_string())?;
        if let Some(entry) = cache.get(&key) {
            if let Ok(elapsed) = entry.timestamp.elapsed() {
                if elapsed.as_secs() < CACHE_TTL {
                    return Ok(entry.node.clone());
                }
            }
        }
    }

    let path_clone = path.clone();
    let result = tauri::async_runtime::spawn_blocking(move || {
        scan_directory(&path_clone)
    }).await.map_err(|e| e.to_string())??;

    // Update cache
    let mut cache = SCAN_CACHE.lock().map_err(|e| e.to_string())?;
    let now = SystemTime::now();
    
    // Cache the main result
    cache.insert(key.clone(), CacheEntry {
        node: result.clone(),
        timestamp: now,
    });
    
    // CACHE LOOKAHEAD: Cache the children nodes too!
    if let Some(children) = &result.children {
        for child in children {
            // We need to clone, but we should probably strip *their* children if we went deeper?
            // Currently scanner goes 2 levels deep. 
            // Level 0: Root (A)
            // Level 1: Child (B) -> Has children details (D, E) populated.
            // Level 2: Grandchild (D) -> children=None.
            
            // So 'child' here is 'B'. It has .children populated.
            // We can cache 'B' directly!
            let child_key = normalize_path(&child.path);
            cache.insert(child_key, CacheEntry {
                node: child.clone(),
                timestamp: now,
            });
        }
    }

    Ok(result)
}

#[command]
pub fn clear_cache() {
    if let Ok(mut cache) = SCAN_CACHE.lock() {
        cache.clear();
    }
}

#[command]
pub fn reveal_in_explorer(path: String) {
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        Command::new("explorer")
            .arg("/select,")
            .arg(&path)
            .spawn()
            .unwrap();
    }
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        Command::new("open")
            .arg("-R")
            .arg(&path)
            .spawn()
            .unwrap();
    }
    #[cfg(target_os = "linux")]
    {
        use std::process::Command;
        // Try to select if possible, otherwise just open parent
        // dbus-send or specific file manager calls would be improved here.
        // For now, let's just open the parent folder.
        let p = std::path::Path::new(&path);
        if let Some(parent) = p.parent() {
             Command::new("xdg-open")
                .arg(parent)
                .spawn()
                .unwrap();
        }
    }
}

#[command]
pub fn open_file(path: String) {
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        Command::new("explorer")
            .arg(&path)
            .spawn()
            .unwrap();
    }
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        Command::new("open")
            .arg(&path)
            .spawn()
            .unwrap();
    }
    #[cfg(target_os = "linux")]
    {
        use std::process::Command;
        Command::new("xdg-open")
            .arg(&path)
            .spawn()
            .unwrap();
    }
}

#[command]
pub fn delete_item(path: String) -> Result<(), String> {
    let p = Path::new(&path);
    if !p.exists() {
        return Err("Path does not exist".to_string());
    }

    if p.is_dir() {
        std::fs::remove_dir_all(p).map_err(|e| e.to_string())?;
    } else {
        std::fs::remove_file(p).map_err(|e| e.to_string())?;
    }
    
    // Invalidate cache for parent or just clear all for safety?
    // Let's clear for now to be safe as size calc up the tree changes.
    clear_cache();
    
    Ok(())
}

#[command]
pub fn get_drives() -> Vec<FileNode> {
    let mut drives = Vec::new();

    #[cfg(target_os = "windows")]
    {
        // Simple iteration from A to Z to find mounts
        for i in b'A'..=b'Z' {
            let drive_letter = i as char;
            let path = format!("{}:\\", drive_letter);
            if Path::new(&path).exists() {
                drives.push(FileNode {
                    name: format!("Local Disk ({}:)", drive_letter),
                    path,
                    size: 0, // Need meaningful size? Scan would be slow. 0 for now.
                    is_dir: true,
                    children: None,
                    last_modified: 0,
                    file_count: 0,
                });
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        // Root
        drives.push(FileNode {
            name: "Macintosh HD".to_string(), // Typical name, hardcoded or just Root
            path: "/".to_string(),
            size: 0,
            is_dir: true,
            children: None,
            last_modified: 0,
            file_count: 0
        });

        // External Volumes
        if let Ok(entries) = std::fs::read_dir("/Volumes") {
            for entry in entries.flatten() {
                 let path = entry.path();
                 let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                 // Skip some typical hidden ones?
                 drives.push(FileNode {
                    name,
                    path: path.to_string_lossy().to_string(),
                    size: 0,
                    is_dir: true,
                    children: None,
                    last_modified: 0,
                    file_count: 0
                });
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        // Just Root for now
        drives.push(FileNode {
            name: "Root /".to_string(),
            path: "/".to_string(),
            size: 0,
            is_dir: true,
            children: None,
            last_modified: 0,
            file_count: 0
        });
        // Could check /media or /mnt
    }

    drives
}
