use crate::pr::commands::pr_resolve_category_options::PrResolveCategoryOptions;

pub(crate) fn run_resolve_category(opts: PrResolveCategoryOptions) -> i32 {
    let mut effective = opts.label_category;

    if (effective == "Unknown" || effective == "Mixed")
        && opts.title_category != "Unknown"
        && opts.title_category != "Mixed"
    {
        effective = opts.title_category;
    }

    if effective == "Unknown" && opts.default_category != "Unknown" {
        effective = opts.default_category;
    }

    println!("{effective}");
    0
}
