use leptos::prelude::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::closure::Closure;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

use crate::system_state::{ModalType, PowerState, SystemState};

/// Menu bar height in pixels (matches CSS --menubar-height)
#[cfg(target_arch = "wasm32")]
const MENU_BAR_HEIGHT: f64 = 25.0;

/// Dialog dimensions for centering calculation
#[cfg(target_arch = "wasm32")]
const DIALOG_WIDTH: f64 = 320.0;

/// Modal overlay component - renders active modal dialogs
#[component]
pub fn ModalOverlay() -> impl IntoView {
    let system_state = expect_context::<SystemState>();

    // Check if showing AboutVirtualMac (needs special handling - no click-outside-to-close)
    let is_about_virtualmac =
        move || system_state.active_modal.get() == Some(ModalType::AboutVirtualMac);
    let is_other_modal = move || {
        let modal = system_state.active_modal.get();
        modal.is_some() && modal != Some(ModalType::AboutVirtualMac)
    };

    view! {
        // About VirtualMac dialog - rendered separately without click-to-close overlay
        <Show when=is_about_virtualmac>
            <AboutVirtualMacDialog />
        </Show>

        // Other modals - rendered with click-to-close overlay
        <Show when=is_other_modal>
            <div class="modal-overlay" on:click=move |_| system_state.close_modal()>
                <div class="modal-container" on:click=|e| e.stop_propagation()>
                    {move || match system_state.active_modal.get() {
                        Some(ModalType::AboutThisMac) => view! { <AboutThisMacModal /> }.into_any(),
                        Some(ModalType::ShutDownConfirm) => view! { <ShutDownModal /> }.into_any(),
                        Some(ModalType::RestartConfirm) => view! { <RestartModal /> }.into_any(),
                        Some(ModalType::LogOutConfirm) => view! { <LogOutModal /> }.into_any(),
                        Some(ModalType::ForceQuit) => view! { <ForceQuitModal /> }.into_any(),
                        Some(ModalType::ResetDesktopConfirm) => view! { <ResetDesktopModal /> }.into_any(),
                        _ => ().into_any(),
                    }}
                </div>
            </div>
        </Show>
    }
}

