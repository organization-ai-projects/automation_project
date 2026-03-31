mod ipc_client;
mod ipc_error;

#[cfg(test)]
mod tests;

pub(crate) use ipc_client::IpcClient;
pub(crate) use ipc_error::IpcError;
