use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    pub fn as_str(&self) -> &'static str {
        match self {
            Theme::Light => "light",
            Theme::Dark => "dark",
        }
    }

    pub fn toggle(&self) -> Self {
        match self {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        }
    }
}

#[derive(Clone, Copy)]
pub struct ThemeContext {
    pub theme: ReadSignal<Theme>,
    pub set_theme: WriteSignal<Theme>,
}

impl ThemeContext {
    pub fn toggle(&self) {
        let current = self.theme.get();
        self.set_theme.set(current.toggle());
    }

    pub fn is_dark(&self) -> bool {
        self.theme.get() == Theme::Dark
    }
}

/// Apply theme to the document root element
#[allow(unused_variables)]
fn apply_theme_to_document(theme: Theme) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(root) = document.document_element() {
                    let _ = root.set_attribute("data-theme", theme.as_str());
                }
            }
        }
    }
}

/// Load saved theme from localStorage
fn load_saved_theme() -> Theme {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(saved)) = storage.get_item("virtualmac-theme") {
                    return match saved.as_str() {
                        "dark" => Theme::Dark,
                        _ => Theme::Light,
                    };
                }
            }
        }
    }
    Theme::Light
}

/// Save theme preference to localStorage
#[allow(unused_variables)]
fn save_theme(theme: Theme) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item("virtualmac-theme", theme.as_str());
            }
        }
    }
}

/// Provide theme context to the application
#[component]
pub fn ThemeProvider(children: Children) -> impl IntoView {
    let initial_theme = load_saved_theme();
    let (theme, set_theme) = signal(initial_theme);

    // Apply theme on mount
    apply_theme_to_document(initial_theme);

    // Watch for theme changes and apply them
    Effect::new(move || {
        let current_theme = theme.get();
        apply_theme_to_document(current_theme);
        save_theme(current_theme);
    });

    let context = ThemeContext { theme, set_theme };
    provide_context(context);

    children()
}

/// Get the current theme context
pub fn use_theme() -> ThemeContext {
    expect_context::<ThemeContext>()
}
