use crate::context_menu::{show_context_menu, ContextMenuState, ContextMenuType};
use crate::system_state::{MinimizedWindow, SystemState};
use leptos::ev::MouseEvent;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[allow(unused_imports)]
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;

#[allow(dead_code)]
const STORAGE_KEY: &str = "virtualmac_dock";
#[allow(dead_code)]
const CURRENT_SCHEMA_VERSION: u32 = 1;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct DockState {
    schema_version: u32,
    pinned_apps: Vec<String>,
}

impl DockState {
    fn default_with_pins() -> Self {
        Self {
            schema_version: CURRENT_SCHEMA_VERSION,
            pinned_apps: default_pinned_apps(),
        }
    }
}

fn default_pinned_apps() -> Vec<String> {
    vec![
        "Finder".to_string(),
        "Safari".to_string(),
        "Messages".to_string(),
        "Mail".to_string(),
        "Photos".to_string(),
        "Music".to_string(),
        "Notes".to_string(),
        "Calendar".to_string(),
        "TextEdit".to_string(),
        "Calculator".to_string(),
        "System Settings".to_string(),
        "Terminal".to_string(),
    ]
}

fn save_to_storage(state: &DockState) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(json) = serde_json::to_string(state) {
                    let _ = storage.set_item(STORAGE_KEY, &json);
                }
            }
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = state;
    }
}

fn load_from_storage() -> DockState {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(json)) = storage.get_item(STORAGE_KEY) {
                    if let Ok(state) = serde_json::from_str::<DockState>(&json) {
                        if state.schema_version == CURRENT_SCHEMA_VERSION
                            && !state.pinned_apps.is_empty()
                        {
                            return state;
                        }
                    }
                }
            }
        }
    }
    DockState::default_with_pins()
}

/// Represents a dock item (app icon)
#[derive(Clone, Debug)]
struct DockItem {
    name: String,
    icon: &'static str,
    icon_class: &'static str,
    is_running: bool,
}

impl DockItem {
    fn new(name: String, icon: &'static str, icon_class: &'static str, is_running: bool) -> Self {
        Self {
            name,
            icon,
            icon_class,
            is_running,
        }
    }
}

/// Calculate magnification scale based on distance from mouse
fn calculate_scale(item_x: f64, mouse_x: f64, max_scale: f64, effect_radius: f64) -> f64 {
    let distance = (item_x - mouse_x).abs();
    if distance > effect_radius {
        1.0
    } else {
        let normalized = 1.0 - (distance / effect_radius);
        1.0 + (max_scale - 1.0) * normalized * normalized
    }
}

/// Individual dock icon component
#[component]
fn DockIcon(
    item: DockItem,
    mouse_x: ReadSignal<f64>,
    is_hovering: ReadSignal<bool>,
    index: usize,
    context_menu_state: WriteSignal<ContextMenuState>,
) -> impl IntoView {
    let (scale, set_scale) = signal(1.0);
    let item_name = item.name.clone();
    let item_icon = item.icon;
    let item_icon_class = item.icon_class;
    let item_is_running = item.is_running;

    // Capture system_state at component creation time, not in event handler
    let system_state = expect_context::<SystemState>();

    // Calculate position-based scale when mouse moves
    Effect::new(move |_| {
        let mx = mouse_x.get();
        let hovering = is_hovering.get();

        if hovering && mx > 0.0 {
            // Approximate item center based on index (56px item width + 6px gap)
            let item_center = 62.0 * (index as f64) + 24.0;
            let new_scale = calculate_scale(item_center, mx, 1.8, 120.0);
            set_scale.set(new_scale);
        } else {
            set_scale.set(1.0);
        }
    });

    let context_name = item_name.clone();
    let on_contextmenu = move |ev: MouseEvent| {
        ev.prevent_default();
        ev.stop_propagation();
        let x = ev.client_x() as f64;
        let y = ev.client_y() as f64;
        show_context_menu(
            context_menu_state,
            x,
            y,
            ContextMenuType::DockItem {
                name: context_name.clone(),
            },
        );
    };

    let click_name = item_name.clone();
    let on_click = move |_: MouseEvent| {
        system_state.request_open_app(&click_name);
    };

    view! {
        <div
            class="dock-item"
            data-tooltip=item_name.clone()
            style:transform=move || format!(
                "scale({}) translateY({}px)",
                scale.get(),
                (scale.get() - 1.0) * -24.0
            )
            on:click=on_click
            on:contextmenu=on_contextmenu
        >
            <div class="dock-icon-wrapper">
                <div class=format!("dock-icon {}", item_icon_class)>
                    {item_icon}
                </div>
            </div>
            <div class=move || {
                if item_is_running {
                    "dock-indicator active"
                } else {
                    "dock-indicator"
                }
            }></div>
        </div>
    }
}

