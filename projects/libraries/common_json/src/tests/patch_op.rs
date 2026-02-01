// projects/libraries/common_json/src/tests/patch_op.rs
use crate::Json;
use crate::patch_op::*;

#[test]
fn test_patch_op_add() {
    let op = PatchOp::Add {
        path: "/path".to_string(),
        value: Json::String("value".to_string()),
    };
    if let PatchOp::Add { path, value } = op {
        assert_eq!(path, "/path");
        assert_eq!(value, Json::String("value".to_string()));
    } else {
        panic!("Expected Add operation");
    }
}

#[test]
fn test_patch_op_remove() {
    let op = PatchOp::Remove {
        path: "/path".to_string(),
    };
    if let PatchOp::Remove { path } = op {
        assert_eq!(path, "/path");
    } else {
        panic!("Expected Remove operation");
    }
}

#[test]
fn test_patch_op_replace() {
    let op = PatchOp::Replace {
        path: "/path".to_string(),
        value: Json::String("new_value".to_string()),
    };
    if let PatchOp::Replace { path, value } = op {
        assert_eq!(path, "/path");
        assert_eq!(value, Json::String("new_value".to_string()));
    } else {
        panic!("Expected Replace operation");
    }
}

#[test]
fn test_patch_op_debug() {
    let op = PatchOp::Add {
        path: "/debug_path".to_string(),
        value: Json::String("debug_value".to_string()),
    };
    assert_eq!(
        format!("{:?}", op),
        "Add { path: \"/debug_path\", value: String(\"debug_value\") }"
    );
}
