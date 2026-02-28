use crate::genome::genome::Genome;
use crate::genome::genome_id::GenomeId;
use crate::seed::seed::Xorshift64;

pub fn uniform_crossover(rng: &mut Xorshift64, a: &Genome, b: &Genome, child_id: GenomeId) -> Genome {
    let rules = a.rules.iter().zip(b.rules.iter()).map(|(ra, rb)| {
        if rng.next_range(2) == 0 {
            ra.clone()
        } else {
            rb.clone()
        }
    }).collect();
    Genome { id: child_id, rules }
}
