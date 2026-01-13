// projects/products/code_agent_sandbox/src/engine/mod.rs
// Modules internes
pub mod engine_config;
pub mod engine_ctx;
pub mod engine_init;
pub mod engine_orchestrator;
pub mod engine_paths;
pub mod generate_code;
pub mod generate_utils;
pub mod path_rights;
pub mod records;
pub mod request;
pub mod response;
pub mod rights;
pub mod workspace_mode;

// ✅ API publique du module `engine`
// Expose uniquement ce qui est nécessaire pour l'extérieur
pub use engine_orchestrator::execute_request;
pub use engine_paths::EnginePaths;
pub use request::Request;
pub use response::Response;
pub use workspace_mode::WorkspaceMode;

// ✅ API interne au crate
// Expose uniquement pour le crate courant
pub(crate) use engine_config::EngineConfig;
pub(crate) use engine_ctx::EngineCtx;
pub(crate) use engine_init::{EngineInit, initialize_engine};
pub(crate) use generate_code::handle_generate_code;
pub(crate) use generate_utils::generate_globs;
pub(crate) use path_rights::{FORBIDDEN, PATH_RIGHTS, READ, WRITE};
pub(crate) use records::{record_action_event, record_and_push_result};
pub(crate) use rights::Rights;
