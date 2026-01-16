use leptos::prelude::*;

#[component]
pub fn MenuBar() -> impl IntoView {
    let (active_menu, set_active_menu) = signal::<Option<&'static str>>(None);
    let (current_time, set_current_time) = signal(get_current_time());
    let _ = &set_current_time; // Used in wasm32 block below

    // Update clock every second (only in WASM)
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::closure::Closure;
        use wasm_bindgen::JsCast;

        let cb = Closure::wrap(Box::new(move || {
            set_current_time.set(get_current_time());
        }) as Box<dyn Fn()>);

        let window = web_sys::window().unwrap();
        window
            .set_interval_with_callback_and_timeout_and_arguments_0(
                cb.as_ref().unchecked_ref(),
                1000,
            )
            .unwrap();
        cb.forget();
    }

    let close_menu = move |_| {
        set_active_menu.set(None);
    };

    view! {
        <div class="menu-bar" on:mouseleave=close_menu>
            <div class="menu-bar-left">
                <MenuItem
                    id="apple"
                    label=""
                    class_name="apple-menu"
                    active_menu=active_menu
                    set_active_menu=set_active_menu
                >
                    <DropdownItem label="About This Mac" />
                    <DropdownSeparator />
                    <DropdownItem label="System Settings..." />
                    <DropdownItem label="App Store..." />
                    <DropdownSeparator />
                    <DropdownItem label="Recent Items" />
                    <DropdownSeparator />
                    <DropdownItem label="Force Quit..." shortcut="⌥⌘⎋" />
                    <DropdownSeparator />
                    <DropdownItem label="Sleep" />
                    <DropdownItem label="Restart..." />
                    <DropdownItem label="Shut Down..." />
                    <DropdownSeparator />
                    <DropdownItem label="Lock Screen" shortcut="⌃⌘Q" />
                    <DropdownItem label="Log Out..." shortcut="⇧⌘Q" />
                </MenuItem>

                <MenuItem
                    id="app"
                    label="VirtualMac"
                    class_name="app-name"
                    active_menu=active_menu
                    set_active_menu=set_active_menu
                >
                    <DropdownItem label="About VirtualMac" />
                    <DropdownSeparator />
                    <DropdownItem label="Settings..." shortcut="⌘," />
                    <DropdownSeparator />
                    <DropdownItem label="Hide VirtualMac" shortcut="⌘H" />
                    <DropdownItem label="Hide Others" shortcut="⌥⌘H" />
                    <DropdownItem label="Show All" />
                    <DropdownSeparator />
                    <DropdownItem label="Quit VirtualMac" shortcut="⌘Q" />
                </MenuItem>

                <MenuItem
                    id="file"
                    label="File"
                    active_menu=active_menu
                    set_active_menu=set_active_menu
                >
                    <DropdownItem label="New Window" shortcut="⌘N" />
                    <DropdownItem label="New Tab" shortcut="⌘T" />
                    <DropdownSeparator />
                    <DropdownItem label="Open..." shortcut="⌘O" />
                    <DropdownItem label="Open Recent" />
                    <DropdownSeparator />
                    <DropdownItem label="Close Window" shortcut="⌘W" />
                    <DropdownItem label="Close All" shortcut="⌥⌘W" />
                    <DropdownSeparator />
                    <DropdownItem label="Save" shortcut="⌘S" disabled=true />
                    <DropdownItem label="Save As..." shortcut="⇧⌘S" disabled=true />
                    <DropdownSeparator />
                    <DropdownItem label="Print..." shortcut="⌘P" />
                </MenuItem>

                <MenuItem
                    id="edit"
                    label="Edit"
                    active_menu=active_menu
                    set_active_menu=set_active_menu
                >
                    <DropdownItem label="Undo" shortcut="⌘Z" disabled=true />
                    <DropdownItem label="Redo" shortcut="⇧⌘Z" disabled=true />
                    <DropdownSeparator />
                    <DropdownItem label="Cut" shortcut="⌘X" disabled=true />
                    <DropdownItem label="Copy" shortcut="⌘C" disabled=true />
                    <DropdownItem label="Paste" shortcut="⌘V" disabled=true />
                    <DropdownItem label="Paste and Match Style" shortcut="⌥⇧⌘V" disabled=true />
                    <DropdownItem label="Delete" disabled=true />
                    <DropdownItem label="Select All" shortcut="⌘A" />
                    <DropdownSeparator />
                    <DropdownItem label="Find" />
                    <DropdownItem label="Spelling and Grammar" />
                </MenuItem>

                <MenuItem
                    id="view"
                    label="View"
                    active_menu=active_menu
                    set_active_menu=set_active_menu
                >
                    <DropdownItem label="Show Tab Bar" />
                    <DropdownItem label="Show All Tabs" />
                    <DropdownSeparator />
                    <DropdownItem label="Show Sidebar" shortcut="⌘S" />
                    <DropdownItem label="Enter Full Screen" shortcut="⌃⌘F" />
                </MenuItem>

                <MenuItem
                    id="window"
                    label="Window"
                    active_menu=active_menu
                    set_active_menu=set_active_menu
                >
                    <DropdownItem label="Minimize" shortcut="⌘M" />
                    <DropdownItem label="Zoom" />
                    <DropdownItem label="Tile Window to Left of Screen" />
                    <DropdownItem label="Tile Window to Right of Screen" />
                    <DropdownSeparator />
                    <DropdownItem label="Move Window to Left Side of Screen" />
                    <DropdownItem label="Move Window to Right Side of Screen" />
                    <DropdownSeparator />
                    <DropdownItem label="Bring All to Front" />
                </MenuItem>

                <MenuItem
                    id="help"
                    label="Help"
                    active_menu=active_menu
                    set_active_menu=set_active_menu
                >
                    <DropdownItem label="VirtualMac Help" />
                    <DropdownSeparator />
                    <DropdownItem label="Search" />
                </MenuItem>
            </div>

            <div class="menu-bar-right">
                <StatusIcons current_time=current_time />
            </div>
        </div>
    }
}

