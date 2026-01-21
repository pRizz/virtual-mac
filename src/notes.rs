use leptos::html;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = document)]
    fn execCommand(command: &str, show_ui: bool, value: &str) -> bool;
}

fn generate_id() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        let timestamp = js_sys::Date::now() as u64;
        let random = (js_sys::Math::random() * 1000000.0) as u64;
        format!("{}-{}", timestamp, random)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        "test-id".to_string()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Note {
    pub id: String,
    pub folder_id: String,
    pub title: String,
    pub content: String, // HTML content
    pub created_at: f64,
    pub updated_at: f64,
    #[serde(default)]
    pub is_pinned: bool,
    pub is_deleted: bool,
    pub deleted_at: Option<f64>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Folder {
    pub id: String,
    pub name: String,
    pub is_system: bool, // true for All Notes, Recently Deleted
    pub created_at: f64,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Default)]
pub enum NotesSortMode {
    #[default]
    UpdatedAt,
    CreatedAt,
    Title,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct NotesState {
    pub folders: Vec<Folder>,
    pub notes: Vec<Note>,
    pub selected_folder_id: Option<String>,
    pub selected_note_id: Option<String>,
    #[serde(default)]
    pub notes_sort_mode: NotesSortMode,
}

impl NotesState {
    pub fn default_with_system_folders() -> Self {
        let now = js_sys::Date::now();
        Self {
            folders: vec![
                Folder {
                    id: "all-notes".to_string(),
                    name: "All Notes".to_string(),
                    is_system: true,
                    created_at: now,
                },
                Folder {
                    id: "recently-deleted".to_string(),
                    name: "Recently Deleted".to_string(),
                    is_system: true,
                    created_at: now,
                },
            ],
            notes: vec![],
            selected_folder_id: Some("all-notes".to_string()),
            selected_note_id: None,
            notes_sort_mode: NotesSortMode::UpdatedAt,
        }
    }
}

fn sort_notes(notes: &mut [Note], sort_mode: NotesSortMode) {
    notes.sort_by(|a, b| {
        if a.is_pinned != b.is_pinned {
            return b.is_pinned.cmp(&a.is_pinned);
        }

        match sort_mode {
            NotesSortMode::UpdatedAt => b
                .updated_at
                .partial_cmp(&a.updated_at)
                .unwrap_or(Ordering::Equal),
            NotesSortMode::CreatedAt => b
                .created_at
                .partial_cmp(&a.created_at)
                .unwrap_or(Ordering::Equal),
            NotesSortMode::Title => a
                .title
                .to_lowercase()
                .cmp(&b.title.to_lowercase()),
        }
    });
}

fn save_to_storage(state: &NotesState) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(json) = serde_json::to_string(state) {
                    let _ = storage.set_item("virtualmac_notes", &json);
                }
            }
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = state;
    }
}

fn load_from_storage() -> NotesState {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(json)) = storage.get_item("virtualmac_notes") {
                    if let Ok(state) = serde_json::from_str(&json) {
                        return state;
                    }
                }
            }
        }
    }
    NotesState::default_with_system_folders()
}

