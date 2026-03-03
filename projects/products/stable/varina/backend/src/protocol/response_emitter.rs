use std::io::{self, Write};

pub fn emit_response(payload: &str) -> io::Result<()> {
    let mut stdout = io::stdout();
    stdout.write_all(payload.as_bytes())?;
    stdout.write_all(b"\n")?;
    stdout.flush()
}
