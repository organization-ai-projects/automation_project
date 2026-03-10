use common_json::to_string_pretty;

use crate::pr::domain::directives::directive_record::DirectiveRecord;

pub(crate) fn print_usage() {
    println!("Usage:");
    println!("  va pr directives --text \"...\" [--format plain|json] [--unique]");
    println!("  va pr directives --stdin [--format plain|json] [--unique]");
    println!("  va pr closure-refs (--text \"...\" | --stdin)");
    println!("  va pr non-closing-refs (--text \"...\" | --stdin)");
    println!(
        "  va pr closure-marker (--text \"...\" | --stdin) --keyword-pattern <regex> --issue <#n> --mode <apply|remove>"
    );
    println!("  va pr directives-state (--text \"...\" | --stdin)");
    println!("  va pr directive-conflicts (--text \"...\" | --stdin) [--source-branch-count <n>]");
    println!("  va pr directive-conflict-guard --pr <number> [--repo owner/name]");
    println!(
        "  va pr duplicate-actions (--text \"...\" | --stdin) --mode <safe|auto-close> --repo owner/name [--assume-yes true|false]"
    );
    println!(
        "  va pr group-by-category (--text \"...\" | --stdin) --mode <resolved|reopen|conflict|directive>"
    );
    println!(
        "  va pr effective-category --labels-raw \"label1||label2\" --title \"...\" --default-category <name>"
    );
    println!("  va pr issue-category-from-labels --labels-raw \"label1||label2\"");
    println!("  va pr issue-category-from-title --title \"...\"");
    println!("  va pr issue-ref-kind --issue <number> [--repo owner/name]");
    println!(
        "  va pr issue-decision --action <Closes|Reopen> --issue <#n> --default-category <name> [--seen-reopen true|false] [--reopen-category <name>] [--inferred-decision <close|reopen|conflict>] [--explicit-decision <close|reopen>] [--allow-inferred true|false]"
    );
    println!(
        "  va pr resolve-category --label-category <name> --title-category <name> --default-category <name>"
    );
    println!("  va pr auto-add-closes --pr <number> [--repo owner/name]");
}

pub(crate) fn emit_plain(records: &[DirectiveRecord]) {
    for record in records {
        let record_type = match record.record_type {
            crate::pr::domain::directives::directive_record_type::DirectiveRecordType::Event => "EV",
            crate::pr::domain::directives::directive_record_type::DirectiveRecordType::Decision => "DEC",
            crate::pr::domain::directives::directive_record_type::DirectiveRecordType::Duplicate => "DUP",
        };
        println!("{}|{}|{}", record_type, record.first, record.second);
    }
}

pub(crate) fn emit_json(records: &[DirectiveRecord]) -> i32 {
    let payload = records.to_vec();
    match to_string_pretty(&payload) {
        Ok(json) => {
            println!("{json}");
            0
        }
        Err(err) => {
            eprintln!("failed to serialize directives as json: {err}");
            1
        }
    }
}
