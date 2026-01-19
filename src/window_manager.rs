use leptos::ev::MouseEvent;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::closure::Closure;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

/// Menu bar height in pixels (matches CSS --menubar-height)
const MENU_BAR_HEIGHT: f64 = 25.0;

use crate::calculator::Calculator;
use crate::finder::Finder;
use crate::notes::Notes;
use crate::notification::NotificationState;
use crate::system_settings::SystemSettings;
use crate::system_state::{MinimizedWindow, SystemState};
use crate::terminal::Terminal;
use crate::textedit::TextEdit;

/// Actions that can be triggered via keyboard shortcuts
#[derive(Clone, Copy, Debug, PartialEq, Default)]
#[allow(dead_code)]
pub enum WindowAction {
    #[default]
    None,
    CloseActive,
    HideActive,
    QuitAll,
}

/// Context for triggering window manager actions from outside the component
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub struct WindowManagerContext {
    pub action_trigger: WriteSignal<WindowAction>,
}

/// Unique identifier for windows
pub type WindowId = usize;

/// Type of application in a window
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AppType {
    Calculator,
    SystemSettings,
    Terminal,
    TextEdit,
    Notes,
    Finder,
}

impl AppType {
    /// Get the icon character for this app type
    pub fn icon(&self) -> &'static str {
        match self {
            AppType::Calculator => "=",
            AppType::SystemSettings => "⚙️",
            AppType::Terminal => ">_",
            AppType::TextEdit => "T",
            AppType::Notes => "📝",
            AppType::Finder => "📂",
        }
    }

    /// Get the CSS class for this app type's icon
    pub fn icon_class(&self) -> &'static str {
        match self {
            AppType::Calculator => "calculator",
            AppType::SystemSettings => "settings",
            AppType::Terminal => "terminal",
            AppType::TextEdit => "textedit",
            AppType::Notes => "notes",
            AppType::Finder => "finder",
        }
    }
}

impl std::fmt::Display for AppType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            AppType::Calculator => "Calculator",
            AppType::SystemSettings => "System Settings",
            AppType::Terminal => "Terminal",
            AppType::TextEdit => "TextEdit",
            AppType::Notes => "Notes",
            AppType::Finder => "Finder",
        };
        write!(f, "{name}")
    }
}

/// Animation state for window minimize/restore
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum AnimationState {
    #[default]
    None,
    Minimizing,
    Restoring,
}

/// Represents the state of a single window
#[derive(Clone, Debug, PartialEq)]
pub struct WindowState {
    pub id: WindowId,
    pub title: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub z_index: i32,
    pub is_minimized: bool,
    pub is_maximized: bool,
    /// Stored position/size before maximizing
    pub pre_maximize: Option<(f64, f64, f64, f64)>,
    /// Type of application in this window
    pub app_type: AppType,
    /// Current animation state
    pub animation: AnimationState,
    /// Target X position for minimize/restore animation (screen coordinates)
    pub animation_target_x: Option<f64>,
}

impl WindowState {
    pub fn new_with_app(
        id: WindowId,
        title: &str,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        app_type: AppType,
    ) -> Self {
        Self {
            id,
            title: title.to_string(),
            x,
            y,
            width,
            height,
            z_index: id as i32,
            is_minimized: false,
            is_maximized: false,
            pre_maximize: None,
            app_type,
            animation: AnimationState::None,
            animation_target_x: None,
        }
    }

    /// Convert to persistable form
    #[allow(dead_code)]
    fn to_persisted(&self) -> PersistedWindow {
        PersistedWindow {
            app_type: self.app_type.clone(),
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
            z_index: self.z_index,
            is_minimized: self.is_minimized,
            is_maximized: self.is_maximized,
            pre_maximize: self.pre_maximize,
        }
    }

    /// Create from persisted form
    #[allow(dead_code)]
    fn from_persisted(persisted: &PersistedWindow, id: WindowId) -> Self {
        let title = match &persisted.app_type {
            AppType::Calculator => "Calculator",
            AppType::SystemSettings => "System Settings",
            AppType::Terminal => "Terminal",
            AppType::TextEdit => "TextEdit",
            AppType::Notes => "Notes",
            AppType::Finder => "Finder",
        };
        Self {
            id,
            title: title.to_string(),
            x: persisted.x,
            y: persisted.y,
            width: persisted.width,
            height: persisted.height,
            z_index: persisted.z_index,
            is_minimized: persisted.is_minimized,
            is_maximized: persisted.is_maximized,
            pre_maximize: persisted.pre_maximize,
            app_type: persisted.app_type.clone(),
            animation: AnimationState::None,
            animation_target_x: None,
        }
    }
}

