use crate::constraints::constraint::Constraint;
use crate::genome::genome::Genome;

pub struct ConstraintEngine {
    pub constraints: Vec<Constraint>,
}

impl ConstraintEngine {
    pub fn new(constraints: Vec<Constraint>) -> Self {
        Self { constraints }
    }

    pub fn satisfies_all(&self, genome: &Genome) -> bool {
        self.violations(genome).is_empty()
    }

    pub fn violations(&self, genome: &Genome) -> Vec<String> {
        let mut v = Vec::new();
        for c in &self.constraints {
            match c {
                Constraint::MinActiveRules(min) => {
                    let active = genome.rules.iter().filter(|r| r.weight > 0).count();
                    if active < *min {
                        v.push(format!("MinActiveRules: need {}, have {}", min, active));
                    }
                }
                Constraint::MaxTotalWeight(max) => {
                    let total: u32 = genome.rules.iter().map(|r| r.weight).sum();
                    if total > *max {
                        v.push(format!("MaxTotalWeight: limit {}, have {}", max, total));
                    }
                }
                Constraint::RequiredRule(name) => {
                    let found = genome.rules.iter().any(|r| &r.name == name && r.weight > 0);
                    if !found {
                        v.push(format!("RequiredRule: '{}' must have weight > 0", name));
                    }
                }
            }
        }
        v
    }
}
