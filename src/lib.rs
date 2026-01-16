use leptos::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;

mod app_switcher;
mod calculator;
mod desktop;
mod dock;
mod finder;
mod menu_bar;
mod mission_control;
mod spotlight;
mod window_manager;

use app_switcher::AppSwitcher;
use desktop::Desktop;
use dock::Dock;
use menu_bar::MenuBar;
use mission_control::MissionControl;
use spotlight::Spotlight;
use window_manager::WindowManager;

#[component]
fn App() -> impl IntoView {
    view! {
        <MenuBar />
        <Desktop />
        <WindowManager />
        <Dock />
        <Spotlight />
        <AppSwitcher />
        <MissionControl />
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
