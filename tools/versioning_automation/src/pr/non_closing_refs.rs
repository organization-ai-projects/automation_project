use std::collections::HashSet;

use crate::pr::model::directive_record_type::DirectiveRecordType;
use crate::pr::model::pr_non_closing_refs_options::PrNonClosingRefsOptions;
use crate::pr::scan::scan_directives;

pub(crate) fn run_non_closing_refs(opts: PrNonClosingRefsOptions) -> i32 {
    let mut seen = HashSet::new();

    for record in scan_directives(&opts.text, false) {
        if record.record_type != DirectiveRecordType::Event {
            continue;
        }
        if record.first != "Part of" {
            continue;
        }

        let key = format!("PART|{}", record.second);
        if seen.insert(key) {
            println!("Part of|{}", record.second);
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use crate::pr::model::pr_non_closing_refs_options::PrNonClosingRefsOptions;

    use super::run_non_closing_refs;

    #[test]
    fn non_closing_refs_command_runs() {
        let opts = PrNonClosingRefsOptions {
            text: "Part of #3\nPart of #3".to_string(),
        };
        let code = run_non_closing_refs(opts);
        assert_eq!(code, 0);
    }
}
