// projects/products/unstable/evolutionary_system_generator/ui/src/screens/candidate_screen.rs
pub fn render_candidate_screen(rank: usize, fitness: f64) -> Vec<String> {
    vec![format!("Candidate rank {}: fitness={:.4}", rank, fitness)]
}
