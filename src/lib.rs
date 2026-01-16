use leptos::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;

mod calculator;
mod desktop;
mod dock;
mod finder;
mod menu_bar;
mod spotlight;
mod window_manager;

use desktop::Desktop;
use dock::Dock;
use menu_bar::MenuBar;
use spotlight::Spotlight;
use window_manager::{WindowManager, ActiveAppContext};

#[component]
fn App() -> impl IntoView {
    // Create active app signal at App level so it's available to all components
    let (active_app, set_active_app) = signal("Finder".to_string());

    // Provide context for active app (used by MenuBar and written by WindowManager)
    provide_context(ActiveAppContext {
        active_app,
        set_active_app,
    });

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
