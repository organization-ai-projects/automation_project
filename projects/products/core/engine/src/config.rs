// projects/products/core/engine/src/config.rs
use std::{env, net::IpAddr, path::PathBuf};

use crate::CorsConfig;

#[derive(Debug)]
pub struct EngineConfig {
    pub host: IpAddr,
    pub port: u16,
    pub projects_dir: PathBuf,
    pub cors: CorsConfig,
    pub jwt_secret: String,
    pub allow_insecure_secret: bool,
}

impl EngineConfig {
    pub fn from_env() -> Result<Self, String> {
        let host = Self::env_ip("ENGINE_HOST", "127.0.0.1")?;
        let port = Self::env_u16("ENGINE_PORT", 3030)?;
        let projects_dir = Self::env_var("ENGINE_PROJECTS_DIR")
            .unwrap_or_else(|| "projects".to_string())
            .into();

        let cors_any = Self::env_var("ENGINE_CORS_ANY").as_deref() == Some("1");
        let cors_origin = Self::env_var("ENGINE_CORS_ORIGIN");

        if cors_any && cors_origin.is_some() {
            tracing::warn!(
                "CORS: both ENGINE_CORS_ANY=1 and ENGINE_CORS_ORIGIN are set. Using allow_any_origin."
            );
        }

        let cors = CorsConfig {
            allow_any_origin: cors_any,
            allow_origin: if cors_any { None } else { cors_origin },
        };

        let allow_insecure_secret =
            Self::env_var("ENGINE_ALLOW_INSECURE_SECRET").as_deref() == Some("1");

        let jwt_secret = match Self::env_var("ENGINE_JWT_SECRET") {
            Some(s) => s,
            None if allow_insecure_secret => {
                tracing::warn!(
                    "ENGINE_JWT_SECRET not set, using default (INSECURE!). Set ENGINE_JWT_SECRET in production."
                );
                "CHANGE_ME_CHANGE_ME_CHANGE_ME_32CHARS_MIN!!".to_string()
            }
            None => return Err("ENGINE_JWT_SECRET is required".to_string()),
        };

        Ok(Self {
            host,
            port,
            projects_dir,
            cors,
            jwt_secret,
            allow_insecure_secret,
        })
    }

    pub fn env_var(key: &str) -> Option<String> {
        env::var(key)
            .ok()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
    }

    pub fn env_u16(key: &str, default: u16) -> Result<u16, String> {
        Ok(Self::env_var(key)
            .and_then(|v| v.parse::<u16>().ok())
            .unwrap_or(default))
    }

    pub fn env_ip(key: &str, default: &str) -> Result<IpAddr, String> {
        Ok(Self::env_var(key)
            .and_then(|v| v.parse::<IpAddr>().ok())
            .unwrap_or_else(|| default.parse::<IpAddr>().expect("default ip invalid")))
    }

}
