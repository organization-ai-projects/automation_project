// projects/products/stable/core/engine/src/cors_config.rs
#[derive(Clone, Debug, Default)]
pub(crate) struct CorsConfig {
    pub(crate) allow_any_origin: bool,
    pub(crate) allow_origin: Option<String>,
}
