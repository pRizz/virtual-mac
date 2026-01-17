use leptos::prelude::*;
use leptos::html;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Note {
    pub id: String,
    pub folder_id: String,
    pub title: String,
    pub content: String, // HTML content
    pub created_at: f64,
    pub updated_at: f64,
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

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct NotesState {
    pub folders: Vec<Folder>,
    pub notes: Vec<Note>,
    pub selected_folder_id: Option<String>,
    pub selected_note_id: Option<String>,
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
        }
    }
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

        st.notes
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
            .collect::<Vec<_>>()
    });

    view! {
        <div class="notes-app">
            <FolderSidebar state=state set_state=set_state />
            <NotesList
                notes=visible_notes
                state=state
                set_state=set_state
                search_query=search_query
                set_search_query=set_search_query
            />
            <NoteEditor state=state set_state=set_state />
        </div>
    }
}

#[component]
fn FolderSidebar(
    state: ReadSignal<NotesState>,
    set_state: WriteSignal<NotesState>,
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
            <div class="notes-sidebar-header">"Folders"</div>
            <For
                each=folders
                key=|folder| folder.id.clone()
                children=move |folder| {
                    let folder_id = folder.id.clone();
                    let folder_id_for_click = folder_id.clone();
                    let folder_id_for_count = folder_id.clone();
                    let is_selected = move || selected_id() == Some(folder_id.clone());
                    let icon = if folder.is_system {
                        if folder.id == "recently-deleted" {
                            "trash"
                        } else {
                            "folder"
                        }
                    } else {
                        "folder-open"
                    };
                    let folder_name = folder.name.clone();
                    view! {
                        <div
                            class=move || {
                                if is_selected() {
                                    "notes-folder-item selected"
                                } else {
                                    "notes-folder-item"
                                }
                            }
                            on:click=move |_| {
                                set_state
                                    .update(|s| {
                                        s.selected_folder_id = Some(folder_id_for_click.clone());
                                        s.selected_note_id = None;
                                    });
                            }
                        >
                            <span class="notes-folder-icon">{icon}</span>
                            <span class="notes-folder-name">{folder_name}</span>
                            <span class="notes-folder-count">
                                {move || note_count(&folder_id_for_count)}
                            </span>
                        </div>
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
) -> impl IntoView {
    let selected_note_id = move || state.get().selected_note_id.clone();

    // Format date for display
    fn format_date(timestamp: f64) -> String {
        #[cfg(target_arch = "wasm32")]
        {
            let date = js_sys::Date::new(&wasm_bindgen::JsValue::from_f64(timestamp));
            let month = date.get_month() + 1.0;
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
                    each=move || notes.get()
                    key=|note| note.id.clone()
                    children=move |note| {
                        let note_id = note.id.clone();
                        let note_id_for_click = note_id.clone();
                        let is_selected = move || selected_note_id() == Some(note_id.clone());
                        let title = note.title.clone();
                        let updated_at = note.updated_at;
                        let content = note.content.clone();
                        view! {
                            <div
                                class=move || {
                                    if is_selected() {
                                        "notes-list-item selected"
                                    } else {
                                        "notes-list-item"
                                    }
                                }
                                on:click=move |_| {
                                    set_state
                                        .update(|s| {
                                            s.selected_note_id = Some(note_id_for_click.clone());
                                        });
                                }
                            >
                                <div class="notes-list-item-title">{title}</div>
                                <div class="notes-list-item-preview">
                                    <span class="notes-list-item-date">
                                        {format_date(updated_at)}
                                    </span>
                                    <span class="notes-list-item-text">{get_preview(&content)}</span>
                                </div>
                            </div>
                        }
                    }
                />
            </div>
        </div>
    }
}

#[component]
fn NoteEditor(
    state: ReadSignal<NotesState>,
    set_state: WriteSignal<NotesState>,
) -> impl IntoView {
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

    // Save content on blur
    let on_blur = move |_| {
        if let Some(el) = editor_ref.get() {
            let content = el.inner_html();
            set_state.update(|s| {
                if let Some(note_id) = &s.selected_note_id {
                    if let Some(note) = s.notes.iter_mut().find(|n| &n.id == note_id) {
                        note.content = content.clone();
                        note.updated_at = js_sys::Date::now();
                        // Update title from first line
                        let title = content
                            .split("<br>")
                            .next()
                            .or_else(|| content.split("<div>").next())
                            .map(|s| {
                                // Strip tags
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
                            .unwrap_or_else(|| "New Note".to_string());
                        note.title = title;
                    }
                }
            });
        }
    };

    view! {
        <div class="notes-editor">
            <div class="notes-editor-toolbar">
                // Placeholder toolbar - formatting buttons added in Plan 03
                <div class="notes-toolbar-placeholder">"Formatting"</div>
            </div>
            <Show
                when=move || selected_note.get().is_some()
                fallback=|| {
                    view! {
                        <div class="notes-editor-empty">
                            <div class="notes-editor-empty-text">
                                "Select a note or create a new one"
                            </div>
                        </div>
                    }
                }
            >
                <div
                    class="notes-editor-content"
                    contenteditable="true"
                    node_ref=editor_ref
                    on:blur=on_blur
                />
            </Show>
        </div>
    }
}
