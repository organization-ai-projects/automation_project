use crate::contract::module_spec::ModuleSpec;

pub struct CrateGenerator {
    spec: ModuleSpec,
}

impl CrateGenerator {
    pub fn new(spec: ModuleSpec) -> Self {
        Self { spec }
    }

    pub fn generate_paths(&self) -> Vec<String> {
        self.spec
            .files
            .iter()
            .map(|file| file.path.clone())
            .collect()
    }
}
