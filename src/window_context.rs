use leptos::prelude::*;

/// Unique identifier for windows
pub type WindowId = usize;

/// Type of application in a window
#[derive(Clone, Debug, PartialEq)]
pub enum AppType {
    Generic,
    Calculator,
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
    pub pre_maximize: Option<(f64, f64, f64, f64)>,
    pub app_type: AppType,
}

impl WindowState {
    pub fn new(id: WindowId, title: &str, x: f64, y: f64, width: f64, height: f64) -> Self {
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
            app_type: AppType::Generic,
        }
    }

    pub fn new_with_app(id: WindowId, title: &str, x: f64, y: f64, width: f64, height: f64, app_type: AppType) -> Self {
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
        }
    }

    /// Get an icon emoji for this window based on its title/app type
    pub fn icon(&self) -> &'static str {
        match self.app_type {
            AppType::Calculator => "\u{1F5A9}",
            AppType::Generic => match self.title.as_str() {
                "Finder" => "\u{1F4C1}",
                "Notes" => "\u{1F4DD}",
                "Safari" => "\u{1F310}",
                "Messages" => "\u{1F4AC}",
                "Mail" => "\u{2709}",
                "Photos" => "\u{1F5BC}",
                "Music" => "\u{1F3B5}",
                _ => "\u{1F5D4}",
            },
        }
    }
}

/// Context for sharing window state across components
#[derive(Clone, Copy)]
pub struct WindowContext {
    pub windows: ReadSignal<Vec<WindowState>>,
    pub set_windows: WriteSignal<Vec<WindowState>>,
    pub top_z_index: ReadSignal<i32>,
    pub set_top_z_index: WriteSignal<i32>,
}

impl WindowContext {
    /// Bring a window to front by updating its z-index
    pub fn bring_to_front(&self, window_id: WindowId) {
        let new_z = self.top_z_index.get() + 1;
        self.set_top_z_index.set(new_z);
        self.set_windows.update(|windows| {
            if let Some(win) = windows.iter_mut().find(|w| w.id == window_id) {
                win.z_index = new_z;
            }
        });
    }

    /// Restore a minimized window and bring it to front
    pub fn restore_window(&self, window_id: WindowId) {
        self.bring_to_front(window_id);
        self.set_windows.update(|windows| {
            if let Some(win) = windows.iter_mut().find(|w| w.id == window_id) {
                win.is_minimized = false;
            }
        });
    }

    /// Get all non-minimized windows
    pub fn visible_windows(&self) -> Vec<WindowState> {
        self.windows.get().iter()
            .filter(|w| !w.is_minimized)
            .cloned()
            .collect()
    }

    /// Get all windows (including minimized)
    pub fn all_windows(&self) -> Vec<WindowState> {
        self.windows.get()
    }
}

/// Provide window context to the component tree
pub fn provide_window_context() -> WindowContext {
    let (windows, set_windows) = signal(vec![
        WindowState::new(1, "Finder", 100.0, 80.0, 600.0, 400.0),
        WindowState::new_with_app(2, "Calculator", 200.0, 150.0, 232.0, 340.0, AppType::Calculator),
        WindowState::new(3, "Notes", 350.0, 200.0, 400.0, 300.0),
    ]);
    let (top_z_index, set_top_z_index) = signal(3i32);

    let ctx = WindowContext {
        windows,
        set_windows,
        top_z_index,
        set_top_z_index,
    };

    provide_context(ctx);
    ctx
}

/// Get window context from the component tree
pub fn use_window_context() -> WindowContext {
    expect_context::<WindowContext>()
}
