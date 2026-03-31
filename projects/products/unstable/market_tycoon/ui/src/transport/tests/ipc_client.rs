use crate::transport::ipc_client::IpcError;

#[test]
fn ipc_error_display() {
    let e = IpcError::Process("spawn failed".into());
    assert_eq!(e.to_string(), "process error: spawn failed");
}

#[test]
fn ipc_error_io() {
    let e = IpcError::Io("broken pipe".into());
    assert_eq!(e.to_string(), "io error: broken pipe");
}
