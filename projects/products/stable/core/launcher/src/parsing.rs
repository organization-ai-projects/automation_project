// projects/products/core/launcher/src/parsing.rs
use std::collections::HashSet;

pub(crate) fn parse_csv(v: &Option<String>) -> Option<HashSet<String>> {
    v.as_ref().map(|s| {
        s.split(',')
            .map(|x| x.trim().to_string())
            .filter(|x| !x.is_empty())
            .collect()
    })
}
