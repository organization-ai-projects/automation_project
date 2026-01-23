// projects/libraries/common_json/src/deserialization/helpers.rs
use crate::{
    Json,
    json_error::{JsonError, JsonErrorCode, JsonResult},
};

pub(crate) fn json_type_name(value: &Json) -> &'static str {
    match value {
        Json::Null => "null",
        Json::Bool(_) => "bool",
        Json::Number(_) => "number",
        Json::String(_) => "string",
        Json::Array(_) => "array",
        Json::Object(_) => "object",
    }
}

pub(crate) fn type_error(expected: &'static str, found: &Json) -> JsonError {
    JsonError::new(JsonErrorCode::TypeMismatch).context(format!(
        "Expected {}, found {}",
        expected,
        json_type_name(found)
    ))
}

pub(crate) fn to_i64(value: &Json) -> JsonResult<i64> {
    match value {
        Json::Number(number) => {
            let v = number.as_f64();
            if v.fract() == 0.0 && v >= i64::MIN as f64 && v <= i64::MAX as f64 {
                Ok(v as i64)
            } else {
                Err(JsonError::new(JsonErrorCode::InvalidInteger))
            }
        }
        other => Err(type_error("number", other)),
    }
}

pub(crate) fn to_u64(value: &Json) -> JsonResult<u64> {
    match value {
        Json::Number(number) => {
            let v = number.as_f64();
            if v.fract() == 0.0 && v >= 0.0 && v <= u64::MAX as f64 {
                Ok(v as u64)
            } else {
                Err(JsonError::new(JsonErrorCode::InvalidInteger))
            }
        }
        other => Err(type_error("number", other)),
    }
}

pub(crate) fn to_f64(value: &Json) -> JsonResult<f64> {
    match value {
        Json::Number(number) => Ok(number.as_f64()),
        other => Err(type_error("number", other)),
    }
}

pub(crate) fn to_bytes(value: &Json) -> JsonResult<Vec<u8>> {
    match value {
        Json::Array(values) => {
            let mut buffer = Vec::with_capacity(values.len());
            for item in values {
                let number = match item {
                    Json::Number(number) => number.as_f64(),
                    other => return Err(type_error("number", other)),
                };
                if number.fract() != 0.0 || number < 0.0 || number > u8::MAX as f64 {
                    return Err(JsonError::new(JsonErrorCode::InvalidByteValue));
                }
                buffer.push(number as u8);
            }
            Ok(buffer)
        }
        other => Err(type_error("array", other)),
    }
}
