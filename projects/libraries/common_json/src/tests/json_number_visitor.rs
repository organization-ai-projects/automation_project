// projects/libraries/common_json/src/tests/json_number_visitor.rs
use crate::json_number::JsonNumber;
use crate::json_number_visitor::JsonNumberVisitor;
use serde::de::Visitor;
use serde::de::value::Error as SerdeError;

#[test]
fn test_visit_i64() {
    let visitor = JsonNumberVisitor;
    match visitor.visit_i64::<SerdeError>(42i64) {
        Ok(result) => assert_eq!(result, JsonNumber::from(42i64)),
        Err(e) => panic!("Erreur lors de la visite de i64 : {:?}", e),
    }
}

#[test]
fn test_visit_u64() {
    let visitor = JsonNumberVisitor;
    match visitor.visit_u64::<SerdeError>(42u64) {
        Ok(result) => assert_eq!(result, JsonNumber::from(42u64)),
        Err(e) => panic!("Erreur lors de la visite de u64 : {:?}", e),
    }
}

#[test]
fn test_visit_f64_valid() {
    let visitor = JsonNumberVisitor;
    match visitor.visit_f64::<SerdeError>(42.0) {
        Ok(result) => {
            let expected = JsonNumber::from_f64(42.0).expect("Valeur valide");
            assert_eq!(result, expected);
        }
        Err(e) => panic!("Erreur lors de la visite de f64 : {:?}", e),
    }
}

#[test]
fn test_visit_f64_invalid() {
    let visitor = JsonNumberVisitor;
    let result = visitor.visit_f64::<SerdeError>(f64::INFINITY);
    assert!(result.is_err());
}
