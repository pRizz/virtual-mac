use leptos::prelude::*;

use crate::system_state::{ModalType, PowerState, SystemState};

/// Modal overlay component - renders active modal dialogs
#[component]
pub fn ModalOverlay() -> impl IntoView {
    let system_state = expect_context::<SystemState>();

    view! {
        <Show when=move || system_state.active_modal.get().is_some()>
            <div class="modal-overlay" on:click=move |_| system_state.close_modal()>
                <div class="modal-container" on:click=|e| e.stop_propagation()>
                    {move || match system_state.active_modal.get() {
                        Some(ModalType::AboutThisMac) => view! { <AboutThisMacModal /> }.into_any(),
                        Some(ModalType::ShutDownConfirm) => view! { <ShutDownModal /> }.into_any(),
                        Some(ModalType::RestartConfirm) => view! { <RestartModal /> }.into_any(),
                        Some(ModalType::LogOutConfirm) => view! { <LogOutModal /> }.into_any(),
                        Some(ModalType::ForceQuit) => view! { <ForceQuitModal /> }.into_any(),
                        Some(ModalType::ResetDesktopConfirm) => view! { <ResetDesktopModal /> }.into_any(),
                        None => view! {}.into_any(),
                    }}
                </div>
            </div>
        </Show>
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
                            _ => view! {}.into_any(),
                        }
                    }}
                </div>
            </div>
        </Show>
    }
}
