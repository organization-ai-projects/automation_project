use crate::search::population::{Individual, Population};
use crate::seed::seed::Xorshift64;

pub fn tournament_select<'a>(rng: &mut Xorshift64, population: &'a Population) -> &'a Individual {
    let n = population.individuals.len();
    let a = rng.next_range(n);
    let b = rng.next_range(n);
    let ia = &population.individuals[a];
    let ib = &population.individuals[b];
    if ia.fitness.0 >= ib.fitness.0 { ia } else { ib }
}
