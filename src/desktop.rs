use leptos::prelude::*;

#[derive(Clone, Copy, Default)]
struct SelectionRect {
    start_x: f64,
    start_y: f64,
    current_x: f64,
    current_y: f64,
    active: bool,
}

impl SelectionRect {
    fn left(&self) -> f64 {
        self.start_x.min(self.current_x)
    }

    fn top(&self) -> f64 {
        self.start_y.min(self.current_y)
    }

    fn width(&self) -> f64 {
        (self.current_x - self.start_x).abs()
    }

    fn height(&self) -> f64 {
        (self.current_y - self.start_y).abs()
    }
}

#[component]
pub fn Desktop() -> impl IntoView {
    let (selection, set_selection) = signal(SelectionRect::default());

    let on_mousedown = move |ev: web_sys::MouseEvent| {
        let x = ev.client_x() as f64;
        let y = ev.client_y() as f64;
        set_selection.set(SelectionRect {
            start_x: x,
            start_y: y,
            current_x: x,
            current_y: y,
            active: true,
        });
    };

    let on_mousemove = move |ev: web_sys::MouseEvent| {
        if selection.get().active {
            let x = ev.client_x() as f64;
            let y = ev.client_y() as f64;
            set_selection.update(|s| {
                s.current_x = x;
                s.current_y = y;
            });
        }
    };

    let on_mouseup = move |_ev: web_sys::MouseEvent| {
        set_selection.update(|s| {
            s.active = false;
        });
    };

    view! {
        <div
            class="desktop"
            on:mousedown=on_mousedown
            on:mousemove=on_mousemove
            on:mouseup=on_mouseup
        >
            <Show when=move || selection.get().active>
                <div
                    class="selection-rect"
                    style:left=move || format!("{}px", selection.get().left())
                    style:top=move || format!("{}px", selection.get().top())
                    style:width=move || format!("{}px", selection.get().width())
                    style:height=move || format!("{}px", selection.get().height())
                />
            </Show>
        </div>
    }
}