/// About VirtualMac dialog - draggable window with credits
#[component]
fn AboutVirtualMacDialog() -> impl IntoView {
    let system_state = expect_context::<SystemState>();

    // Position state - will be set on mount
    let (x, set_x) = signal(0.0_f64);
    let (y, set_y) = signal(0.0_f64);

    // Drag state
    #[cfg(target_arch = "wasm32")]
    let (dragging, set_dragging) = signal(false);
    #[cfg(target_arch = "wasm32")]
    let (drag_start_x, set_drag_start_x) = signal(0.0_f64);
    #[cfg(target_arch = "wasm32")]
    let (drag_start_y, set_drag_start_y) = signal(0.0_f64);
    #[cfg(target_arch = "wasm32")]
    let (dialog_start_x, set_dialog_start_x) = signal(0.0_f64);
    #[cfg(target_arch = "wasm32")]
    let (dialog_start_y, set_dialog_start_y) = signal(0.0_f64);

    // Center dialog on mount (WASM only)
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            let viewport_width = window
                .inner_width()
                .ok()
                .and_then(|v| v.as_f64())
                .unwrap_or(1920.0);
            let viewport_height = window
                .inner_height()
                .ok()
                .and_then(|v| v.as_f64())
                .unwrap_or(1080.0);

            // Center horizontally, position at 1/3 from top (like macOS About dialogs)
            let initial_x = (viewport_width - DIALOG_WIDTH) / 2.0;
            let initial_y = viewport_height / 3.0;
            set_x.set(initial_x);
            set_y.set(initial_y.max(MENU_BAR_HEIGHT));
        }
    }

    // For non-WASM, use static centered position
    #[cfg(not(target_arch = "wasm32"))]
    {
        set_x.set(800.0);
        set_y.set(300.0);
    }

    // Set up document-level drag listeners (WASM only)
    #[cfg(target_arch = "wasm32")]
    {
        // Mouse move handler
        let mousemove_handler = Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
            if dragging.get_untracked() {
                let dx = e.client_x() as f64 - drag_start_x.get_untracked();
                let dy = e.client_y() as f64 - drag_start_y.get_untracked();
                set_x.set(dialog_start_x.get_untracked() + dx);
                // Constrain Y so titlebar stays accessible (above menu bar)
                set_y.set((dialog_start_y.get_untracked() + dy).max(MENU_BAR_HEIGHT));
            }
        }) as Box<dyn Fn(web_sys::MouseEvent)>);

        // Mouse up handler
        let mouseup_handler = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            set_dragging.set(false);
        }) as Box<dyn Fn(web_sys::MouseEvent)>);

        // Add document-level listeners
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                let _ = document.add_event_listener_with_callback(
                    "mousemove",
                    mousemove_handler.as_ref().unchecked_ref(),
                );
                let _ = document.add_event_listener_with_callback(
                    "mouseup",
                    mouseup_handler.as_ref().unchecked_ref(),
                );
            }
        }

        // Keep closures alive
        mousemove_handler.forget();
        mouseup_handler.forget();
    }

    // Start drag handler (WASM only - does nothing in SSR)
    #[cfg(target_arch = "wasm32")]
    let start_drag = move |e: leptos::ev::MouseEvent| {
        e.prevent_default();
        set_dragging.set(true);
        set_drag_start_x.set(e.client_x() as f64);
        set_drag_start_y.set(e.client_y() as f64);
        set_dialog_start_x.set(x.get_untracked());
        set_dialog_start_y.set(y.get_untracked());
    };
    #[cfg(not(target_arch = "wasm32"))]
    let start_drag = move |_: leptos::ev::MouseEvent| {};

    // Close handler (stop propagation to prevent titlebar drag)
    let close_handler = move |e: leptos::ev::MouseEvent| {
        e.stop_propagation();
        system_state.close_modal();
    };

    // Style for positioning
    let style = move || format!("left: {}px; top: {}px;", x.get(), y.get());

    view! {
        <div class="about-virtualmac-overlay">
            <div class="about-virtualmac-dialog" style=style>
                <div class="about-virtualmac-titlebar" on:mousedown=start_drag>
                    <button
                        class="about-close-btn"
                        on:mousedown=|e: leptos::ev::MouseEvent| e.stop_propagation()
                        on:click=close_handler
                    ></button>
                </div>
                <div class="about-virtualmac-content">
                    // Icon
                    <div class="about-virtualmac-icon">"🖥️"</div>
                    // Title
                    <div class="about-virtualmac-title">"VirtualMac"</div>
                    // Version
                    <div class="about-virtualmac-version">"Version 2.0 (Build 2026.01.20)"</div>
                    // Tagline
                    <div class="about-virtualmac-tagline">"A macOS experience in the browser"</div>
                    // Primary links
                    <div class="about-virtualmac-links">
                        <a href="https://github.com/pRizz/virtual-mac" target="_blank" rel="noopener">"GitHub"</a>
                        <a href="https://prizz.github.io/virtual-mac/" target="_blank" rel="noopener">"Live Demo"</a>
                    </div>
                    // Credits section
                    <div class="about-virtualmac-credits">
                        <div class="about-credits-heading">"Built with"</div>
                        <a href="https://claude.ai/download" target="_blank" rel="noopener">"Claude Code"</a>" · "
                        <a href="https://github.com/anthropics/claude-code/tree/main/.claude/docs/gsd" target="_blank" rel="noopener">"GSD"</a>" · "
                        <a href="https://cursor.com" target="_blank" rel="noopener">"Cursor"</a>" · "
                        <a href="https://leptos.dev" target="_blank" rel="noopener">"Rust + Leptos"</a>
                        <div class="about-vibe-coded">"vibe coded"</div>
                    </div>
                    // Creator section
                    <div class="about-virtualmac-creator">
                        "by Peter Ryszkiewicz "
                        <a href="https://github.com/pRizz" target="_blank" rel="noopener">"GitHub"</a>" · "
                        <a href="https://www.linkedin.com/in/peter-ryszkiewicz/" target="_blank" rel="noopener">"LinkedIn"</a>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// About This Mac modal - shows system information
#[component]
fn AboutThisMacModal() -> impl IntoView {
    let system_state = expect_context::<SystemState>();

    view! {
        <div class="modal about-modal">
            <div class="about-header">
                <div class="about-logo">
                    <div class="macos-logo"></div>
                </div>
                <div class="about-title">"macOS"</div>
                <div class="about-version">"VirtualMac"</div>
            </div>
            <div class="about-content">
                <div class="about-row">
                    <span class="about-label">"Chip"</span>
                    <span class="about-value">"Apple M2 Pro"</span>
                </div>
                <div class="about-row">
                    <span class="about-label">"Memory"</span>
                    <span class="about-value">"16 GB"</span>
                </div>
                <div class="about-row">
                    <span class="about-label">"Startup Disk"</span>
                    <span class="about-value">"Macintosh HD"</span>
                </div>
                <div class="about-row">
                    <span class="about-label">"Serial Number"</span>
                    <span class="about-value">"VMAC2024PRO"</span>
                </div>
                <div class="about-row">
                    <span class="about-label">"macOS Sonoma"</span>
                    <span class="about-value">"Version 14.0"</span>
                </div>
            </div>
            <div class="about-footer">
                <button class="modal-button" on:click=move |_| system_state.close_modal()>
                    "OK"
                </button>
            </div>
        </div>
    }
}

/// Shut Down confirmation modal
#[component]
fn ShutDownModal() -> impl IntoView {
    let system_state = expect_context::<SystemState>();

    let on_shut_down = move |_| {
        system_state.close_modal();
        system_state.shut_down();
    };

    view! {
        <div class="modal confirm-modal">
            <div class="confirm-icon shutdown-icon"></div>
            <div class="confirm-title">"Are you sure you want to shut down your computer now?"</div>
            <div class="confirm-message">"If you do nothing, the system will shut down automatically."</div>
            <div class="confirm-buttons">
                <button class="modal-button secondary" on:click=move |_| system_state.close_modal()>
                    "Cancel"
                </button>
                <button class="modal-button primary" on:click=on_shut_down>
                    "Shut Down"
                </button>
            </div>
        </div>
    }
}

/// Restart confirmation modal
#[component]
fn RestartModal() -> impl IntoView {
    let system_state = expect_context::<SystemState>();

    let on_restart = move |_| {
        system_state.close_modal();
        system_state.restart();
    };

    view! {
        <div class="modal confirm-modal">
            <div class="confirm-icon restart-icon"></div>
            <div class="confirm-title">"Are you sure you want to restart your computer now?"</div>
            <div class="confirm-message">"If you do nothing, the system will restart automatically."</div>
            <div class="confirm-buttons">
                <button class="modal-button secondary" on:click=move |_| system_state.close_modal()>
                    "Cancel"
                </button>
                <button class="modal-button primary" on:click=on_restart>
                    "Restart"
                </button>
            </div>
        </div>
    }
}

/// Log Out confirmation modal
#[component]
fn LogOutModal() -> impl IntoView {
    let system_state = expect_context::<SystemState>();

    let on_log_out = move |_| {
        system_state.close_modal();
        system_state.lock_screen();
    };

    view! {
        <div class="modal confirm-modal">
            <div class="confirm-icon logout-icon"></div>
            <div class="confirm-title">"Are you sure you want to quit all applications and log out now?"</div>
            <div class="confirm-message">"If you do nothing, the system will log out automatically."</div>
            <div class="confirm-buttons">
                <button class="modal-button secondary" on:click=move |_| system_state.close_modal()>
                    "Cancel"
                </button>
                <button class="modal-button primary" on:click=on_log_out>
                    "Log Out"
                </button>
            </div>
        </div>
    }
}

/// Force Quit Applications modal
#[component]
fn ForceQuitModal() -> impl IntoView {
    let system_state = expect_context::<SystemState>();

    view! {
        <div class="modal force-quit-modal">
            <div class="force-quit-header">"Force Quit Applications"</div>
            <div class="force-quit-message">"If an app doesn't respond for a while, select its name and click Force Quit."</div>
            <div class="force-quit-list">
                <div class="force-quit-item">
                    <span class="app-icon finder-icon"></span>
                    <span class="app-name">"Finder"</span>
                </div>
                <div class="force-quit-item">
                    <span class="app-icon calculator-icon"></span>
                    <span class="app-name">"Calculator"</span>
                </div>
                <div class="force-quit-item">
                    <span class="app-icon notes-icon"></span>
                    <span class="app-name">"Notes"</span>
                </div>
            </div>
            <div class="force-quit-footer">
                <button class="modal-button secondary" on:click=move |_| system_state.close_modal()>
                    "Cancel"
                </button>
                <button class="modal-button primary disabled">"Force Quit"</button>
            </div>
        </div>
    }
}

/// Reset Desktop confirmation modal
#[component]
fn ResetDesktopModal() -> impl IntoView {
    let system_state = expect_context::<SystemState>();

    let on_reset = move |_| {
        system_state.close_modal();
        system_state.reset_desktop.set(true);
    };

    view! {
        <div class="modal confirm-modal">
            <div class="confirm-icon reset-icon"></div>
            <div class="confirm-title">"Reset Desktop to Default?"</div>
            <div class="confirm-message">"This will close all windows and restore the desktop to its default layout. Your files and settings will not be affected."</div>
            <div class="confirm-buttons">
                <button class="modal-button secondary" on:click=move |_| system_state.close_modal()>
                    "Cancel"
                </button>
                <button class="modal-button primary" on:click=on_reset>
                    "Reset Desktop"
                </button>
            </div>
        </div>
    }
}

/// Lock Screen overlay
#[component]
pub fn LockScreen() -> impl IntoView {
    let system_state = expect_context::<SystemState>();

    let on_unlock = move |_| {
        system_state.unlock_screen();
    };

    view! {
        <Show when=move || system_state.is_locked.get()>
            <div class="lock-screen" on:click=on_unlock>
                <div class="lock-screen-content">
                    <div class="lock-avatar"></div>
                    <div class="lock-username">"User"</div>
                    <div class="lock-hint">"Click anywhere to unlock"</div>
                </div>
            </div>
        </Show>
    }
}

/// Sleep/Power overlay for shutdown and restart animations
#[component]
pub fn PowerOverlay() -> impl IntoView {
    let system_state = expect_context::<SystemState>();

    let on_wake = move |_| {
        let state = system_state.power_state.get();
        match state {
            PowerState::Sleeping => system_state.wake(),
            PowerState::ShuttingDown | PowerState::Restarting => {
                // After animation, reset to running (simulating boot)
                system_state.power_state.set(PowerState::Running);
            }
            _ => {}
        }
    };

    view! {
        <Show when=move || system_state.power_state.get() != PowerState::Running>
            <div
                class=move || {
                    let state = system_state.power_state.get();
                    match state {
                        PowerState::Sleeping => "power-overlay sleep-overlay",
                        PowerState::ShuttingDown => "power-overlay shutdown-overlay",
                        PowerState::Restarting => "power-overlay restart-overlay",
                        _ => "power-overlay",
                    }
                }
                on:click=on_wake
            >
                <div class="power-content">
                    {move || {
                        let state = system_state.power_state.get();
                        match state {
                            PowerState::Sleeping => view! {
                                <div class="power-icon sleep-icon"></div>
                                <div class="power-message">"Click to wake"</div>
                            }.into_any(),
                            PowerState::ShuttingDown => view! {
                                <div class="power-icon shutdown-icon-large"></div>
                                <div class="power-message">"Shutting down..."</div>
                            }.into_any(),
                            PowerState::Restarting => view! {
                                <div class="power-icon restart-icon-large"></div>
                                <div class="power-message">"Restarting..."</div>
                            }.into_any(),
                            _ => ().into_any(),
                        }
                    }}
                </div>
            </div>
        </Show>
    }
}
