use leptos::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;

mod calculator;
mod desktop;
mod dock;
mod finder;
mod menu_bar;
mod modals;
mod system_settings;
mod system_state;
mod window_manager;

use desktop::Desktop;
use dock::Dock;
use menu_bar::MenuBar;
use modals::{LockScreen, ModalOverlay, PowerOverlay};
use system_state::SystemState;
use window_manager::WindowManager;

#[component]
fn App() -> impl IntoView {
    // Provide system state context for all child components
    let system_state = SystemState::new();
    provide_context(system_state);

    view! {
        <MenuBar />
        <Desktop />
        <WindowManager />
        <Dock />
        <ModalOverlay />
        <LockScreen />
        <PowerOverlay />
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
