use std::fs;

use regex::Regex;

use crate::pr::commands::pr_sort_bullets_options::PrSortBulletsOptions;

pub(crate) fn run_sort_bullets(opts: PrSortBulletsOptions) -> i32 {
    let content = match fs::read_to_string(&opts.input_file) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("failed to read input file '{}': {err}", opts.input_file);
            return 2;
        }
    };

    let mut rows = Vec::new();
    for line in content.lines() {
        let pr_number = extract_pr_number(line).unwrap_or(999_999);
        rows.push((pr_number, line.to_string()));
    }
    rows.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));

    for (_, line) in rows {
        println!("{line}");
    }
    0
}

fn extract_pr_number(line: &str) -> Option<u32> {
    let trailing_re = Regex::new(r"\(#([0-9]+)\)$").expect("valid regex");
    if let Some(captures) = trailing_re.captures(line) {
        let digits = captures.get(1)?.as_str();
        return digits.parse::<u32>().ok();
    }

    let inline_re = Regex::new(r"#([0-9]+)").expect("valid regex");
    if let Some(captures) = inline_re.captures(line) {
        let digits = captures.get(1)?.as_str();
        return digits.parse::<u32>().ok();
    }

    None
}
