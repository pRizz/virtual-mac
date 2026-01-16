use leptos::prelude::*;

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
    };

    let clear = move || {
        set_display.set(String::from("0"));
        set_stored_value.set(0.0);
        set_current_op.set(Operation::None);
        set_clear_on_next.set(false);
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
        }
    };

    view! {
        <div class="calculator">
            <div class="calc-display">
                <span class="calc-display-text">{move || display.get()}</span>
            </div>
            <div class="calc-buttons">
                <button class="calc-btn function" on:click=move |_| clear()>"AC"</button>
                <button class="calc-btn function" on:click=move |_| negate()>"+/−"</button>
                <button class="calc-btn function" on:click=move |_| percent()>"%"</button>
                <button class="calc-btn operator" on:click=move |_| set_operation(Operation::Divide)>"÷"</button>

                <button class="calc-btn digit" on:click=move |_| append_digit("7")>"7"</button>
                <button class="calc-btn digit" on:click=move |_| append_digit("8")>"8"</button>
                <button class="calc-btn digit" on:click=move |_| append_digit("9")>"9"</button>
                <button class="calc-btn operator" on:click=move |_| set_operation(Operation::Multiply)>"×"</button>

                <button class="calc-btn digit" on:click=move |_| append_digit("4")>"4"</button>
                <button class="calc-btn digit" on:click=move |_| append_digit("5")>"5"</button>
                <button class="calc-btn digit" on:click=move |_| append_digit("6")>"6"</button>
                <button class="calc-btn operator" on:click=move |_| set_operation(Operation::Subtract)>"−"</button>

                <button class="calc-btn digit" on:click=move |_| append_digit("1")>"1"</button>
                <button class="calc-btn digit" on:click=move |_| append_digit("2")>"2"</button>
                <button class="calc-btn digit" on:click=move |_| append_digit("3")>"3"</button>
                <button class="calc-btn operator" on:click=move |_| set_operation(Operation::Add)>"+"</button>

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
