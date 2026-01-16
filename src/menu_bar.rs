use leptos::prelude::*;

use crate::system_state::{ModalType, SystemState};

#[component]
pub fn MenuBar() -> impl IntoView {
    let system_state = expect_context::<SystemState>();
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

    // Apple menu action handlers
    let on_about = Callback::new(move |_| {
        set_active_menu.set(None);
        system_state.show_modal(ModalType::AboutThisMac);
    });
    let on_system_settings = Callback::new(move |_| {
        set_active_menu.set(None);
        system_state.open_system_settings.set(true);
    });
    let on_force_quit = Callback::new(move |_| {
        set_active_menu.set(None);
        system_state.show_modal(ModalType::ForceQuit);
    });
    let on_sleep = Callback::new(move |_| {
        set_active_menu.set(None);
        system_state.sleep();
    });
    let on_restart = Callback::new(move |_| {
        set_active_menu.set(None);
        system_state.show_modal(ModalType::RestartConfirm);
    });
    let on_shut_down = Callback::new(move |_| {
        set_active_menu.set(None);
        system_state.show_modal(ModalType::ShutDownConfirm);
    });
    let on_lock_screen = Callback::new(move |_| {
        set_active_menu.set(None);
        system_state.lock_screen();
    });
    let on_log_out = Callback::new(move |_| {
        set_active_menu.set(None);
        system_state.show_modal(ModalType::LogOutConfirm);
    });

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
                    <DropdownItem label="About This Mac" on_click=on_about />
                    <DropdownSeparator />
                    <DropdownItem label="System Settings..." on_click=on_system_settings />
                    <DropdownItem label="App Store..." />
                    <DropdownSeparator />
                    <DropdownItem label="Recent Items" />
                    <DropdownSeparator />
                    <DropdownItem label="Force Quit..." shortcut="⌥⌘⎋" on_click=on_force_quit />
                    <DropdownSeparator />
                    <DropdownItem label="Sleep" on_click=on_sleep />
                    <DropdownItem label="Restart..." on_click=on_restart />
                    <DropdownItem label="Shut Down..." on_click=on_shut_down />
                    <DropdownSeparator />
                    <DropdownItem label="Lock Screen" shortcut="⌃⌘Q" on_click=on_lock_screen />
                    <DropdownItem label="Log Out..." shortcut="⇧⌘Q" on_click=on_log_out />
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
    #[prop(optional, into)] on_click: Option<Callback<leptos::ev::MouseEvent>>,
) -> impl IntoView {
    let class = if disabled {
        "dropdown-item disabled"
    } else {
        "dropdown-item"
    };

    let handler = move |e: leptos::ev::MouseEvent| {
        if let Some(callback) = on_click {
            callback.run(e);
        }
    };

    view! {
        <div class=class on:click=handler>
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
    view! {
        <div class="status-icon">
            <WifiIcon />
        </div>
        <div class="status-icon">
            <BatteryIcon />
        </div>
        <div class="status-icon control-center-icon">
            <span></span>
            <span></span>
        </div>
        <div class="status-icon spotlight-icon"></div>
        <div class="status-icon siri-icon"></div>
        <div class="status-icon status-clock">
            {move || current_time.get()}
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
