use leptos::prelude::*;
#[allow(unused_imports)]
use wasm_bindgen::prelude::*;
#[allow(unused_imports)]
use wasm_bindgen::JsCast;

#[derive(Clone, Copy, PartialEq)]
enum Operation {
    None,
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[component]
pub fn Calculator() -> impl IntoView {
    let (display, set_display) = signal(String::from("0"));
    let (stored_value, set_stored_value) = signal(0.0f64);
    let (current_op, set_current_op) = signal(Operation::None);
    let (clear_on_next, set_clear_on_next) = signal(false);
    let (active_operator, set_active_operator) = signal::<Option<Operation>>(None);

    let append_digit = move |digit: &str| {
        if clear_on_next.get() {
            set_display.set(digit.to_string());
            set_clear_on_next.set(false);
        } else {
            let current = display.get();
            if current == "0" && digit != "." {
                set_display.set(digit.to_string());
            } else if digit == "." && current.contains('.') {
                // Don't add another decimal point
            } else {
                set_display.set(format!("{}{}", current, digit));
            }
        }
        set_active_operator.set(None);
    };

    let clear = move || {
        set_display.set(String::from("0"));
        set_stored_value.set(0.0);
        set_current_op.set(Operation::None);
        set_clear_on_next.set(false);
        set_active_operator.set(None);
    };

    let negate = move || {
        let current = display.get();
        if let Ok(val) = current.parse::<f64>() {
            let negated = -val;
            set_display.set(format_result(negated));
        }
    };

    let percent = move || {
        let current = display.get();
        if let Ok(val) = current.parse::<f64>() {
            let result = val / 100.0;
            set_display.set(format_result(result));
        }
    };

    let do_calculate = move || {
        let current = display.get();
        if let Ok(current_val) = current.parse::<f64>() {
            let stored = stored_value.get();
            let result = match current_op.get() {
                Operation::None => current_val,
                Operation::Add => stored + current_val,
                Operation::Subtract => stored - current_val,
                Operation::Multiply => stored * current_val,
                Operation::Divide => {
                    if current_val != 0.0 {
                        stored / current_val
                    } else {
                        f64::NAN
                    }
                }
            };
            set_display.set(format_result(result));
            set_current_op.set(Operation::None);
            set_clear_on_next.set(true);
            set_active_operator.set(None);
        }
    };

    let set_operation = move |op: Operation| {
        let current = display.get();
        if let Ok(_val) = current.parse::<f64>() {
            // If there's a pending operation, calculate first
            if current_op.get() != Operation::None && !clear_on_next.get() {
                do_calculate();
            }
            set_stored_value.set(display.get().parse().unwrap_or(0.0));
            set_current_op.set(op);
            set_clear_on_next.set(true);
            set_active_operator.set(Some(op));
        }
    };

    // Keyboard event listener for calculator shortcuts
    Effect::new(move |_| {
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::closure::Closure;

            let closure = Closure::wrap(Box::new(move |evt: web_sys::KeyboardEvent| {
                match evt.key().as_str() {
                    "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "." => {
                        let digit = evt.key();
                        // Inline append_digit logic
                        if clear_on_next.get() {
                            set_display.set(digit.clone());
                            set_clear_on_next.set(false);
                        } else {
                            let current = display.get();
                            if current == "0" && digit != "." {
                                set_display.set(digit);
                            } else if digit == "." && current.contains('.') {
                                // Don't add another decimal point
                            } else {
                                set_display.set(format!("{}{}", current, digit));
                            }
                        }
                        set_active_operator.set(None);
                    }
                    "+" => {
                        let current = display.get();
                        if current.parse::<f64>().is_ok() {
                            if current_op.get() != Operation::None && !clear_on_next.get() {
                                // Trigger calculation first
                                let current_val = current.parse::<f64>().unwrap_or(0.0);
                                let stored = stored_value.get();
                                let result = match current_op.get() {
                                    Operation::None => current_val,
                                    Operation::Add => stored + current_val,
                                    Operation::Subtract => stored - current_val,
                                    Operation::Multiply => stored * current_val,
                                    Operation::Divide => if current_val != 0.0 { stored / current_val } else { f64::NAN },
                                };
                                set_display.set(format_result(result));
                            }
                            set_stored_value.set(display.get().parse().unwrap_or(0.0));
                            set_current_op.set(Operation::Add);
                            set_clear_on_next.set(true);
                            set_active_operator.set(Some(Operation::Add));
                        }
                    }
                    "-" => {
                        let current = display.get();
                        if current.parse::<f64>().is_ok() {
                            if current_op.get() != Operation::None && !clear_on_next.get() {
                                let current_val = current.parse::<f64>().unwrap_or(0.0);
                                let stored = stored_value.get();
                                let result = match current_op.get() {
                                    Operation::None => current_val,
                                    Operation::Add => stored + current_val,
                                    Operation::Subtract => stored - current_val,
                                    Operation::Multiply => stored * current_val,
                                    Operation::Divide => if current_val != 0.0 { stored / current_val } else { f64::NAN },
                                };
                                set_display.set(format_result(result));
                            }
                            set_stored_value.set(display.get().parse().unwrap_or(0.0));
                            set_current_op.set(Operation::Subtract);
                            set_clear_on_next.set(true);
                            set_active_operator.set(Some(Operation::Subtract));
                        }
                    }
                    "*" => {
                        let current = display.get();
                        if current.parse::<f64>().is_ok() {
                            if current_op.get() != Operation::None && !clear_on_next.get() {
                                let current_val = current.parse::<f64>().unwrap_or(0.0);
                                let stored = stored_value.get();
                                let result = match current_op.get() {
                                    Operation::None => current_val,
                                    Operation::Add => stored + current_val,
                                    Operation::Subtract => stored - current_val,
                                    Operation::Multiply => stored * current_val,
                                    Operation::Divide => if current_val != 0.0 { stored / current_val } else { f64::NAN },
                                };
                                set_display.set(format_result(result));
                            }
                            set_stored_value.set(display.get().parse().unwrap_or(0.0));
                            set_current_op.set(Operation::Multiply);
                            set_clear_on_next.set(true);
                            set_active_operator.set(Some(Operation::Multiply));
                        }
                    }
                    "/" => {
                        evt.prevent_default(); // Prevent browser quick-find
                        let current = display.get();
                        if current.parse::<f64>().is_ok() {
                            if current_op.get() != Operation::None && !clear_on_next.get() {
                                let current_val = current.parse::<f64>().unwrap_or(0.0);
                                let stored = stored_value.get();
                                let result = match current_op.get() {
                                    Operation::None => current_val,
                                    Operation::Add => stored + current_val,
                                    Operation::Subtract => stored - current_val,
                                    Operation::Multiply => stored * current_val,
                                    Operation::Divide => if current_val != 0.0 { stored / current_val } else { f64::NAN },
                                };
                                set_display.set(format_result(result));
                            }
                            set_stored_value.set(display.get().parse().unwrap_or(0.0));
                            set_current_op.set(Operation::Divide);
                            set_clear_on_next.set(true);
                            set_active_operator.set(Some(Operation::Divide));
                        }
                    }
                    "=" | "Enter" => {
                        // do_calculate logic inline
                        let current = display.get();
                        if let Ok(current_val) = current.parse::<f64>() {
                            let stored = stored_value.get();
                            let result = match current_op.get() {
                                Operation::None => current_val,
                                Operation::Add => stored + current_val,
                                Operation::Subtract => stored - current_val,
                                Operation::Multiply => stored * current_val,
                                Operation::Divide => {
                                    if current_val != 0.0 {
                                        stored / current_val
                                    } else {
                                        f64::NAN
                                    }
                                }
                            };
                            set_display.set(format_result(result));
                            set_current_op.set(Operation::None);
                            set_clear_on_next.set(true);
                            set_active_operator.set(None);
                        }
                    }
                    "Escape" | "c" | "C" => {
                        // clear logic inline
                        set_display.set(String::from("0"));
                        set_stored_value.set(0.0);
                        set_current_op.set(Operation::None);
                        set_clear_on_next.set(false);
                        set_active_operator.set(None);
                    }
                    "%" => {
                        // percent logic inline
                        let current = display.get();
                        if let Ok(val) = current.parse::<f64>() {
                            let result = val / 100.0;
                            set_display.set(format_result(result));
                        }
                    }
                    "Backspace" | "Delete" => {
                        let current = display.get();
                        if current.len() > 1 {
                            set_display.set(current[..current.len()-1].to_string());
                        } else {
                            set_display.set(String::from("0"));
                        }
                    }
                    _ => {}
                }
            }) as Box<dyn FnMut(_)>);

            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    let _ = document.add_event_listener_with_callback(
                        "keydown",
                        closure.as_ref().unchecked_ref(),
                    );
                }
            }
            closure.forget();
        }
    });

    view! {
        <div class="calculator">
            <div class="calc-display">
                <span class="calc-display-text">{move || display.get()}</span>
            </div>
            <div class="calc-buttons">
                <button class="calc-btn function" on:click=move |_| clear()>"AC"</button>
                <button class="calc-btn function" on:click=move |_| negate()>"+/−"</button>
                <button class="calc-btn function" on:click=move |_| percent()>"%"</button>
                <button
                    class=move || if active_operator.get() == Some(Operation::Divide) { "calc-btn operator active" } else { "calc-btn operator" }
                    on:click=move |_| set_operation(Operation::Divide)
                >"÷"</button>

                <button class="calc-btn digit" on:click=move |_| append_digit("7")>"7"</button>
                <button class="calc-btn digit" on:click=move |_| append_digit("8")>"8"</button>
                <button class="calc-btn digit" on:click=move |_| append_digit("9")>"9"</button>
                <button
                    class=move || if active_operator.get() == Some(Operation::Multiply) { "calc-btn operator active" } else { "calc-btn operator" }
                    on:click=move |_| set_operation(Operation::Multiply)
                >"×"</button>

                <button class="calc-btn digit" on:click=move |_| append_digit("4")>"4"</button>
                <button class="calc-btn digit" on:click=move |_| append_digit("5")>"5"</button>
                <button class="calc-btn digit" on:click=move |_| append_digit("6")>"6"</button>
                <button
                    class=move || if active_operator.get() == Some(Operation::Subtract) { "calc-btn operator active" } else { "calc-btn operator" }
                    on:click=move |_| set_operation(Operation::Subtract)
                >"−"</button>

                <button class="calc-btn digit" on:click=move |_| append_digit("1")>"1"</button>
                <button class="calc-btn digit" on:click=move |_| append_digit("2")>"2"</button>
                <button class="calc-btn digit" on:click=move |_| append_digit("3")>"3"</button>
                <button
                    class=move || if active_operator.get() == Some(Operation::Add) { "calc-btn operator active" } else { "calc-btn operator" }
                    on:click=move |_| set_operation(Operation::Add)
                >"+"</button>

                <button class="calc-btn digit zero" on:click=move |_| append_digit("0")>"0"</button>
                <button class="calc-btn digit" on:click=move |_| append_digit(".")>"."</button>
                <button class="calc-btn operator" on:click=move |_| do_calculate()>"="</button>
            </div>
        </div>
    }
}

fn format_result(val: f64) -> String {
    if val.is_nan() {
        return String::from("Error");
    }
    if val.is_infinite() {
        return String::from("Error");
    }
    // Remove trailing zeros for cleaner display
    let s = format!("{}", val);
    if s.contains('.') {
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    } else {
        s
    }
}