/// Trash icon component (separate for different styling)
#[component]
fn TrashIcon(
    mouse_x: ReadSignal<f64>,
    is_hovering: ReadSignal<bool>,
    index: usize,
    context_menu_state: WriteSignal<ContextMenuState>,
) -> impl IntoView {
    let (scale, set_scale) = signal(1.0);

    Effect::new(move |_| {
        let mx = mouse_x.get();
        let hovering = is_hovering.get();

        if hovering && mx > 0.0 {
            let item_center = 62.0 * (index as f64) + 24.0;
            let new_scale = calculate_scale(item_center, mx, 1.8, 120.0);
            set_scale.set(new_scale);
        } else {
            set_scale.set(1.0);
        }
    });

    let on_contextmenu = move |ev: MouseEvent| {
        ev.prevent_default();
        ev.stop_propagation();
        let x = ev.client_x() as f64;
        let y = ev.client_y() as f64;
        show_context_menu(context_menu_state, x, y, ContextMenuType::Trash);
    };

    view! {
        <div
            class="dock-item"
            data-tooltip="Trash"
            style:transform=move || format!(
                "scale({}) translateY({}px)",
                scale.get(),
                (scale.get() - 1.0) * -24.0
            )
            on:contextmenu=on_contextmenu
        >
            <div class="dock-icon-wrapper">
                <div class="dock-icon trash">
                    "🗑"
                </div>
            </div>
            <div class="dock-indicator"></div>
        </div>
    }
}

/// Minimized window dock item component
#[component]
fn MinimizedDockItem(window: MinimizedWindow) -> impl IntoView {
    let system_state = expect_context::<SystemState>();
    let window_id = window.id;
    let icon = window.icon.clone();
    let icon_class = format!("dock-icon {}", window.icon_class);

    let on_click = move |_: MouseEvent| {
        system_state.restore_window_id.set(Some(window_id));
    };

    view! {
        <div
            class="dock-item minimized-item"
            data-tooltip=window.title.clone()
            on:click=on_click
        >
            <div class="dock-icon-wrapper">
                <div class=icon_class>
                    {icon}
                </div>
            </div>
            <div class="dock-indicator active"></div>
        </div>
    }
}

