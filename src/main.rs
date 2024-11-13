use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    let mut items = use_signal(|| vec![1, 2, 3]);

    rsx! {
        div { width: "100vw", height: "85vh",
            div {
                h1 { "Android! 😎" }
                img {
                    src: asset!("/assets/default_icon.png"),
                    width: "150px",
                    height: "150px",
                }
            }
            div {
                button { onclick: move |_| { items.write().push(1) }, "Add item??" }
            }
            div { "{items:#?}" }
        }
    }
}
