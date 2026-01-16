use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{IdbDatabase, IdbObjectStore, IdbRequest, IdbTransaction, IdbTransactionMode};

const DB_NAME: &str = "virtualmac_fs";
const DB_VERSION: u32 = 1;
const STORE_NAME: &str = "files";

/// File system entry type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EntryType {
    File,
    Directory,
}

/// Metadata for a file system entry
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileMetadata {
    pub name: String,
    pub path: String,
    pub entry_type: EntryType,
    pub size: usize,
    pub icon: String,
    pub created: f64,
    pub modified: f64,
}

/// A file system entry (file or directory)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileEntry {
    pub metadata: FileMetadata,
    /// For files: the content. For directories: None
    pub content: Option<String>,
    /// For directories: list of child paths. For files: None
    pub children: Option<Vec<String>>,
}

impl FileEntry {
    pub fn new_file(path: &str, name: &str, content: &str, icon: &str) -> Self {
        let now = js_sys::Date::now();
        Self {
            metadata: FileMetadata {
                name: name.to_string(),
                path: path.to_string(),
                entry_type: EntryType::File,
                size: content.len(),
                icon: icon.to_string(),
                created: now,
                modified: now,
            },
            content: Some(content.to_string()),
            children: None,
        }
    }

    pub fn new_directory(path: &str, name: &str) -> Self {
        let now = js_sys::Date::now();
        Self {
            metadata: FileMetadata {
                name: name.to_string(),
                path: path.to_string(),
                entry_type: EntryType::Directory,
                size: 0,
                icon: "📁".to_string(),
                created: now,
                modified: now,
            },
            content: None,
            children: Some(Vec::new()),
        }
    }

    pub fn is_directory(&self) -> bool {
        matches!(self.metadata.entry_type, EntryType::Directory)
    }

    pub fn is_file(&self) -> bool {
        matches!(self.metadata.entry_type, EntryType::File)
    }
}

/// Virtual file system state
#[derive(Clone)]
pub struct VirtualFileSystem {
    /// In-memory cache of file entries (path -> entry)
    pub entries: RwSignal<HashMap<String, FileEntry>>,
    /// Signal to trigger re-renders when FS changes
    pub version: RwSignal<u32>,
    /// Whether the FS has been initialized
    pub initialized: RwSignal<bool>,
}

impl VirtualFileSystem {
    pub fn new() -> Self {
        Self {
            entries: RwSignal::new(HashMap::new()),
            version: RwSignal::new(0),
            initialized: RwSignal::new(false),
        }
    }

