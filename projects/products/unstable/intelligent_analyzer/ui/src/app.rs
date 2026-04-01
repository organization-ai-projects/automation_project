use dioxus::prelude::*;

pub fn launch() {
    dioxus::launch(app);
}

#[component]
fn app() -> Element {
    rsx! {
        div {
            class: "intelligent-analyzer",
            h1 { "Intelligent Analyzer" }
            p { "Neurosymbolic code analysis dashboard" }
        }
    }
}
