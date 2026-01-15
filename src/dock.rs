use leptos::prelude::*;
use leptos::ev::MouseEvent;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;

/// Represents a dock item (app icon)
#[derive(Clone, Debug)]
struct DockItem {
    name: &'static str,
    icon: &'static str,
    icon_class: &'static str,
    is_running: bool,
}

impl DockItem {
    fn new(name: &'static str, icon: &'static str, icon_class: &'static str, is_running: bool) -> Self {
        Self { name, icon, icon_class, is_running }
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
) -> impl IntoView {
    let (scale, set_scale) = signal(1.0);
    let item_name = item.name;
    let item_icon = item.icon;
    let item_icon_class = item.icon_class;
    let item_is_running = item.is_running;

    // Calculate position-based scale when mouse moves
    Effect::new(move |_| {
        let mx = mouse_x.get();
        let hovering = is_hovering.get();

        if hovering && mx > 0.0 {
            // Approximate item center based on index (48px icon + 8px gap)
            let item_center = 56.0 * (index as f64) + 24.0;
            let new_scale = calculate_scale(item_center, mx, 1.8, 120.0);
            set_scale.set(new_scale);
        } else {
            set_scale.set(1.0);
        }
    });

    view! {
        <div
            class="dock-item"
            data-tooltip=item_name
            style:transform=move || format!(
                "scale({}) translateY({}px)",
                scale.get(),
                (scale.get() - 1.0) * -24.0
            )
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
) -> impl IntoView {
    let (scale, set_scale) = signal(1.0);

    Effect::new(move |_| {
        let mx = mouse_x.get();
        let hovering = is_hovering.get();

        if hovering && mx > 0.0 {
            let item_center = 56.0 * (index as f64) + 24.0;
            let new_scale = calculate_scale(item_center, mx, 1.8, 120.0);
            set_scale.set(new_scale);
        } else {
            set_scale.set(1.0);
        }
    });

    view! {
        <div
            class="dock-item"
            data-tooltip="Trash"
            style:transform=move || format!(
                "scale({}) translateY({}px)",
                scale.get(),
                (scale.get() - 1.0) * -24.0
            )
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

/// Main dock component
#[component]
pub fn Dock() -> impl IntoView {
    let (mouse_x, set_mouse_x) = signal(0.0);
    let (is_hovering, set_is_hovering) = signal(false);

    // App dock items
    let apps = vec![
        DockItem::new("Finder", "🔍", "finder", true),
        DockItem::new("Safari", "🧭", "safari", true),
        DockItem::new("Messages", "💬", "messages", false),
        DockItem::new("Mail", "✉️", "mail", true),
        DockItem::new("Photos", "🖼", "photos", false),
        DockItem::new("Music", "🎵", "music", false),
        DockItem::new("Notes", "📝", "notes", false),
        DockItem::new("Calendar", "📅", "calendar", false),
        DockItem::new("System Settings", "⚙️", "settings", false),
        DockItem::new("Terminal", "⌨", "terminal", true),
    ];

    let num_apps = apps.len();

    view! {
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
                {apps.into_iter().enumerate().map(|(idx, item)| {
                    view! {
                        <DockIcon
                            item=item
                            mouse_x=mouse_x
                            is_hovering=is_hovering
                            index=idx
                        />
                    }
                }).collect::<Vec<_>>()}

                // Separator
                <div class="dock-separator"></div>

                // Downloads folder
                <div
                    class="dock-item"
                    data-tooltip="Downloads"
                    style:transform=move || {
                        let mx = mouse_x.get();
                        let hovering = is_hovering.get();
                        let idx = num_apps;
                        if hovering && mx > 0.0 {
                            let item_center = 56.0 * (idx as f64) + 44.0; // +44 for separator
                            let scale = calculate_scale(item_center, mx, 1.8, 120.0);
                            format!("scale({}) translateY({}px)", scale, (scale - 1.0) * -24.0)
                        } else {
                            "scale(1) translateY(0px)".to_string()
                        }
                    }
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
                    index=num_apps + 1
                />
            </div>
        </div>
    }
}
