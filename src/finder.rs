use leptos::prelude::*;

/// Represents a file or folder item
#[derive(Clone, Debug, PartialEq)]
pub struct FileItem {
    pub name: String,
    pub is_folder: bool,
    pub icon: &'static str,
}

impl FileItem {
    fn folder(name: &str) -> Self {
        Self {
            name: name.to_string(),
            is_folder: true,
            icon: "📁",
        }
    }

    fn file(name: &str, icon: &'static str) -> Self {
        Self {
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
                FileItem::file("Safari", "🧭"),
                FileItem::file("Mail", "✉️"),
                FileItem::file("Calendar", "📅"),
                FileItem::file("Notes", "📝"),
                FileItem::file("Reminders", "☑️"),
                FileItem::file("Music", "🎵"),
                FileItem::file("Photos", "🖼"),
                FileItem::file("Messages", "💬"),
                FileItem::file("FaceTime", "📹"),
                FileItem::file("Maps", "🗺"),
                FileItem::file("Terminal", "⌨"),
                FileItem::file("System Settings", "⚙️"),
            ],
            "Desktop" => vec![
                FileItem::folder("Projects"),
                FileItem::file("Screenshot.png", "🖼"),
                FileItem::file("Notes.txt", "📄"),
            ],
            "Documents" => vec![
                FileItem::folder("Work"),
                FileItem::folder("Personal"),
                FileItem::file("Resume.pdf", "📕"),
                FileItem::file("Budget.xlsx", "📊"),
                FileItem::file("Notes.txt", "📄"),
            ],
            "Downloads" => vec![
                FileItem::file("installer.dmg", "💿"),
                FileItem::file("photo.jpg", "🖼"),
                FileItem::file("document.pdf", "📕"),
                FileItem::file("archive.zip", "📦"),
            ],
            "Recents" => vec![
                FileItem::file("document.pdf", "📕"),
                FileItem::file("photo.jpg", "🖼"),
                FileItem::folder("Projects"),
                FileItem::file("notes.txt", "📄"),
                FileItem::file("spreadsheet.xlsx", "📊"),
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
                            key=|item| item.name.clone()
                            children=move |item| {
                                let name = item.name.clone();
                                let name_for_click = name.clone();
                                let name_for_check = name.clone();
                                let is_selected = move || selected_items.get().contains(&name_for_check);
                                view! {
                                    <div
                                        class=move || if is_selected() { "finder-item selected" } else { "finder-item" }
                                        on:click=move |_| toggle_selection(name_for_click.clone())
                                    >
                                        <div class="finder-item-icon">{item.icon}</div>
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
