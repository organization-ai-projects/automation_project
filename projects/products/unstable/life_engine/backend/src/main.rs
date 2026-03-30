mod diagnostics;
mod engine;
mod io;
mod model;

use crate::diagnostics::AppError;
use crate::engine::RuleEngine;
use crate::io::JsonCodec;
use crate::model::{Aspirations, EventMetadata, EventType, LifeEvent, Profile};

fn main() -> Result<(), AppError> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        print_usage();
        return Ok(());
    }

    match args[1].as_str() {
        "evaluate" => {
            let profile = parse_profile(&args)?;
            let aspirations = parse_aspirations(&args);
            let event = parse_event(&args)?;
            let output = RuleEngine::evaluate(&profile, &aspirations, &event);
            let json = JsonCodec::encode(&output)
                .map_err(|e| AppError::Process(format!("serialization failed: {e}")))?;
            println!("{json}");
            Ok(())
        }
        "demo" => {
            run_demo();
            Ok(())
        }
        _ => {
            print_usage();
            Ok(())
        }
    }
}

fn parse_profile(args: &[String]) -> Result<Profile, AppError> {
    let mut profile = Profile {
        user_id: "cli_user".to_string(),
        ..Profile::default()
    };

    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--income" => {
                i += 1;
                if i < args.len() {
                    profile.income_before = args[i].parse().ok();
                }
            }
            "--location" => {
                i += 1;
                if i < args.len() {
                    profile.location = Some(args[i].clone());
                }
            }
            _ => {}
        }
        i += 1;
    }
    Ok(profile)
}

fn parse_aspirations(args: &[String]) -> Option<Aspirations> {
    let mut goal: Option<String> = None;
    let mut i = 2;
    while i < args.len() {
        if args[i] == "--goal" {
            i += 1;
            if i < args.len() {
                goal = Some(args[i].clone());
            }
        }
        i += 1;
    }
    goal.map(|g| Aspirations {
        goal: Some(g),
        priorities: vec![],
    })
}

fn parse_event(args: &[String]) -> Result<LifeEvent, AppError> {
    let mut event_type: Option<EventType> = None;
    let mut reason: Option<String> = None;
    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--event" => {
                i += 1;
                if i < args.len() {
                    event_type = match args[i].as_str() {
                        "job_loss" => Some(EventType::JobLoss),
                        "new_job" => Some(EventType::NewJob),
                        "health_issue" => Some(EventType::HealthIssue),
                        _ => None,
                    };
                }
            }
            "--reason" => {
                i += 1;
                if i < args.len() {
                    reason = Some(args[i].clone());
                }
            }
            _ => {}
        }
        i += 1;
    }
    let event_type = event_type.ok_or_else(|| {
        AppError::Config("--event required (job_loss|new_job|health_issue)".to_string())
    })?;
    Ok(LifeEvent {
        event_type,
        date: "2026-03-30".to_string(),
        metadata: EventMetadata {
            reason,
            additional_data: None,
        },
    })
}

fn run_demo() {
    println!("=== Life Engine MVP Demo ===\n");

    let profile = Profile {
        user_id: "demo_user".to_string(),
        status: Some(model::EmploymentStatus::Employed),
        income_before: Some(2800.0),
        location: Some("Lyon".to_string()),
    };

    let aspirations = Some(Aspirations {
        goal: Some("reconversion into tech".to_string()),
        priorities: vec![],
    });

    let event = LifeEvent {
        event_type: EventType::JobLoss,
        date: "2026-03-30".to_string(),
        metadata: EventMetadata {
            reason: Some("inaptitude".to_string()),
            additional_data: None,
        },
    };

    let output = RuleEngine::evaluate(&profile, &aspirations, &event);
    let json = JsonCodec::encode(&output).expect("serialization should not fail");
    println!("{json}");
}

fn print_usage() {
    println!("life_engine_backend - life event decision engine");
    println!();
    println!("Commands:");
    println!(
        "  evaluate --event <type> [--income <amount>] [--location <loc>] [--goal <goal>] [--reason <reason>]"
    );
    println!("  demo     Run a demonstration scenario");
    println!();
    println!("Event types: job_loss, new_job, health_issue");
}
