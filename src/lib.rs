use leptos::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;

mod app_switcher;
mod calculator;
mod context_menu;
mod desktop;
mod dock;
pub mod file_system;
mod finder;
mod menu_bar;
mod modals;
mod spotlight;
mod system_settings;
mod system_state;
mod terminal;
mod textedit;
pub mod theme;
mod window_manager;

use app_switcher::AppSwitcher;
use context_menu::{ContextMenu, ContextMenuState};
use desktop::Desktop;
use dock::Dock;
use file_system::FileSystemProvider;
use menu_bar::MenuBar;
use modals::{LockScreen, ModalOverlay, PowerOverlay};
use spotlight::Spotlight;
use system_state::SystemState;
use theme::ThemeProvider;
use window_manager::WindowManager;

#[component]
fn App() -> impl IntoView {
    // Global context menu state
    let (context_menu_state, set_context_menu_state) = signal(ContextMenuState::default());

    // Provide system state context for all child components
    let system_state = SystemState::new();
    provide_context(system_state);

    view! {
        <ThemeProvider>
            <FileSystemProvider>
                <MenuBar />
                <Desktop context_menu_state=set_context_menu_state />
                <WindowManager />
                <Dock context_menu_state=set_context_menu_state />
                <Spotlight />
                <AppSwitcher />
                <ContextMenu state=context_menu_state set_state=set_context_menu_state />
                <ModalOverlay />
                <LockScreen />
                <PowerOverlay />
            </FileSystemProvider>
        </ThemeProvider>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
