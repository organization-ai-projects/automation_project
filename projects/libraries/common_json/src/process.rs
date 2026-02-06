// projects/libraries/common_json/src/process.rs
use crate::json_error::{self, JsonResult};
use crate::json_error_code::JsonErrorCode;
use crate::{Json, from_json_str};

/// Parse JSON from a process stdout buffer and return contextual errors.
pub fn parse_json_stdout(output: &std::process::Output, input: &str) -> JsonResult<Json> {
    let stdout = String::from_utf8_lossy(&output.stdout);
    from_json_str(&stdout).map_err(|err| {
        json_error::JsonError::new(JsonErrorCode::ParseError).context(format!(
            "stdout is not valid JSON: {err}\nstatus: {:?}\nstdout:\n{}\nstderr:\n{}\ninput:\n{}",
            output.status.code(),
            stdout,
            String::from_utf8_lossy(&output.stderr),
            input
        ))
    })
}
