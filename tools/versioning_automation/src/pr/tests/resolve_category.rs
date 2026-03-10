use crate::pr::commands::pr_resolve_category_options::PrResolveCategoryOptions;
use crate::pr::resolve_category::run_resolve_category;

#[test]
fn resolve_category_command_runs() {
    let opts = PrResolveCategoryOptions {
        label_category: "Unknown".to_string(),
        title_category: "UI".to_string(),
        default_category: "Mixed".to_string(),
    };
    let code = run_resolve_category(opts);
    assert_eq!(code, 0);
}
