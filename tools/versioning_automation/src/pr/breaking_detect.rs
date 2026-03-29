//! tools/versioning_automation/src/pr/breaking_detect.rs
use regex::Regex;

pub(crate) fn labels_indicate_breaking(labels_raw: &str) -> bool {
    labels_raw
        .split("||")
        .any(|label| label.trim().eq_ignore_ascii_case("breaking"))
}

pub(crate) fn text_indicates_breaking(text: &str) -> bool {
    let non_breaking_re = Regex::new(r"non[\s-]?breaking[\s_-]*change").expect("valid regex");
    let no_breaking_re =
        Regex::new(r"^[\s]*(no|without)[\s]+breaking[\s_-]*changes?").expect("valid regex");
    let checked_re =
        Regex::new(r"^[\s]*-[\s]*\[[xX]\][\s]*breaking[\s_-]*change([\s]|$)").expect("valid regex");
    let heading_re = Regex::new(r"^[\s]*breaking[\s_-]*change[\s]*:").expect("valid regex");
    let cc_breaking_re =
        Regex::new(r"^[\s]*[a-z][a-z0-9_-]*(\([a-z0-9_./,-]+\))?!:[\s]+").expect("valid regex");

    for line in text.lines() {
        let lower = line.to_lowercase();

        if non_breaking_re.is_match(&lower) {
            continue;
        }
        if no_breaking_re.is_match(&lower) {
            continue;
        }
        if checked_re.is_match(&lower) {
            return true;
        }
        if heading_re.is_match(&lower) {
            return true;
        }
        if cc_breaking_re.is_match(&lower) {
            return true;
        }
    }

    false
}
