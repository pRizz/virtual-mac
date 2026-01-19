use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::KeyboardEvent;

/// Searchable item for Spotlight results
#[derive(Clone, Debug, PartialEq)]
pub struct SearchResult {
    pub name: String,
    pub kind: SearchResultKind,
    pub icon: &'static str,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SearchResultKind {
    Application,
    Document,
    Folder,
}

impl SearchResult {
    fn applications() -> Vec<Self> {
        vec![
            SearchResult {
                name: "Finder".to_string(),
                kind: SearchResultKind::Application,
                icon: "\u{1F4C1}",
            },
            SearchResult {
                name: "Calculator".to_string(),
                kind: SearchResultKind::Application,
                icon: "\u{1F5A9}",
            },
            SearchResult {
                name: "Notes".to_string(),
                kind: SearchResultKind::Application,
                icon: "\u{1F4DD}",
            },
            SearchResult {
                name: "Safari".to_string(),
                kind: SearchResultKind::Application,
                icon: "\u{1F310}",
            },
            SearchResult {
                name: "Messages".to_string(),
                kind: SearchResultKind::Application,
                icon: "\u{1F4AC}",
            },
            SearchResult {
                name: "Mail".to_string(),
                kind: SearchResultKind::Application,
                icon: "\u{2709}",
            },
            SearchResult {
                name: "Photos".to_string(),
                kind: SearchResultKind::Application,
                icon: "\u{1F5BC}",
            },
            SearchResult {
                name: "Music".to_string(),
                kind: SearchResultKind::Application,
                icon: "\u{1F3B5}",
            },
            SearchResult {
                name: "Calendar".to_string(),
                kind: SearchResultKind::Application,
                icon: "\u{1F4C5}",
            },
            SearchResult {
                name: "Settings".to_string(),
                kind: SearchResultKind::Application,
                icon: "\u{2699}",
            },
            SearchResult {
                name: "Terminal".to_string(),
                kind: SearchResultKind::Application,
                icon: "\u{1F4BB}",
            },
        ]
    }

    fn documents() -> Vec<Self> {
        vec![
            SearchResult {
                name: "Documents".to_string(),
                kind: SearchResultKind::Folder,
                icon: "\u{1F4C2}",
            },
            SearchResult {
                name: "Downloads".to_string(),
                kind: SearchResultKind::Folder,
                icon: "\u{1F4E5}",
            },
            SearchResult {
                name: "Desktop".to_string(),
                kind: SearchResultKind::Folder,
                icon: "\u{1F5A5}",
            },
            SearchResult {
                name: "README.md".to_string(),
                kind: SearchResultKind::Document,
                icon: "\u{1F4C4}",
            },
            SearchResult {
                name: "notes.txt".to_string(),
                kind: SearchResultKind::Document,
                icon: "\u{1F4C4}",
            },
        ]
    }

    fn all() -> Vec<Self> {
        let mut all = Self::applications();
        all.extend(Self::documents());
        all
    }

    fn search(query: &str) -> Vec<Self> {
        if query.is_empty() {
            return vec![];
        }
        let query_lower = query.to_lowercase();
        Self::all()
            .into_iter()
            .filter(|item| item.name.to_lowercase().contains(&query_lower))
            .take(8)
            .collect()
    }
}

/// Spotlight search overlay component
#[component]
pub fn Spotlight() -> impl IntoView {
    let (is_visible, set_is_visible) = signal(false);
    let (query, set_query) = signal(String::new());
    let (selected_index, set_selected_index) = signal(0usize);

    // Compute search results based on query
    let results = Memo::new(move |_| SearchResult::search(&query.get()));

    // Set up global keyboard listener for Cmd+Space
    Effect::new(move |_| {
        let closure = Closure::wrap(Box::new(move |e: KeyboardEvent| {
            // Check for Cmd+Space (meta key + space) or Ctrl+Space
            if e.code() == "Space" && (e.meta_key() || e.ctrl_key()) {
                e.prevent_default();
                let currently_visible = is_visible.get();
                set_is_visible.set(!currently_visible);
                if currently_visible {
                    // Reset when closing - will be clean for next open
                } else {
                    // Reset when opening
                    set_query.set(String::new());
                    set_selected_index.set(0);
                }
            }

            // Handle Escape to close
            if e.code() == "Escape" && is_visible.get() {
                e.prevent_default();
                set_is_visible.set(false);
            }

            // Handle arrow keys and Enter when visible
            if is_visible.get_untracked() {
                let results_vec = results.get_untracked();
                match e.code().as_str() {
                    "ArrowDown" => {
                        e.prevent_default();
                        let max = results_vec.len().saturating_sub(1);
                        set_selected_index.update(|i| {
                            if *i < max {
                                *i += 1;
                            }
                        });
                    }
                    "ArrowUp" => {
                        e.prevent_default();
                        set_selected_index.update(|i| {
                            if *i > 0 {
                                *i -= 1;
                            }
                        });
                    }
                    "Enter" => {
                        e.prevent_default();
                        // Close spotlight on selection
                        set_is_visible.set(false);
                    }
                    _ => {}
                }
            }
        }) as Box<dyn FnMut(KeyboardEvent)>);

        let window = web_sys::window().expect("no window");
        window
            .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
            .expect("failed to add keydown listener");

        // Keep the closure alive
        closure.forget();
    });

    // Handle input changes
    let on_input = move |ev: web_sys::Event| {
        let target = ev.target().unwrap();
        let input: web_sys::HtmlInputElement = target.unchecked_into();
        set_query.set(input.value());
        set_selected_index.set(0);
    };

    // Handle clicking outside to close
    let on_backdrop_click = move |_| {
        set_is_visible.set(false);
    };

    view! {
        <Show when=move || is_visible.get()>
            <div class="spotlight-backdrop" on:click=on_backdrop_click>
                <div class="spotlight-container" on:click=move |e: web_sys::MouseEvent| e.stop_propagation()>
                    <div class="spotlight-search-bar">
                        <span class="spotlight-icon">{"\u{1F50D}"}</span>
                        <input
                            type="text"
                            class="spotlight-input"
                            placeholder="Spotlight Search"
                            prop:value=move || query.get()
                            on:input=on_input
                            autofocus=true
                        />
                    </div>
                    <Show when=move || !results.get().is_empty()>
                        <div class="spotlight-results">
                            {move || {
                                results.get().into_iter().enumerate().map(|(index, item)| {
                                    let is_selected = move || selected_index.get() == index;
                                    let item_class = move || {
                                        if is_selected() {
                                            "spotlight-result-item selected"
                                        } else {
                                            "spotlight-result-item"
                                        }
                                    };
                                    let kind_label = match item.kind {
                                        SearchResultKind::Application => "Application",
                                        SearchResultKind::Document => "Document",
                                        SearchResultKind::Folder => "Folder",
                                    };
                                    let name = item.name.clone();
                                    let icon = item.icon;

                                    view! {
                                        <div
                                            class=item_class
                                            on:mouseenter=move |_| set_selected_index.set(index)
                                            on:click=move |_| set_is_visible.set(false)
                                        >
                                            <span class="spotlight-result-icon">{icon}</span>
                                            <div class="spotlight-result-info">
                                                <span class="spotlight-result-name">{name}</span>
                                                <span class="spotlight-result-kind">{kind_label}</span>
                                            </div>
                                        </div>
                                    }
                                }).collect::<Vec<_>>()
                            }}
                        </div>
                    </Show>
                </div>
            </div>
        </Show>
    }
}
