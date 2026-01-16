use leptos::prelude::*;
use wasm_bindgen::JsCast;

/// Represents a desktop icon with position
#[derive(Clone, Debug, PartialEq)]
pub struct DesktopIcon {
    pub id: u32,
    pub name: String,
    pub icon: &'static str,
    pub is_folder: bool,
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Copy, Default)]
struct SelectionRect {
    start_x: f64,
    start_y: f64,
    current_x: f64,
    current_y: f64,
    active: bool,
}

impl SelectionRect {
    fn left(&self) -> f64 {
        self.start_x.min(self.current_x)
    }

    fn top(&self) -> f64 {
        self.start_y.min(self.current_y)
    }

    fn width(&self) -> f64 {
        (self.current_x - self.start_x).abs()
    }

    fn height(&self) -> f64 {
        (self.current_y - self.start_y).abs()
    }
}

#[component]
pub fn Desktop() -> impl IntoView {
    let (selection, set_selection) = signal(SelectionRect::default());
    let (is_drag_over, set_is_drag_over) = signal(false);
    let (desktop_icons, set_desktop_icons) = signal(vec![
        DesktopIcon {
            id: 1,
            name: "Macintosh HD".to_string(),
            icon: "💾",
            is_folder: true,
            x: 20.0,
            y: 40.0,
        },
    ]);
    let (next_id, set_next_id) = signal(1000u32);
    let (selected_icon, set_selected_icon) = signal(Option::<u32>::None);

    let on_mousedown = move |ev: web_sys::MouseEvent| {
        // Only start selection if clicking on desktop background, not on icons
        let target = ev.target();
        if let Some(element) = target.and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok()) {
            let class_list = element.class_list();
            if class_list.contains("desktop") || class_list.contains("desktop-drop-zone") {
                set_selected_icon.set(None);
                let x = ev.client_x() as f64;
                let y = ev.client_y() as f64;
                set_selection.set(SelectionRect {
                    start_x: x,
                    start_y: y,
                    current_x: x,
                    current_y: y,
                    active: true,
                });
            }
        }
    };

    let on_mousemove = move |ev: web_sys::MouseEvent| {
        if selection.get().active {
            let x = ev.client_x() as f64;
            let y = ev.client_y() as f64;
            set_selection.update(|s| {
                s.current_x = x;
                s.current_y = y;
            });
        }
    };

    let on_mouseup = move |_ev: web_sys::MouseEvent| {
        set_selection.update(|s| {
            s.active = false;
        });
    };

    let on_dragover = move |ev: web_sys::DragEvent| {
        ev.prevent_default();
        if let Some(dt) = ev.data_transfer() {
            dt.set_drop_effect("move");
        }
        set_is_drag_over.set(true);
    };

    let on_dragleave = move |_ev: web_sys::DragEvent| {
        set_is_drag_over.set(false);
    };

    let on_drop = move |ev: web_sys::DragEvent| {
        ev.prevent_default();
        set_is_drag_over.set(false);

        if let Some(dt) = ev.data_transfer() {
            if let Ok(data) = dt.get_data("application/x-virtualmac-file") {
                if !data.is_empty() {
                    // Parse the JSON data manually
                    let drop_x = ev.client_x() as f64;
                    let drop_y = ev.client_y() as f64;

                    // Simple JSON parsing for our known format
                    if let Some(name) = extract_json_string(&data, "name") {
                        let icon = extract_json_string(&data, "icon").unwrap_or("📄".to_string());
                        let is_folder = data.contains("\"is_folder\":true");

                        let new_id = next_id.get();
                        set_next_id.update(|id| *id += 1);

                        // Leak the icon string to get a static reference
                        let icon_static: &'static str = Box::leak(icon.into_boxed_str());

                        set_desktop_icons.update(|icons| {
                            icons.push(DesktopIcon {
                                id: new_id,
                                name,
                                icon: icon_static,
                                is_folder,
                                x: drop_x - 40.0,
                                y: drop_y - 40.0,
                            });
                        });
                    }
                }
            }
        }
    };

    view! {
        <div
            class="desktop"
            on:mousedown=on_mousedown
            on:mousemove=on_mousemove
            on:mouseup=on_mouseup
        >
            <div
                class=move || if is_drag_over.get() { "desktop-drop-zone drag-over" } else { "desktop-drop-zone" }
                on:dragover=on_dragover
                on:dragleave=on_dragleave
                on:drop=on_drop
            >
                // Desktop icons
                <For
                    each=move || desktop_icons.get()
                    key=|icon| icon.id
                    children=move |icon| {
                        let icon_id = icon.id;
                        let name = icon.name.clone();
                        let emoji = icon.icon;
                        let x = icon.x;
                        let y = icon.y;
                        let is_folder = icon.is_folder;
                        let is_selected = move || selected_icon.get() == Some(icon_id);

                        // Create drag data for dragging back to Finder
                        let drag_data = format!(
                            r#"{{"id":{},"name":"{}","icon":"{}","is_folder":{}}}"#,
                            icon_id, name, emoji, is_folder
                        );

                        view! {
                            <div
                                class=move || if is_selected() { "desktop-icon selected" } else { "desktop-icon" }
                                style=move || format!("left: {}px; top: {}px;", x, y)
                                draggable="true"
                                on:click=move |ev: web_sys::MouseEvent| {
                                    ev.stop_propagation();
                                    set_selected_icon.set(Some(icon_id));
                                }
                                on:dragstart=move |ev: web_sys::DragEvent| {
                                    ev.stop_propagation();
                                    if let Some(dt) = ev.data_transfer() {
                                        let _ = dt.set_data("application/x-virtualmac-file", &drag_data);
                                        dt.set_effect_allowed("move");
                                    }
                                }
                                on:dragend=move |_ev: web_sys::DragEvent| {
                                    // Remove icon from desktop when dragged away
                                    set_desktop_icons.update(|icons| {
                                        icons.retain(|i| i.id != icon_id);
                                    });
                                }
                            >
                                <div class="desktop-icon-image">{emoji}</div>
                                <div class="desktop-icon-name">{name}</div>
                            </div>
                        }
                    }
                />
            </div>

            <Show when=move || selection.get().active>
                <div
                    class="selection-rect"
                    style:left=move || format!("{}px", selection.get().left())
                    style:top=move || format!("{}px", selection.get().top())
                    style:width=move || format!("{}px", selection.get().width())
                    style:height=move || format!("{}px", selection.get().height())
                />
            </Show>
        </div>
    }
}

/// Simple helper to extract a string value from JSON
fn extract_json_string(json: &str, key: &str) -> Option<String> {
    let search = format!("\"{}\":\"", key);
    if let Some(start) = json.find(&search) {
        let value_start = start + search.len();
        if let Some(end) = json[value_start..].find('"') {
            return Some(json[value_start..value_start + end].to_string());
        }
    }
    None
}
