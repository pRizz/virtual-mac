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
            let current = strip_separators(&display.get());
            if current == "0" && digit != "." {
                set_display.set(digit.to_string());
            } else if digit == "." && current.contains('.') {
                // Don't add another decimal point
            } else {
                let new_value = format!("{}{}", current, digit);
                set_display.set(format_display_with_separators(&new_value));
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
        let current = strip_separators(&display.get());
        if let Ok(val) = current.parse::<f64>() {
            let negated = -val;
            set_display.set(format_result(negated));
        }
    };

    let percent = move || {
        let current = strip_separators(&display.get());
        if let Ok(val) = current.parse::<f64>() {
            let result = val / 100.0;
            set_display.set(format_result(result));
        }
    };

    let do_calculate = move || {
        let current = strip_separators(&display.get());
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
        let current = strip_separators(&display.get());
        if let Ok(_val) = current.parse::<f64>() {
            // If there's a pending operation, calculate first
            if current_op.get() != Operation::None && !clear_on_next.get() {
                do_calculate();
            }
            set_stored_value.set(strip_separators(&display.get()).parse().unwrap_or(0.0));
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
                // Skip handling if event is from an input element (e.g., Terminal input)
                if let Some(target) = evt.target() {
                    if let Some(element) = target.dyn_ref::<web_sys::HtmlInputElement>() {
                        // Allow the input to handle its own events
                        let _ = element;
                        return;
                    }
                    if let Some(element) = target.dyn_ref::<web_sys::HtmlTextAreaElement>() {
                        let _ = element;
                        return;
                    }
                    // Also check contenteditable elements
                    if let Some(element) = target.dyn_ref::<web_sys::HtmlElement>() {
                        if element.is_content_editable() {
                            return;
                        }
                    }
                }
                match evt.key().as_str() {
                    "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "." => {
                        let digit = evt.key();
                        // Inline append_digit logic
                        if clear_on_next.get() {
                            set_display.set(digit.clone());
                            set_clear_on_next.set(false);
                        } else {
                            let current = strip_separators(&display.get());
                            if current == "0" && digit != "." {
                                set_display.set(digit);
                            } else if digit == "." && current.contains('.') {
                                // Don't add another decimal point
                            } else {
                                let new_value = format!("{}{}", current, digit);
                                set_display.set(format_display_with_separators(&new_value));
                            }
                        }
                        set_active_operator.set(None);
                    }
                    "+" => {
                        let current = strip_separators(&display.get());
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
                                    Operation::Divide => {
                                        if current_val != 0.0 {
                                            stored / current_val
                                        } else {
                                            f64::NAN
                                        }
                                    }
                                };
                                set_display.set(format_result(result));
                            }
                            set_stored_value
                                .set(strip_separators(&display.get()).parse().unwrap_or(0.0));
                            set_current_op.set(Operation::Add);
                            set_clear_on_next.set(true);
                            set_active_operator.set(Some(Operation::Add));
                        }
                    }
                    "-" => {
                        let current = strip_separators(&display.get());
                        if current.parse::<f64>().is_ok() {
                            if current_op.get() != Operation::None && !clear_on_next.get() {
                                let current_val = current.parse::<f64>().unwrap_or(0.0);
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
                            }
                            set_stored_value
                                .set(strip_separators(&display.get()).parse().unwrap_or(0.0));
                            set_current_op.set(Operation::Subtract);
                            set_clear_on_next.set(true);
                            set_active_operator.set(Some(Operation::Subtract));
                        }
                    }
                    "*" => {
                        let current = strip_separators(&display.get());
                        if current.parse::<f64>().is_ok() {
                            if current_op.get() != Operation::None && !clear_on_next.get() {
                                let current_val = current.parse::<f64>().unwrap_or(0.0);
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
                            }
                            set_stored_value
                                .set(strip_separators(&display.get()).parse().unwrap_or(0.0));
                            set_current_op.set(Operation::Multiply);
                            set_clear_on_next.set(true);
                            set_active_operator.set(Some(Operation::Multiply));
                        }
                    }
                    "/" => {
                        evt.prevent_default(); // Prevent browser quick-find
                        let current = strip_separators(&display.get());
                        if current.parse::<f64>().is_ok() {
                            if current_op.get() != Operation::None && !clear_on_next.get() {
                                let current_val = current.parse::<f64>().unwrap_or(0.0);
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
                            }
                            set_stored_value
                                .set(strip_separators(&display.get()).parse().unwrap_or(0.0));
                            set_current_op.set(Operation::Divide);
                            set_clear_on_next.set(true);
                            set_active_operator.set(Some(Operation::Divide));
                        }
                    }
                    "=" | "Enter" => {
                        // do_calculate logic inline
                        let current = strip_separators(&display.get());
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
                        let current = strip_separators(&display.get());
                        if let Ok(val) = current.parse::<f64>() {
                            let result = val / 100.0;
                            set_display.set(format_result(result));
                        }
                    }
                    "Backspace" | "Delete" => {
                        let current = strip_separators(&display.get());
                        if current.len() > 1 {
                            let new_value = current[..current.len() - 1].to_string();
                            set_display.set(format_display_with_separators(&new_value));
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
                <button class="calc-btn function" on:click=move |_| clear()>
                    {move || if current_op.get() != Operation::None || stored_value.get() != 0.0 { "C" } else { "AC" }}
                </button>
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

    // Check if it's effectively an integer
    if val.fract() == 0.0 && val.abs() < 1e15 {
        // Format integer with thousands separators
        format_with_separators(val as i64)
    } else {
        // Decimal number - limit precision and remove trailing zeros
        let s = format!("{:.9}", val);
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    }
}

fn format_with_separators(n: i64) -> String {
    let negative = n < 0;
    let s = n.abs().to_string();
    let chars: Vec<char> = s.chars().rev().collect();
    let formatted: String = chars
        .chunks(3)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(",")
        .chars()
        .rev()
        .collect();
    if negative {
        format!("-{}", formatted)
    } else {
        formatted
    }
}

/// Format a display string with thousand separators while preserving decimal part
fn format_display_with_separators(s: &str) -> String {
    // Handle empty or just "0"
    if s.is_empty() || s == "0" {
        return s.to_string();
    }

    // Split on decimal point
    let parts: Vec<&str> = s.split('.').collect();
    let integer_part = parts[0].replace(",", ""); // Remove existing commas
    let decimal_part = parts.get(1);

    // Format integer part with separators
    let negative = integer_part.starts_with('-');
    let digits: String = if negative {
        integer_part[1..].to_string()
    } else {
        integer_part.clone()
    };

    if digits.is_empty() {
        return s.to_string();
    }

    let chars: Vec<char> = digits.chars().rev().collect();
    let formatted: String = chars
        .chunks(3)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(",")
        .chars()
        .rev()
        .collect();

    let formatted_integer = if negative {
        format!("-{}", formatted)
    } else {
        formatted
    };

    // Recombine with decimal part if present
    match decimal_part {
        Some(dec) => format!("{}.{}", formatted_integer, dec),
        None => formatted_integer,
    }
}

/// Strip commas from display string for parsing
fn strip_separators(s: &str) -> String {
    s.replace(",", "")
}
