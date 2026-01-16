// projects/products/core/engine/src/cors_config.rs
#[derive(Clone, Debug, Default)]
pub struct CorsConfig {
    pub allow_any_origin: bool,
    pub allow_origin: Option<String>,
}
