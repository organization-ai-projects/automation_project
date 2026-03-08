#![cfg(test)]

use crate::protocol::stdout_writer::StdoutWriter;

#[test]
fn test_stdout_writer_exists() {
    let writer_type = std::mem::size_of::<StdoutWriter>();
    assert_eq!(writer_type, 0);
}
