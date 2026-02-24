// projects/products/unstable/platform_versioning/backend/src/issues/issue_visibility.rs
/// Controls which issues a user may list or view.
///
/// Used by the backend to filter issue queries before returning results.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IssueVisibility {
    /// The user may see all issues regardless of assignment (admin mode).
    All,
    /// The user may only see issues they are assigned to or that have been
    /// explicitly shared with them.
    AssignedOrShared,
}

impl IssueVisibility {
    /// Returns the visibility level appropriate for a given `is_admin` flag.
    pub fn for_role(is_admin: bool) -> Self {
        if is_admin {
            Self::All
        } else {
            Self::AssignedOrShared
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn admin_gets_all() {
        assert_eq!(IssueVisibility::for_role(true), IssueVisibility::All);
    }

    #[test]
    fn non_admin_gets_assigned_or_shared() {
        assert_eq!(
            IssueVisibility::for_role(false),
            IssueVisibility::AssignedOrShared
        );
    }
}
