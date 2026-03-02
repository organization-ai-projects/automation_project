use crate::app::app_state::AppState;

pub struct BundleScreen;

impl BundleScreen {
    pub fn render(state: &AppState) {
        println!("=== Bundle Screen ===");
        if let Some(ref hash) = state.bundle_hash {
            println!("Bundle hash: {hash}");
            println!("Manifest:");
            for entry in &state.bundle_manifest {
                println!("  - {entry}");
            }
        } else {
            println!("No bundle available.");
        }
    }
}
