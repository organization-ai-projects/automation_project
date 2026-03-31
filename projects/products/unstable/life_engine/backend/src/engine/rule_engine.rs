use crate::model::{Aspirations, EventType, LifeEvent, Profile, RecommendationOutput};

pub struct RuleEngine;

impl RuleEngine {
    pub fn evaluate(
        profile: &Profile,
        aspirations: &Option<Aspirations>,
        event: &LifeEvent,
    ) -> RecommendationOutput {
        match event.event_type {
            EventType::JobLoss => Self::handle_job_loss(profile, aspirations, event),
            EventType::NewJob => Self::handle_new_job(profile, aspirations),
            EventType::HealthIssue => Self::handle_health_issue(profile, aspirations),
        }
    }

    fn handle_job_loss(
        profile: &Profile,
        aspirations: &Option<Aspirations>,
        event: &LifeEvent,
    ) -> RecommendationOutput {
        let mut output = RecommendationOutput::default();

        output.actions.push("Declare situation to CAF".to_string());
        output
            .actions
            .push("Prepare France Travail registration".to_string());
        output
            .actions
            .push("Check mutuelle portability".to_string());

        if let Some(income) = profile.income_before {
            let estimated_benefit = income * 0.57;
            output.estimations.push(format!(
                "Estimated monthly unemployment benefit: {estimated_benefit:.2} EUR"
            ));
        }

        if let Some(ref reason) = event.metadata.reason {
            if reason == "inaptitude" {
                output
                    .actions
                    .push("Request inaptitude certificate from employer".to_string());
                output
                    .warnings
                    .push("Inaptitude dismissal has specific legal deadlines".to_string());
            }
        }

        output
            .warnings
            .push("Risk of missing France Travail registration deadline (12 months)".to_string());

        output
            .opportunities
            .push("Explore training programs (CPF)".to_string());
        output
            .opportunities
            .push("Job suggestions (basic placeholder)".to_string());

        if let Some(asp) = aspirations {
            if let Some(goal) = &asp.goal {
                output
                    .opportunities
                    .push(format!("Consider opportunities aligned with goal: {goal}"));
            }
        }

        output
    }

    fn handle_new_job(
        profile: &Profile,
        _aspirations: &Option<Aspirations>,
    ) -> RecommendationOutput {
        let mut output = RecommendationOutput::default();

        output
            .actions
            .push("Update CAF with new employment status".to_string());
        output
            .actions
            .push("Notify France Travail of employment".to_string());

        if profile.income_before.is_some() {
            output
                .estimations
                .push("Compare previous vs new income for benefit adjustment".to_string());
        }

        output
    }

    fn handle_health_issue(
        profile: &Profile,
        _aspirations: &Option<Aspirations>,
    ) -> RecommendationOutput {
        let mut output = RecommendationOutput::default();

        output
            .actions
            .push("Contact CPAM for sick leave declaration".to_string());
        output
            .actions
            .push("Notify employer within 48 hours".to_string());

        if profile.income_before.is_some() {
            output
                .estimations
                .push("Estimate daily sick leave compensation (IJSS)".to_string());
        }

        output
            .warnings
            .push("48-hour deadline for sick leave declaration".to_string());

        output
    }
}

#[cfg(test)]
mod tests;
