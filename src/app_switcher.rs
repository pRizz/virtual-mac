use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::KeyboardEvent;

/// Represents an app that can be switched to
#[derive(Clone, Debug, PartialEq)]
pub struct SwitchableApp {
    pub name: String,
    pub icon: &'static str,
}

impl SwitchableApp {
    fn all() -> Vec<Self> {
        vec![
            SwitchableApp {
                name: "Finder".to_string(),
                icon: "\u{1F4C1}",
            },
            SwitchableApp {
                name: "Calculator".to_string(),
                icon: "\u{1F5A9}",
            },
            SwitchableApp {
                name: "Notes".to_string(),
                icon: "\u{1F4DD}",
            },
            SwitchableApp {
                name: "Safari".to_string(),
                icon: "\u{1F310}",
            },
            SwitchableApp {
                name: "Messages".to_string(),
                icon: "\u{1F4AC}",
            },
            SwitchableApp {
                name: "Mail".to_string(),
                icon: "\u{2709}",
            },
            SwitchableApp {
                name: "Photos".to_string(),
                icon: "\u{1F5BC}",
            },
            SwitchableApp {
                name: "Music".to_string(),
                icon: "\u{1F3B5}",
            },
        ]
    }
}

/// App switcher overlay component (Cmd+Tab)
#[component]
pub fn AppSwitcher() -> impl IntoView {
    let (is_visible, set_is_visible) = signal(false);
    let (selected_index, set_selected_index) = signal(0usize);
    let (cmd_held, set_cmd_held) = signal(false);

    let apps = SwitchableApp::all();
    let app_count = apps.len();

    // Set up global keyboard listener for Cmd+Tab
    Effect::new(move |_| {
        let keydown_closure = Closure::wrap(Box::new(move |e: KeyboardEvent| {
            let is_cmd = e.meta_key() || e.ctrl_key();

            // Cmd+Tab to show switcher and cycle forward
            if e.code() == "Tab" && is_cmd {
                e.prevent_default();

                if !is_visible.get() {
                    // First press: show switcher, select second item (cycling from current)
                    set_is_visible.set(true);
                    set_cmd_held.set(true);
                    // Start at index 1 (next app) if there are multiple apps
                    if app_count > 1 {
                        set_selected_index.set(1);
                    } else {
                        set_selected_index.set(0);
                    }
                } else {
                    // Subsequent Tab presses while visible: cycle through apps
                    if e.shift_key() {
                        // Shift+Tab: cycle backward
                        set_selected_index.update(|i| {
                            if *i == 0 {
                                *i = app_count - 1;
                            } else {
                                *i -= 1;
                            }
                        });
                    } else {
                        // Tab: cycle forward
                        set_selected_index.update(|i| {
                            *i = (*i + 1) % app_count;
                        });
                    }
                }
            }

            // Escape to close without selecting
            if e.code() == "Escape" && is_visible.get() {
                e.prevent_default();
                set_is_visible.set(false);
                set_cmd_held.set(false);
                set_selected_index.set(0);
            }
        }) as Box<dyn FnMut(KeyboardEvent)>);

        let keyup_closure = Closure::wrap(Box::new(move |e: KeyboardEvent| {
            // When Cmd/Ctrl is released while switcher is visible, select the app
            if (e.code() == "MetaLeft"
                || e.code() == "MetaRight"
                || e.code() == "ControlLeft"
                || e.code() == "ControlRight")
                && is_visible.get()
                && cmd_held.get()
            {
                // Selection happens - close the switcher
                set_is_visible.set(false);
                set_cmd_held.set(false);
                // Note: In a real implementation, this would trigger window focus
                // For now, we just close the overlay
            }
        }) as Box<dyn FnMut(KeyboardEvent)>);

        let window = web_sys::window().expect("no window");
        window
            .add_event_listener_with_callback("keydown", keydown_closure.as_ref().unchecked_ref())
            .expect("failed to add keydown listener");
        window
            .add_event_listener_with_callback("keyup", keyup_closure.as_ref().unchecked_ref())
            .expect("failed to add keyup listener");

        // Keep the closures alive
        keydown_closure.forget();
        keyup_closure.forget();
    });

    // Handle clicking outside to close
    let on_backdrop_click = move |_| {
        set_is_visible.set(false);
        set_cmd_held.set(false);
        set_selected_index.set(0);
    };

    // Create app items once as static data
    let apps_signal = StoredValue::new(SwitchableApp::all());

    view! {
        <Show when=move || is_visible.get()>
            <div class="app-switcher-backdrop" on:click=on_backdrop_click>
                <div class="app-switcher-container" on:click=move |e: web_sys::MouseEvent| e.stop_propagation()>
                    <div class="app-switcher-apps">
                        {move || {
                            apps_signal.get_value().into_iter().enumerate().map(|(index, app)| {
                                let is_selected = move || selected_index.get() == index;
                                let item_class = move || {
                                    if is_selected() {
                                        "app-switcher-item selected"
                                    } else {
                                        "app-switcher-item"
                                    }
                                };
                                let name = app.name.clone();
                                let icon = app.icon;

                                view! {
                                    <div
                                        class=item_class
                                        on:mouseenter=move |_| set_selected_index.set(index)
                                        on:click=move |_| {
                                            set_is_visible.set(false);
                                            set_cmd_held.set(false);
                                        }
                                    >
                                        <div class="app-switcher-icon">{icon}</div>
                                        <div class="app-switcher-name">{name}</div>
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
