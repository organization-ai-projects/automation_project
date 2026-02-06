// projects/products/stable/core/launcher/src/service.rs
use std::collections::{HashMap, HashSet, VecDeque};

use anyhow::{Result, bail};
use serde::Deserialize;

use crate::restart_policy::{RestartPolicy, default_backoff, default_restart};

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Service {
    pub(crate) name: String,
    pub(crate) bin: String,

    #[serde(default)]
    pub(crate) args: Vec<String>,
    #[serde(default)]
    pub(crate) env: Vec<String>,
    #[serde(default)]
    pub(crate) cwd: Option<String>,

    #[serde(default)]
    pub(crate) depends_on: Vec<String>,

    #[serde(default = "default_restart")]
    pub(crate) restart: RestartPolicy,
    #[serde(default)]
    pub(crate) restart_max: u32, // 0 = infinite
    #[serde(default = "default_backoff")]
    pub(crate) restart_backoff_ms: u64,

    #[serde(default)]
    pub(crate) ready_http: Option<String>,
}

pub(crate) fn validate_services(services: &[Service]) -> Result<()> {
    let mut names = HashSet::new();
    for s in services {
        if s.name.trim().is_empty() {
            bail!("service with empty name");
        }
        if !names.insert(s.name.clone()) {
            bail!("duplicate service name: {}", s.name);
        }
    }
    // deps exist
    let set: HashSet<_> = services.iter().map(|s| s.name.as_str()).collect();
    for s in services {
        for d in &s.depends_on {
            if !set.contains(d.as_str()) {
                bail!("service `{}` depends on unknown service `{}`", s.name, d);
            }
        }
    }
    Ok(())
}

/// Performs a topological sort on the given services based on their dependencies.
/// Returns a sorted vector of service names or an error if a cycle is detected.
pub(crate) fn topo_sort(services: &[Service]) -> Result<Vec<String>> {
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
                if let Some(e) = indeg.get_mut(m) {
                    *e -= 1;
                    if *e == 0 {
                        q.push_back(m.clone());
                    }
                } else {
                    eprintln!("Key not found in indeg");
                }
            }
        }
    }

    if out.len() != services.len() {
        bail!("dependency cycle detected in services (topo sort failed)");
    }
    Ok(out)
}