#[component]
pub fn Notes() -> impl IntoView {
    let (state, set_state) = signal(load_from_storage());
    let (search_query, set_search_query) = signal(String::new());

    // Auto-save on state changes
    Effect::new(move |_| {
        let current_state = state.get();
        save_to_storage(&current_state);
    });

    // Compute visible notes based on selected folder and search
    let visible_notes = Memo::new(move |_| {
        let st = state.get();
        let query = search_query.get().to_lowercase();
        let folder_id = st.selected_folder_id.clone();

        let mut filtered = st
            .notes
            .iter()
            .filter(|n| {
                // Filter by folder
                let folder_match = match folder_id.as_deref() {
                    Some("all-notes") => !n.is_deleted,
                    Some("recently-deleted") => n.is_deleted,
                    Some(fid) => n.folder_id == fid && !n.is_deleted,
                    None => !n.is_deleted,
                };
                // Filter by search
                let search_match = query.is_empty()
                    || n.title.to_lowercase().contains(&query)
                    || n.content.to_lowercase().contains(&query);
                folder_match && search_match
            })
            .cloned()
            .collect::<Vec<_>>();

        sort_notes(&mut filtered, st.notes_sort_mode);
        filtered
    });

    // Create new note handler
    let create_note = move |_| {
        let now = js_sys::Date::now();
        let folder_id = state
            .get()
            .selected_folder_id
            .clone()
            .unwrap_or_else(|| "all-notes".to_string());

        // Don't create notes in Recently Deleted
        let actual_folder = if folder_id == "recently-deleted" {
            "all-notes".to_string()
        } else {
            folder_id
        };

        let new_note = Note {
            id: generate_id(),
            folder_id: actual_folder,
            title: "New Note".to_string(),
            content: String::new(),
            created_at: now,
            updated_at: now,
            is_pinned: false,
            is_deleted: false,
            deleted_at: None,
        };

        let new_note_id = new_note.id.clone();

        set_state.update(|s| {
            s.notes.insert(0, new_note);
            s.selected_note_id = Some(new_note_id);
        });
    };

    // Delete note handler (soft delete - moves to Recently Deleted)
    let delete_note = move |note_id: String| {
        let now = js_sys::Date::now();
        set_state.update(|s| {
            if let Some(note) = s.notes.iter_mut().find(|n| n.id == note_id) {
                note.is_deleted = true;
                note.deleted_at = Some(now);
            }
            // Clear selection if deleted note was selected
            if s.selected_note_id.as_ref() == Some(&note_id) {
                s.selected_note_id = None;
            }
        });
    };

    // Permanently delete note handler
    let permanent_delete_note = move |note_id: String| {
        set_state.update(|s| {
            s.notes.retain(|n| n.id != note_id);
            if s.selected_note_id.as_ref() == Some(&note_id) {
                s.selected_note_id = None;
            }
        });
    };

    // Restore note from Recently Deleted
    let restore_note = move |note_id: String| {
        set_state.update(|s| {
            if let Some(note) = s.notes.iter_mut().find(|n| n.id == note_id) {
                note.is_deleted = false;
                note.deleted_at = None;
            }
        });
    };

    let toggle_pin = move |note_id: String| {
        let now = js_sys::Date::now();
        set_state.update(|s| {
            if let Some(note) = s.notes.iter_mut().find(|n| n.id == note_id) {
                note.is_pinned = !note.is_pinned;
                note.updated_at = now;
            }
        });
    };

    let update_sort_mode = move |sort_mode: NotesSortMode| {
        set_state.update(|s| {
            s.notes_sort_mode = sort_mode;
        });
    };

    // Create new folder handler
    let create_folder = move |_| {
        let now = js_sys::Date::now();
        let new_folder = Folder {
            id: generate_id(),
            name: "New Folder".to_string(),
            is_system: false,
            created_at: now,
        };
        let new_folder_id = new_folder.id.clone();

        set_state.update(|s| {
            s.folders.push(new_folder);
            s.selected_folder_id = Some(new_folder_id);
        });
    };

    // Delete folder handler (only for non-system folders)
    let delete_folder = move |folder_id: String| {
        set_state.update(|s| {
            // Don't delete system folders
            if let Some(folder) = s.folders.iter().find(|f| f.id == folder_id) {
                if folder.is_system {
                    return;
                }
            }

            // Move notes from deleted folder to "all-notes" (they become unfiled)
            for note in s.notes.iter_mut() {
                if note.folder_id == folder_id {
                    note.folder_id = "all-notes".to_string();
                }
            }

            // Remove the folder
            s.folders.retain(|f| f.id != folder_id);

            // Select "All Notes" if deleted folder was selected
            if s.selected_folder_id.as_ref() == Some(&folder_id) {
                s.selected_folder_id = Some("all-notes".to_string());
            }
        });
    };

    // Rename folder handler
    let rename_folder = move |folder_id: String, new_name: String| {
        set_state.update(|s| {
            if let Some(folder) = s.folders.iter_mut().find(|f| f.id == folder_id) {
                // Don't rename system folders
                if !folder.is_system {
                    folder.name = new_name;
                }
            }
        });
    };

    view! {
        <div class="notes-app">
            <FolderSidebar
                state=state
                set_state=set_state
                on_create_folder=create_folder
                on_delete_folder=delete_folder
                on_rename_folder=rename_folder
            />
            <NotesList
                notes=visible_notes
                state=state
                set_state=set_state
                search_query=search_query
                set_search_query=set_search_query
                notes_sort_mode=Memo::new(move |_| state.get().notes_sort_mode)
                on_create=create_note
                on_delete=delete_note
                on_restore=restore_note
                on_permanent_delete=permanent_delete_note
                on_toggle_pin=toggle_pin
                on_update_sort_mode=update_sort_mode
            />
            <NoteEditor state=state set_state=set_state />
        </div>
    }
}

