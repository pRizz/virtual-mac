use crate::context_menu::{show_context_menu, ContextMenuState, ContextMenuType};
use crate::wallpaper::{get_wallpaper_gradient, use_wallpaper_context};
use leptos::prelude::*;

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
pub fn Desktop(context_menu_state: WriteSignal<ContextMenuState>) -> impl IntoView {
    let wallpaper_ctx = use_wallpaper_context();
    let (selection, set_selection) = signal(SelectionRect::default());

    let background_style = move || {
        let gradient = get_wallpaper_gradient(wallpaper_ctx.current.get());
        format!("background: {}", gradient)
    };

    let on_mousedown = move |ev: web_sys::MouseEvent| {
        // Only start selection on left click
        if ev.button() != 0 {
            return;
        }
        let x = ev.client_x() as f64;
        let y = ev.client_y() as f64;
        set_selection.set(SelectionRect {
            start_x: x,
            start_y: y,
            current_x: x,
            current_y: y,
            active: true,
        });
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

    let on_contextmenu = move |ev: web_sys::MouseEvent| {
        ev.prevent_default();
        let x = ev.client_x() as f64;
        let y = ev.client_y() as f64;
        show_context_menu(context_menu_state, x, y, ContextMenuType::Desktop);
    };

    view! {
        <div
            class="desktop"
            style=background_style
            on:mousedown=on_mousedown
            on:mousemove=on_mousemove
            on:mouseup=on_mouseup
            on:contextmenu=on_contextmenu
        >
            <div class="feature-tour">
                <h1 class="feature-tour-title">"Welcome to VirtualMac"</h1>
                <p class="feature-tour-subtitle">"A macOS-style desktop experience in your browser"</p>
                <div class="feature-tour-items">
                    <div class="feature-tour-item">
                        <span class="feature-tour-icon">"🖱️"</span>
                        <span class="feature-tour-text">"Click and drag to select"</span>
                    </div>
                    <div class="feature-tour-item">
                        <span class="feature-tour-icon">"📁"</span>
                        <span class="feature-tour-text">"Open Finder to browse files"</span>
                    </div>
                    <div class="feature-tour-item">
                        <span class="feature-tour-icon">"🧮"</span>
                        <span class="feature-tour-text">"Launch Calculator from the dock"</span>
                    </div>
                    <div class="feature-tour-item">
                        <span class="feature-tour-icon">"🪟"</span>
                        <span class="feature-tour-text">"Drag windows to move, resize from edges"</span>
                    </div>
                </div>
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
            <div class="build-info">
                <span class="attribution">"Vibe coded by Peter Ryszkiewicz"</span>
                <span class="attribution">"with Claude Code, GSD, Cursor, and Ralph"</span>
                <span class="build-version">"v"{env!("CARGO_PKG_VERSION")}" · Built at "{env!("BUILD_TIME")}</span>
            </div>
        </div>
    }
}
