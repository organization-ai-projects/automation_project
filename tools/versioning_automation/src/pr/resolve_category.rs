use crate::pr::model::pr_resolve_category_options::PrResolveCategoryOptions;

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

#[cfg(test)]
mod tests {
    use crate::pr::model::pr_resolve_category_options::PrResolveCategoryOptions;

    use super::run_resolve_category;

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
}
