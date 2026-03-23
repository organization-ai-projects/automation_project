use crate::automation::commands::ChangedCratesOptions;

#[test]
fn changed_crates_options_can_be_built() {
    let _opts = ChangedCratesOptions {
        ref1: None,
        ref2: None,
        output_format: None,
    };
}
