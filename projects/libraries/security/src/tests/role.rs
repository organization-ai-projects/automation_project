use std::str::FromStr;

use crate::{Permission, Role};

#[test]
fn test_admin_has_all_permissions() {
    let admin = Role::Admin;
    for &permission in Permission::all() {
        assert!(
            admin.has_permission(permission),
            "Admin should have {:?} permission",
            permission
        );
    }
}

#[test]
fn test_guest_only_read() {
    let guest = Role::Guest;
    assert!(guest.has_permission(Permission::Read));
    assert!(!guest.has_permission(Permission::Write));
    assert!(!guest.has_permission(Permission::Execute));
    assert!(!guest.has_permission(Permission::Delete));
    assert!(!guest.has_permission(Permission::Admin));
}

#[test]
fn test_moderator_cannot_delete_or_admin() {
    let moderator = Role::Moderator;
    assert!(moderator.has_permission(Permission::Read));
    assert!(moderator.has_permission(Permission::Write));
    assert!(moderator.has_permission(Permission::Execute));
    assert!(!moderator.has_permission(Permission::Delete));
    assert!(!moderator.has_permission(Permission::Admin));
    assert!(!moderator.has_permission(Permission::ConfigureSystem));
}

#[test]
fn test_user_basic_permissions() {
    let user = Role::User;
    assert!(user.has_permission(Permission::Read));
    assert!(user.has_permission(Permission::Write));
    assert!(user.has_permission(Permission::Execute));
    assert!(!user.has_permission(Permission::Delete));
    assert!(!user.has_permission(Permission::Train));
}

#[test]
fn test_privilege_levels() {
    assert!(Role::Admin.privilege_level() > Role::Moderator.privilege_level());
    assert!(Role::Moderator.privilege_level() > Role::User.privilege_level());
    assert!(Role::User.privilege_level() > Role::Guest.privilege_level());
}

#[test]
fn test_higher_privilege() {
    assert!(Role::Admin.has_higher_privilege_than(&Role::User));
    assert!(Role::Moderator.has_higher_privilege_than(&Role::Guest));
    assert!(!Role::Guest.has_higher_privilege_than(&Role::User));
    assert!(!Role::User.has_higher_privilege_than(&Role::Admin));
}

#[test]
fn test_role_from_str() {
    assert_eq!(Role::from_str("admin"), Ok(Role::Admin));
    assert_eq!(Role::from_str("ADMIN"), Ok(Role::Admin));
    assert_eq!(Role::from_str("moderator"), Ok(Role::Moderator));
    assert_eq!(Role::from_str("mod"), Ok(Role::Moderator));
    assert_eq!(Role::from_str("user"), Ok(Role::User));
    assert_eq!(Role::from_str("guest"), Ok(Role::Guest));
    assert!(Role::from_str("invalid").is_err());
}

#[test]
fn test_permission_from_str() {
    assert_eq!(Permission::from_str("read"), Ok(Permission::Read));
    assert_eq!(Permission::from_str("READ"), Ok(Permission::Read));
    assert_eq!(Permission::from_str("write"), Ok(Permission::Write));
    assert_eq!(Permission::from_str("view_logs"), Ok(Permission::ViewLogs));
    assert_eq!(Permission::from_str("viewlogs"), Ok(Permission::ViewLogs));
    assert!(Permission::from_str("invalid").is_err());
}