    /// Initialize the file system with default structure
    pub fn init_default_structure(&self) {
        let mut entries = HashMap::new();

        // Root directories
        entries.insert("/".to_string(), FileEntry::new_directory("/", ""));
        entries.insert("/Applications".to_string(), FileEntry::new_directory("/Applications", "Applications"));
        entries.insert("/Desktop".to_string(), FileEntry::new_directory("/Desktop", "Desktop"));
        entries.insert("/Documents".to_string(), FileEntry::new_directory("/Documents", "Documents"));
        entries.insert("/Downloads".to_string(), FileEntry::new_directory("/Downloads", "Downloads"));

        // Applications
        let apps = vec![
            ("Safari", "🧭"),
            ("Mail", "✉️"),
            ("Calendar", "📅"),
            ("Notes", "📝"),
            ("Reminders", "☑️"),
            ("Music", "🎵"),
            ("Photos", "🖼"),
            ("Messages", "💬"),
            ("FaceTime", "📹"),
            ("Maps", "🗺"),
            ("Terminal", "⌨️"),
            ("System Settings", "⚙️"),
        ];
        for (name, icon) in apps {
            let path = format!("/Applications/{}", name);
            entries.insert(path.clone(), FileEntry::new_file(&path, name, "", icon));
        }

        // Desktop files
        entries.insert(
            "/Desktop/Projects".to_string(),
            FileEntry::new_directory("/Desktop/Projects", "Projects"),
        );
        entries.insert(
            "/Desktop/Screenshot.png".to_string(),
            FileEntry::new_file("/Desktop/Screenshot.png", "Screenshot.png", "", "🖼"),
        );
        entries.insert(
            "/Desktop/Notes.txt".to_string(),
            FileEntry::new_file("/Desktop/Notes.txt", "Notes.txt", "My notes go here.", "📄"),
        );

        // Documents
        entries.insert(
            "/Documents/Work".to_string(),
            FileEntry::new_directory("/Documents/Work", "Work"),
        );
        entries.insert(
            "/Documents/Personal".to_string(),
            FileEntry::new_directory("/Documents/Personal", "Personal"),
        );
        entries.insert(
            "/Documents/Resume.pdf".to_string(),
            FileEntry::new_file("/Documents/Resume.pdf", "Resume.pdf", "", "📕"),
        );
        entries.insert(
            "/Documents/Budget.xlsx".to_string(),
            FileEntry::new_file("/Documents/Budget.xlsx", "Budget.xlsx", "", "📊"),
        );
        entries.insert(
            "/Documents/Notes.txt".to_string(),
            FileEntry::new_file("/Documents/Notes.txt", "Notes.txt", "Document notes.", "📄"),
        );

        // Downloads
        entries.insert(
            "/Downloads/installer.dmg".to_string(),
            FileEntry::new_file("/Downloads/installer.dmg", "installer.dmg", "", "💿"),
        );
        entries.insert(
            "/Downloads/photo.jpg".to_string(),
            FileEntry::new_file("/Downloads/photo.jpg", "photo.jpg", "", "🖼"),
        );
        entries.insert(
            "/Downloads/document.pdf".to_string(),
            FileEntry::new_file("/Downloads/document.pdf", "document.pdf", "", "📕"),
        );
        entries.insert(
            "/Downloads/archive.zip".to_string(),
            FileEntry::new_file("/Downloads/archive.zip", "archive.zip", "", "📦"),
        );

        // Update children lists for directories
        let paths: Vec<String> = entries.keys().cloned().collect();
        for path in &paths {
            if path == "/" {
                continue;
            }
            let parent_path = get_parent_path(path);
            if let Some(parent) = entries.get_mut(&parent_path) {
                if let Some(ref mut children) = parent.children {
                    if !children.contains(path) {
                        children.push(path.clone());
                    }
                }
            }
        }

        // Update root children
        if let Some(root) = entries.get_mut("/") {
            root.children = Some(vec![
                "/Applications".to_string(),
                "/Desktop".to_string(),
                "/Documents".to_string(),
                "/Downloads".to_string(),
            ]);
        }

        self.entries.set(entries);
        self.initialized.set(true);
        self.bump_version();
    }

    /// Bump version to trigger re-renders
    fn bump_version(&self) {
        self.version.update(|v| *v += 1);
    }

    /// List directory contents
    pub fn list_dir(&self, path: &str) -> Vec<FileEntry> {
        let entries = self.entries.get();
        let normalized_path = normalize_path(path);

        if let Some(dir) = entries.get(&normalized_path) {
            if let Some(ref children) = dir.children {
                return children
                    .iter()
                    .filter_map(|child_path| entries.get(child_path).cloned())
                    .collect();
            }
        }
        Vec::new()
    }

    /// Get a file or directory by path
    pub fn get(&self, path: &str) -> Option<FileEntry> {
        let entries = self.entries.get();
        entries.get(&normalize_path(path)).cloned()
    }

    /// Read file content
    pub fn read_file(&self, path: &str) -> Option<String> {
        self.get(path).and_then(|e| e.content)
    }

    /// Write/create a file
    pub fn write_file(&self, path: &str, content: &str, icon: &str) {
        let normalized_path = normalize_path(path);
        let name = get_file_name(&normalized_path);
        let parent_path = get_parent_path(&normalized_path);

        self.entries.update(|entries| {
            // Create or update the file
            let entry = FileEntry::new_file(&normalized_path, &name, content, icon);
            entries.insert(normalized_path.clone(), entry);

            // Add to parent's children
            if let Some(parent) = entries.get_mut(&parent_path) {
                if let Some(ref mut children) = parent.children {
                    if !children.contains(&normalized_path) {
                        children.push(normalized_path.clone());
                    }
                }
            }
        });

        self.bump_version();
        self.save_to_storage();
    }

    /// Create a directory
    pub fn create_dir(&self, path: &str) {
        let normalized_path = normalize_path(path);
        let name = get_file_name(&normalized_path);
        let parent_path = get_parent_path(&normalized_path);

        self.entries.update(|entries| {
            // Create the directory
            let entry = FileEntry::new_directory(&normalized_path, &name);
            entries.insert(normalized_path.clone(), entry);

            // Add to parent's children
            if let Some(parent) = entries.get_mut(&parent_path) {
                if let Some(ref mut children) = parent.children {
                    if !children.contains(&normalized_path) {
                        children.push(normalized_path.clone());
                    }
                }
            }
        });

        self.bump_version();
        self.save_to_storage();
    }

