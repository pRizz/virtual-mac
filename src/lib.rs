use leptos::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;

mod calculator;
mod desktop;
mod dock;
pub mod file_system;
mod finder;
mod menu_bar;
mod safari;
mod spotlight;
mod window_manager;

use desktop::Desktop;
use dock::Dock;
use file_system::FileSystemProvider;
use menu_bar::MenuBar;
use spotlight::Spotlight;
use window_manager::WindowManager;

#[component]
fn App() -> impl IntoView {
    view! {
        <FileSystemProvider>
            <MenuBar />
            <Desktop />
            <WindowManager />
            <Dock />
            <Spotlight />
        </FileSystemProvider>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
