use crate::automation::commands::ReleasePrepareOptions;

#[test]
fn release_prepare_options_can_be_built() {
    let _opts = ReleasePrepareOptions {
        version: "1.2.3".to_string(),
        auto_changelog: true,
    };
}
