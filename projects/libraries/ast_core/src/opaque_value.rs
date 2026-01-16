use crate::ExtId;

/// An opaque/extension value for custom data.
#[derive(Clone, Debug, PartialEq)]
pub struct OpaqueValue {
    pub kind: ExtId,
    pub bytes: Vec<u8>,
}
