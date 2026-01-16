use leptos::prelude::*;
use leptos::ev::MouseEvent;

#[allow(unused_imports)]
use wasm_bindgen::prelude::*;

use crate::finder::Finder;
use crate::calculator::Calculator;
use crate::window_context::{WindowId, AppType, use_window_context};

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
    // Get shared window context
    let ctx = use_window_context();
    let windows = ctx.windows;
    let set_windows = ctx.set_windows;
    let top_z_index = ctx.top_z_index;
    let set_top_z_index = ctx.set_top_z_index;

    let (drag_op, set_drag_op) = signal(DragOperation::None);

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

                    let content_class = if is_calculator {
                        "window-content calculator-content"
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
