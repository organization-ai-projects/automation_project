// projects/products/unstable/autonomous_dev_ai/src/persistence/utils.rs
use std::collections::HashMap;
use std::{
    fs, io,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

pub(crate) fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

pub(crate) fn top_entry_key(map: &HashMap<String, usize>) -> Option<String> {
    map.iter()
        .max_by_key(|(_, v)| *v)
        .map(|(k, v)| format!("{k}:{v}"))
}

pub(crate) fn ensure_parent_dir_exists(path: &str) -> io::Result<()> {
    let p = Path::new(path);
    if let Some(parent) = p.parent()
        && !parent.as_os_str().is_empty()
    {
        fs::create_dir_all(parent)?;
    }
    Ok(())
}
