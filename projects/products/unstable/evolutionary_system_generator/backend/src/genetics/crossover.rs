// projects/products/unstable/evolutionary_system_generator/backend/src/genetics/crossover.rs
use crate::genetics::genome::Genome;
use crate::genetics::genome_id::GenomeId;
use crate::seed::xorshift64::Xorshift64;

pub fn uniform_crossover(
    rng: &mut Xorshift64,
    a: &Genome,
    b: &Genome,
    child_id: GenomeId,
) -> Genome {
    let rules = a
        .rules
        .iter()
        .zip(b.rules.iter())
        .map(|(ra, rb)| {
            if rng.next_range(2) == 0 {
                ra.clone()
            } else {
                rb.clone()
            }
        })
        .collect();
    Genome {
        id: child_id,
        rules,
    }
}
