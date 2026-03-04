// projects/products/unstable/evolutionary_system_generator/ui/src/app/screen.rs
#[derive(Debug, Default, Clone, PartialEq)]
pub enum Screen {
    #[default]
    Config,
    Running,
    Candidates,
    Report,
}
