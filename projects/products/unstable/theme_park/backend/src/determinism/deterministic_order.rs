#![allow(dead_code)]

/// Stable ordering helpers — no HashMap iteration dependence.
pub struct DeterministicOrder;

impl DeterministicOrder {
    /// Sort a slice of items by a computed key, stable across builds.
    pub fn sort_by_key<T, K: Ord>(items: &mut [T], key: impl Fn(&T) -> K) {
        items.sort_by_key(key);
    }

    /// Sort a slice whose elements are `Ord` in place.
    pub fn sort<T: Ord>(items: &mut [T]) {
        items.sort();
    }

    /// Collect a BTreeMap into a deterministically ordered vec of values.
    pub fn btree_values<K: Ord + Clone, V: Clone>(
        map: &std::collections::BTreeMap<K, V>,
    ) -> Vec<V> {
        map.values().cloned().collect()
    }

    /// Collect a BTreeMap into a deterministically ordered vec of (key, value) pairs.
    pub fn btree_pairs<K: Ord + Clone, V: Clone>(
        map: &std::collections::BTreeMap<K, V>,
    ) -> Vec<(K, V)> {
        map.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
    }
}
