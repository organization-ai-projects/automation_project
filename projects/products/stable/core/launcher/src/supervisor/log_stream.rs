// projects/products/stable/core/launcher/src/supervisor/log_stream.rs

#[derive(Debug, Clone, Copy)]
pub(crate) enum LogStream {
    Stdout,
    Stderr,
}
