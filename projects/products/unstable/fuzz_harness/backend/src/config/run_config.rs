use crate::diagnostics::FuzzHarnessError;
use std::path::PathBuf;

pub(crate) struct RunConfig {
    pub(crate) target_name: String,
    pub(crate) seed: u64,
    pub(crate) iterations: u64,
    pub(crate) json_output: bool,
    pub(crate) file_path: Option<PathBuf>,
    pub(crate) out_path: Option<PathBuf>,
}

impl RunConfig {
    pub(crate) fn parse_run(args: &[String]) -> Result<Self, FuzzHarnessError> {
        let mut target_name = None;
        let mut seed = 0u64;
        let mut iterations = 1000u64;
        let mut json_output = false;
        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--target" => {
                    i += 1;
                    target_name = Some(
                        args.get(i)
                            .ok_or_else(|| {
                                FuzzHarnessError::InvalidConfig(
                                    "missing --target value".to_string(),
                                )
                            })?
                            .clone(),
                    );
                }
                "--seed" => {
                    i += 1;
                    let val = args.get(i).ok_or_else(|| {
                        FuzzHarnessError::InvalidConfig("missing --seed value".to_string())
                    })?;
                    seed = val.parse().map_err(|_| {
                        FuzzHarnessError::InvalidConfig(format!("invalid seed: {val}"))
                    })?;
                }
                "--iters" => {
                    i += 1;
                    let val = args.get(i).ok_or_else(|| {
                        FuzzHarnessError::InvalidConfig("missing --iters value".to_string())
                    })?;
                    iterations = val.parse().map_err(|_| {
                        FuzzHarnessError::InvalidConfig(format!("invalid iters: {val}"))
                    })?;
                }
                "--json" => {
                    json_output = true;
                }
                _ => {}
            }
            i += 1;
        }
        let name = target_name.ok_or_else(|| {
            FuzzHarnessError::InvalidConfig("--target is required".to_string())
        })?;
        Ok(Self {
            target_name: name,
            seed,
            iterations,
            json_output,
            file_path: None,
            out_path: None,
        })
    }

    pub(crate) fn parse_replay(args: &[String]) -> Result<Self, FuzzHarnessError> {
        let mut file_path = None;
        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--target" => {
                    i += 1;
                    file_path = Some(std::path::PathBuf::from(
                        args.get(i)
                            .ok_or_else(|| {
                                FuzzHarnessError::InvalidConfig(
                                    "missing --target value".to_string(),
                                )
                            })?
                            .as_str(),
                    ));
                }
                _ => {}
            }
            i += 1;
        }
        Ok(Self {
            target_name: String::new(),
            seed: 0,
            iterations: 0,
            json_output: false,
            file_path,
            out_path: None,
        })
    }

    pub(crate) fn parse_shrink(args: &[String]) -> Result<Self, FuzzHarnessError> {
        let mut file_path = None;
        let mut out_path = None;
        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--target" => {
                    i += 1;
                    file_path = Some(std::path::PathBuf::from(
                        args.get(i)
                            .ok_or_else(|| {
                                FuzzHarnessError::InvalidConfig(
                                    "missing --target value".to_string(),
                                )
                            })?
                            .as_str(),
                    ));
                }
                "--out" => {
                    i += 1;
                    out_path = Some(std::path::PathBuf::from(
                        args.get(i)
                            .ok_or_else(|| {
                                FuzzHarnessError::InvalidConfig("missing --out value".to_string())
                            })?
                            .as_str(),
                    ));
                }
                _ => {}
            }
            i += 1;
        }
        Ok(Self {
            target_name: String::new(),
            seed: 0,
            iterations: 0,
            json_output: false,
            file_path,
            out_path,
        })
    }
}
