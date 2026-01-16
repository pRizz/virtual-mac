use leptos::prelude::*;
use crate::wallpaper::{use_wallpaper_context, WALLPAPERS};

/// System Settings app content
#[component]
pub fn SystemSettings() -> impl IntoView {
    let (selected_pane, set_selected_pane) = signal("general");

    view! {
        <div class="system-settings">
            <div class="settings-sidebar">
                <div class="settings-search">
                    <input type="text" placeholder="Search" />
                </div>
                <div class="settings-nav">
                    <SettingsNavItem
                        id="general"
                        icon="gear"
                        label="General"
                        selected=selected_pane
                        set_selected=set_selected_pane
                    />
                    <SettingsNavItem
                        id="appearance"
                        icon="paintbrush"
                        label="Appearance"
                        selected=selected_pane
                        set_selected=set_selected_pane
                    />
                    <SettingsNavItem
                        id="desktop"
                        icon="display"
                        label="Desktop & Dock"
                        selected=selected_pane
                        set_selected=set_selected_pane
                    />
                    <SettingsNavItem
                        id="displays"
                        icon="display"
                        label="Displays"
                        selected=selected_pane
                        set_selected=set_selected_pane
                    />
                    <SettingsNavItem
                        id="wallpaper"
                        icon="photo"
                        label="Wallpaper"
                        selected=selected_pane
                        set_selected=set_selected_pane
                    />
                    <div class="settings-separator"></div>
                    <SettingsNavItem
                        id="wifi"
                        icon="wifi"
                        label="Wi-Fi"
                        selected=selected_pane
                        set_selected=set_selected_pane
                    />
                    <SettingsNavItem
                        id="bluetooth"
                        icon="bluetooth"
                        label="Bluetooth"
                        selected=selected_pane
                        set_selected=set_selected_pane
                    />
                    <SettingsNavItem
                        id="network"
                        icon="network"
                        label="Network"
                        selected=selected_pane
                        set_selected=set_selected_pane
                    />
                    <div class="settings-separator"></div>
                    <SettingsNavItem
                        id="notifications"
                        icon="bell"
                        label="Notifications"
                        selected=selected_pane
                        set_selected=set_selected_pane
                    />
                    <SettingsNavItem
                        id="sound"
                        icon="speaker"
                        label="Sound"
                        selected=selected_pane
                        set_selected=set_selected_pane
                    />
                </div>
            </div>
            <div class="settings-content">
                {move || match selected_pane.get() {
                    "general" => view! { <GeneralPane /> }.into_any(),
                    "appearance" => view! { <AppearancePane /> }.into_any(),
                    "desktop" => view! { <DesktopDockPane /> }.into_any(),
                    "wallpaper" => view! { <WallpaperPane /> }.into_any(),
                    _ => view! { <PlaceholderPane name=selected_pane.get() /> }.into_any(),
                }}
            </div>
        </div>
    }
}

#[component]
fn SettingsNavItem(
    id: &'static str,
    icon: &'static str,
    label: &'static str,
    selected: ReadSignal<&'static str>,
    set_selected: WriteSignal<&'static str>,
) -> impl IntoView {
    let is_selected = move || selected.get() == id;

    view! {
        <div
            class=move || if is_selected() { "settings-nav-item selected" } else { "settings-nav-item" }
            on:click=move |_| set_selected.set(id)
        >
            <span class=format!("settings-icon settings-icon-{}", icon)></span>
            <span class="settings-label">{label}</span>
        </div>
    }
}

