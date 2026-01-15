use leptos::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;

mod menu_bar;
use menu_bar::MenuBar;

#[component]
fn App() -> impl IntoView {
    view! {
        <MenuBar />
        <main class="main-content">
        </main>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
