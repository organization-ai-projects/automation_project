// projects/products/core/launcher/src/supervisor/log_stream.rs

#[derive(Debug, Clone, Copy)]
pub(crate) enum LogStream {
    Stdout,
    Stderr,
}
