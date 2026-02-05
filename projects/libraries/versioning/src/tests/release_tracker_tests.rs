#[cfg(test)]
mod tests {
    use crate::release_id::ReleaseId;
    use crate::release_tracker::ReleaseTracker;
    use crate::revision_log::{ModificationCategory, ModificationEntry};
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn can_initialize_tracker() {
        let tracker = ReleaseTracker::initialize("MyProject".to_string());
        assert_eq!(tracker.active_release(), &ReleaseId::initial());
        assert_eq!(tracker.log().get_project_title(), "MyProject");
    }

    #[test]
    fn initial_tracker_has_first_entry() {
        let tracker = ReleaseTracker::initialize("MyProject".to_string());
        let entries = tracker.log().get_entries();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].get_release(), &ReleaseId::initial());
    }

    #[test]
    fn can_register_major_release() {
        let mut tracker = ReleaseTracker::initialize("MyProject".to_string());
        
        let mods = vec![ModificationEntry::create(
            "Complete API redesign".to_string(),
            ModificationCategory::BreakingModification,
        )];
        
        tracker.register_major_release(mods, vec!["Developer A".to_string()]);
        
        assert_eq!(tracker.active_release().first_tier(), 2);
        assert_eq!(tracker.active_release().second_tier(), 0);
        assert_eq!(tracker.active_release().third_tier(), 0);
    }

    #[test]
    fn can_register_feature_release() {
        let mut tracker = ReleaseTracker::initialize("MyProject".to_string());
        
        let mods = vec![ModificationEntry::create(
            "Add user dashboard".to_string(),
            ModificationCategory::NewCapability,
        )];
        
        tracker.register_feature_release(mods, vec!["Developer B".to_string()]);
        
        assert_eq!(tracker.active_release().first_tier(), 1);
        assert_eq!(tracker.active_release().second_tier(), 1);
        assert_eq!(tracker.active_release().third_tier(), 0);
    }

    #[test]
    fn can_register_correction_release() {
        let mut tracker = ReleaseTracker::initialize("MyProject".to_string());
        
        let mods = vec![ModificationEntry::create(
            "Fix login issue".to_string(),
            ModificationCategory::CorrectionApplied,
        )];
        
        tracker.register_correction_release(mods, vec!["Developer C".to_string()]);
        
        assert_eq!(tracker.active_release().first_tier(), 1);
        assert_eq!(tracker.active_release().second_tier(), 0);
        assert_eq!(tracker.active_release().third_tier(), 1);
    }

    #[test]
    fn multiple_releases_tracked() {
        let mut tracker = ReleaseTracker::initialize("MyProject".to_string());
        
        tracker.register_feature_release(
            vec![ModificationEntry::create("Feature 1".to_string(), ModificationCategory::NewCapability)],
            vec![],
        );
        tracker.register_feature_release(
            vec![ModificationEntry::create("Feature 2".to_string(), ModificationCategory::NewCapability)],
            vec![],
        );
        tracker.register_correction_release(
            vec![ModificationEntry::create("Bug fix".to_string(), ModificationCategory::CorrectionApplied)],
            vec![],
        );
        
        assert_eq!(tracker.active_release().to_string(), "1.2.1");
        assert_eq!(tracker.log().get_entries().len(), 4); // Initial + 3 new releases
    }

    #[test]
    fn can_persist_and_load_tracker() {
        let temp_path = PathBuf::from("/tmp/test_tracker.json");
        
        let mut tracker = ReleaseTracker::initialize("TestProject".to_string());
        tracker.register_feature_release(
            vec![ModificationEntry::create("New feature".to_string(), ModificationCategory::NewCapability)],
            vec!["Dev1".to_string()],
        );
        
        tracker.persist_to_file(&temp_path).unwrap();
        
        let loaded = ReleaseTracker::load_from_file(&temp_path).unwrap();
        assert_eq!(loaded.active_release(), tracker.active_release());
        assert_eq!(loaded.log().get_entries().len(), tracker.log().get_entries().len());
        
        // Cleanup
        let _ = fs::remove_file(temp_path);
    }

    #[test]
    fn loaded_tracker_preserves_history() {
        let temp_path = PathBuf::from("/tmp/test_tracker_history.json");
        
        let mut original = ReleaseTracker::initialize("TestProject".to_string());
        original.register_major_release(
            vec![ModificationEntry::create("Breaking change".to_string(), ModificationCategory::BreakingModification)],
            vec!["Alice".to_string()],
        );
        original.register_feature_release(
            vec![ModificationEntry::create("New feature".to_string(), ModificationCategory::NewCapability)],
            vec!["Bob".to_string()],
        );
        
        original.persist_to_file(&temp_path).unwrap();
        let loaded = ReleaseTracker::load_from_file(&temp_path).unwrap();
        
        assert_eq!(loaded.log().get_entries().len(), 3); // Initial + 2 new
        assert_eq!(loaded.active_release().to_string(), "2.1.0");
        
        // Cleanup
        let _ = fs::remove_file(temp_path);
    }
}
