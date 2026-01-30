// projects/libraries/common_json/src/json_error_code.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum JsonErrorCode {
    Serialize,
    TypeMismatch,
    MissingField,
    IndexOutOfBounds,
    InvalidPath,
    UnexpectedNull,
    ParseError,
    UnsupportedOperation,
    Io,
    Custom,
    InvalidInteger,
    InvalidByteValue,
    MissingEnumValueError,
    ValueSerializedBeforeKey,
    FieldNotFound,
    ExpectedSingleCharacter,
    ValueIsMissing,
}