#[component]
fn FolderSidebar(
    state: ReadSignal<NotesState>,
    set_state: WriteSignal<NotesState>,
    on_create_folder: impl Fn(leptos::ev::MouseEvent) + 'static + Clone + Send + Sync,
    on_delete_folder: impl Fn(String) + 'static + Clone + Send + Sync,
    on_rename_folder: impl Fn(String, String) + 'static + Clone + Send + Sync,
) -> impl IntoView {
    let folders = move || state.get().folders.clone();
    let selected_id = move || state.get().selected_folder_id.clone();

    // Count notes per folder
    let note_count = move |folder_id: &str| {
        let st = state.get();
        if folder_id == "all-notes" {
            st.notes.iter().filter(|n| !n.is_deleted).count()
        } else if folder_id == "recently-deleted" {
            st.notes.iter().filter(|n| n.is_deleted).count()
        } else {
            st.notes
                .iter()
                .filter(|n| n.folder_id == folder_id && !n.is_deleted)
                .count()
        }
    };

    view! {
        <div class="notes-folder-sidebar">
            <div class="notes-sidebar-header-row">
                <span class="notes-sidebar-header">"Folders"</span>
                <button
                    class="notes-action-btn secondary icon"
                    on:click=on_create_folder.clone()
                    title="New Folder"
                >
                    "+"
                </button>
            </div>
            <For
                each=folders
                key=|folder| folder.id.clone()
                children={
                    let on_delete_folder = on_delete_folder.clone();
                    let on_rename_folder = on_rename_folder.clone();
                    move |folder| {
                        let folder_id = folder.id.clone();
                        let folder_id_for_selected = folder_id.clone();
                        let folder_id_for_click = folder_id.clone();
                        let folder_id_for_count = folder_id.clone();
                        let folder_id_for_delete = folder_id.clone();
                        let folder_id_for_rename = folder_id.clone();
                        let folder_id_for_icon = folder_id.clone();
                        let is_system = folder.is_system;
                        let folder_name = folder.name.clone();
                        let folder_name_for_edit = folder_name.clone();

                        let (is_editing, set_is_editing) = signal(false);
                        let (edit_value, set_edit_value) = signal(folder_name.clone());

                        let is_selected = move || selected_id() == Some(folder_id_for_selected.clone());

                        let icon = if is_system {
                            if folder_id_for_icon == "recently-deleted" { "trash" } else { "folder" }
                        } else {
                            "folder-open"
                        };

                        let on_delete_folder = on_delete_folder.clone();
                        let on_rename_folder = on_rename_folder.clone();
                        let on_rename_folder_blur = on_rename_folder.clone();
                        let on_rename_folder_key = on_rename_folder.clone();

                        view! {
                            <div
                                class=move || if is_selected() { "notes-folder-item selected" } else { "notes-folder-item" }
                                on:click=move |_| {
                                    if !is_editing.get() {
                                        set_state.update(|s| {
                                            s.selected_folder_id = Some(folder_id_for_click.clone());
                                            s.selected_note_id = None;
                                        });
                                    }
                                }
                                on:dblclick={
                                    let folder_name_for_edit = folder_name_for_edit.clone();
                                    move |_| {
                                        if !is_system {
                                            set_edit_value.set(folder_name_for_edit.clone());
                                            set_is_editing.set(true);
                                        }
                                    }
                                }
                            >
                                <span class="notes-folder-icon">{icon}</span>
                                <Show
                                    when=move || is_editing.get()
                                    fallback={
                                        let folder_name = folder_name.clone();
                                        move || view! {
                                            <span class="notes-folder-name">{folder_name.clone()}</span>
                                        }
                                    }
                                >
                                    {
                                        let folder_id_rename_blur = folder_id_for_rename.clone();
                                        let folder_id_rename_key = folder_id_for_rename.clone();
                                        view! {
                                            <input
                                                type="text"
                                                class="notes-folder-item-input"
                                                prop:value=move || edit_value.get()
                                                on:input=move |e| {
                                                    set_edit_value.set(event_target_value(&e));
                                                }
                                                on:blur={
                                                    let on_rename_folder_blur = on_rename_folder_blur.clone();
                                                    move |_| {
                                                        let new_name = edit_value.get();
                                                        if !new_name.trim().is_empty() {
                                                            on_rename_folder_blur(folder_id_rename_blur.clone(), new_name);
                                                        }
                                                        set_is_editing.set(false);
                                                    }
                                                }
                                                on:keydown={
                                                    let on_rename_folder_key = on_rename_folder_key.clone();
                                                    move |e: leptos::ev::KeyboardEvent| {
                                                        if e.key() == "Enter" {
                                                            let new_name = edit_value.get();
                                                            if !new_name.trim().is_empty() {
                                                                on_rename_folder_key(folder_id_rename_key.clone(), new_name);
                                                            }
                                                            set_is_editing.set(false);
                                                        } else if e.key() == "Escape" {
                                                            set_is_editing.set(false);
                                                        }
                                                    }
                                                }
                                            />
                                        }
                                    }
                                </Show>
                                <span class="notes-folder-count">{move || note_count(&folder_id_for_count)}</span>
                                {if !is_system {
                                    let on_delete_folder = on_delete_folder.clone();
                                    let folder_id_del = folder_id_for_delete.clone();
                                    view! {
                                        <button
                                            class="notes-folder-delete-btn"
                                            on:click=move |e: leptos::ev::MouseEvent| {
                                                e.stop_propagation();
                                                on_delete_folder(folder_id_del.clone());
                                            }
                                            title="Delete Folder"
                                        >
                                            "x"
                                        </button>
                                    }.into_any()
                                } else {
                                    view! { <span></span> }.into_any()
                                }}
                            </div>
                        }
                    }
                }
            />
        </div>
    }
}

