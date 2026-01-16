use leptos::prelude::*;
use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;

/// Safari browser component
#[component]
pub fn Safari() -> impl IntoView {
    let (url, set_url) = signal("https://www.example.com".to_string());
    let (display_url, set_display_url) = signal("https://www.example.com".to_string());
    let (history, set_history) = signal(vec!["https://www.example.com".to_string()]);
    let (history_index, set_history_index) = signal(0usize);
    let (is_loading, set_is_loading) = signal(false);

    // Navigate to a URL
    let navigate_to = move |new_url: String| {
        let normalized_url = if !new_url.starts_with("http://") && !new_url.starts_with("https://") {
            format!("https://{}", new_url)
        } else {
            new_url
        };

        set_url.set(normalized_url.clone());
        set_display_url.set(normalized_url.clone());
        set_is_loading.set(true);

        // Add to history
        set_history.update(|h| {
            let idx = history_index.get();
            h.truncate(idx + 1);
            h.push(normalized_url);
        });
        set_history_index.update(|idx| *idx += 1);

        // Simulate loading complete after a short delay
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::closure::Closure;
            use wasm_bindgen::JsCast;

            let cb = Closure::once(Box::new(move || {
                set_is_loading.set(false);
            }) as Box<dyn FnOnce()>);

            if let Some(window) = web_sys::window() {
                let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                    cb.as_ref().unchecked_ref(),
                    500,
                );
            }
            cb.forget();
        }
    };

    // Go back in history
    let go_back = move |_| {
        let idx = history_index.get();
        if idx > 0 {
            set_history_index.set(idx - 1);
            let h = history.get();
            if let Some(prev_url) = h.get(idx - 1) {
                set_url.set(prev_url.clone());
                set_display_url.set(prev_url.clone());
            }
        }
    };

    // Go forward in history
    let go_forward = move |_| {
        let idx = history_index.get();
        let h = history.get();
        if idx + 1 < h.len() {
            set_history_index.set(idx + 1);
            if let Some(next_url) = h.get(idx + 1) {
                set_url.set(next_url.clone());
                set_display_url.set(next_url.clone());
            }
        }
    };

    // Refresh current page
    let refresh = move |_| {
        let current_url = url.get();
        set_is_loading.set(true);
        // Force iframe reload by setting URL to empty then back
        set_url.set(String::new());

        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::closure::Closure;
            use wasm_bindgen::JsCast;

            let url_to_set = current_url.clone();
            let cb = Closure::once(Box::new(move || {
                set_url.set(url_to_set);
                set_is_loading.set(false);
            }) as Box<dyn FnOnce()>);

            if let Some(window) = web_sys::window() {
                let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                    cb.as_ref().unchecked_ref(),
                    100,
                );
            }
            cb.forget();
        }
    };

    // Check if can go back/forward
    let can_go_back = move || history_index.get() > 0;
    let can_go_forward = move || {
        let idx = history_index.get();
        let len = history.get().len();
        idx + 1 < len
    };

    // Handle URL input submit
    let on_url_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        navigate_to(display_url.get());
    };

    // Handle URL input change
    let on_url_input = move |ev: web_sys::Event| {
        let target = ev.target().unwrap();
        let input = target.unchecked_ref::<HtmlInputElement>();
        set_display_url.set(input.value());
    };

    // Handle keyboard enter in URL bar
    let on_url_keydown = move |ev: web_sys::KeyboardEvent| {
        if ev.key() == "Enter" {
            ev.prevent_default();
            navigate_to(display_url.get());
        }
    };

    view! {
        <div class="safari">
            // Toolbar
            <div class="safari-toolbar">
                // Navigation buttons
                <div class="safari-nav-buttons">
                    <button
                        class=move || if can_go_back() { "safari-nav-btn" } else { "safari-nav-btn disabled" }
                        on:click=go_back
                        title="Go Back"
                    >
                        <span class="safari-nav-icon">"◀"</span>
                    </button>
                    <button
                        class=move || if can_go_forward() { "safari-nav-btn" } else { "safari-nav-btn disabled" }
                        on:click=go_forward
                        title="Go Forward"
                    >
                        <span class="safari-nav-icon">"▶"</span>
                    </button>
                </div>

                // URL bar
                <form class="safari-url-bar" on:submit=on_url_submit>
                    <div class="safari-url-container">
                        <span class=move || if is_loading.get() { "safari-loading active" } else { "safari-loading" }></span>
                        <button
                            type="button"
                            class="safari-refresh-btn"
                            on:click=refresh
                            title="Reload"
                        >
                            <span class="refresh-icon">"↻"</span>
                        </button>
                        <input
                            type="text"
                            class="safari-url-input"
                            prop:value=move || display_url.get()
                            on:input=on_url_input
                            on:keydown=on_url_keydown
                            placeholder="Search or enter website name"
                        />
                    </div>
                </form>

                // Share/sidebar buttons (decorative)
                <div class="safari-toolbar-right">
                    <button class="safari-toolbar-btn" title="Share">
                        <span>"↗"</span>
                    </button>
                    <button class="safari-toolbar-btn" title="Show Sidebar">
                        <span>"☰"</span>
                    </button>
                </div>
            </div>

            // Tab bar (single tab for now)
            <div class="safari-tab-bar">
                <div class="safari-tab active">
                    <span class="safari-tab-favicon">"🌐"</span>
                    <span class="safari-tab-title">
                        {move || {
                            let u = url.get();
                            if u.is_empty() {
                                "New Tab".to_string()
                            } else {
                                // Extract domain for display
                                u.replace("https://", "")
                                    .replace("http://", "")
                                    .split('/')
                                    .next()
                                    .unwrap_or("New Tab")
                                    .to_string()
                            }
                        }}
                    </span>
                    <button class="safari-tab-close">"×"</button>
                </div>
                <button class="safari-new-tab-btn" title="New Tab">"+"</button>
            </div>

            // Content area with iframe
            <div class="safari-content">
                <Show
                    when=move || !url.get().is_empty()
                    fallback=|| view! {
                        <div class="safari-start-page">
                            <div class="safari-start-content">
                                <div class="safari-start-icon">"🧭"</div>
                                <h1 class="safari-start-title">"Safari"</h1>
                                <p class="safari-start-subtitle">"Enter a URL to get started"</p>
                                <div class="safari-start-favorites">
                                    <div class="safari-favorite-item">
                                        <div class="safari-favorite-icon">"🍎"</div>
                                        <span>"Apple"</span>
                                    </div>
                                    <div class="safari-favorite-item">
                                        <div class="safari-favorite-icon">"📰"</div>
                                        <span>"News"</span>
                                    </div>
                                    <div class="safari-favorite-item">
                                        <div class="safari-favorite-icon">"🎬"</div>
                                        <span>"YouTube"</span>
                                    </div>
                                    <div class="safari-favorite-item">
                                        <div class="safari-favorite-icon">"🐙"</div>
                                        <span>"GitHub"</span>
                                    </div>
                                </div>
                            </div>
                        </div>
                    }
                >
                    <iframe
                        class="safari-iframe"
                        src=move || url.get()
                        sandbox="allow-scripts allow-same-origin allow-forms allow-popups"
                        referrerpolicy="no-referrer"
                    ></iframe>
                </Show>
            </div>
        </div>
    }
}
