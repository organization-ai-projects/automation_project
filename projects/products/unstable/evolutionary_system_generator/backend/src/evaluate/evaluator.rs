use crate::constraints::constraint_engine::ConstraintEngine;
use crate::evaluate::evaluation_report::EvaluationReport;
use crate::evaluate::fitness::Fitness;
use crate::genome::genome::Genome;

pub struct Evaluator {
    pub constraint_engine: ConstraintEngine,
}

impl Evaluator {
    pub fn new(constraint_engine: ConstraintEngine) -> Self {
        Self { constraint_engine }
    }

    pub fn evaluate(&self, genome: &Genome) -> (Fitness, EvaluationReport) {
        let active_rules: Vec<_> = genome.rules.iter().filter(|r| r.weight > 0).collect();
        let active_count = active_rules.len();
        let total_weight: u32 = genome.rules.iter().map(|r| r.weight).sum();
        let violations = self.constraint_engine.violations(genome);
        let satisfied = violations.is_empty();

        let rule_score = if genome.rules.is_empty() {
            0.0
        } else {
            let max_possible = genome.rules.len() as f64 * 10.0;
            (total_weight as f64 / max_possible).min(1.0)
        };

        let constraint_bonus = if satisfied { 0.5 } else { 0.0 };

        let diversity_score = if genome.rules.is_empty() {
            0.0
        } else {
            active_count as f64 / genome.rules.len() as f64
        };

        let raw = (rule_score + constraint_bonus + diversity_score) / 3.0;
        let fitness_val = raw.clamp(0.0, 1.0);

        let fitness = Fitness(fitness_val);
        let report = EvaluationReport {
            genome_id: genome.id,
            fitness: fitness.clone(),
            active_rule_count: active_count,
            total_weight,
            constraint_violations: violations,
            satisfied_constraints: satisfied,
        };
        (fitness, report)
    }
}
