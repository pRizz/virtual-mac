use leptos::prelude::*;

/// A single notification
#[derive(Clone, Debug)]
pub struct Notification {
    pub id: usize,
    pub title: String,
    pub message: String,
    #[allow(dead_code)]
    pub icon: Option<String>,
    pub exiting: bool,
    #[allow(dead_code)]
    pub timeout_handle: Option<i32>,
}

/// Global notification state
#[derive(Clone, Copy)]
pub struct NotificationState {
    notifications: RwSignal<Vec<Notification>>,
    next_id: RwSignal<usize>,
}

impl NotificationState {
    pub fn new() -> Self {
        Self {
            notifications: RwSignal::new(Vec::new()),
            next_id: RwSignal::new(1),
        }
    }

    /// Show a notification
    pub fn show(&self, title: impl Into<String>, message: impl Into<String>) {
        let id = self.next_id.get();
        self.next_id.set(id + 1);

        let notification = Notification {
            id,
            title: title.into(),
            message: message.into(),
            icon: None,
            exiting: false,
            timeout_handle: None,
        };

        self.notifications.update(|n| n.push(notification));

        // Auto-dismiss after 5 seconds (uses dismiss() for exit animation)
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::closure::Closure;
            use wasm_bindgen::JsCast;

            let notifications = self.notifications;
            let cb = Closure::once(Box::new(move || {
                // Set exiting state
                notifications.update(|n| {
                    if let Some(notif) = n.iter_mut().find(|notif| notif.id == id) {
                        notif.exiting = true;
                    }
                });
                // Schedule actual removal after animation
                let notifications_inner = notifications;
                let remove_cb = Closure::once(Box::new(move || {
                    notifications_inner.update(|n| n.retain(|notif| notif.id != id));
                }) as Box<dyn FnOnce()>);
                if let Some(window) = web_sys::window() {
                    let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                        remove_cb.as_ref().unchecked_ref(),
                        400,
                    );
                    remove_cb.forget();
                }
            }) as Box<dyn FnOnce()>);

            if let Some(window) = web_sys::window() {
                if let Ok(handle) = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                    cb.as_ref().unchecked_ref(),
                    5000,
                ) {
                    // Store handle for cancellation on hover
                    self.notifications.update(|n| {
                        if let Some(notif) = n.iter_mut().find(|notif| notif.id == id) {
                            notif.timeout_handle = Some(handle);
                        }
                    });
                }
                cb.forget();
            }
        }
    }

    /// Pause auto-dismiss timer when hovering
    pub fn pause_auto_dismiss(&self, id: usize) {
        #[cfg(target_arch = "wasm32")]
        {
            self.notifications.update(|n| {
                if let Some(notif) = n.iter_mut().find(|notif| notif.id == id) {
                    if let Some(handle) = notif.timeout_handle.take() {
                        if let Some(window) = web_sys::window() {
                            window.clear_timeout_with_handle(handle);
                        }
                    }
                }
            });
        }
        // Non-wasm: no-op
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = id;
        }
    }

    /// Resume auto-dismiss timer when hover ends
    pub fn resume_auto_dismiss(&self, id: usize) {
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::closure::Closure;
            use wasm_bindgen::JsCast;

            let notifications = self.notifications;
            let cb = Closure::once(Box::new(move || {
                // Set exiting state
                notifications.update(|n| {
                    if let Some(notif) = n.iter_mut().find(|notif| notif.id == id) {
                        notif.exiting = true;
                    }
                });
                // Schedule actual removal after animation
                let notifications_inner = notifications;
                let remove_cb = Closure::once(Box::new(move || {
                    notifications_inner.update(|n| n.retain(|notif| notif.id != id));
                }) as Box<dyn FnOnce()>);
                if let Some(window) = web_sys::window() {
                    let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                        remove_cb.as_ref().unchecked_ref(),
                        400,
                    );
                    remove_cb.forget();
                }
            }) as Box<dyn FnOnce()>);

            if let Some(window) = web_sys::window() {
                if let Ok(handle) = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                    cb.as_ref().unchecked_ref(),
                    5000,
                ) {
                    self.notifications.update(|n| {
                        if let Some(notif) = n.iter_mut().find(|notif| notif.id == id) {
                            notif.timeout_handle = Some(handle);
                        }
                    });
                }
                cb.forget();
            }
        }
        // Non-wasm: no-op
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = id;
        }
    }

    /// Dismiss a notification by ID (with exit animation)
    pub fn dismiss(&self, id: usize) {
        // Set exiting state (triggers CSS animation)
        self.notifications.update(|n| {
            if let Some(notif) = n.iter_mut().find(|notif| notif.id == id) {
                notif.exiting = true;
            }
        });

        // Actually remove after animation completes
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::closure::Closure;
            use wasm_bindgen::JsCast;

            let notifications = self.notifications;
            let cb = Closure::once(Box::new(move || {
                notifications.update(|n| n.retain(|notif| notif.id != id));
            }) as Box<dyn FnOnce()>);

            if let Some(window) = web_sys::window() {
                let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                    cb.as_ref().unchecked_ref(),
                    400, // Match CSS animation duration
                );
                cb.forget();
            }
        }

        // Non-wasm fallback (immediate removal)
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.notifications
                .update(|n| n.retain(|notif| notif.id != id));
        }
    }
}

