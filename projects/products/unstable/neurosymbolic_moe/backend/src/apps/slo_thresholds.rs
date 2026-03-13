//! projects/products/unstable/neurosymbolic_moe/backend/src/apps/slo_thresholds.rs
use crate::apps::DynError;

#[derive(Debug, Clone)]
pub(crate) struct SloThresholds {
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
}
