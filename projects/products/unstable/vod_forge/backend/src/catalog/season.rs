use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use crate::catalog::episode::Episode;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Season {
    pub number: u32,
    pub episodes: BTreeMap<u32, Episode>,
}
