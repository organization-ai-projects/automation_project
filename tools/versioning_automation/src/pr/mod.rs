mod auto_add;
mod model;
mod parse;
mod render;
mod scan;

#[cfg(test)]
mod tests;

use auto_add::run_auto_add_closes;
use model::pr_action::PrAction;
use model::pr_directives_format::PrDirectivesFormat;
use parse::parse;
use render::{emit_json, emit_plain, print_usage};
use scan::scan_directives;

pub fn run(args: &[String]) -> i32 {
    match parse(args) {
        Ok(PrAction::Help) => {
            print_usage();
            0
        }
        Ok(PrAction::Directives(opts)) => {
            let records = scan_directives(&opts.text, opts.unique);
            match opts.format {
                PrDirectivesFormat::Plain => {
                    emit_plain(&records);
                    0
                }
                PrDirectivesFormat::Json => emit_json(&records),
            }
        }
        Ok(PrAction::AutoAddCloses(opts)) => run_auto_add_closes(opts),
        Err(message) => {
            eprintln!("{message}");
            2
        }
    }
}
