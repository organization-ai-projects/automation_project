use crate::diagnostics::engine_error::EngineError;
use crate::io::binary_codec;
use crate::io::ron_codec;
use crate::report::sim_report::SimReport;
use std::path::PathBuf;

pub struct SaveController;

impl SaveController {
    pub fn load_and_convert(args: &[String]) -> Result<String, EngineError> {
        let mut input_bin: Option<PathBuf> = None;
        let mut input_ron: Option<PathBuf> = None;
        let mut output_bin: Option<PathBuf> = None;
        let mut output_ron: Option<PathBuf> = None;

        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--input-bin" => {
                    i += 1;
                    if let Some(v) = args.get(i) {
                        input_bin = Some(PathBuf::from(v));
                    }
                }
                "--input-ron" => {
                    i += 1;
                    if let Some(v) = args.get(i) {
                        input_ron = Some(PathBuf::from(v));
                    }
                }
                "--output-bin" => {
                    i += 1;
                    if let Some(v) = args.get(i) {
                        output_bin = Some(PathBuf::from(v));
                    }
                }
                "--output-ron" => {
                    i += 1;
                    if let Some(v) = args.get(i) {
                        output_ron = Some(PathBuf::from(v));
                    }
                }
                _ => {}
            }
            i += 1;
        }

        let report: SimReport = if let Some(bin_path) = input_bin {
            binary_codec::load_binary(&bin_path)?
        } else if let Some(ron_path) = input_ron {
            ron_codec::load_ron(&ron_path)?
        } else {
            return Err(EngineError::Sim(
                "--input-bin or --input-ron required".to_string(),
            ));
        };

        if let Some(bin_path) = output_bin {
            binary_codec::save_binary(&report, &bin_path)?;
        }
        if let Some(ron_path) = output_ron {
            ron_codec::save_ron(&report, &ron_path)?;
        }

        Ok("Conversion complete.".to_string())
    }
}
