// projects/libraries/common_json/src/deserialization/tests/json_map_access.rs
use crate::Json;
use crate::deserialization::json_map_access::*;
use serde::de::MapAccess;
use std::collections::HashMap;
use std::marker::PhantomData;

#[test]
fn test_json_map_access_valid() {
    let key1 = "key1".to_string();
    let key2 = "key2".to_string();
    let value1 = Json::Null;
    let value2 = Json::Null;
    let data = vec![(&key1, &value1), (&key2, &value2)];
    let mut map = JsonMapAccess::new(data.into_iter());
    map.next_key_seed(PhantomData::<()>).ok(); // Initialisation de `self.value`
    let seed = PhantomData::<String>;
    let result = map.next_key_seed(seed);
    assert!(result.is_ok());
}

#[test]
fn test_json_map_access_empty() {
    let data: HashMap<String, Json> = HashMap::new();
    let mut map = JsonMapAccess::new(data.iter());
    let seed = PhantomData::<String>;
    let result = map.next_key_seed(seed);
    assert!(result.is_ok());
}
