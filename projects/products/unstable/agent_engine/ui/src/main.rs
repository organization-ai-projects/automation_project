//! projects/products/unstable/agent_engine/ui/src/main.rs
mod app;

fn main() {
    dioxus::launch(app::app);
}

#[cfg(test)]
mod tests;