    /// Delete a file or directory
    pub fn delete(&self, path: &str) {
        let normalized_path = normalize_path(path);
        let parent_path = get_parent_path(&normalized_path);

        self.entries.update(|entries| {
            // If it's a directory, recursively delete children
            if let Some(entry) = entries.get(&normalized_path).cloned() {
                if let Some(children) = entry.children {
                    for child_path in children {
                        entries.remove(&child_path);
                    }
                }
            }

            // Remove the entry
            entries.remove(&normalized_path);

            // Remove from parent's children
            if let Some(parent) = entries.get_mut(&parent_path) {
                if let Some(ref mut children) = parent.children {
                    children.retain(|p| p != &normalized_path);
                }
            }
        });

        self.bump_version();
        self.save_to_storage();
    }

    /// Rename/move a file or directory
    pub fn rename(&self, old_path: &str, new_path: &str) {
        let old_normalized = normalize_path(old_path);
        let new_normalized = normalize_path(new_path);
        let old_parent = get_parent_path(&old_normalized);
        let new_parent = get_parent_path(&new_normalized);
        let new_name = get_file_name(&new_normalized);

        self.entries.update(|entries| {
            if let Some(mut entry) = entries.remove(&old_normalized) {
                // Update entry metadata
                entry.metadata.path = new_normalized.clone();
                entry.metadata.name = new_name;
                entry.metadata.modified = js_sys::Date::now();

                // Insert at new path
                entries.insert(new_normalized.clone(), entry);

                // Update old parent's children
                if let Some(parent) = entries.get_mut(&old_parent) {
                    if let Some(ref mut children) = parent.children {
                        children.retain(|p| p != &old_normalized);
                    }
                }

                // Update new parent's children
                if let Some(parent) = entries.get_mut(&new_parent) {
                    if let Some(ref mut children) = parent.children {
                        if !children.contains(&new_normalized) {
                            children.push(new_normalized.clone());
                        }
                    }
                }
            }
        });

        self.bump_version();
        self.save_to_storage();
    }

    /// Check if a path exists
    pub fn exists(&self, path: &str) -> bool {
        self.entries.get().contains_key(&normalize_path(path))
    }

    /// Save file system to localStorage
    pub fn save_to_storage(&self) {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    let entries = self.entries.get();
                    if let Ok(json) = serde_json::to_string(&entries) {
                        let _ = storage.set_item("virtualmac_fs", &json);
                    }
                }
            }
        }
    }

    /// Load file system from localStorage
    pub fn load_from_storage(&self) -> bool {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    if let Ok(Some(json)) = storage.get_item("virtualmac_fs") {
                        if let Ok(entries) = serde_json::from_str::<HashMap<String, FileEntry>>(&json) {
                            self.entries.set(entries);
                            self.initialized.set(true);
                            self.bump_version();
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    /// Get recent files (most recently modified)
    pub fn get_recents(&self, limit: usize) -> Vec<FileEntry> {
        let entries = self.entries.get();
        let mut files: Vec<_> = entries
            .values()
            .filter(|e| e.is_file())
            .cloned()
            .collect();
        files.sort_by(|a, b| {
            b.metadata.modified.partial_cmp(&a.metadata.modified).unwrap_or(std::cmp::Ordering::Equal)
        });
        files.truncate(limit);
        files
    }
}

impl Default for VirtualFileSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Normalize a file path (ensure leading /, remove trailing /)
fn normalize_path(path: &str) -> String {
    let path = if path.starts_with('/') {
        path.to_string()
    } else {
        format!("/{}", path)
    };

    if path.len() > 1 && path.ends_with('/') {
        path[..path.len() - 1].to_string()
    } else {
        path
    }
}

/// Get parent directory path
fn get_parent_path(path: &str) -> String {
    let normalized = normalize_path(path);
    if normalized == "/" {
        return "/".to_string();
    }
    match normalized.rfind('/') {
        Some(0) => "/".to_string(),
        Some(idx) => normalized[..idx].to_string(),
        None => "/".to_string(),
    }
}

/// Get file/directory name from path
fn get_file_name(path: &str) -> String {
    let normalized = normalize_path(path);
    match normalized.rfind('/') {
        Some(idx) => normalized[idx + 1..].to_string(),
        None => normalized,
    }
}

/// Provider component for file system context
#[component]
pub fn FileSystemProvider(children: Children) -> impl IntoView {
    let fs = VirtualFileSystem::new();

    // Try to load from storage, otherwise initialize default
    if !fs.load_from_storage() {
        fs.init_default_structure();
        fs.save_to_storage();
    }

    provide_context(fs);

    children()
}

/// Hook to use the file system
pub fn use_file_system() -> VirtualFileSystem {
    expect_context::<VirtualFileSystem>()
}
