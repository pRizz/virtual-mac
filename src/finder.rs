use leptos::prelude::*;

/// Represents a file or folder item
#[derive(Clone, Debug, PartialEq)]
pub struct FileItem {
    pub id: u32,
    pub name: String,
    pub is_folder: bool,
    pub icon: &'static str,
}

impl FileItem {
    pub fn folder(id: u32, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            is_folder: true,
            icon: "📁",
        }
    }

    pub fn file(id: u32, name: &str, icon: &'static str) -> Self {
        Self {
            id,
            name: name.to_string(),
            is_folder: false,
            icon,
        }
    }
}

/// Sidebar item for favorites
#[derive(Clone, Debug)]
struct SidebarItem {
    name: &'static str,
    icon: &'static str,
}

/// The Finder application component
#[component]
pub fn Finder() -> impl IntoView {
    let (selected_sidebar, set_selected_sidebar) = signal("Recents");
    let (selected_items, set_selected_items) = signal(Vec::<String>::new());

    // Sidebar favorites
    let sidebar_favorites = vec![
        SidebarItem { name: "AirDrop", icon: "📡" },
        SidebarItem { name: "Recents", icon: "🕐" },
        SidebarItem { name: "Applications", icon: "📲" },
        SidebarItem { name: "Desktop", icon: "🖥" },
        SidebarItem { name: "Documents", icon: "📄" },
        SidebarItem { name: "Downloads", icon: "📥" },
    ];

    // Sidebar locations
    let sidebar_locations = vec![
        SidebarItem { name: "Macintosh HD", icon: "💾" },
        SidebarItem { name: "Network", icon: "🌐" },
    ];

    // Mock files for the current view
    let files = move || {
        match selected_sidebar.get() {
            "Applications" => vec![
                FileItem::file(101, "Safari", "🧭"),
                FileItem::file(102, "Mail", "✉️"),
                FileItem::file(103, "Calendar", "📅"),
                FileItem::file(104, "Notes", "📝"),
                FileItem::file(105, "Reminders", "☑️"),
                FileItem::file(106, "Music", "🎵"),
                FileItem::file(107, "Photos", "🖼"),
                FileItem::file(108, "Messages", "💬"),
                FileItem::file(109, "FaceTime", "📹"),
                FileItem::file(110, "Maps", "🗺"),
                FileItem::file(111, "Terminal", "⌨"),
                FileItem::file(112, "System Settings", "⚙️"),
            ],
            "Desktop" => vec![
                FileItem::folder(201, "Projects"),
                FileItem::file(202, "Screenshot.png", "🖼"),
                FileItem::file(203, "Notes.txt", "📄"),
            ],
            "Documents" => vec![
                FileItem::folder(301, "Work"),
                FileItem::folder(302, "Personal"),
                FileItem::file(303, "Resume.pdf", "📕"),
                FileItem::file(304, "Budget.xlsx", "📊"),
                FileItem::file(305, "Notes.txt", "📄"),
            ],
            "Downloads" => vec![
                FileItem::file(401, "installer.dmg", "💿"),
                FileItem::file(402, "photo.jpg", "🖼"),
                FileItem::file(403, "document.pdf", "📕"),
                FileItem::file(404, "archive.zip", "📦"),
            ],
            "Recents" => vec![
                FileItem::file(501, "document.pdf", "📕"),
                FileItem::file(502, "photo.jpg", "🖼"),
                FileItem::folder(503, "Projects"),
                FileItem::file(504, "notes.txt", "📄"),
                FileItem::file(505, "spreadsheet.xlsx", "📊"),
            ],
            _ => vec![],
        }
    };

    let toggle_selection = move |name: String| {
        set_selected_items.update(|items| {
            if items.contains(&name) {
                items.retain(|n| n != &name);
            } else {
                items.push(name);
            }
        });
    };

    view! {
        <div class="finder">
            // Toolbar
            <div class="finder-toolbar">
                <div class="finder-toolbar-left">
                    <button class="finder-nav-btn" title="Back">
                        <span class="nav-icon">"◀"</span>
                    </button>
                    <button class="finder-nav-btn" title="Forward">
                        <span class="nav-icon">"▶"</span>
                    </button>
                </div>
                <div class="finder-toolbar-title">
                    {move || selected_sidebar.get()}
                </div>
                <div class="finder-toolbar-right">
                    <div class="finder-view-btns">
                        <button class="finder-view-btn active" title="Icons">
                            <span>"⊞"</span>
                        </button>
                        <button class="finder-view-btn" title="List">
                            <span>"☰"</span>
                        </button>
                        <button class="finder-view-btn" title="Columns">
                            <span>"❘❘❘"</span>
                        </button>
                        <button class="finder-view-btn" title="Gallery">
                            <span>"▭"</span>
                        </button>
                    </div>
                    <div class="finder-search">
                        <span class="search-icon">"🔍"</span>
                        <input type="text" placeholder="Search" class="finder-search-input" />
                    </div>
                </div>
            </div>

            <div class="finder-body">
                // Sidebar
                <div class="finder-sidebar">
                    <div class="sidebar-section">
                        <div class="sidebar-header">"Favorites"</div>
                        {sidebar_favorites.into_iter().map(|item| {
                            let name = item.name;
                            let icon = item.icon;
                            let is_selected = move || selected_sidebar.get() == name;
                            view! {
                                <div
                                    class=move || if is_selected() { "sidebar-item selected" } else { "sidebar-item" }
                                    on:click=move |_| set_selected_sidebar.set(name)
                                >
                                    <span class="sidebar-icon">{icon}</span>
                                    <span class="sidebar-name">{name}</span>
                                </div>
                            }
                        }).collect::<Vec<_>>()}
                    </div>

                    <div class="sidebar-section">
                        <div class="sidebar-header">"Locations"</div>
                        {sidebar_locations.into_iter().map(|item| {
                            let name = item.name;
                            let icon = item.icon;
                            let is_selected = move || selected_sidebar.get() == name;
                            view! {
                                <div
                                    class=move || if is_selected() { "sidebar-item selected" } else { "sidebar-item" }
                                    on:click=move |_| set_selected_sidebar.set(name)
                                >
                                    <span class="sidebar-icon">{icon}</span>
                                    <span class="sidebar-name">{name}</span>
                                </div>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                </div>

                // Main content area
                <div class="finder-content">
                    <div class="finder-grid">
                        <For
                            each=files
                            key=|item| item.id
                            children=move |item| {
                                let name = item.name.clone();
                                let name_for_click = name.clone();
                                let name_for_check = name.clone();
                                let icon = item.icon;
                                let item_id = item.id;
                                let is_folder = item.is_folder;
                                let is_selected = move || selected_items.get().contains(&name_for_check);

                                // Create drag data as JSON string
                                let drag_data = format!(
                                    r#"{{"id":{},"name":"{}","icon":"{}","is_folder":{}}}"#,
                                    item_id, name, icon, is_folder
                                );

                                view! {
                                    <div
                                        class=move || if is_selected() { "finder-item selected" } else { "finder-item" }
                                        draggable="true"
                                        on:click=move |_| toggle_selection(name_for_click.clone())
                                        on:dragstart=move |ev: web_sys::DragEvent| {
                                            if let Some(dt) = ev.data_transfer() {
                                                let _ = dt.set_data("application/x-virtualmac-file", &drag_data);
                                                dt.set_effect_allowed("move");
                                            }
                                        }
                                    >
                                        <div class="finder-item-icon">{icon}</div>
                                        <div class="finder-item-name">{name}</div>
                                    </div>
                                }
                            }
                        />
                    </div>

                    // Status bar
                    <div class="finder-statusbar">
                        {move || {
                            let count = files().len();
                            format!("{} items", count)
                        }}
                    </div>
                </div>
            </div>
        </div>
    }
}
