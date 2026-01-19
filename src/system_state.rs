use leptos::prelude::*;

/// Minimized window info for dock display
#[derive(Clone, Debug, PartialEq)]
pub struct MinimizedWindow {
    pub id: usize,
    pub title: String,
    pub icon: String,
    pub icon_class: String,
}

/// System-wide state for VirtualMac
#[derive(Clone, Copy)]
pub struct SystemState {
    /// Whether the lock screen is active
    pub is_locked: RwSignal<bool>,
    /// Current system power state
    pub power_state: RwSignal<PowerState>,
    /// Active modal dialog
    pub active_modal: RwSignal<Option<ModalType>>,
    /// Whether System Settings should be opened
    pub open_system_settings: RwSignal<bool>,
    /// App to open from dock click (app name)
    pub open_app: RwSignal<Option<String>>,
    /// Whether a desktop reset was requested
    pub reset_desktop: RwSignal<bool>,
    /// Currently active/foreground app name (shown in menu bar)
    pub active_app: RwSignal<String>,
    /// Minimized windows for dock display
    pub minimized_windows: RwSignal<Vec<MinimizedWindow>>,
    /// Open windows for dock running indicators
    pub open_windows: RwSignal<Vec<String>>,
    /// Request to restore a minimized window by ID
    pub restore_window_id: RwSignal<Option<usize>>,
}

/// Power state of the system
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum PowerState {
    #[default]
    Running,
    Sleeping,
    ShuttingDown,
    Restarting,
}

/// Types of modal dialogs
#[derive(Clone, Debug, PartialEq)]
pub enum ModalType {
    AboutThisMac,
    ShutDownConfirm,
    RestartConfirm,
    LogOutConfirm,
    ForceQuit,
    ResetDesktopConfirm,
}

impl SystemState {
    pub fn new() -> Self {
        Self {
            is_locked: RwSignal::new(false),
            power_state: RwSignal::new(PowerState::Running),
            active_modal: RwSignal::new(None),
            open_system_settings: RwSignal::new(false),
            open_app: RwSignal::new(None),
            reset_desktop: RwSignal::new(false),
            active_app: RwSignal::new("Finder".to_string()), // Default to Finder like real macOS
            minimized_windows: RwSignal::new(Vec::new()),
            open_windows: RwSignal::new(Vec::new()),
            restore_window_id: RwSignal::new(None),
        }
    }

    /// Set the active/foreground app (shown in menu bar)
    pub fn set_active_app(&self, app_name: &str) {
        self.active_app.set(app_name.to_string());
    }

    pub fn request_open_app(&self, app_name: &str) {
        self.open_app.set(Some(app_name.to_string()));
    }

    pub fn lock_screen(&self) {
        self.is_locked.set(true);
    }

    pub fn unlock_screen(&self) {
        self.is_locked.set(false);
    }

    pub fn sleep(&self) {
        self.power_state.set(PowerState::Sleeping);
    }

    pub fn wake(&self) {
        self.power_state.set(PowerState::Running);
    }

    pub fn shut_down(&self) {
        self.power_state.set(PowerState::ShuttingDown);
    }

    pub fn restart(&self) {
        self.power_state.set(PowerState::Restarting);
    }

    pub fn show_modal(&self, modal: ModalType) {
        self.active_modal.set(Some(modal));
    }

    pub fn close_modal(&self) {
        self.active_modal.set(None);
    }

    #[allow(dead_code)]
    pub fn toggle_system_settings(&self) {
        self.open_system_settings.update(|v| *v = !*v);
    }
}

impl Default for SystemState {
    fn default() -> Self {
        Self::new()
    }
}
