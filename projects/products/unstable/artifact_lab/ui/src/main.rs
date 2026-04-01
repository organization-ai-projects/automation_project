#[cfg(test)]
mod tests;

#[cfg(target_arch = "wasm32")]
fn main() {
    dioxus::launch(app);
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    println!("artifact_lab_ui (wasm build only)");
    println!();
    println!("This UI displays artifact bundle contents and verification results.");
    println!("Build for wasm32 target to run in browser.");
    println!();
    println!("CLI (backend):");
    println!("  artifact_lab pack   --root <dir> --out <bundle>");
    println!("  artifact_lab unpack --bundle <bundle> --out <dir>");
    println!("  artifact_lab verify --bundle <bundle> [--json]");
}

#[cfg(target_arch = "wasm32")]
fn app() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        div {
            class: "artifact-lab",
            h1 { "Artifact Lab" }
            p { "Deterministic artifact bundler — pack, unpack, verify." }
            section {
                h2 { "Usage" }
                pre { "artifact_lab pack   --root <dir> --out <bundle>" }
                pre { "artifact_lab unpack --bundle <bundle> --out <dir>" }
                pre { "artifact_lab verify --bundle <bundle> [--json]" }
            }
        }
    }
}
