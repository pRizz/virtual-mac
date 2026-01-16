use leptos::prelude::*;

/// Represents the type of context for the menu
#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)]
pub enum ContextMenuType {
    Desktop,
    DesktopIcon { name: String },
    DockItem { name: String },
    FinderItem { name: String, is_folder: bool },
    Trash,
}

/// State for the context menu
#[derive(Clone, Debug, PartialEq)]
pub struct ContextMenuState {
    pub visible: bool,
    pub x: f64,
    pub y: f64,
    pub menu_type: ContextMenuType,
}

impl Default for ContextMenuState {
    fn default() -> Self {
        Self {
            visible: false,
            x: 0.0,
            y: 0.0,
            menu_type: ContextMenuType::Desktop,
        }
    }
}

/// Individual menu item with optional shortcut and disabled state
#[derive(Clone)]
pub struct ContextMenuItem {
    pub label: &'static str,
    pub shortcut: Option<&'static str>,
    pub disabled: bool,
    pub is_separator: bool,
}

impl ContextMenuItem {
    pub fn new(label: &'static str) -> Self {
        Self {
            label,
            shortcut: None,
            disabled: false,
            is_separator: false,
        }
    }

    pub fn with_shortcut(mut self, shortcut: &'static str) -> Self {
        self.shortcut = Some(shortcut);
        self
    }

    #[allow(dead_code)]
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub fn separator() -> Self {
        Self {
            label: "",
            shortcut: None,
            disabled: false,
            is_separator: true,
        }
    }
}

/// Get menu items based on context type
fn get_menu_items(menu_type: &ContextMenuType) -> Vec<ContextMenuItem> {
    match menu_type {
        ContextMenuType::Desktop => vec![
            ContextMenuItem::new("New Folder"),
            ContextMenuItem::separator(),
            ContextMenuItem::new("Get Info"),
            ContextMenuItem::new("Change Desktop Background..."),
            ContextMenuItem::separator(),
            ContextMenuItem::new("Use Stacks"),
            ContextMenuItem::new("Sort By"),
            ContextMenuItem::new("Clean Up"),
            ContextMenuItem::new("Clean Up By"),
            ContextMenuItem::separator(),
            ContextMenuItem::new("Show View Options"),
        ],
        ContextMenuType::DesktopIcon { name: _ } => vec![
            ContextMenuItem::new("Open"),
            ContextMenuItem::new("Open With"),
            ContextMenuItem::separator(),
            ContextMenuItem::new("Move to Trash"),
            ContextMenuItem::separator(),
            ContextMenuItem::new("Get Info").with_shortcut("⌘I"),
            ContextMenuItem::new("Rename"),
            ContextMenuItem::separator(),
            ContextMenuItem::new("Compress"),
            ContextMenuItem::new("Duplicate").with_shortcut("⌘D"),
            ContextMenuItem::new("Make Alias"),
            ContextMenuItem::new("Quick Look").with_shortcut("Space"),
            ContextMenuItem::separator(),
            ContextMenuItem::new("Copy").with_shortcut("⌘C"),
            ContextMenuItem::new("Share..."),
        ],
        ContextMenuType::DockItem { name } => {
            let is_running = matches!(
                name.as_str(),
                "Finder" | "Safari" | "Mail" | "Terminal"
            );
            let mut items = vec![
                ContextMenuItem::new("Options"),
                ContextMenuItem::separator(),
            ];
            if is_running {
                items.push(ContextMenuItem::new("Show All Windows"));
                items.push(ContextMenuItem::new("Hide"));
                items.push(ContextMenuItem::separator());
            }
            items.push(ContextMenuItem::new("Show in Finder"));
            items.push(ContextMenuItem::separator());
            if is_running {
                items.push(ContextMenuItem::new("Quit"));
            } else {
                items.push(ContextMenuItem::new("Open"));
            }
            items
        }
        ContextMenuType::FinderItem { name: _, is_folder } => {
            let mut items = vec![
                ContextMenuItem::new("Open"),
                ContextMenuItem::new("Open With"),
            ];
            if *is_folder {
                items.push(ContextMenuItem::new("Open in New Tab"));
                items.push(ContextMenuItem::new("Open in New Window"));
            }
            items.extend(vec![
                ContextMenuItem::separator(),
                ContextMenuItem::new("Move to Trash"),
                ContextMenuItem::separator(),
                ContextMenuItem::new("Get Info").with_shortcut("⌘I"),
                ContextMenuItem::new("Rename"),
                ContextMenuItem::separator(),
                ContextMenuItem::new("Compress"),
                ContextMenuItem::new("Duplicate").with_shortcut("⌘D"),
                ContextMenuItem::new("Make Alias"),
                ContextMenuItem::new("Quick Look").with_shortcut("Space"),
                ContextMenuItem::separator(),
                ContextMenuItem::new("Copy").with_shortcut("⌘C"),
                ContextMenuItem::new("Share..."),
            ]);
            items
        }
        ContextMenuType::Trash => vec![
            ContextMenuItem::new("Open"),
            ContextMenuItem::separator(),
            ContextMenuItem::new("Empty Trash"),
        ],
    }
}

/// Context menu component
#[component]
pub fn ContextMenu(
    state: ReadSignal<ContextMenuState>,
    set_state: WriteSignal<ContextMenuState>,
) -> impl IntoView {
    let close_menu = move |_| {
        set_state.update(|s| s.visible = false);
    };

    let items = move || get_menu_items(&state.get().menu_type);

    view! {
        <Show when=move || state.get().visible>
            // Backdrop to catch clicks outside
            <div
                class="context-menu-backdrop"
                on:click=close_menu
                on:contextmenu=move |e| {
                    e.prevent_default();
                    close_menu(e);
                }
            />
            <div
                class="context-menu"
                style:left=move || format!("{}px", state.get().x)
                style:top=move || format!("{}px", state.get().y)
            >
                {move || items().into_iter().map(|item| {
                    if item.is_separator {
                        view! { <div class="context-menu-separator"></div> }.into_any()
                    } else {
                        let class = if item.disabled {
                            "context-menu-item disabled"
                        } else {
                            "context-menu-item"
                        };
                        let shortcut = item.shortcut;
                        view! {
                            <div class=class on:click=close_menu>
                                <span class="context-menu-label">{item.label}</span>
                                {shortcut.map(|s| view! {
                                    <span class="context-menu-shortcut">{s}</span>
                                })}
                            </div>
                        }.into_any()
                    }
                }).collect::<Vec<_>>()}
            </div>
        </Show>
    }
}

/// Helper to show context menu at a specific position
pub fn show_context_menu(
    set_state: WriteSignal<ContextMenuState>,
    x: f64,
    y: f64,
    menu_type: ContextMenuType,
) {
    set_state.set(ContextMenuState {
        visible: true,
        x,
        y,
        menu_type,
    });
}
