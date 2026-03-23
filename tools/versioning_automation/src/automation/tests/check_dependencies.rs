use crate::automation::commands::CheckDependenciesOptions;

#[test]
fn check_dependencies_options_can_be_built() {
    let _opts = CheckDependenciesOptions {
        check_outdated: false,
        check_unused: false,
    };
}
