// projects/products/core/launcher/src/service.rs
use std::collections::{HashMap, HashSet, VecDeque};

use serde::Deserialize;
use thiserror::Error;

use crate::{RestartPolicy, default_backoff, default_restart};

#[derive(Debug, Deserialize, Clone)]
pub struct Service {
    pub name: String,
    pub bin: String,

    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub env: Vec<String>,
    #[serde(default)]
    pub cwd: Option<String>,

    #[serde(default)]
    pub depends_on: Vec<String>,

    #[serde(default = "default_restart")]
    pub restart: RestartPolicy,
    #[serde(default)]
    pub restart_max: u32, // 0 = infinite
    #[serde(default = "default_backoff")]
    pub restart_backoff_ms: u64,

    #[serde(default)]
    pub ready_http: Option<String>,
}

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("service with empty name")]
    EmptyServiceName,
    #[error("duplicate service name: {0}")]
    DuplicateServiceName(String),
    #[error("service `{0}` depends on unknown service `{1}`")]
    UnknownServiceDependency(String, String),
    #[error("dependency cycle detected in services (topo sort failed)")]
    DependencyCycle,
}

pub fn validate_services(services: &[Service]) -> Result<(), ServiceError> {
    let mut names = HashSet::new();
    for s in services {
        if s.name.trim().is_empty() {
            return Err(ServiceError::EmptyServiceName);
        }
        if !names.insert(s.name.clone()) {
            return Err(ServiceError::DuplicateServiceName(s.name.clone()));
        }
    }
    // deps exist
    let set: HashSet<_> = services.iter().map(|s| s.name.as_str()).collect();
    for s in services {
        for d in &s.depends_on {
            if !set.contains(d.as_str()) {
                return Err(ServiceError::UnknownServiceDependency(
                    s.name.clone(),
                    d.clone(),
                ));
            }
        }
    }
    Ok(())
}

/// Performs a topological sort on the given services based on their dependencies.
/// Returns a sorted vector of service names or an error if a cycle is detected.
pub fn topo_sort(services: &[Service]) -> Result<Vec<String>, ServiceError> {
    let mut indeg: HashMap<String, usize> = HashMap::new();
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for s in services {
        indeg.entry(s.name.clone()).or_insert(0);
        for d in &s.depends_on {
            graph.entry(d.clone()).or_default().push(s.name.clone());
            *indeg.entry(s.name.clone()).or_insert(0) += 1;
        }
    }

    let mut q: VecDeque<String> = indeg
        .iter()
        .filter(|(_, v)| **v == 0)
        .map(|(k, _)| k.clone())
        .collect();

    let mut out = Vec::new();
    while let Some(n) = q.pop_front() {
        out.push(n.clone());
        if let Some(nexts) = graph.get(&n) {
            for m in nexts {
                let e = indeg.get_mut(m).ok_or(ServiceError::DependencyCycle)?;
                *e -= 1;
                if *e == 0 {
                    q.push_back(m.clone());
                }
            }
        }
    }

    if out.len() != services.len() {
        return Err(ServiceError::DependencyCycle);
    }
    Ok(out)
}
