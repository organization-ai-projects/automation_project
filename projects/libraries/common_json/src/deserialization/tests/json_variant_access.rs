// projects/libraries/common_json/src/deserialization/tests/json_variant_access.rs
use crate::Json;
use crate::deserialization::json_variant_access::*;
use serde::de::VariantAccess;
use std::marker::PhantomData;

#[test]
fn test_json_variant_access_unit_variant() {
    let variant = JsonVariantAccess {
        value: Some(&Json::Null),
    };
    assert!(variant.unit_variant().is_ok());
}

#[test]
fn test_json_variant_access_newtype_variant() {
    let variant = JsonVariantAccess {
        value: Some(&Json::Null),
    };
    let seed = PhantomData::<()>; // Using a valid type
    assert!(variant.newtype_variant_seed(seed).is_ok());
}
