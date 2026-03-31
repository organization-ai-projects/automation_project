use crate::engine::RuleEngine;
use crate::model::{
    Aspirations, EmploymentStatus, EventMetadata, EventType, LifeEvent, Priority, Profile,
};

fn make_profile(income: Option<f64>) -> Profile {
    Profile {
        user_id: "user_1".to_string(),
        status: Some(EmploymentStatus::Employed),
        income_before: income,
        location: Some("Paris".to_string()),
    }
}

fn make_job_loss_event(reason: Option<&str>) -> LifeEvent {
    LifeEvent {
        event_type: EventType::JobLoss,
        date: "2026-03-30".to_string(),
        metadata: EventMetadata {
            reason: reason.map(|r| r.to_string()),
            additional_data: None,
        },
    }
}

#[test]
fn job_loss_produces_actions() {
    let profile = make_profile(None);
    let event = make_job_loss_event(None);
    let output = RuleEngine::evaluate(&profile, &None, &event);

    assert!(!output.actions.is_empty());
    assert!(output.actions.iter().any(|a| a.contains("CAF")));
    assert!(output.actions.iter().any(|a| a.contains("France Travail")));
    assert!(output.actions.iter().any(|a| a.contains("mutuelle")));
}

#[test]
fn job_loss_produces_estimations_with_income() {
    let profile = make_profile(Some(3000.0));
    let event = make_job_loss_event(None);
    let output = RuleEngine::evaluate(&profile, &None, &event);

    assert!(!output.estimations.is_empty());
    assert!(
        output
            .estimations
            .iter()
            .any(|e| e.contains("unemployment benefit"))
    );
    assert!(output.estimations.iter().any(|e| e.contains("1710.00")));
}

#[test]
fn job_loss_no_estimation_without_income() {
    let profile = make_profile(None);
    let event = make_job_loss_event(None);
    let output = RuleEngine::evaluate(&profile, &None, &event);

    assert!(output.estimations.is_empty());
}

#[test]
fn job_loss_produces_warnings() {
    let profile = make_profile(None);
    let event = make_job_loss_event(None);
    let output = RuleEngine::evaluate(&profile, &None, &event);

    assert!(!output.warnings.is_empty());
    assert!(output.warnings.iter().any(|w| w.contains("deadline")));
}

#[test]
fn job_loss_produces_opportunities() {
    let profile = make_profile(None);
    let event = make_job_loss_event(None);
    let output = RuleEngine::evaluate(&profile, &None, &event);

    assert!(!output.opportunities.is_empty());
    assert!(
        output
            .opportunities
            .iter()
            .any(|o| o.contains("training") || o.contains("Training"))
    );
}

#[test]
fn job_loss_inaptitude_adds_specific_actions() {
    let profile = make_profile(Some(2500.0));
    let event = make_job_loss_event(Some("inaptitude"));
    let output = RuleEngine::evaluate(&profile, &None, &event);

    assert!(
        output
            .actions
            .iter()
            .any(|a| a.contains("inaptitude certificate"))
    );
    assert!(output.warnings.iter().any(|w| w.contains("Inaptitude")));
}

#[test]
fn job_loss_with_aspirations_adds_goal_opportunity() {
    let profile = make_profile(None);
    let event = make_job_loss_event(None);
    let aspirations = Some(Aspirations {
        goal: Some("become a freelancer".to_string()),
        priorities: vec![Priority {
            name: "freedom".to_string(),
            weight: 9,
        }],
    });
    let output = RuleEngine::evaluate(&profile, &aspirations, &event);

    assert!(
        output
            .opportunities
            .iter()
            .any(|o| o.contains("freelancer"))
    );
}

#[test]
fn works_with_minimal_profile() {
    let profile = Profile {
        user_id: "minimal".to_string(),
        status: None,
        income_before: None,
        location: None,
    };
    let event = make_job_loss_event(None);
    let output = RuleEngine::evaluate(&profile, &None, &event);

    assert!(!output.actions.is_empty());
}

#[test]
fn new_job_event_produces_actions() {
    let profile = make_profile(Some(3000.0));
    let event = LifeEvent {
        event_type: EventType::NewJob,
        date: "2026-04-01".to_string(),
        metadata: EventMetadata::default(),
    };
    let output = RuleEngine::evaluate(&profile, &None, &event);

    assert!(!output.actions.is_empty());
    assert!(output.actions.iter().any(|a| a.contains("CAF")));
}

#[test]
fn health_issue_event_produces_actions() {
    let profile = make_profile(Some(2800.0));
    let event = LifeEvent {
        event_type: EventType::HealthIssue,
        date: "2026-04-01".to_string(),
        metadata: EventMetadata::default(),
    };
    let output = RuleEngine::evaluate(&profile, &None, &event);

    assert!(!output.actions.is_empty());
    assert!(output.actions.iter().any(|a| a.contains("CPAM")));
    assert!(!output.warnings.is_empty());
}

#[test]
fn serialization_round_trip() {
    let profile = make_profile(Some(3000.0));
    let event = make_job_loss_event(Some("inaptitude"));
    let output = RuleEngine::evaluate(&profile, &None, &event);

    let json_str = common_json::to_json_string_pretty(&output).unwrap();
    let deserialized: crate::model::RecommendationOutput =
        common_json::from_str(&json_str).unwrap();

    assert_eq!(output.actions, deserialized.actions);
    assert_eq!(output.estimations, deserialized.estimations);
    assert_eq!(output.warnings, deserialized.warnings);
    assert_eq!(output.opportunities, deserialized.opportunities);
}
