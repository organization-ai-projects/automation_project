use serde::Serialize;

#[derive(Serialize)]
pub struct IpcEnvelope<'a, T: Serialize> {
    pub id: u64,
    pub payload: &'a T,
}
