use leptos::prelude::*;
use leptos::ev::MouseEvent;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = document)]
    fn execCommand(command: &str, show_ui: bool, value: &str) -> bool;
}

#[component]
pub fn TextEdit() -> impl IntoView {
    let (font_size, set_font_size) = signal(16u32);
    let (is_bold, set_is_bold) = signal(false);
    let (is_italic, set_is_italic) = signal(false);

    let toggle_bold = move |_: MouseEvent| {
        execCommand("bold", false, "");
        set_is_bold.update(|b| *b = !*b);
    };

    let toggle_italic = move |_: MouseEvent| {
        execCommand("italic", false, "");
        set_is_italic.update(|i| *i = !*i);
    };

    let increase_font = move |_: MouseEvent| {
        set_font_size.update(|s| {
            if *s < 72 {
                *s += 2;
            }
        });
        let size = font_size.get();
        execCommand("fontSize", false, "7");
    };

    let decrease_font = move |_: MouseEvent| {
        set_font_size.update(|s| {
            if *s > 8 {
                *s -= 2;
            }
        });
        execCommand("fontSize", false, "1");
    };

    view! {
        <div class="textedit">
            <div class="textedit-toolbar">
                <div class="textedit-toolbar-group">
                    <button
                        class=move || if is_bold.get() { "textedit-btn active" } else { "textedit-btn" }
                        on:click=toggle_bold
                        title="Bold"
                    >
                        <strong>"B"</strong>
                    </button>
                    <button
                        class=move || if is_italic.get() { "textedit-btn active" } else { "textedit-btn" }
                        on:click=toggle_italic
                        title="Italic"
                    >
                        <em>"I"</em>
                    </button>
                </div>
                <div class="textedit-toolbar-separator"></div>
                <div class="textedit-toolbar-group">
                    <button
                        class="textedit-btn"
                        on:click=decrease_font
                        title="Decrease font size"
                    >
                        "A-"
                    </button>
                    <span class="textedit-font-size">{move || format!("{}px", font_size.get())}</span>
                    <button
                        class="textedit-btn"
                        on:click=increase_font
                        title="Increase font size"
                    >
                        "A+"
                    </button>
                </div>
            </div>
            <div
                class="textedit-content"
                contenteditable="true"
                style=move || format!("font-size: {}px;", font_size.get())
            >
                "Start typing here..."
            </div>
            <div class="textedit-statusbar">
                <span>"TextEdit"</span>
            </div>
        </div>
    }
}
