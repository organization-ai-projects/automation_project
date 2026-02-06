// projects/products/accounts/ui/src/permission_picker.rs
use dioxus::prelude::*;

/// Permission selector with pill buttons for toggling
pub fn permission_picker(mut selected: Signal<Vec<String>>) -> Element {
    let options = [
        "read",
        "write",
        "execute",
        "delete",
        "admin",
        "train",
        "view_logs",
        "configure_system",
    ];

    rsx! {
        label { "Extra permissions" }
        div { class: "pill-grid",
            for option in options.iter() {
                {
                    let is_selected = selected.read().contains(&option.to_string());
                    let opt = option.to_string();
                    rsx! {
                        button {
                            class: if is_selected { "pill active" } else { "pill" },
                            onclick: move |_| {
                                let mut next = selected.read().clone();
                                if let Some(pos) = next.iter().position(|v| v == &opt) {
                                    next.remove(pos);
                                } else {
                                    next.push(opt.clone());
                                }
                                selected.set(next);
                            },
                            "{option}"
                        }
                    }
                }
            }
        }
    }
}