#[component]
fn MenuItem(
    id: &'static str,
    label: &'static str,
    #[prop(optional)] class_name: &'static str,
    active_menu: ReadSignal<Option<&'static str>>,
    set_active_menu: WriteSignal<Option<&'static str>>,
    children: Children,
) -> impl IntoView {
    let is_active = move || active_menu.get() == Some(id);

    let on_click = move |_| {
        if is_active() {
            set_active_menu.set(None);
        } else {
            set_active_menu.set(Some(id));
        }
    };

    let on_mouse_enter = move |_| {
        if active_menu.get().is_some() {
            set_active_menu.set(Some(id));
        }
    };

    let class = move || {
        let mut classes = vec!["menu-item"];
        if !class_name.is_empty() {
            classes.push(class_name);
        }
        if is_active() {
            classes.push("active");
        }
        classes.join(" ")
    };

    view! {
        <div class=class on:click=on_click on:mouseenter=on_mouse_enter>
            {if id == "apple" {
                view! { <span class="apple-logo">{""}</span> }.into_any()
            } else {
                view! { <span>{label}</span> }.into_any()
            }}
            <div class="menu-dropdown">
                {children()}
            </div>
        </div>
    }
}

#[component]
fn DropdownItem(
    label: &'static str,
    #[prop(optional)] shortcut: &'static str,
    #[prop(optional)] disabled: bool,
) -> impl IntoView {
    let class = if disabled {
        "dropdown-item disabled"
    } else {
        "dropdown-item"
    };

    view! {
        <div class=class>
            <span>{label}</span>
            {if !shortcut.is_empty() {
                view! { <span class="dropdown-shortcut">{shortcut}</span> }.into_any()
            } else {
                view! {}.into_any()
            }}
        </div>
    }
}

#[component]
fn DropdownSeparator() -> impl IntoView {
    view! { <div class="dropdown-separator"></div> }
}

#[component]
fn StatusIcons(current_time: ReadSignal<String>) -> impl IntoView {
    let (control_center_open, set_control_center_open) = signal(false);

    let toggle_control_center = move |_| {
        set_control_center_open.update(|v| *v = !*v);
    };

    let close_control_center = move |_| {
        set_control_center_open.set(false);
    };

    view! {
        <div class="status-icon">
            <WifiIcon />
        </div>
        <div class="status-icon">
            <BatteryIcon />
        </div>
        <div
            class="status-icon control-center-icon"
            on:click=toggle_control_center
        >
            <span></span>
            <span></span>
            <ControlCenter
                is_open=control_center_open
                on_close=close_control_center
            />
        </div>
        <div class="status-icon spotlight-icon"></div>
        <div class="status-icon siri-icon"></div>
        <div class="status-icon status-clock">
            {move || current_time.get()}
        </div>
    }
}

