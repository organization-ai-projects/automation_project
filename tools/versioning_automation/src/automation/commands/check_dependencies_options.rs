#[derive(Debug, Clone)]
pub(crate) struct CheckDependenciesOptions {
    pub(crate) check_outdated: bool,
    pub(crate) check_unused: bool,
}
