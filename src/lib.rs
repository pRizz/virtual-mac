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
mod notification;
mod spotlight;
mod system_settings;
mod system_state;
mod terminal;
mod textedit;
mod notes;
pub mod theme;
mod wallpaper;
mod window_manager;

use app_switcher::AppSwitcher;
use context_menu::{ContextMenu, ContextMenuState};
use desktop::Desktop;
use dock::Dock;
use file_system::FileSystemProvider;
use menu_bar::MenuBar;
use modals::{LockScreen, ModalOverlay, PowerOverlay};
use notification::{NotificationContainer, NotificationState};
use spotlight::Spotlight;
use system_state::SystemState;
use theme::ThemeProvider;
use wallpaper::provide_wallpaper_context;
use window_manager::WindowManager;

#[component]
fn App() -> impl IntoView {
    // Global context menu state
    let (context_menu_state, set_context_menu_state) = signal(ContextMenuState::default());

    // Provide system state context for all child components
    let system_state = SystemState::new();
    provide_context(system_state);

    // Provide notification state context
    let notification_state = NotificationState::new();
    provide_context(notification_state);

    // Provide wallpaper context
    provide_wallpaper_context();

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
                <NotificationContainer />
            </FileSystemProvider>
        </ThemeProvider>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