/// Persisted window state (excludes animation state)
#[derive(Clone, Debug, Serialize, Deserialize)]
#[allow(dead_code)]
struct PersistedWindow {
    app_type: AppType,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    z_index: i32,
    is_minimized: bool,
    is_maximized: bool,
    pre_maximize: Option<(f64, f64, f64, f64)>,
}

/// Full desktop state for persistence
#[derive(Clone, Debug, Serialize, Deserialize)]
#[allow(dead_code)]
struct PersistedDesktopState {
    schema_version: u32,
    windows: Vec<PersistedWindow>,
    next_window_id: usize,
    top_z_index: i32,
}

#[allow(dead_code)]
const CURRENT_SCHEMA_VERSION: u32 = 1;
#[allow(dead_code)]
const STORAGE_KEY: &str = "virtualmac_desktop";

/// Save desktop state to localStorage
#[allow(unused_variables)]
fn save_desktop_state(windows: &[WindowState], next_id: usize, top_z: i32) {
    #[cfg(target_arch = "wasm32")]
    {
        let state = PersistedDesktopState {
            schema_version: CURRENT_SCHEMA_VERSION,
            windows: windows.iter().map(|w| w.to_persisted()).collect(),
            next_window_id: next_id,
            top_z_index: top_z,
        };
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(json) = serde_json::to_string(&state) {
                    let _ = storage.set_item(STORAGE_KEY, &json);
                }
            }
        }
    }
}

/// Load desktop state from localStorage
/// Returns (windows, next_id, top_z, schema_mismatch)
fn load_desktop_state() -> Option<(Vec<WindowState>, usize, i32, bool)> {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(json)) = storage.get_item(STORAGE_KEY) {
                    if let Ok(state) = serde_json::from_str::<PersistedDesktopState>(&json) {
                        let schema_mismatch = state.schema_version != CURRENT_SCHEMA_VERSION;
                        let windows: Vec<WindowState> = state
                            .windows
                            .iter()
                            .enumerate()
                            .map(|(i, pw)| WindowState::from_persisted(pw, i + 1))
                            .collect();
                        return Some((
                            windows,
                            state.next_window_id,
                            state.top_z_index,
                            schema_mismatch,
                        ));
                    }
                }
            }
        }
    }
    None
}

/// Clear desktop state from localStorage
#[allow(dead_code)]
fn clear_desktop_state() {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.remove_item(STORAGE_KEY);
            }
        }
    }
}

