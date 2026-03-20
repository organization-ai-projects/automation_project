use crate::automation::commands::LabelsSyncOptions;

#[test]
fn labels_sync_options_can_be_built() {
    let _opts = LabelsSyncOptions {
        labels_file: "labels.json".to_string(),
        prune: false,
    };
}
