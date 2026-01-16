use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::KeyboardEvent;

use crate::window_context::use_window_context;

/// Mission Control view component (F3 key)
/// Shows a grid of all open window thumbnails
#[component]
pub fn MissionControl() -> impl IntoView {
    let (is_visible, set_is_visible) = signal(false);
    let ctx = use_window_context();

    // Set up global keyboard listener for F3
    Effect::new(move |_| {
        let closure = Closure::wrap(Box::new(move |e: KeyboardEvent| {
            // F3 to toggle Mission Control
            if e.code() == "F3" {
                e.prevent_default();
                set_is_visible.update(|v| *v = !*v);
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

    // Handle window thumbnail click - focus the window and close Mission Control
    let on_window_click = move |window_id: usize| {
        ctx.restore_window(window_id);
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
                            ctx.all_windows().into_iter().map(|window| {
                                let window_id = window.id;
                                let title = window.title.clone();
                                let icon = window.icon();
                                let is_minimized = window.is_minimized;

                                // Calculate thumbnail dimensions (scaled down)
                                let thumb_width = (window.width * 0.3).min(200.0).max(120.0);
                                let thumb_height = (window.height * 0.3).min(150.0).max(80.0);

                                let thumb_style = format!(
                                    "width: {}px; height: {}px;",
                                    thumb_width, thumb_height
                                );

                                let item_class = if is_minimized {
                                    "mission-control-window minimized"
                                } else {
                                    "mission-control-window"
                                };

                                view! {
                                    <div
                                        class=item_class
                                        on:click=move |e: web_sys::MouseEvent| {
                                            e.stop_propagation();
                                            on_window_click(window_id);
                                        }
                                    >
                                        <div class="mission-control-thumbnail" style=thumb_style>
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
                                        <div class="mission-control-label">{title}</div>
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
