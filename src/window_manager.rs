use leptos::prelude::*;
use leptos::ev::MouseEvent;

#[allow(unused_imports)]
use wasm_bindgen::prelude::*;

use crate::finder::Finder;
use crate::calculator::Calculator;
use crate::system_settings::SystemSettings;
use crate::system_state::SystemState;
use crate::terminal::Terminal;
use crate::textedit::TextEdit;

/// Actions that can be triggered via keyboard shortcuts
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum WindowAction {
    #[default]
    None,
    CloseActive,
    HideActive,
    QuitAll,
}

/// Context for triggering window manager actions from outside the component
#[derive(Clone, Copy)]
pub struct WindowManagerContext {
    pub action_trigger: WriteSignal<WindowAction>,
}

/// Unique identifier for windows
pub type WindowId = usize;

/// Type of application in a window
#[derive(Clone, Debug, PartialEq)]
pub enum AppType {
    Generic,
    Calculator,
    SystemSettings,
    Terminal,
    TextEdit,
}

/// Represents the state of a single window
#[derive(Clone, Debug, PartialEq)]
pub struct WindowState {
    pub id: WindowId,
    pub title: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub z_index: i32,
    pub is_minimized: bool,
    pub is_maximized: bool,
    /// Stored position/size before maximizing
    pub pre_maximize: Option<(f64, f64, f64, f64)>,
    /// Type of application in this window
    pub app_type: AppType,
}

impl WindowState {
    pub fn new(id: WindowId, title: &str, x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            id,
            title: title.to_string(),
            x,
            y,
            width,
            height,
            z_index: id as i32,
            is_minimized: false,
            is_maximized: false,
            pre_maximize: None,
            app_type: AppType::Generic,
        }
    }

    pub fn new_with_app(id: WindowId, title: &str, x: f64, y: f64, width: f64, height: f64, app_type: AppType) -> Self {
        Self {
            id,
            title: title.to_string(),
            x,
            y,
            width,
            height,
            z_index: id as i32,
            is_minimized: false,
            is_maximized: false,
            pre_maximize: None,
            app_type,
        }
    }
}

