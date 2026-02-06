// projects/products/stable/core/engine/src/config.rs
use std::{env, net::IpAddr, path::PathBuf};

use anyhow::bail;

use crate::CorsConfig;

#[derive(Debug)]
pub(crate) struct EngineConfig {
    pub(crate) host: IpAddr,
    pub(crate) port: u16,
    pub(crate) projects_dir: PathBuf,
    pub(crate) cors: CorsConfig,
    pub(crate) jwt_secret: String,
    pub(crate) allow_insecure_secret: bool,
}

impl EngineConfig {
    pub(crate) fn from_env() -> anyhow::Result<Self> {
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
            None => bail!("ENGINE_JWT_SECRET is required"),
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

    pub(crate) fn env_var(key: &str) -> Option<String> {
        env::var(key)
            .ok()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
    }

    pub(crate) fn env_u16(key: &str, default: u16) -> anyhow::Result<u16> {
        Ok(Self::env_var(key)
            .and_then(|v| v.parse::<u16>().ok())
            .unwrap_or(default))
    }

    pub(crate) fn env_ip(key: &str, default: &str) -> anyhow::Result<IpAddr> {
        Ok(Self::env_var(key)
            .and_then(|v| v.parse::<IpAddr>().ok())
            .unwrap_or_else(|| default.parse::<IpAddr>().expect("default ip invalid")))
    }
}
