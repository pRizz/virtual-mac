use leptos::prelude::*;

/// A single notification
#[derive(Clone, Debug)]
pub struct Notification {
    pub id: usize,
    pub title: String,
    pub message: String,
    pub icon: Option<String>,
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
        };

        self.notifications.update(|n| n.push(notification));

        // Auto-dismiss after 5 seconds
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
                    5000,
                );
                cb.forget();
            }
        }
    }

    /// Dismiss a notification by ID
    #[allow(dead_code)]
    pub fn dismiss(&self, id: usize) {
        self.notifications.update(|n| n.retain(|notif| notif.id != id));
    }
}

impl Default for NotificationState {
    fn default() -> Self {
        Self::new()
    }
}

/// Notification container component - renders all active notifications
#[component]
pub fn NotificationContainer() -> impl IntoView {
    let notification_state = expect_context::<NotificationState>();

    view! {
        <div class="notification-container">
            <For
                each=move || notification_state.notifications.get()
                key=|n| n.id
                children=move |notification| {
                    let id = notification.id;
                    let title = notification.title.clone();
                    let message = notification.message.clone();

                    view! {
                        <NotificationItem
                            id=id
                            title=title
                            message=message
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
) -> impl IntoView {
    let notification_state = expect_context::<NotificationState>();

    let on_dismiss = move |_| {
        notification_state.dismiss(id);
    };

    view! {
        <div class="notification">
            <div class="notification-content">
                <div class="notification-icon">
                    <div class="notification-app-icon">"⚙️"</div>
                </div>
                <div class="notification-text">
                    <div class="notification-title">{title}</div>
                    <div class="notification-message">{message}</div>
                </div>
            </div>
            <button class="notification-dismiss" on:click=on_dismiss>
                "×"
            </button>
        </div>
    }
}