#[component]
fn NotesList(
    notes: Memo<Vec<Note>>,
    state: ReadSignal<NotesState>,
    set_state: WriteSignal<NotesState>,
    search_query: ReadSignal<String>,
    set_search_query: WriteSignal<String>,
    notes_sort_mode: Memo<NotesSortMode>,
    on_create: impl Fn(leptos::ev::MouseEvent) + 'static + Clone + Send + Sync,
    on_delete: impl Fn(String) + 'static + Clone + Send + Sync,
    on_restore: impl Fn(String) + 'static + Clone + Send + Sync,
    on_permanent_delete: impl Fn(String) + 'static + Clone + Send + Sync,
    on_toggle_pin: impl Fn(String) + 'static + Clone + Send + Sync,
    on_update_sort_mode: impl Fn(NotesSortMode) + 'static + Clone + Send + Sync,
) -> impl IntoView {
    #[derive(Clone, PartialEq)]
    enum NotesListEntry {
        Header(String),
        Note(Note),
    }

    let selected_note_id = move || state.get().selected_note_id.clone();
    let list_entries = Memo::new(move |_| {
        let current_notes = notes.get();
        let mut entries = Vec::new();
        let mut pinned = Vec::new();
        let mut unpinned = Vec::new();

        for note in current_notes {
            if note.is_pinned {
                pinned.push(note);
            } else {
                unpinned.push(note);
            }
        }

        if !pinned.is_empty() {
            entries.push(NotesListEntry::Header("Pinned".to_string()));
            entries.extend(pinned.into_iter().map(NotesListEntry::Note));
        }

        entries.extend(unpinned.into_iter().map(NotesListEntry::Note));
        entries
    });

    // Format date for display
    fn format_date(timestamp: f64) -> String {
        #[cfg(target_arch = "wasm32")]
        {
            let date = js_sys::Date::new(&wasm_bindgen::JsValue::from_f64(timestamp));
            let month = date.get_month() + 1;
            let day = date.get_date();
            format!("{}/{}", month as u32, day as u32)
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = timestamp;
            "1/1".to_string()
        }
    }

    // Extract preview text from HTML content
    fn get_preview(content: &str) -> String {
        // Strip HTML tags for preview
        let text = content
            .replace("<br>", " ")
            .replace("<br/>", " ")
            .replace("</p>", " ")
            .replace("</div>", " ");
        // Simple tag stripping
        let mut result = String::new();
        let mut in_tag = false;
        for c in text.chars() {
            if c == '<' {
                in_tag = true;
            } else if c == '>' {
                in_tag = false;
            } else if !in_tag {
                result.push(c);
            }
        }
        let truncated: String = result.chars().take(50).collect();
        if result.len() > 50 {
            format!("{}...", truncated)
        } else {
            truncated
        }
    }

    view! {
        <div class="notes-list">
            <div class="notes-list-header">
                <span class="notes-list-count">{move || format!("{} Notes", notes.get().len())}</span>
                <div class="notes-list-header-actions">
                    <select
                        class="notes-sort-select"
                        prop:value=move || match notes_sort_mode.get() {
                            NotesSortMode::UpdatedAt => "updated".to_string(),
                            NotesSortMode::CreatedAt => "created".to_string(),
                            NotesSortMode::Title => "title".to_string(),
                        }
                        on:change={
                            let on_update_sort_mode = on_update_sort_mode.clone();
                            move |e| {
                                let value = event_target_value(&e);
                                let sort_mode = match value.as_str() {
                                    "created" => NotesSortMode::CreatedAt,
                                    "title" => NotesSortMode::Title,
                                    _ => NotesSortMode::UpdatedAt,
                                };
                                on_update_sort_mode(sort_mode);
                            }
                        }
                    >
                        <option value="updated">"Date Edited"</option>
                        <option value="created">"Date Created"</option>
                        <option value="title">"Title"</option>
                    </select>
                    <button
                        class="notes-action-btn icon"
                        on:click=on_create.clone()
                        title="New Note"
                    >
                        "+"
                    </button>
                </div>
            </div>
            <div class="notes-search-bar">
                <span class="notes-search-icon">"search"</span>
                <input
                    type="text"
                    class="notes-search-input"
                    placeholder="Search"
                    prop:value=move || search_query.get()
                    on:input=move |e| {
                        set_search_query.set(event_target_value(&e));
                    }
                />
            </div>
            <div class="notes-list-items">
                <For
                    each=move || list_entries.get()
                    key=|entry| match entry {
                        NotesListEntry::Header(label) => format!("header-{}", label),
                        NotesListEntry::Note(note) => note.id.clone(),
                    }
                    children={
                        let on_delete = on_delete.clone();
                        let on_restore = on_restore.clone();
                        let on_permanent_delete = on_permanent_delete.clone();
                        let on_toggle_pin = on_toggle_pin.clone();
                        move |entry| match entry {
                            NotesListEntry::Header(label) => {
                                view! { <div class="notes-list-section-label">{label}</div> }.into_any()
                            }
                            NotesListEntry::Note(note) => {
                                let note_id = note.id.clone();
                                let note_id_for_click = note_id.clone();
                                let note_id_for_action = note_id.clone();
                                let is_deleted = note.is_deleted;
                                let is_pinned = note.is_pinned;
                                let is_selected = {
                                    let note_id = note_id.clone();
                                    move || selected_note_id() == Some(note_id.clone())
                                };

                                let on_delete = on_delete.clone();
                                let on_restore = on_restore.clone();
                                let on_permanent_delete = on_permanent_delete.clone();
                                let on_toggle_pin = on_toggle_pin.clone();

                                let title = note.title.clone();
                                let updated_at = note.updated_at;
                                let content = note.content.clone();

                                view! {
                                    <div
                                        class=move || if is_selected() { "notes-list-item selected" } else { "notes-list-item" }
                                        on:click=move |_| {
                                            set_state.update(|s| {
                                                s.selected_note_id = Some(note_id_for_click.clone());
                                            });
                                        }
                                    >
                                        <div class="notes-list-item-content">
                                            <div class="notes-list-item-title">{title}</div>
                                            <div class="notes-list-item-preview">
                                                <span class="notes-list-item-date">{format_date(updated_at)}</span>
                                                <span class="notes-list-item-text">{get_preview(&content)}</span>
                                            </div>
                                        </div>
                                        <div class="notes-list-item-actions">
                                            {if is_deleted {
                                                let note_id_restore = note_id_for_action.clone();
                                                let note_id_perm = note_id_for_action.clone();
                                                let on_restore = on_restore.clone();
                                                let on_permanent_delete = on_permanent_delete.clone();
                                                view! {
                                                    <button
                                                        class="notes-action-btn ghost"
                                                        on:click=move |e: leptos::ev::MouseEvent| {
                                                            e.stop_propagation();
                                                            on_restore(note_id_restore.clone());
                                                        }
                                                        title="Restore"
                                                    >
                                                        "Restore"
                                                    </button>
                                                    <button
                                                        class="notes-action-btn danger"
                                                        on:click=move |e: leptos::ev::MouseEvent| {
                                                            e.stop_propagation();
                                                            on_permanent_delete(note_id_perm.clone());
                                                        }
                                                        title="Delete Permanently"
                                                    >
                                                        "Delete"
                                                    </button>
                                                }.into_any()
                                            } else {
                                                let note_id_del = note_id_for_action.clone();
                                                let on_delete = on_delete.clone();
                                                let note_id_pin = note_id_for_action.clone();
                                                let on_toggle_pin = on_toggle_pin.clone();
                                                view! {
                                                    <button
                                                        class="notes-action-btn ghost"
                                                        on:click=move |e: leptos::ev::MouseEvent| {
                                                            e.stop_propagation();
                                                            on_toggle_pin(note_id_pin.clone());
                                                        }
                                                        title=move || if is_pinned { "Unpin" } else { "Pin" }
                                                    >
                                                        {move || if is_pinned { "Unpin" } else { "Pin" }}
                                                    </button>
                                                    <button
                                                        class="notes-action-btn ghost"
                                                        on:click=move |e: leptos::ev::MouseEvent| {
                                                            e.stop_propagation();
                                                            on_delete(note_id_del.clone());
                                                        }
                                                        title="Delete"
                                                    >
                                                        "Delete"
                                                    </button>
                                                }.into_any()
                                            }}
                                        </div>
                                    </div>
                                }
                                .into_any()
                            }
                        }
                    }
                />
            </div>
        </div>
    }
}

