use crate::pr::commands::pr_directives_apply_options::PrDirectivesApplyOptions;
use crate::pr::domain::directives::directive_record_type::DirectiveRecordType;
use crate::pr::state::build_state;

pub(crate) fn run_directives_apply(opts: PrDirectivesApplyOptions) -> i32 {
    let state = build_state(&opts.text);

    let mut explicit_keys: Vec<String> = state.explicit_decisions.keys().cloned().collect();
    explicit_keys.sort_by_key(|issue| issue_number(issue));
    for issue in explicit_keys {
        if let Some(decision) = state.explicit_decisions.get(&issue) {
            println!("SET_DEC|{}|{}", issue, decision);
        }
    }

    let mut inferred_keys: Vec<String> = state.inferred_decisions.keys().cloned().collect();
    inferred_keys.sort_by_key(|issue| issue_number(issue));
    for issue in inferred_keys {
        if let Some(decision) = state.inferred_decisions.get(&issue) {
            println!("SET_INF|{}|{}", issue, decision);
        }
    }

    for record in state.action_records {
        match record.record_type {
            DirectiveRecordType::Event => {
                if record.first == "Closes" {
                    println!("ADD_CLOSE|{}", record.second);
                } else if record.first == "Reopen" {
                    println!("ADD_REOPEN|{}", record.second);
                }
            }
            DirectiveRecordType::Duplicate => {
                println!("ADD_DUP|{}|{}", record.first, record.second);
            }
            DirectiveRecordType::Decision => {}
        }
    }

    0
}

fn issue_number(issue_key: &str) -> u32 {
    issue_key
        .trim_start_matches('#')
        .parse::<u32>()
        .unwrap_or(u32::MAX)
}
