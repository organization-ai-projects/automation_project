// projects/products/unstable/code_forge_engine/ui/src/screens/contract_screen.rs
pub struct ContractScreen {
    pub contract_path: Option<String>,
}

impl ContractScreen {
    pub fn new() -> Self {
        Self { contract_path: None }
    }

    pub fn set_path(&mut self, path: impl Into<String>) {
        self.contract_path = Some(path.into());
    }
}