/// Drag/resize operation state
#[derive(Clone, Debug, PartialEq)]
enum DragOperation {
    None,
    Move {
        window_id: WindowId,
        start_x: f64,
        start_y: f64,
        window_start_x: f64,
        window_start_y: f64,
    },
    Resize {
        window_id: WindowId,
        direction: ResizeDirection,
        start_x: f64,
        start_y: f64,
        window_start_x: f64,
        window_start_y: f64,
        window_start_width: f64,
        window_start_height: f64,
    },
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum ResizeDirection {
    N,
    S,
    E,
    W,
    NE,
    NW,
    SE,
    SW,
}

/// Default windows when no persisted state exists
fn default_windows() -> Vec<WindowState> {
    vec![
        WindowState::new_with_app(1, "Finder", 100.0, 80.0, 600.0, 400.0, AppType::Finder),
        WindowState::new_with_app(2, "Terminal", 255.0, 165.0, 600.0, 400.0, AppType::Terminal),
        WindowState::new_with_app(
            3,
            "TextEdit",
            1200.0,
            150.0,
            500.0,
            400.0,
            AppType::TextEdit,
        ),
        WindowState::new_with_app(
            4,
            "Calculator",
            900.0,
            100.0,
            280.0,
            540.0,
            AppType::Calculator,
        ),
    ]
}

/// Desktop component that contains the window manager
#[component]
pub fn WindowManager() -> impl IntoView {
    let system_state = expect_context::<SystemState>();
    let notification_state = expect_context::<NotificationState>();

    // Load persisted state or use defaults
    let (initial_windows, initial_next_id, initial_top_z, schema_mismatch) = load_desktop_state()
        .unwrap_or_else(|| {
            let defaults = default_windows();
            let count = defaults.len();
            (defaults, count + 1, count as i32, false)
        });

    // Show notification if schema changed (desktop was reset due to update)
    if schema_mismatch {
        notification_state.show(
            "Desktop Updated",
            "An update has reset some desktop settings. Your files and apps are safe.",
        );
    }

    // Global state for all windows
    let (windows, set_windows) = signal(initial_windows);
    let (next_id, set_next_id) = signal(initial_next_id);
    let (drag_op, set_drag_op) = signal(DragOperation::None);
    let (top_z_index, set_top_z_index) = signal(initial_top_z);

    // Auto-sync active app from windows state
    // This Effect watches windows and automatically updates the active app in menu bar
    // whenever windows change (open, close, focus, minimize, restore, etc.)
    Effect::new(move |_| {
        let current_windows = windows.get();
        let active_app = current_windows
            .iter()
            .filter(|w| !w.is_minimized)
            .max_by_key(|w| w.z_index)
            .map(|w| w.title.clone())
            .unwrap_or_else(|| "Finder".to_string());
        system_state.set_active_app(&active_app);
    });

    // Sync minimized windows to SystemState for dock display
    Effect::new(move |_| {
        let current_windows = windows.get();
        let minimized: Vec<MinimizedWindow> = current_windows
            .iter()
            .filter(|w| w.is_minimized)
            .map(|w| MinimizedWindow {
                id: w.id,
                title: w.title.clone(),
                icon: w.app_type.icon().to_string(),
                icon_class: w.app_type.icon_class().to_string(),
            })
            .collect();
        system_state.minimized_windows.set(minimized);
    });

    // Sync open window app names for dock running indicators
    Effect::new(move |_| {
        let current_windows = windows.get();
        let mut open_apps: Vec<String> = current_windows
            .iter()
            .map(|w| w.app_type.to_string())
            .collect();
        open_apps.sort();
        open_apps.dedup();
        system_state.open_windows.set(open_apps);
    });

    // Save state when windows change (debounced via effect)
    Effect::new(move |_| {
        let current_windows = windows.get();
        let current_next_id = next_id.get();
        let current_top_z = top_z_index.get();
        save_desktop_state(&current_windows, current_next_id, current_top_z);
    });

    // Watch for desktop reset request
    Effect::new(move |_| {
        if system_state.reset_desktop.get() {
            system_state.reset_desktop.set(false);
            // Clear persisted state
            clear_desktop_state();
            // Reset to defaults
            let defaults = default_windows();
            let count = defaults.len();
            set_windows.set(defaults);
            set_next_id.set(count + 1);
            set_top_z_index.set(count as i32);
        }
    });

    // Watch for System Settings open request
    Effect::new(move |_| {
        if system_state.open_system_settings.get() {
            // Reset the signal
            system_state.open_system_settings.set(false);

            // Check if System Settings is already open
            let already_open = windows
                .get()
                .iter()
                .any(|w| w.app_type == AppType::SystemSettings);
            if already_open {
                // Bring existing window to front
                if let Some(win) = windows
                    .get()
                    .iter()
                    .find(|w| w.app_type == AppType::SystemSettings)
                {
                    let window_id = win.id;
                    let new_z = top_z_index.get() + 1;
                    set_top_z_index.set(new_z);
                    set_windows.update(|windows| {
                        if let Some(w) = windows.iter_mut().find(|w| w.id == window_id) {
                            w.z_index = new_z;
                            w.is_minimized = false;
                        }
                    });
                }
            } else {
                // Create new System Settings window
                let id = next_id.get();
                set_next_id.set(id + 1);
                let new_z = top_z_index.get() + 1;
                set_top_z_index.set(new_z);
                set_windows.update(|windows| {
                    let mut new_window = WindowState::new_with_app(
                        id,
                        "System Settings",
                        150.0,
                        100.0,
                        680.0,
                        500.0,
                        AppType::SystemSettings,
                    );
                    new_window.z_index = new_z;
                    windows.push(new_window);
                });
            }
        }
    });

    // Watch for dock app open requests
    Effect::new(move |_| {
        if let Some(app_name) = system_state.open_app.get() {
            // Reset the signal
            system_state.open_app.set(None);

            // Map app name to AppType
            let app_type = match app_name.as_str() {
                "Finder" => Some(AppType::Finder),
                "Calculator" => Some(AppType::Calculator),
                "Terminal" => Some(AppType::Terminal),
                "TextEdit" => Some(AppType::TextEdit),
                "Notes" => Some(AppType::Notes),
                "System Settings" => Some(AppType::SystemSettings),
                _ => None,
            };

            if let Some(target_type) = app_type {
                // Check if app is already open
                let existing = windows
                    .get()
                    .iter()
                    .find(|w| w.app_type == target_type)
                    .map(|w| (w.id, w.is_minimized));

                if let Some((window_id, is_minimized)) = existing {
                    // Bring existing window to front
                    let new_z = top_z_index.get() + 1;
                    set_top_z_index.set(new_z);
                    set_windows.update(|windows| {
                        if let Some(w) = windows.iter_mut().find(|w| w.id == window_id) {
                            w.z_index = new_z;
                            if is_minimized {
                                w.is_minimized = false;
                            }
                        }
                    });
                } else {
                    // Create new window for this app
                    let id = next_id.get();
                    set_next_id.set(id + 1);
                    let new_z = top_z_index.get() + 1;
                    set_top_z_index.set(new_z);

                    // Get default dimensions for each app
                    let (title, x, y, w, h) = match target_type {
                        AppType::Calculator => ("Calculator", 200.0, 150.0, 280.0, 540.0),
                        AppType::Terminal => ("Terminal", 300.0, 120.0, 600.0, 400.0),
                        AppType::TextEdit => ("TextEdit", 350.0, 200.0, 500.0, 400.0),
                        AppType::Notes => ("Notes", 450.0, 220.0, 700.0, 500.0),
                        AppType::SystemSettings => ("System Settings", 150.0, 100.0, 680.0, 500.0),
                        AppType::Finder => ("Finder", 100.0, 80.0, 600.0, 400.0),
                    };

                    set_windows.update(|windows| {
                        let mut new_window =
                            WindowState::new_with_app(id, title, x, y, w, h, target_type);
                        new_window.z_index = new_z;
                        windows.push(new_window);
                    });
                }
            }
        }
    });

    // Action trigger for keyboard shortcuts
    let (action_trigger, set_action_trigger) = signal(WindowAction::None);

    // Provide context for keyboard shortcut handler
    provide_context(WindowManagerContext {
        action_trigger: set_action_trigger,
    });

    // Set up keyboard shortcut listener
    #[cfg(target_arch = "wasm32")]
    {
        let cb = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
            // Check for Cmd (Meta) key on Mac or Ctrl on other platforms
            if e.meta_key() || e.ctrl_key() {
                match e.key().as_str() {
                    "q" | "Q" => {
                        e.prevent_default();
                        set_action_trigger.set(WindowAction::QuitAll);
                    }
                    "w" | "W" => {
                        e.prevent_default();
                        set_action_trigger.set(WindowAction::CloseActive);
                    }
                    "h" | "H" => {
                        // Only handle Cmd+H, not Ctrl+H (which might be browser history)
                        if e.meta_key() {
                            e.prevent_default();
                            set_action_trigger.set(WindowAction::HideActive);
                        }
                    }
                    _ => {}
                }
            }
        }) as Box<dyn Fn(web_sys::KeyboardEvent)>);

        let window = web_sys::window().unwrap();
        window
            .add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref())
            .unwrap();
        cb.forget();
    }

    // Bring window to front (active app auto-updates via Effect watching windows)
    let bring_to_front = move |window_id: WindowId| {
        let new_z = top_z_index.get() + 1;
        set_top_z_index.set(new_z);

        set_windows.update(|windows| {
            if let Some(win) = windows.iter_mut().find(|w| w.id == window_id) {
                win.z_index = new_z;
            }
        });
    };

    // Close window (active app auto-updates via Effect watching windows)
    let close_window = move |window_id: WindowId| {
        set_windows.update(|windows| {
            windows.retain(|w| w.id != window_id);
        });
    };

    // Minimize window with genie animation
    let minimize_window = move |window_id: WindowId| {
        // Calculate the target X position for the animation
        // The minimized dock appears to the right of the main dock, centered as a whole
        #[cfg(target_arch = "wasm32")]
        let target_x = {
            let screen_width = web_sys::window()
                .and_then(|w| Some(w.inner_width().ok()?.as_f64()?))
                .unwrap_or(1920.0);

            // Count current minimized windows (this will be the new item's index)
            let minimized_count = windows.get().iter().filter(|w| w.is_minimized).count();

            // Main dock is approximately 930px wide (14 apps + separator + downloads + trash)
            let main_dock_width = 930.0;
            // Minimized dock: separator margin (8px each side) + dock padding (12px each side)
            let minimized_dock_start_offset = 8.0 + 12.0;
            // Each minimized item is 56px with 6px gap
            let item_width = 56.0;
            let item_gap = 6.0;

            // Calculate center offset based on number of minimized items
            // As items are added, the whole dock shifts left to stay centered
            let minimized_dock_width = if minimized_count == 0 {
                // This will be the first item
                12.0 + item_width + 12.0 // padding + item + padding
            } else {
                12.0 + (minimized_count as f64 + 1.0) * item_width
                    + (minimized_count as f64) * item_gap
                    + 12.0
            };

            // The dock wrapper centers both docks together
            let total_width = main_dock_width + 16.0 + minimized_dock_width; // 16 = separator margins
            let dock_start_x = (screen_width - total_width) / 2.0;

            // Position of the new item in the minimized dock
            dock_start_x
                + main_dock_width
                + minimized_dock_start_offset
                + (minimized_count as f64 * (item_width + item_gap))
                + item_width / 2.0
        };
        #[cfg(not(target_arch = "wasm32"))]
        let target_x = 960.0; // Default center for non-wasm

        // Start the minimize animation with target position
        set_windows.update(|windows| {
            if let Some(win) = windows.iter_mut().find(|w| w.id == window_id) {
                win.animation = AnimationState::Minimizing;
                win.animation_target_x = Some(target_x);
            }
        });

        // After animation completes (400ms), set minimized state
        #[cfg(target_arch = "wasm32")]
        {
            let cb = Closure::once(Box::new(move || {
                set_windows.update(|windows| {
                    if let Some(win) = windows.iter_mut().find(|w| w.id == window_id) {
                        win.animation = AnimationState::None;
                        win.is_minimized = true;
                    }
                });
            }) as Box<dyn FnOnce()>);

            let window = web_sys::window().unwrap();
            window
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    cb.as_ref().unchecked_ref(),
                    400, // Match animation duration
                )
                .unwrap();
            cb.forget();
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            set_windows.update(|windows| {
                if let Some(win) = windows.iter_mut().find(|w| w.id == window_id) {
                    win.animation = AnimationState::None;
                    win.is_minimized = true;
                }
            });
        }
    };

    // Maximize/restore window
    let maximize_window = move |window_id: WindowId| {
        set_windows.update(|windows| {
            if let Some(win) = windows.iter_mut().find(|w| w.id == window_id) {
                if win.is_maximized {
                    // Restore
                    if let Some((x, y, w, h)) = win.pre_maximize {
                        win.x = x;
                        win.y = y;
                        win.width = w;
                        win.height = h;
                    }
                    win.is_maximized = false;
                    win.pre_maximize = None;
                } else {
                    // Maximize
                    win.pre_maximize = Some((win.x, win.y, win.width, win.height));
                    win.is_maximized = true;
                }
            }
        });
    };

    // Restore minimized window with genie animation
    let restore_window = move |window_id: WindowId| {
        // Calculate the source X position (where the item is in the minimized dock)
        #[cfg(target_arch = "wasm32")]
        let source_x = {
            let screen_width = web_sys::window()
                .and_then(|w| Some(w.inner_width().ok()?.as_f64()?))
                .unwrap_or(1920.0);

            // Find the index of this window in the minimized windows list
            let minimized_windows: Vec<_> = windows
                .get()
                .iter()
                .filter(|w| w.is_minimized)
                .map(|w| w.id)
                .collect();
            let item_index = minimized_windows
                .iter()
                .position(|&id| id == window_id)
                .unwrap_or(0);
            let minimized_count = minimized_windows.len();

            // Main dock is approximately 930px wide
            let main_dock_width = 930.0;
            let minimized_dock_start_offset = 8.0 + 12.0; // separator margin + dock padding
            let item_width = 56.0;
            let item_gap = 6.0;

            // Calculate minimized dock width
            let minimized_dock_width = 12.0
                + (minimized_count as f64) * item_width
                + ((minimized_count - 1).max(0) as f64) * item_gap
                + 12.0;

            // The dock wrapper centers both docks together
            let total_width = main_dock_width + 16.0 + minimized_dock_width;
            let dock_start_x = (screen_width - total_width) / 2.0;

            // Position of this item in the minimized dock
            dock_start_x
                + main_dock_width
                + minimized_dock_start_offset
                + (item_index as f64 * (item_width + item_gap))
                + item_width / 2.0
        };
        #[cfg(not(target_arch = "wasm32"))]
        let source_x = 960.0;

        bring_to_front(window_id);
        // Start restore animation - remove minimized and add restoring
        set_windows.update(|windows| {
            if let Some(win) = windows.iter_mut().find(|w| w.id == window_id) {
                win.is_minimized = false;
                win.animation = AnimationState::Restoring;
                win.animation_target_x = Some(source_x);
            }
        });

        // After animation completes (400ms), clear animation state
        #[cfg(target_arch = "wasm32")]
        {
            let cb = Closure::once(Box::new(move || {
                set_windows.update(|windows| {
                    if let Some(win) = windows.iter_mut().find(|w| w.id == window_id) {
                        win.animation = AnimationState::None;
                    }
                });
            }) as Box<dyn FnOnce()>);

            let window = web_sys::window().unwrap();
            window
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    cb.as_ref().unchecked_ref(),
                    400, // Match animation duration
                )
                .unwrap();
            cb.forget();
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            set_windows.update(|windows| {
                if let Some(win) = windows.iter_mut().find(|w| w.id == window_id) {
                    win.animation = AnimationState::None;
                }
            });
        }
    };

    // Watch for restore window requests from dock
    Effect::new(move |_| {
        if let Some(window_id) = system_state.restore_window_id.get() {
            system_state.restore_window_id.set(None);
            restore_window(window_id);
        }
    });

    // Start dragging (moving) a window
    let start_drag = move |window_id: WindowId, e: MouseEvent| {
        e.prevent_default();
        bring_to_front(window_id);

        let windows_val = windows.get();
        if let Some(win) = windows_val.iter().find(|w| w.id == window_id) {
            if !win.is_maximized {
                set_drag_op.set(DragOperation::Move {
                    window_id,
                    start_x: e.client_x() as f64,
                    start_y: e.client_y() as f64,
                    window_start_x: win.x,
                    window_start_y: win.y,
                });
            }
        }
    };

    // Start resizing a window
    let start_resize = move |window_id: WindowId, direction: ResizeDirection, e: MouseEvent| {
        e.prevent_default();
        e.stop_propagation();
        bring_to_front(window_id);

        let windows_val = windows.get();
        if let Some(win) = windows_val.iter().find(|w| w.id == window_id) {
            if !win.is_maximized {
                set_drag_op.set(DragOperation::Resize {
                    window_id,
                    direction,
                    start_x: e.client_x() as f64,
                    start_y: e.client_y() as f64,
                    window_start_x: win.x,
                    window_start_y: win.y,
                    window_start_width: win.width,
                    window_start_height: win.height,
                });
            }
        }
    };

    // Handle mouse move for drag/resize
    let on_mouse_move = move |e: MouseEvent| {
        let op = drag_op.get();
        match op {
            DragOperation::None => {}
            DragOperation::Move {
                window_id,
                start_x,
                start_y,
                window_start_x,
                window_start_y,
            } => {
                let dx = e.client_x() as f64 - start_x;
                let dy = e.client_y() as f64 - start_y;
                set_windows.update(|windows| {
                    if let Some(win) = windows.iter_mut().find(|w| w.id == window_id) {
                        win.x = window_start_x + dx;
                        win.y = (window_start_y + dy).max(MENU_BAR_HEIGHT);
                    }
                });
            }
            DragOperation::Resize {
                window_id,
                direction,
                start_x,
                start_y,
                window_start_x,
                window_start_y,
                window_start_width,
                window_start_height,
            } => {
                let dx = e.client_x() as f64 - start_x;
                let dy = e.client_y() as f64 - start_y;

                set_windows.update(|windows| {
                    if let Some(win) = windows.iter_mut().find(|w| w.id == window_id) {
                        let min_width = 200.0;
                        let min_height = 100.0;

                        match direction {
                            ResizeDirection::E => {
                                win.width = (window_start_width + dx).max(min_width);
                            }
                            ResizeDirection::W => {
                                let new_width = (window_start_width - dx).max(min_width);
                                let actual_dx = window_start_width - new_width;
                                win.x = window_start_x + actual_dx;
                                win.width = new_width;
                            }
                            ResizeDirection::S => {
                                win.height = (window_start_height + dy).max(min_height);
                            }
                            ResizeDirection::N => {
                                let new_height = (window_start_height - dy).max(min_height);
                                let actual_dy = window_start_height - new_height;
                                win.y = window_start_y + actual_dy;
                                win.height = new_height;
                            }
                            ResizeDirection::SE => {
                                win.width = (window_start_width + dx).max(min_width);
                                win.height = (window_start_height + dy).max(min_height);
                            }
                            ResizeDirection::SW => {
                                let new_width = (window_start_width - dx).max(min_width);
                                let actual_dx = window_start_width - new_width;
                                win.x = window_start_x + actual_dx;
                                win.width = new_width;
                                win.height = (window_start_height + dy).max(min_height);
                            }
                            ResizeDirection::NE => {
                                win.width = (window_start_width + dx).max(min_width);
                                let new_height = (window_start_height - dy).max(min_height);
                                let actual_dy = window_start_height - new_height;
                                win.y = window_start_y + actual_dy;
                                win.height = new_height;
                            }
                            ResizeDirection::NW => {
                                let new_width = (window_start_width - dx).max(min_width);
                                let actual_dx = window_start_width - new_width;
                                win.x = window_start_x + actual_dx;
                                win.width = new_width;
                                let new_height = (window_start_height - dy).max(min_height);
                                let actual_dy = window_start_height - new_height;
                                win.y = window_start_y + actual_dy;
                                win.height = new_height;
                            }
                        }
                    }
                });
            }
        }
    };

    // Handle mouse up to end drag/resize
    let on_mouse_up = move |_: MouseEvent| {
        set_drag_op.set(DragOperation::None);
    };

    // Set up document-level mouse event listeners for drag/resize operations.
    // This ensures drag continues even when mouse moves over menu bar or other elements.
    #[cfg(target_arch = "wasm32")]
    {
        let doc_mousemove_handler = Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let op = drag_op.get_untracked();
            match op {
                DragOperation::None => {}
                DragOperation::Move {
                    window_id,
                    start_x,
                    start_y,
                    window_start_x,
                    window_start_y,
                } => {
                    let dx = e.client_x() as f64 - start_x;
                    let dy = e.client_y() as f64 - start_y;
                    set_windows.update(|windows| {
                        if let Some(win) = windows.iter_mut().find(|w| w.id == window_id) {
                            win.x = window_start_x + dx;
                            win.y = (window_start_y + dy).max(MENU_BAR_HEIGHT);
                        }
                    });
                }
                DragOperation::Resize {
                    window_id,
                    direction,
                    start_x,
                    start_y,
                    window_start_x,
                    window_start_y,
                    window_start_width,
                    window_start_height,
                } => {
                    let dx = e.client_x() as f64 - start_x;
                    let dy = e.client_y() as f64 - start_y;

                    set_windows.update(|windows| {
                        if let Some(win) = windows.iter_mut().find(|w| w.id == window_id) {
                            let min_width = 200.0;
                            let min_height = 100.0;

                            match direction {
                                ResizeDirection::E => {
                                    win.width = (window_start_width + dx).max(min_width);
                                }
                                ResizeDirection::W => {
                                    let new_width = (window_start_width - dx).max(min_width);
                                    let actual_dx = window_start_width - new_width;
                                    win.x = window_start_x + actual_dx;
                                    win.width = new_width;
                                }
                                ResizeDirection::S => {
                                    win.height = (window_start_height + dy).max(min_height);
                                }
                                ResizeDirection::N => {
                                    let new_height = (window_start_height - dy).max(min_height);
                                    let actual_dy = window_start_height - new_height;
                                    win.y = window_start_y + actual_dy;
                                    win.height = new_height;
                                }
                                ResizeDirection::SE => {
                                    win.width = (window_start_width + dx).max(min_width);
                                    win.height = (window_start_height + dy).max(min_height);
                                }
                                ResizeDirection::SW => {
                                    let new_width = (window_start_width - dx).max(min_width);
                                    let actual_dx = window_start_width - new_width;
                                    win.x = window_start_x + actual_dx;
                                    win.width = new_width;
                                    win.height = (window_start_height + dy).max(min_height);
                                }
                                ResizeDirection::NE => {
                                    win.width = (window_start_width + dx).max(min_width);
                                    let new_height = (window_start_height - dy).max(min_height);
                                    let actual_dy = window_start_height - new_height;
                                    win.y = window_start_y + actual_dy;
                                    win.height = new_height;
                                }
                                ResizeDirection::NW => {
                                    let new_width = (window_start_width - dx).max(min_width);
                                    let actual_dx = window_start_width - new_width;
                                    win.x = window_start_x + actual_dx;
                                    win.width = new_width;
                                    let new_height = (window_start_height - dy).max(min_height);
                                    let actual_dy = window_start_height - new_height;
                                    win.y = window_start_y + actual_dy;
                                    win.height = new_height;
                                }
                            }
                        }
                    });
                }
            }
        }) as Box<dyn Fn(web_sys::MouseEvent)>);

        let doc_mouseup_handler = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            set_drag_op.set(DragOperation::None);
        }) as Box<dyn Fn(web_sys::MouseEvent)>);

        // Add document-level listeners
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                let _ = document.add_event_listener_with_callback(
                    "mousemove",
                    doc_mousemove_handler.as_ref().unchecked_ref(),
                );
                let _ = document.add_event_listener_with_callback(
                    "mouseup",
                    doc_mouseup_handler.as_ref().unchecked_ref(),
                );
            }
        }

        // Keep closures alive for the lifetime of the app
        doc_mousemove_handler.forget();
        doc_mouseup_handler.forget();
    }

    // Get the active (top z-index non-minimized) window
    let active_window_id = move || {
        windows
            .get()
            .iter()
            .filter(|w| !w.is_minimized)
            .max_by_key(|w| w.z_index)
            .map(|w| w.id)
    };

    // Handle keyboard shortcut actions
    Effect::new(move |_| {
        let action = action_trigger.get();
        match action {
            WindowAction::None => {}
            WindowAction::CloseActive => {
                if let Some(window_id) = active_window_id() {
                    set_windows.update(|windows| {
                        windows.retain(|w| w.id != window_id);
                    });
                }
                set_action_trigger.set(WindowAction::None);
            }
            WindowAction::HideActive => {
                if let Some(window_id) = active_window_id() {
                    set_windows.update(|windows| {
                        if let Some(win) = windows.iter_mut().find(|w| w.id == window_id) {
                            win.is_minimized = true;
                        }
                    });
                }
                set_action_trigger.set(WindowAction::None);
            }
            WindowAction::QuitAll => {
                set_windows.update(|windows| {
                    windows.clear();
                });
                set_action_trigger.set(WindowAction::None);
            }
        }
    });

    view! {
        <div
            class="windows-container"
            on:mousemove=on_mouse_move
            on:mouseup=on_mouse_up
        >
            <For
                each=move || windows.get()
                key=|window| window.id
                children=move |window| {
                    let window_id = window.id;
                    let is_active = move || active_window_id() == Some(window_id);

                    let app_type_for_class = window.app_type.clone();
                    let class_str = move || {
                        let win = windows.get().iter().find(|w| w.id == window_id).cloned();
                        let mut classes = vec!["window"];
                        if app_type_for_class == AppType::Terminal {
                            classes.push("terminal-window");
                        }
                        if let Some(w) = &win {
                            if is_active() { classes.push("active"); }
                            if w.is_minimized { classes.push("minimized"); }
                            if w.is_maximized { classes.push("maximized"); }
                            match w.animation {
                                AnimationState::Minimizing => classes.push("minimizing"),
                                AnimationState::Restoring => classes.push("restoring"),
                                AnimationState::None => {}
                            }
                        }
                        classes.join(" ")
                    };

                    let style_str = move || {
                        let win = windows.get().iter().find(|w| w.id == window_id).cloned();
                        if let Some(w) = win {
                            let base_style = format!(
                                "left: {}px; top: {}px; width: {}px; height: {}px; z-index: {};",
                                w.x, w.y, w.width, w.height, w.z_index
                            );
                            // Add animation target position as CSS custom property
                            if let Some(target_x) = w.animation_target_x {
                                // Calculate the horizontal offset from window center to dock target
                                let window_center_x = w.x + w.width / 2.0;
                                let offset_x = target_x - window_center_x;
                                format!("{} --dock-target-x: {}px;", base_style, offset_x)
                            } else {
                                base_style
                            }
                        } else {
                            String::new()
                        }
                    };

                    let title = window.title.clone();
                    let title_for_content = title.clone();
                    let app_type = window.app_type.clone();
                    let is_calculator = app_type == AppType::Calculator;
                    let is_system_settings = app_type == AppType::SystemSettings;
                    let is_terminal = app_type == AppType::Terminal;
                    let is_textedit = app_type == AppType::TextEdit;
                    let is_notes = app_type == AppType::Notes;
                    let is_finder = app_type == AppType::Finder;

                    let content_class = if is_calculator {
                        "window-content calculator-content"
                    } else if is_system_settings {
                        "window-content settings-content"
                    } else if is_terminal {
                        "window-content terminal-content"
                    } else if is_textedit {
                        "window-content textedit-content"
                    } else if is_notes {
                        "window-content notes-content"
                    } else if is_finder {
                        "window-content finder-content"
                    } else {
                        "window-content"
                    };

                    view! {
                        <div
                            class=class_str
                            style=style_str
                            on:mousedown=move |_| bring_to_front(window_id)
                        >
                            // Title bar
                            <div
                                class="window-titlebar"
                                on:mousedown=move |e: MouseEvent| {
                                    if e.button() == 0 {
                                        start_drag(window_id, e);
                                    }
                                }
                            >
                                <div class="traffic-lights">
                                    <button
                                        class="traffic-light close"
                                        on:mousedown=move |e: MouseEvent| e.stop_propagation()
                                        on:click=move |_| close_window(window_id)
                                    />
                                    <button
                                        class="traffic-light minimize"
                                        on:mousedown=move |e: MouseEvent| e.stop_propagation()
                                        on:click=move |_| minimize_window(window_id)
                                    />
                                    <button
                                        class="traffic-light maximize"
                                        on:mousedown=move |e: MouseEvent| e.stop_propagation()
                                        on:click=move |_| maximize_window(window_id)
                                    />
                                </div>
                                <div class="window-title">{title}</div>
                            </div>

                            // Window content
                            <div class=content_class>
                                {if is_calculator {
                                    view! { <Calculator /> }.into_any()
                                } else if is_system_settings {
                                    view! { <SystemSettings /> }.into_any()
                                } else if is_terminal {
                                    view! { <Terminal /> }.into_any()
                                } else if is_textedit {
                                    view! { <TextEdit /> }.into_any()
                                } else if is_notes {
                                    view! { <Notes /> }.into_any()
                                } else if is_finder {
                                    view! { <Finder /> }.into_any()
                                } else {
                                    view! { <p>"Window: " {title_for_content}</p> }.into_any()
                                }}
                            </div>

                            // Resize handles
                            <div class="resize-handle n" on:mousedown=move |e| start_resize(window_id, ResizeDirection::N, e) />
                            <div class="resize-handle s" on:mousedown=move |e| start_resize(window_id, ResizeDirection::S, e) />
                            <div class="resize-handle e" on:mousedown=move |e| start_resize(window_id, ResizeDirection::E, e) />
                            <div class="resize-handle w" on:mousedown=move |e| start_resize(window_id, ResizeDirection::W, e) />
                            <div class="resize-handle ne" on:mousedown=move |e| start_resize(window_id, ResizeDirection::NE, e) />
                            <div class="resize-handle nw" on:mousedown=move |e| start_resize(window_id, ResizeDirection::NW, e) />
                            <div class="resize-handle se" on:mousedown=move |e| start_resize(window_id, ResizeDirection::SE, e) />
                            <div class="resize-handle sw" on:mousedown=move |e| start_resize(window_id, ResizeDirection::SW, e) />
                        </div>
                    }
                }
            />

        </div>
    }
}