/// Main dock component
#[component]
pub fn Dock(context_menu_state: WriteSignal<ContextMenuState>) -> impl IntoView {
    let system_state = expect_context::<SystemState>();
    let (mouse_x, set_mouse_x) = signal(0.0);
    let (is_hovering, set_is_hovering) = signal(false);

    let (dock_state, _set_dock_state) = signal(load_from_storage());

    let app_catalog: HashMap<&'static str, (&'static str, &'static str)> = HashMap::from([
        ("Finder", ("📂", "finder")),
        ("Safari", ("🧭", "safari")),
        ("Messages", ("💬", "messages")),
        ("Mail", ("✉️", "mail")),
        ("Photos", ("🖼", "photos")),
        ("Music", ("🎵", "music")),
        ("Notes", ("📝", "notes")),
        ("Calendar", ("📅", "calendar")),
        ("TextEdit", ("T", "textedit")),
        ("Calculator", ("=", "calculator")),
        ("System Settings", ("⚙️", "settings")),
        ("Terminal", (">_", "terminal")),
    ]);

    Effect::new(move |_| {
        let current_state = dock_state.get();
        save_to_storage(&current_state);
    });

    let dock_items = move || {
        let running_apps = system_state.open_windows.get();
        dock_state
            .get()
            .pinned_apps
            .into_iter()
            .filter_map(|app_name| {
                let app_name_str = app_name.as_str();
                let (icon, icon_class) = app_catalog.get(app_name_str)?;
                Some(DockItem::new(
                    app_name.clone(),
                    icon,
                    icon_class,
                    running_apps.contains(&app_name),
                ))
            })
            .collect::<Vec<_>>()
    };

    let dock_items_for_count = dock_items.clone();
    let num_apps = move || dock_items_for_count().len();

    let downloads_contextmenu = move |ev: MouseEvent| {
        ev.prevent_default();
        ev.stop_propagation();
        let x = ev.client_x() as f64;
        let y = ev.client_y() as f64;
        show_context_menu(
            context_menu_state,
            x,
            y,
            ContextMenuType::DockItem {
                name: "Downloads".to_string(),
            },
        );
    };

    // Get minimized windows from system state
    let minimized_windows = move || system_state.minimized_windows.get();
    let has_minimized = move || !minimized_windows().is_empty();

    view! {
        <div class="dock-wrapper">
            <div class="dock-container">
                <div
                    class="dock"
                    on:mouseenter=move |_| set_is_hovering.set(true)
                    on:mouseleave=move |_| {
                        set_is_hovering.set(false);
                        set_mouse_x.set(0.0);
                    }
                    on:mousemove=move |ev: MouseEvent| {
                        let target = ev.current_target().unwrap();
                        let rect = target.unchecked_ref::<web_sys::Element>().get_bounding_client_rect();
                        set_mouse_x.set(ev.client_x() as f64 - rect.left());
                    }
                >
                    // App icons
                    {dock_items().into_iter().enumerate().map(|(idx, item)| {
                        view! {
                            <DockIcon
                                item=item
                                mouse_x=mouse_x
                                is_hovering=is_hovering
                                index=idx
                                context_menu_state=context_menu_state
                            />
                        }
                    }).collect::<Vec<_>>()}

                    // Separator
                    <div class="dock-separator"></div>

                    // Downloads folder
                    <div
                        class="dock-item"
                        data-tooltip="Downloads"
                        style:transform={
                            let num_apps = num_apps.clone();
                            move || {
                                let mx = mouse_x.get();
                                let hovering = is_hovering.get();
                                let idx = num_apps();
                                if hovering && mx > 0.0 {
                                    let item_center = 62.0 * (idx as f64) + 44.0; // +44 for separator
                                    let scale = calculate_scale(item_center, mx, 1.8, 120.0);
                                    format!(
                                        "scale({}) translateY({}px)",
                                        scale,
                                        (scale - 1.0) * -24.0
                                    )
                                } else {
                                    "scale(1) translateY(0px)".to_string()
                                }
                            }
                        }
                        on:contextmenu=downloads_contextmenu
                    >
                        <div class="dock-icon-wrapper">
                            <div class="dock-icon downloads">"📥"</div>
                        </div>
                        <div class="dock-indicator"></div>
                    </div>

                    // Trash
                    <TrashIcon
                        mouse_x=mouse_x
                        is_hovering=is_hovering
                        index=num_apps() + 1
                        context_menu_state=context_menu_state
                    />
                </div>
            </div>

            // Minimized windows dock - only show if there are minimized windows
            <Show when=has_minimized>
                <div class="minimized-dock-separator"></div>
                <div class="minimized-dock">
                    <For
                        each=minimized_windows
                        key=|w| w.id
                        children=move |window| {
                            view! { <MinimizedDockItem window=window /> }
                        }
                    />
                </div>
            </Show>
        </div>
    }
}
