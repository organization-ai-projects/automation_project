use std::io::{BufRead, Write};

use crate::diagnostics::error::Error;
use crate::protocol::request::Request;
use crate::protocol::response::Response;

pub struct JsonCodec;

impl JsonCodec {
    pub fn read_request(reader: &std::io::Stdin) -> Result<Request, Error> {
        let mut line = String::new();
        let bytes_read = reader.lock().read_line(&mut line)?;
        if bytes_read == 0 {
            return Err(Error::EndOfInput);
        }
        let request: Request = common_json::from_str(line.trim())
            .map_err(|e| Error::Serialization(e.to_string()))?;
        Ok(request)
    }

    pub fn write_response(writer: &std::io::Stdout, response: &Response) -> Result<(), Error> {
        let json =
            common_json::to_string(response).map_err(|e| Error::Serialization(e.to_string()))?;
        let mut handle = writer.lock();
        writeln!(handle, "{json}")?;
        handle.flush()?;
        Ok(())
    }
}
