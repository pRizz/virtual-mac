use leptos::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;

mod calculator;
mod desktop;
mod dock;
mod finder;
mod menu_bar;
pub mod theme;
mod window_manager;

use desktop::Desktop;
use dock::Dock;
use menu_bar::MenuBar;
use theme::ThemeProvider;
use window_manager::WindowManager;

#[component]
fn App() -> impl IntoView {
    view! {
        <ThemeProvider>
            <MenuBar />
            <Desktop />
            <WindowManager />
            <Dock />
        </ThemeProvider>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