#[component]
fn GeneralPane() -> impl IntoView {
    view! {
        <div class="settings-pane">
            <h1 class="settings-pane-title">"General"</h1>
            <div class="settings-group">
                <div class="settings-row">
                    <span class="settings-row-label">"Appearance"</span>
                    <div class="settings-row-control">
                        <select>
                            <option>"Auto"</option>
                            <option>"Light"</option>
                            <option selected>"Dark"</option>
                        </select>
                    </div>
                </div>
                <div class="settings-row">
                    <span class="settings-row-label">"Accent color"</span>
                    <div class="settings-row-control color-picker">
                        <span class="color-dot blue selected"></span>
                        <span class="color-dot purple"></span>
                        <span class="color-dot pink"></span>
                        <span class="color-dot red"></span>
                        <span class="color-dot orange"></span>
                        <span class="color-dot yellow"></span>
                        <span class="color-dot green"></span>
                        <span class="color-dot graphite"></span>
                    </div>
                </div>
                <div class="settings-row">
                    <span class="settings-row-label">"Sidebar icon size"</span>
                    <div class="settings-row-control">
                        <select>
                            <option>"Small"</option>
                            <option selected>"Medium"</option>
                            <option>"Large"</option>
                        </select>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn AppearancePane() -> impl IntoView {
    view! {
        <div class="settings-pane">
            <h1 class="settings-pane-title">"Appearance"</h1>
            <div class="settings-group">
                <div class="appearance-options">
                    <div class="appearance-option selected">
                        <div class="appearance-preview light"></div>
                        <span>"Light"</span>
                    </div>
                    <div class="appearance-option">
                        <div class="appearance-preview dark"></div>
                        <span>"Dark"</span>
                    </div>
                    <div class="appearance-option">
                        <div class="appearance-preview auto"></div>
                        <span>"Auto"</span>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn DesktopDockPane() -> impl IntoView {
    view! {
        <div class="settings-pane">
            <h1 class="settings-pane-title">"Desktop & Dock"</h1>
            <div class="settings-group">
                <h2 class="settings-group-title">"Dock"</h2>
                <div class="settings-row">
                    <span class="settings-row-label">"Size"</span>
                    <div class="settings-row-control">
                        <input type="range" min="0" max="100" value="50" />
                    </div>
                </div>
                <div class="settings-row">
                    <span class="settings-row-label">"Magnification"</span>
                    <div class="settings-row-control">
                        <label class="toggle">
                            <input type="checkbox" checked />
                            <span class="toggle-slider"></span>
                        </label>
                    </div>
                </div>
                <div class="settings-row">
                    <span class="settings-row-label">"Position on screen"</span>
                    <div class="settings-row-control">
                        <select>
                            <option>"Left"</option>
                            <option selected>"Bottom"</option>
                            <option>"Right"</option>
                        </select>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn WallpaperPane() -> impl IntoView {
    let wallpaper_ctx = use_wallpaper_context();

    view! {
        <div class="settings-pane">
            <h1 class="settings-pane-title">"Wallpaper"</h1>
            <p class="settings-pane-description">"Choose a wallpaper for your desktop"</p>
            <div class="wallpaper-grid">
                {WALLPAPERS.iter().map(|wallpaper| {
                    let id = wallpaper.id;
                    let gradient = wallpaper.gradient;
                    let name = wallpaper.name;
                    let is_selected = move || wallpaper_ctx.current.get() == id;
                    view! {
                        <div
                            class=move || if is_selected() { "wallpaper-item selected" } else { "wallpaper-item" }
                            on:click=move |_| wallpaper_ctx.set_current.set(id)
                        >
                            <div class="wallpaper-thumbnail" style=format!("background: {}", gradient) />
                            <span class="wallpaper-name">{name}</span>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

#[component]
fn PlaceholderPane(name: &'static str) -> impl IntoView {
    let title = match name {
        "wifi" => "Wi-Fi",
        "bluetooth" => "Bluetooth",
        "network" => "Network",
        "notifications" => "Notifications",
        "sound" => "Sound",
        "displays" => "Displays",
        _ => name,
    };

    view! {
        <div class="settings-pane">
            <h1 class="settings-pane-title">{title}</h1>
            <div class="settings-placeholder">
                <p>"Settings for " {title} " would appear here."</p>
            </div>
        </div>
    }
}
