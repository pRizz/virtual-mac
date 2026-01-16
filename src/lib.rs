use leptos::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;

mod calculator;
mod desktop;
mod dock;
mod finder;
mod menu_bar;
mod spotlight;
pub mod system_state;
mod window_manager;

use desktop::Desktop;
use dock::Dock;
use menu_bar::MenuBar;
use spotlight::Spotlight;
use system_state::provide_clipboard_context;
use window_manager::WindowManager;

#[component]
fn App() -> impl IntoView {
    // Provide global clipboard state
    provide_clipboard_context();

    view! {
        <MenuBar />
        <Desktop />
        <WindowManager />
        <Dock />
        <Spotlight />
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
