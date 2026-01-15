use leptos::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;

mod desktop;
mod menu_bar;

use desktop::Desktop;
use menu_bar::MenuBar;

#[component]
fn App() -> impl IntoView {
    view! {
        <MenuBar />
        <Desktop />
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
