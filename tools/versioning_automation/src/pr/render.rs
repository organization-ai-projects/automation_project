use common_json::to_string_pretty;

use crate::pr::model::directive_record::DirectiveRecord;

pub(crate) fn print_usage() {
    println!("Usage:");
    println!("  va pr directives --text \"...\" [--format plain|json] [--unique]");
    println!("  va pr directives --stdin [--format plain|json] [--unique]");
}

pub(crate) fn emit_plain(records: &[DirectiveRecord]) {
    for record in records {
        let record_type = match record.record_type {
            crate::pr::model::directive_record_type::DirectiveRecordType::Event => "EV",
            crate::pr::model::directive_record_type::DirectiveRecordType::Decision => "DEC",
            crate::pr::model::directive_record_type::DirectiveRecordType::Duplicate => "DUP",
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
