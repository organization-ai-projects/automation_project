use crate::diagnostics::GraphvizorError;

/// Configuration parsed from CLI args for the render command.
pub struct RenderConfig {
    pub input_path: std::path::PathBuf,
    pub output_path: std::path::PathBuf,
    pub layout: String,
}

impl RenderConfig {
    pub fn parse(args: &[String]) -> Result<Self, GraphvizorError> {
        let mut input_path = None;
        let mut output_path = None;
        let mut layout = String::from("layered");

        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--in" => {
                    i += 1;
                    input_path = Some(std::path::PathBuf::from(
                        args.get(i).ok_or_else(|| {
                            GraphvizorError::InvalidConfig("missing --in value".to_string())
                        })?,
                    ));
                }
                "--out" => {
                    i += 1;
                    output_path = Some(std::path::PathBuf::from(
                        args.get(i).ok_or_else(|| {
                            GraphvizorError::InvalidConfig("missing --out value".to_string())
                        })?,
                    ));
                }
                "--layout" => {
                    i += 1;
                    layout = args
                        .get(i)
                        .ok_or_else(|| {
                            GraphvizorError::InvalidConfig("missing --layout value".to_string())
                        })?
                        .clone();
                }
                _ => {}
            }
            i += 1;
        }

        Ok(Self {
            input_path: input_path.ok_or_else(|| {
                GraphvizorError::InvalidConfig("--in is required".to_string())
            })?,
            output_path: output_path.ok_or_else(|| {
                GraphvizorError::InvalidConfig("--out is required".to_string())
            })?,
            layout,
        })
    }
}
