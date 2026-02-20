// projects/libraries/common_json/src/merge_strategy.rs
/// Merge strategy for combining JSON values.
///
/// Determines how values are combined during a merge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MergeStrategy {
    /// Replaces the target with the source (no merging).
    #[default]
    Replace,
    /// Recursively merges objects, replaces other types.
    DeepMerge,
    /// Concatenates arrays, recursively merges objects.
    Concat,
}