#[component]
fn ControlCenter(
    is_open: ReadSignal<bool>,
    _on_close: impl Fn(leptos::ev::MouseEvent) + 'static,
) -> impl IntoView {
    let (wifi_on, set_wifi_on) = signal(true);
    let (bluetooth_on, set_bluetooth_on) = signal(true);
    let (airdrop_on, set_airdrop_on) = signal(false);
    let (dnd_on, set_dnd_on) = signal(false);
    let (brightness, set_brightness) = signal(75i32);
    let (volume, set_volume) = signal(50i32);

    let toggle_wifi = move |_| set_wifi_on.update(|v| *v = !*v);
    let toggle_bluetooth = move |_| set_bluetooth_on.update(|v| *v = !*v);
    let toggle_airdrop = move |_| set_airdrop_on.update(|v| *v = !*v);
    let toggle_dnd = move |_| set_dnd_on.update(|v| *v = !*v);

    let on_brightness_change = move |e: leptos::ev::Event| {
        let value = event_target_value(&e).parse().unwrap_or(75);
        set_brightness.set(value);
    };

    let on_volume_change = move |e: leptos::ev::Event| {
        let value = event_target_value(&e).parse().unwrap_or(50);
        set_volume.set(value);
    };

    let panel_class = move || {
        if is_open.get() {
            "control-center-panel open"
        } else {
            "control-center-panel"
        }
    };

    // Prevent clicks inside panel from closing it
    let stop_propagation = move |e: leptos::ev::MouseEvent| {
        e.stop_propagation();
    };

    view! {
        <div class=panel_class on:click=stop_propagation>
            <div class="cc-section cc-toggles">
                <div
                    class=move || if wifi_on.get() { "cc-toggle active" } else { "cc-toggle" }
                    on:click=toggle_wifi
                >
                    <div class="cc-toggle-icon">"📶"</div>
                    <div class="cc-toggle-label">"Wi-Fi"</div>
                </div>
                <div
                    class=move || if bluetooth_on.get() { "cc-toggle active" } else { "cc-toggle" }
                    on:click=toggle_bluetooth
                >
                    <div class="cc-toggle-icon">"ᛒ"</div>
                    <div class="cc-toggle-label">"Bluetooth"</div>
                </div>
                <div
                    class=move || if airdrop_on.get() { "cc-toggle active" } else { "cc-toggle" }
                    on:click=toggle_airdrop
                >
                    <div class="cc-toggle-icon">"📡"</div>
                    <div class="cc-toggle-label">"AirDrop"</div>
                </div>
            </div>

            <div class="cc-section cc-focus">
                <div
                    class=move || if dnd_on.get() { "cc-focus-toggle active" } else { "cc-focus-toggle" }
                    on:click=toggle_dnd
                >
                    <div class="cc-focus-icon">"🌙"</div>
                    <div class="cc-focus-info">
                        <div class="cc-focus-label">"Do Not Disturb"</div>
                        <div class="cc-focus-status">
                            {move || if dnd_on.get() { "On" } else { "Off" }}
                        </div>
                    </div>
                </div>
            </div>

            <div class="cc-section cc-slider-section">
                <div class="cc-slider">
                    <span class="cc-slider-icon">"🔆"</span>
                    <input
                        type="range"
                        min="0"
                        max="100"
                        prop:value=move || brightness.get()
                        on:input=on_brightness_change
                        class="cc-range"
                    />
                </div>
            </div>

            <div class="cc-section cc-slider-section">
                <div class="cc-slider">
                    <span class="cc-slider-icon">"🔊"</span>
                    <input
                        type="range"
                        min="0"
                        max="100"
                        prop:value=move || volume.get()
                        on:input=on_volume_change
                        class="cc-range"
                    />
                </div>
            </div>
        </div>
    }
}

#[component]
fn WifiIcon() -> impl IntoView {
    view! {
        <div class="wifi-icon">
            <span></span>
            <span class="wifi-dot"></span>
        </div>
    }
}

#[component]
fn BatteryIcon() -> impl IntoView {
    view! {
        <div class="battery-container">
            <span class="battery-percent">"85%"</span>
            <div class="battery-icon">
                <div class="battery-level"></div>
            </div>
        </div>
    }
}

fn get_current_time() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        let date = js_sys::Date::new_0();
        let hours = date.get_hours();
        let minutes = date.get_minutes();
        let period = if hours >= 12 { "PM" } else { "AM" };
        let display_hours = if hours == 0 {
            12
        } else if hours > 12 {
            hours - 12
        } else {
            hours
        };
        format!("{} {:02} {}", get_day_abbrev(&date), display_hours, period)
            + &format!(":{:02}", minutes)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        "Wed 12:00 PM".to_string()
    }
}

#[cfg(target_arch = "wasm32")]
fn get_day_abbrev(date: &js_sys::Date) -> &'static str {
    match date.get_day() {
        0 => "Sun",
        1 => "Mon",
        2 => "Tue",
        3 => "Wed",
        4 => "Thu",
        5 => "Fri",
        6 => "Sat",
        _ => "???",
    }
}
