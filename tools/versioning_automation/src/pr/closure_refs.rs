use std::collections::HashSet;

use crate::pr::model::directive_record_type::DirectiveRecordType;
use crate::pr::model::pr_closure_refs_options::PrClosureRefsOptions;
use crate::pr::scan::scan_directives;

pub(crate) fn run_closure_refs(opts: PrClosureRefsOptions) -> i32 {
    let mut seen = HashSet::new();

    for record in scan_directives(&opts.text, false) {
        if record.record_type != DirectiveRecordType::Event {
            continue;
        }

        if record.first == "Closes" {
            let key = format!("CLOSE|{}", record.second);
            if seen.insert(key) {
                println!("CLOSE|Closes|{}", record.second);
            }
        } else if record.first == "Closes rejected" {
            let key = format!("PRE|{}", record.second);
            if seen.insert(key) {
                println!("PRE|Closes|{}", record.second);
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use crate::pr::model::pr_closure_refs_options::PrClosureRefsOptions;

    use super::run_closure_refs;

    #[test]
    fn closure_refs_command_runs() {
        let opts = PrClosureRefsOptions {
            text: "Closes #1\nCloses rejected #2".to_string(),
        };
        let code = run_closure_refs(opts);
        assert_eq!(code, 0);
    }
}
