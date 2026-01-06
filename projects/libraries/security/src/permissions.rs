use crate::role::Role;

pub fn has_permission(role: &Role, action: &str) -> bool {
    match role {
        Role::Admin => true,
        Role::Moderator => action != "delete",
        Role::User => action == "read" || action == "write",
        Role::Guest => action == "read",
    }
}
