use crate::automation::commands::CleanArtifactsOptions;

#[test]
fn clean_artifacts_options_can_be_built() {
    let _opts = CleanArtifactsOptions {
        include_node_modules: false,
    };
}