impl Default for NotificationState {
    fn default() -> Self {
        Self::new()
    }
}

/// Maximum number of visible notifications at once
const MAX_VISIBLE_NOTIFICATIONS: usize = 3;

/// Notification container component - renders all active notifications
#[component]
pub fn NotificationContainer() -> impl IntoView {
    let notification_state = expect_context::<NotificationState>();

    view! {
        <div class="notification-container">
            <For
                each=move || {
                    let all = notification_state.notifications.get();
                    // Show exiting notifications (so they can animate out)
                    // plus the first MAX_VISIBLE non-exiting ones
                    let mut visible = Vec::new();
                    let mut non_exiting_count = 0;

                    for notif in all {
                        if notif.exiting {
                            // Always show exiting (animating out)
                            visible.push(notif);
                        } else if non_exiting_count < MAX_VISIBLE_NOTIFICATIONS {
                            visible.push(notif);
                            non_exiting_count += 1;
                        }
                        // Beyond MAX_VISIBLE non-exiting: queued, not rendered
                    }
                    visible
                }
                key=|n| n.id
                children=move |notification| {
                    let id = notification.id;
                    let title = notification.title.clone();
                    let message = notification.message.clone();
                    let icon = notification.icon.clone();

                    view! {
                        <NotificationItem
                            id=id
                            title=title
                            message=message
                            icon=icon
                        />
                    }
                }
            />
        </div>
    }
}

/// Individual notification component
#[component]
fn NotificationItem(
    id: usize,
    title: String,
    message: String,
    icon: Option<String>,
) -> impl IntoView {
    let notification_state = expect_context::<NotificationState>();

    // Click anywhere to dismiss
    let on_click = move |_| {
        notification_state.dismiss(id);
    };

    // Hover to pause auto-dismiss
    let on_hover_start = move |_| {
        notification_state.pause_auto_dismiss(id);
    };

    let on_hover_end = move |_| {
        notification_state.resume_auto_dismiss(id);
    };

    // Reactively look up exiting state so class updates when dismiss is called
    let class_name = move || {
        let is_exiting = notification_state
            .notifications
            .get()
            .iter()
            .find(|n| n.id == id)
            .map(|n| n.exiting)
            .unwrap_or(false);
        if is_exiting {
            "notification exiting"
        } else {
            "notification"
        }
    };

    let icon_display = icon.unwrap_or_else(|| "⚙️".to_string());

    view! {
        <div
            class=class_name
            on:click=on_click
            on:mouseenter=on_hover_start
            on:mouseleave=on_hover_end
        >
            <div class="notification-content">
                <div class="notification-icon">
                    <div class="notification-app-icon">{icon_display}</div>
                </div>
                <div class="notification-text">
                    <div class="notification-title">{title}</div>
                    <div class="notification-message">{message}</div>
                </div>
            </div>
        </div>
    }
}
