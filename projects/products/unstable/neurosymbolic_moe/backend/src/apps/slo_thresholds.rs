//! projects/products/unstable/neurosymbolic_moe/backend/src/apps/slo_thresholds.rs
use crate::apps::DynError;

#[derive(Debug, Clone)]
pub(crate) struct SloThresholds {
    pub(crate) profile: String,
    pub(crate) runtime_min_successes: u64,
    pub(crate) runtime_max_rejections: u64,
    pub(crate) runtime_max_parse_failures: u64,
    pub(crate) concurrent_max_contention_rate: f64,
    pub(crate) concurrent_max_timeout_rate: f64,
    pub(crate) concurrent_min_successes: u64,
    pub(crate) concurrent_max_rejections: u64,
    pub(crate) concurrent_max_parse_failures: u64,
}

impl Default for SloThresholds {
    fn default() -> Self {
        Self {
            profile: "balanced".to_string(),
            runtime_min_successes: 1,
            runtime_max_rejections: 0,
            runtime_max_parse_failures: 0,
            concurrent_max_contention_rate: 1.0,
            concurrent_max_timeout_rate: 0.2,
            concurrent_min_successes: 1,
            concurrent_max_rejections: 0,
            concurrent_max_parse_failures: 0,
        }
    }
}

impl SloThresholds {
    pub(crate) fn profile_name(&self) -> &str {
        self.profile.as_str()
    }

    pub(crate) fn parse_args(args: &[String]) -> Result<Self, DynError> {
        let mut out = Self::default();
        let mut idx = 0_usize;
        while idx < args.len() {
            let key = &args[idx];
            if !key.starts_with("--") {
                return Err(std::io::Error::other(format!("unknown argument: {key}")).into());
            }
            let value = args
                .get(idx + 1)
                .ok_or_else(|| std::io::Error::other(format!("missing value for {key}")))?;
            match key.as_str() {
                "--profile" => out = Self::from_profile(value)?,
                "--runtime-min-successes" => out.runtime_min_successes = value.parse()?,
                "--runtime-max-rejections" => out.runtime_max_rejections = value.parse()?,
                "--runtime-max-parse-failures" => out.runtime_max_parse_failures = value.parse()?,
                "--concurrent-max-contention-rate" => {
                    out.concurrent_max_contention_rate = value.parse()?
                }
                "--concurrent-max-timeout-rate" => {
                    out.concurrent_max_timeout_rate = value.parse()?
                }
                "--concurrent-min-successes" => out.concurrent_min_successes = value.parse()?,
                "--concurrent-max-rejections" => out.concurrent_max_rejections = value.parse()?,
                "--concurrent-max-parse-failures" => {
                    out.concurrent_max_parse_failures = value.parse()?
                }
                _ => return Err(std::io::Error::other(format!("unknown flag: {key}")).into()),
            }
            idx += 2;
        }
        Ok(out)
    }

    fn from_profile(profile: &str) -> Result<Self, DynError> {
        let out = match profile {
            "strict" => Self {
                profile: "strict".to_string(),
                runtime_min_successes: 2,
                runtime_max_rejections: 0,
                runtime_max_parse_failures: 0,
                concurrent_max_contention_rate: 0.5,
                concurrent_max_timeout_rate: 0.05,
                concurrent_min_successes: 2,
                concurrent_max_rejections: 0,
                concurrent_max_parse_failures: 0,
            },
            "balanced" => Self::default(),
            "exploratory" => Self {
                profile: "exploratory".to_string(),
                runtime_min_successes: 0,
                runtime_max_rejections: 5,
                runtime_max_parse_failures: 5,
                concurrent_max_contention_rate: 2.0,
                concurrent_max_timeout_rate: 0.8,
                concurrent_min_successes: 0,
                concurrent_max_rejections: 5,
                concurrent_max_parse_failures: 5,
            },
            _ => {
                return Err(std::io::Error::other(format!(
                    "unknown SLO profile: {profile} (expected strict|balanced|exploratory)"
                ))
                .into());
            }
        };
        Ok(out)
    }
}