/// Drag/resize operation state
#[derive(Clone, Debug, PartialEq)]
enum DragOperation {
    None,
    Move { window_id: WindowId, start_x: f64, start_y: f64, window_start_x: f64, window_start_y: f64 },
    Resize {
        window_id: WindowId,
        direction: ResizeDirection,
        start_x: f64,
        start_y: f64,
        window_start_x: f64,
        window_start_y: f64,
        window_start_width: f64,
        window_start_height: f64
    },
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum ResizeDirection {
    N, S, E, W, NE, NW, SE, SW,
}

/// Desktop component that contains the window manager
#[component]
pub fn WindowManager() -> impl IntoView {
    let system_state = expect_context::<SystemState>();

    // Global state for all windows
    let (windows, set_windows) = signal(vec![
        WindowState::new(1, "Finder", 100.0, 80.0, 600.0, 400.0),
        WindowState::new_with_app(2, "Calculator", 200.0, 150.0, 232.0, 340.0, AppType::Calculator),
        WindowState::new_with_app(3, "Terminal", 300.0, 120.0, 600.0, 400.0, AppType::Terminal),
        WindowState::new_with_app(4, "TextEdit", 350.0, 200.0, 500.0, 400.0, AppType::TextEdit),
    ]);

    let (next_id, set_next_id) = signal(5usize);
    let (drag_op, set_drag_op) = signal(DragOperation::None);
    let (top_z_index, set_top_z_index) = signal(4i32);

    // Watch for System Settings open request
    Effect::new(move |_| {
        if system_state.open_system_settings.get() {
            // Reset the signal
            system_state.open_system_settings.set(false);

            // Check if System Settings is already open
            let already_open = windows.get().iter().any(|w| w.app_type == AppType::SystemSettings);
            if already_open {
                // Bring existing window to front
                if let Some(win) = windows.get().iter().find(|w| w.app_type == AppType::SystemSettings) {
                    let window_id = win.id;
                    let new_z = top_z_index.get() + 1;
                    set_top_z_index.set(new_z);
                    set_windows.update(|windows| {
                        if let Some(w) = windows.iter_mut().find(|w| w.id == window_id) {
                            w.z_index = new_z;
                            w.is_minimized = false;
                        }
                    });
                }
            } else {
                // Create new System Settings window
                let id = next_id.get();
                set_next_id.set(id + 1);
                let new_z = top_z_index.get() + 1;
                set_top_z_index.set(new_z);
                set_windows.update(|windows| {
                    let mut new_window = WindowState::new_with_app(
                        id,
                        "System Settings",
                        150.0,
                        100.0,
                        680.0,
                        500.0,
                        AppType::SystemSettings,
                    );
                    new_window.z_index = new_z;
                    windows.push(new_window);
                });
            }
        }
    });

    // Action trigger for keyboard shortcuts
    let (action_trigger, set_action_trigger) = signal(WindowAction::None);

    // Provide context for keyboard shortcut handler
    provide_context(WindowManagerContext {
        action_trigger: set_action_trigger,
    });

    // Set up keyboard shortcut listener
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::closure::Closure;
        use wasm_bindgen::JsCast;

        let cb = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
            // Check for Cmd (Meta) key on Mac or Ctrl on other platforms
            if e.meta_key() || e.ctrl_key() {
                match e.key().as_str() {
                    "q" | "Q" => {
                        e.prevent_default();
                        set_action_trigger.set(WindowAction::QuitAll);
                    }
                    "w" | "W" => {
                        e.prevent_default();
                        set_action_trigger.set(WindowAction::CloseActive);
                    }
                    "h" | "H" => {
                        // Only handle Cmd+H, not Ctrl+H (which might be browser history)
                        if e.meta_key() {
                            e.prevent_default();
                            set_action_trigger.set(WindowAction::HideActive);
                        }
                    }
                    _ => {}
                }
            }
        }) as Box<dyn Fn(web_sys::KeyboardEvent)>);

        let window = web_sys::window().unwrap();
        window
            .add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref())
            .unwrap();
        cb.forget();
    }

    // Bring window to front
    let bring_to_front = move |window_id: WindowId| {
        let new_z = top_z_index.get() + 1;
        set_top_z_index.set(new_z);
        set_windows.update(|windows| {
            if let Some(win) = windows.iter_mut().find(|w| w.id == window_id) {
                win.z_index = new_z;
            }
        });
    };

    // Close window
    let close_window = move |window_id: WindowId| {
        set_windows.update(|windows| {
            windows.retain(|w| w.id != window_id);
        });
    };

    // Minimize window
    let minimize_window = move |window_id: WindowId| {
        set_windows.update(|windows| {
            if let Some(win) = windows.iter_mut().find(|w| w.id == window_id) {
                win.is_minimized = true;
            }
        });
    };

    // Maximize/restore window
    let maximize_window = move |window_id: WindowId| {
        set_windows.update(|windows| {
            if let Some(win) = windows.iter_mut().find(|w| w.id == window_id) {
                if win.is_maximized {
                    // Restore
                    if let Some((x, y, w, h)) = win.pre_maximize {
                        win.x = x;
                        win.y = y;
                        win.width = w;
                        win.height = h;
                    }
                    win.is_maximized = false;
                    win.pre_maximize = None;
                } else {
                    // Maximize
                    win.pre_maximize = Some((win.x, win.y, win.width, win.height));
                    win.is_maximized = true;
                }
            }
        });
    };

    // Restore minimized window
    let restore_window = move |window_id: WindowId| {
        bring_to_front(window_id);
        set_windows.update(|windows| {
            if let Some(win) = windows.iter_mut().find(|w| w.id == window_id) {
                win.is_minimized = false;
            }
        });
    };

    // Start dragging (moving) a window
    let start_drag = move |window_id: WindowId, e: MouseEvent| {
        e.prevent_default();
        bring_to_front(window_id);

        let windows_val = windows.get();
        if let Some(win) = windows_val.iter().find(|w| w.id == window_id) {
            if !win.is_maximized {
                set_drag_op.set(DragOperation::Move {
                    window_id,
                    start_x: e.client_x() as f64,
                    start_y: e.client_y() as f64,
                    window_start_x: win.x,
                    window_start_y: win.y,
                });
            }
        }
    };

    // Start resizing a window
    let start_resize = move |window_id: WindowId, direction: ResizeDirection, e: MouseEvent| {
        e.prevent_default();
        e.stop_propagation();
        bring_to_front(window_id);

        let windows_val = windows.get();
        if let Some(win) = windows_val.iter().find(|w| w.id == window_id) {
            if !win.is_maximized {
                set_drag_op.set(DragOperation::Resize {
                    window_id,
                    direction,
                    start_x: e.client_x() as f64,
                    start_y: e.client_y() as f64,
                    window_start_x: win.x,
                    window_start_y: win.y,
                    window_start_width: win.width,
                    window_start_height: win.height,
                });
            }
        }
    };

    // Handle mouse move for drag/resize
    let on_mouse_move = move |e: MouseEvent| {
        let op = drag_op.get();
        match op {
            DragOperation::None => {}
            DragOperation::Move { window_id, start_x, start_y, window_start_x, window_start_y } => {
                let dx = e.client_x() as f64 - start_x;
                let dy = e.client_y() as f64 - start_y;
                set_windows.update(|windows| {
                    if let Some(win) = windows.iter_mut().find(|w| w.id == window_id) {
                        win.x = (window_start_x + dx).max(0.0);
                        win.y = (window_start_y + dy).max(0.0);
                    }
                });
            }
            DragOperation::Resize { window_id, direction, start_x, start_y, window_start_x, window_start_y, window_start_width, window_start_height } => {
                let dx = e.client_x() as f64 - start_x;
                let dy = e.client_y() as f64 - start_y;

                set_windows.update(|windows| {
                    if let Some(win) = windows.iter_mut().find(|w| w.id == window_id) {
                        let min_width = 200.0;
                        let min_height = 100.0;

                        match direction {
                            ResizeDirection::E => {
                                win.width = (window_start_width + dx).max(min_width);
                            }
                            ResizeDirection::W => {
                                let new_width = (window_start_width - dx).max(min_width);
                                let actual_dx = window_start_width - new_width;
                                win.x = window_start_x + actual_dx;
                                win.width = new_width;
                            }
                            ResizeDirection::S => {
                                win.height = (window_start_height + dy).max(min_height);
                            }
                            ResizeDirection::N => {
                                let new_height = (window_start_height - dy).max(min_height);
                                let actual_dy = window_start_height - new_height;
                                win.y = window_start_y + actual_dy;
                                win.height = new_height;
                            }
                            ResizeDirection::SE => {
                                win.width = (window_start_width + dx).max(min_width);
                                win.height = (window_start_height + dy).max(min_height);
                            }
                            ResizeDirection::SW => {
                                let new_width = (window_start_width - dx).max(min_width);
                                let actual_dx = window_start_width - new_width;
                                win.x = window_start_x + actual_dx;
                                win.width = new_width;
                                win.height = (window_start_height + dy).max(min_height);
                            }
                            ResizeDirection::NE => {
                                win.width = (window_start_width + dx).max(min_width);
                                let new_height = (window_start_height - dy).max(min_height);
                                let actual_dy = window_start_height - new_height;
                                win.y = window_start_y + actual_dy;
                                win.height = new_height;
                            }
                            ResizeDirection::NW => {
                                let new_width = (window_start_width - dx).max(min_width);
                                let actual_dx = window_start_width - new_width;
                                win.x = window_start_x + actual_dx;
                                win.width = new_width;
                                let new_height = (window_start_height - dy).max(min_height);
                                let actual_dy = window_start_height - new_height;
                                win.y = window_start_y + actual_dy;
                                win.height = new_height;
                            }
                        }
                    }
                });
            }
        }
    };

    // Handle mouse up to end drag/resize
    let on_mouse_up = move |_: MouseEvent| {
        set_drag_op.set(DragOperation::None);
    };

    // Get minimized windows for dock
    let minimized_windows = move || {
        windows.get().iter()
            .filter(|w| w.is_minimized)
            .cloned()
            .collect::<Vec<_>>()
    };

    // Get the active (top z-index non-minimized) window
    let active_window_id = move || {
        windows.get().iter()
            .filter(|w| !w.is_minimized)
            .max_by_key(|w| w.z_index)
            .map(|w| w.id)
    };

    // Handle keyboard shortcut actions
    Effect::new(move |_| {
        let action = action_trigger.get();
        match action {
            WindowAction::None => {}
            WindowAction::CloseActive => {
                if let Some(window_id) = active_window_id() {
                    set_windows.update(|windows| {
                        windows.retain(|w| w.id != window_id);
                    });
                }
                set_action_trigger.set(WindowAction::None);
            }
            WindowAction::HideActive => {
                if let Some(window_id) = active_window_id() {
                    set_windows.update(|windows| {
                        if let Some(win) = windows.iter_mut().find(|w| w.id == window_id) {
                            win.is_minimized = true;
                        }
                    });
                }
                set_action_trigger.set(WindowAction::None);
            }
            WindowAction::QuitAll => {
                set_windows.update(|windows| {
                    windows.clear();
                });
                set_action_trigger.set(WindowAction::None);
            }
        }
    });

    view! {
        <div
            class="desktop"
            on:mousemove=on_mouse_move
            on:mouseup=on_mouse_up
        >
            <For
                each=move || windows.get()
                key=|window| window.id
                children=move |window| {
                    let window_id = window.id;
                    let is_active = move || active_window_id() == Some(window_id);

                    let class_str = move || {
                        let win = windows.get().iter().find(|w| w.id == window_id).cloned();
                        let mut classes = vec!["window"];
                        if let Some(w) = &win {
                            if is_active() { classes.push("active"); }
                            if w.is_minimized { classes.push("minimized"); }
                            if w.is_maximized { classes.push("maximized"); }
                        }
                        classes.join(" ")
                    };

                    let style_str = move || {
                        let win = windows.get().iter().find(|w| w.id == window_id).cloned();
                        if let Some(w) = win {
                            format!(
                                "left: {}px; top: {}px; width: {}px; height: {}px; z-index: {};",
                                w.x, w.y, w.width, w.height, w.z_index
                            )
                        } else {
                            String::new()
                        }
                    };

                    let title = window.title.clone();
                    let title_for_content = title.clone();
                    let app_type = window.app_type.clone();
                    let is_calculator = app_type == AppType::Calculator;
                    let is_system_settings = app_type == AppType::SystemSettings;
                    let is_terminal = app_type == AppType::Terminal;
                    let is_textedit = app_type == AppType::TextEdit;

                    let content_class = if is_calculator {
                        "window-content calculator-content"
                    } else if is_system_settings {
                        "window-content settings-content"
                    } else if is_terminal {
                        "window-content terminal-content"
                    } else if is_textedit {
                        "window-content textedit-content"
                    } else {
                        "window-content"
                    };

                    view! {
                        <div
                            class=class_str
                            style=style_str
                            on:mousedown=move |_| bring_to_front(window_id)
                        >
                            // Title bar
                            <div
                                class="window-titlebar"
                                on:mousedown=move |e: MouseEvent| {
                                    if e.button() == 0 {
                                        start_drag(window_id, e);
                                    }
                                }
                            >
                                <div class="traffic-lights">
                                    <button
                                        class="traffic-light close"
                                        on:mousedown=move |e: MouseEvent| e.stop_propagation()
                                        on:click=move |_| close_window(window_id)
                                    />
                                    <button
                                        class="traffic-light minimize"
                                        on:mousedown=move |e: MouseEvent| e.stop_propagation()
                                        on:click=move |_| minimize_window(window_id)
                                    />
                                    <button
                                        class="traffic-light maximize"
                                        on:mousedown=move |e: MouseEvent| e.stop_propagation()
                                        on:click=move |_| maximize_window(window_id)
                                    />
                                </div>
                                <div class="window-title">{title}</div>
                            </div>

                            // Window content
                            <div class=content_class>
                                {if is_calculator {
                                    view! { <Calculator /> }.into_any()
                                } else if is_system_settings {
                                    view! { <SystemSettings /> }.into_any()
                                } else if is_terminal {
                                    view! { <Terminal /> }.into_any()
                                } else if is_textedit {
                                    view! { <TextEdit /> }.into_any()
                                } else if title_for_content == "Finder" {
                                    view! { <Finder /> }.into_any()
                                } else {
                                    view! { <p>"Window: " {title_for_content}</p> }.into_any()
                                }}
                            </div>

                            // Resize handles
                            <div class="resize-handle n" on:mousedown=move |e| start_resize(window_id, ResizeDirection::N, e) />
                            <div class="resize-handle s" on:mousedown=move |e| start_resize(window_id, ResizeDirection::S, e) />
                            <div class="resize-handle e" on:mousedown=move |e| start_resize(window_id, ResizeDirection::E, e) />
                            <div class="resize-handle w" on:mousedown=move |e| start_resize(window_id, ResizeDirection::W, e) />
                            <div class="resize-handle ne" on:mousedown=move |e| start_resize(window_id, ResizeDirection::NE, e) />
                            <div class="resize-handle nw" on:mousedown=move |e| start_resize(window_id, ResizeDirection::NW, e) />
                            <div class="resize-handle se" on:mousedown=move |e| start_resize(window_id, ResizeDirection::SE, e) />
                            <div class="resize-handle sw" on:mousedown=move |e| start_resize(window_id, ResizeDirection::SW, e) />
                        </div>
                    }
                }
            />

            // Dock for minimized windows
            <div class="dock">
                <For
                    each=minimized_windows
                    key=|window| window.id
                    children=move |window| {
                        let window_id = window.id;
                        let title = window.title.chars().take(2).collect::<String>().to_uppercase();
                        view! {
                            <div
                                class="dock-item"
                                on:click=move |_| restore_window(window_id)
                            >
                                {title}
                            </div>
                        }
                    }
                />
            </div>
        </div>
    }
}