// Helper function to extract title from HTML content
fn extract_title(content: &str) -> String {
    content
        .split("<br>")
        .next()
        .or_else(|| content.split("<div>").next())
        .map(|s| {
            // Strip HTML tags
            let mut result = String::new();
            let mut in_tag = false;
            for c in s.chars() {
                if c == '<' {
                    in_tag = true;
                } else if c == '>' {
                    in_tag = false;
                } else if !in_tag {
                    result.push(c);
                }
            }
            result.trim().to_string()
        })
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "New Note".to_string())
}

#[component]
fn NoteEditor(state: ReadSignal<NotesState>, set_state: WriteSignal<NotesState>) -> impl IntoView {
    let editor_ref: NodeRef<html::Div> = NodeRef::new();

    let selected_note = Memo::new(move |_| {
        let st = state.get();
        st.selected_note_id
            .as_ref()
            .and_then(|id| st.notes.iter().find(|n| &n.id == id).cloned())
    });

    // Load content into editor when selected note changes
    Effect::new(move |_| {
        if let Some(note) = selected_note.get() {
            if let Some(el) = editor_ref.get() {
                el.set_inner_html(&note.content);
            }
        } else if let Some(el) = editor_ref.get() {
            el.set_inner_html("");
        }
    });

    // Set up click handler for checkboxes in editor
    Effect::new(move |_| {
        // Re-run when selected note changes to attach handlers to new content
        let _ = selected_note.get();

        #[cfg(target_arch = "wasm32")]
        {
            if let Some(el) = editor_ref.get() {
                use wasm_bindgen::JsCast;

                // Use event delegation on the editor element for checkbox clicks
                let el_clone = el.clone();
                let set_state_clone = set_state;

                let handler =
                    wasm_bindgen::closure::Closure::wrap(Box::new(move |e: web_sys::Event| {
                        if let Some(target) = e.target() {
                            if let Ok(input) = target.dyn_into::<web_sys::HtmlInputElement>() {
                                if input.class_list().contains("note-checkbox") {
                                    // Checkbox was clicked, save the state
                                    let content = el_clone.inner_html();
                                    set_state_clone.update(|s| {
                                        if let Some(note_id) = &s.selected_note_id {
                                            if let Some(note) =
                                                s.notes.iter_mut().find(|n| &n.id == note_id)
                                            {
                                                note.content = content.clone();
                                                note.updated_at = js_sys::Date::now();
                                            }
                                        }
                                    });
                                }
                            }
                        }
                    })
                        as Box<dyn Fn(web_sys::Event)>);

                let _ =
                    el.add_event_listener_with_callback("click", handler.as_ref().unchecked_ref());
                handler.forget(); // Keep the closure alive
            }
        }
    });

    // Save content helper
    let save_content = move || {
        if let Some(el) = editor_ref.get() {
            let content = el.inner_html();
            set_state.update(|s| {
                if let Some(note_id) = &s.selected_note_id {
                    if let Some(note) = s.notes.iter_mut().find(|n| &n.id == note_id) {
                        note.content = content.clone();
                        note.updated_at = js_sys::Date::now();
                        // Update title from first line
                        let title = extract_title(&content);
                        note.title = title;
                    }
                }
            });
        }
    };

    // Save content on blur
    let on_blur = move |_| {
        save_content();
    };

    let on_input = move |_| {
        save_content();
    };

    let checklist_html = r#"<div class=\"note-checklist-item\"><input type=\"checkbox\" class=\"note-checkbox\" /><span>&nbsp;</span></div>"#;

    let on_keydown = move |e: leptos::ev::KeyboardEvent| {
        if e.key() == "Tab" {
            e.prevent_default();
            if e.shift_key() {
                execCommand("outdent", false, "");
            } else {
                execCommand("indent", false, "");
            }
            if let Some(el) = editor_ref.get() {
                let _ = el.focus();
            }
            return;
        }

        if e.key() == "Enter" {
            #[cfg(target_arch = "wasm32")]
            {
                use wasm_bindgen::JsCast;

                if let Some(window) = web_sys::window() {
                    if let Ok(Some(selection)) = window.get_selection() {
                        if let Some(anchor_node) = selection.anchor_node() {
                            let mut maybe_element = anchor_node
                                .dyn_ref::<web_sys::Element>()
                                .cloned()
                                .or_else(|| anchor_node.parent_element());

                            while let Some(element) = maybe_element {
                                if element.class_list().contains("note-checklist-item") {
                                    e.prevent_default();
                                    execCommand("insertHTML", false, checklist_html);
                                    if let Some(el) = editor_ref.get() {
                                        let _ = el.focus();
                                    }
                                    break;
                                }
                                maybe_element = element.parent_element();
                            }
                        }
                    }
                }
            }
        }
    };

    // Formatting handlers - refocus editor after each operation
    let format_bold = move |_: leptos::ev::MouseEvent| {
        execCommand("bold", false, "");
        if let Some(el) = editor_ref.get() {
            let _ = el.focus();
        }
    };

    let format_italic = move |_: leptos::ev::MouseEvent| {
        execCommand("italic", false, "");
        if let Some(el) = editor_ref.get() {
            let _ = el.focus();
        }
    };

    let format_underline = move |_: leptos::ev::MouseEvent| {
        execCommand("underline", false, "");
        if let Some(el) = editor_ref.get() {
            let _ = el.focus();
        }
    };

    let format_strikethrough = move |_: leptos::ev::MouseEvent| {
        execCommand("strikeThrough", false, "");
        if let Some(el) = editor_ref.get() {
            let _ = el.focus();
        }
    };

    let insert_bullet_list = move |_: leptos::ev::MouseEvent| {
        execCommand("insertUnorderedList", false, "");
        if let Some(el) = editor_ref.get() {
            let _ = el.focus();
        }
    };

    let insert_numbered_list = move |_: leptos::ev::MouseEvent| {
        execCommand("insertOrderedList", false, "");
        if let Some(el) = editor_ref.get() {
            let _ = el.focus();
        }
    };

    // Insert checklist item using insertHTML
    let insert_checklist = move |_: leptos::ev::MouseEvent| {
        execCommand("insertHTML", false, checklist_html);
        if let Some(el) = editor_ref.get() {
            let _ = el.focus();
        }
    };

    view! {
        <div class="notes-editor">
            <div class="notes-editor-toolbar">
                <div class="notes-toolbar-group">
                    <button
                        class="notes-toolbar-btn"
                        on:click=format_bold
                        title="Bold (Cmd+B)"
                    >
                        <strong>"B"</strong>
                    </button>
                    <button
                        class="notes-toolbar-btn"
                        on:click=format_italic
                        title="Italic (Cmd+I)"
                    >
                        <em>"I"</em>
                    </button>
                    <button
                        class="notes-toolbar-btn"
                        on:click=format_underline
                        title="Underline (Cmd+U)"
                    >
                        <u>"U"</u>
                    </button>
                    <button
                        class="notes-toolbar-btn"
                        on:click=format_strikethrough
                        title="Strikethrough"
                    >
                        <s>"S"</s>
                    </button>
                </div>

                <div class="notes-toolbar-separator"></div>

                <div class="notes-toolbar-group">
                    <button
                        class="notes-toolbar-btn"
                        on:click=insert_bullet_list
                        title="Bullet List"
                    >
                        "..."
                    </button>
                    <button
                        class="notes-toolbar-btn"
                        on:click=insert_numbered_list
                        title="Numbered List"
                    >
                        "1."
                    </button>
                    <button
                        class="notes-toolbar-btn"
                        on:click=insert_checklist
                        title="Checklist"
                    >
                        "[x]"
                    </button>
                </div>
            </div>

            <Show
                when=move || selected_note.get().is_some()
                fallback=|| view! {
                    <div class="notes-editor-empty">
                        <div class="notes-editor-empty-text">"Select a note or create a new one"</div>
                    </div>
                }
            >
                <div
                    class="notes-editor-content"
                    contenteditable="true"
                    node_ref=editor_ref
                    on:blur=on_blur
                    on:input=on_input
                    on:keydown=on_keydown
                />
            </Show>
        </div>
    }
}
