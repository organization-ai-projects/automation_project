// projects/products/unstable/evolutionary_system_generator/ui/src/screens/run_screen.rs
pub fn render_run_screen(generation: u32, best_fitness: f64, done: bool) -> Vec<String> {
    vec![format!(
        "Generation: {} | Best Fitness: {:.4} | Done: {}",
        generation, best_fitness, done
    )]
}
