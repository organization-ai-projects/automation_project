// projects/products/core/engine/src/lib.rs
pub mod config;
pub mod const_values;
pub mod cors_config;
pub mod engine_state;
pub mod project_metadata;
pub mod registry;
pub mod requires;
pub mod routes;
pub mod runtime;
pub mod ws;

pub use bootstrap::{
    BootstrapError, OwnerClaim, SetupState, consume_claim, ensure_owner_claim, setup_complete,
    validate_claim,
};
pub use config::EngineConfig;
pub use const_values::*;
pub use cors_config::CorsConfig;
pub use engine_state::EngineState;
pub use project_metadata::ProjectMetadata;
pub use protocol::accounts::{
    LoginRequest, LoginResponse, SetupAdminRequest, SetupAdminResponse, SetupStatusResponse,
};
pub use registry::Registry;
pub use requires::{require_permission, require_project_exists};
pub use routes::build_routes;
pub use runtime::*;
pub mod bootstrap;
