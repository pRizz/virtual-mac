use leptos::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;

mod calculator;
mod context_menu;
mod desktop;
mod dock;
mod finder;
mod menu_bar;
mod spotlight;
mod window_manager;

use context_menu::{ContextMenu, ContextMenuState};
use desktop::Desktop;
use dock::Dock;
use menu_bar::MenuBar;
use spotlight::Spotlight;
use window_manager::WindowManager;

#[component]
fn App() -> impl IntoView {
    // Global context menu state
    let (context_menu_state, set_context_menu_state) = signal(ContextMenuState::default());

    view! {
        <MenuBar />
        <Desktop context_menu_state=set_context_menu_state />
        <WindowManager />
        <Dock context_menu_state=set_context_menu_state />
        <Spotlight />
        <ContextMenu state=context_menu_state set_state=set_context_menu_state />
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
