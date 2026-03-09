use std::io::{self, Read};

use crate::pr::model::pr_action::PrAction;
use crate::pr::model::pr_directives_format::PrDirectivesFormat;
use crate::pr::model::pr_directives_options::PrDirectivesOptions;

pub(crate) fn parse(args: &[String]) -> Result<PrAction, String> {
    if args.is_empty() {
        return Ok(PrAction::Help);
    }

    match args[0].as_str() {
        "help" | "--help" | "-h" => Ok(PrAction::Help),
        "directives" => parse_directives(&args[1..]).map(PrAction::Directives),
        unknown => Err(format!("Unknown pr subcommand: {unknown}")),
    }
}

fn parse_directives(args: &[String]) -> Result<PrDirectivesOptions, String> {
    let mut text: Option<String> = None;
    let mut read_stdin = false;
    let mut format = PrDirectivesFormat::Plain;
    let mut unique = false;

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--text" => {
                text = Some(take_value("--text", args, &mut i)?);
            }
            "--stdin" => {
                read_stdin = true;
                i += 1;
            }
            "--format" => {
                let value = take_value("--format", args, &mut i)?;
                format = match value.as_str() {
                    "plain" => PrDirectivesFormat::Plain,
                    "json" => PrDirectivesFormat::Json,
                    _ => return Err("--format must be 'plain' or 'json'".to_string()),
                };
            }
            "--unique" => {
                unique = true;
                i += 1;
            }
            unknown => return Err(format!("Unknown option for directives: {unknown}")),
        }
    }

    let resolved_text = if read_stdin {
        read_stdin_text()?
    } else {
        text.unwrap_or_default()
    };

    if resolved_text.is_empty() {
        return Err("directives requires --text <value> or --stdin".to_string());
    }

    Ok(PrDirectivesOptions {
        text: resolved_text,
        format,
        unique,
    })
}

fn take_value(flag: &str, args: &[String], index: &mut usize) -> Result<String, String> {
    let value_index = *index + 1;
    if value_index >= args.len() {
        return Err(format!("{flag} requires a value"));
    }
    let value = args[value_index].clone();
    *index += 2;
    Ok(value)
}

fn read_stdin_text() -> Result<String, String> {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .map_err(|err| format!("failed to read stdin: {err}"))?;
    Ok(input)
}
