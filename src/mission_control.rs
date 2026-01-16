use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::KeyboardEvent;

/// Represents a window thumbnail for Mission Control display
#[derive(Clone, Debug, PartialEq)]
struct WindowThumbnail {
    name: String,
    icon: &'static str,
}

impl WindowThumbnail {
    fn all() -> Vec<Self> {
        vec![
            WindowThumbnail { name: "Finder".to_string(), icon: "\u{1F4C1}" },
            WindowThumbnail { name: "Calculator".to_string(), icon: "\u{1F5A9}" },
            WindowThumbnail { name: "Notes".to_string(), icon: "\u{1F4DD}" },
        ]
    }
}

/// Mission Control view component (F3 key)
/// Shows a grid of all open window thumbnails
#[component]
pub fn MissionControl() -> impl IntoView {
    let (is_visible, set_is_visible) = signal(false);
    let (selected_index, set_selected_index) = signal::<Option<usize>>(None);

    let windows = StoredValue::new(WindowThumbnail::all());

    // Set up global keyboard listener for F3
    Effect::new(move |_| {
        let closure = Closure::wrap(Box::new(move |e: KeyboardEvent| {
            // F3 to toggle Mission Control
            if e.code() == "F3" {
                e.prevent_default();
                set_is_visible.update(|v| *v = !*v);
                set_selected_index.set(None);
            }

            // Escape to close
            if e.code() == "Escape" && is_visible.get() {
                e.prevent_default();
                set_is_visible.set(false);
            }
        }) as Box<dyn FnMut(KeyboardEvent)>);

        let window = web_sys::window().expect("no window");
        window
            .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
            .expect("failed to add keydown listener");

        // Keep the closure alive
        closure.forget();
    });

    // Handle clicking outside to close
    let on_backdrop_click = move |_| {
        set_is_visible.set(false);
    };

    view! {
        <Show when=move || is_visible.get()>
            <div class="mission-control-backdrop" on:click=on_backdrop_click>
                <div class="mission-control-container">
                    <div class="mission-control-header">
                        <span class="mission-control-title">"Mission Control"</span>
                    </div>
                    <div class="mission-control-windows">
                        {move || {
                            windows.get_value().into_iter().enumerate().map(|(index, window)| {
                                let name = window.name.clone();
                                let icon = window.icon;
                                let is_selected = move || selected_index.get() == Some(index);

                                let item_class = move || {
                                    if is_selected() {
                                        "mission-control-window selected"
                                    } else {
                                        "mission-control-window"
                                    }
                                };

                                view! {
                                    <div
                                        class=item_class
                                        on:mouseenter=move |_| set_selected_index.set(Some(index))
                                        on:mouseleave=move |_| set_selected_index.set(None)
                                        on:click=move |e: web_sys::MouseEvent| {
                                            e.stop_propagation();
                                            set_is_visible.set(false);
                                        }
                                    >
                                        <div class="mission-control-thumbnail">
                                            <div class="mission-control-thumb-titlebar">
                                                <div class="mission-control-thumb-dots">
                                                    <span class="dot red"></span>
                                                    <span class="dot yellow"></span>
                                                    <span class="dot green"></span>
                                                </div>
                                            </div>
                                            <div class="mission-control-thumb-content">
                                                <span class="mission-control-thumb-icon">{icon}</span>
                                            </div>
                                        </div>
                                        <div class="mission-control-label">{name}</div>
                                    </div>
                                }
                            }).collect::<Vec<_>>()
                        }}
                    </div>
                </div>
            </div>
        </Show>
    }
}
