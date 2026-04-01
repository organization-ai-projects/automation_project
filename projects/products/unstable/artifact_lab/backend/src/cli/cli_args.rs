use crate::diagnostics::Error;

#[derive(Debug)]
pub enum CliCommand {
    Pack { root: String, out: String },
    Unpack { bundle: String, out: String },
    Verify { bundle: String, json: bool },
}

pub struct CliParser;

impl CliParser {
    pub fn parse(args: &[String]) -> Result<CliCommand, Error> {
        let command = args.first().map(String::as_str).ok_or_else(|| {
            Error::InvalidUsage("no command given (expected: pack, unpack, verify)".to_string())
        })?;

        match command {
            "pack" => Self::parse_pack(&args[1..]),
            "unpack" => Self::parse_unpack(&args[1..]),
            "verify" => Self::parse_verify(&args[1..]),
            other => Err(Error::InvalidUsage(format!(
                "unknown command '{other}' (expected: pack, unpack, verify)"
            ))),
        }
    }

    fn parse_pack(args: &[String]) -> Result<CliCommand, Error> {
        let mut root: Option<String> = None;
        let mut out: Option<String> = None;
        let mut i = 0;

        while i < args.len() {
            match args[i].as_str() {
                "--root" => {
                    i += 1;
                    root = Some(
                        args.get(i)
                            .ok_or_else(|| {
                                Error::InvalidUsage("--root requires a value".to_string())
                            })?
                            .clone(),
                    );
                }
                "--out" => {
                    i += 1;
                    out = Some(
                        args.get(i)
                            .ok_or_else(|| {
                                Error::InvalidUsage("--out requires a value".to_string())
                            })?
                            .clone(),
                    );
                }
                other => {
                    return Err(Error::InvalidUsage(format!(
                        "unknown flag '{other}' for pack"
                    )));
                }
            }
            i += 1;
        }

        Ok(CliCommand::Pack {
            root: root.ok_or_else(|| Error::InvalidUsage("pack requires --root".to_string()))?,
            out: out.ok_or_else(|| Error::InvalidUsage("pack requires --out".to_string()))?,
        })
    }

    fn parse_unpack(args: &[String]) -> Result<CliCommand, Error> {
        let mut bundle: Option<String> = None;
        let mut out: Option<String> = None;
        let mut i = 0;

        while i < args.len() {
            match args[i].as_str() {
                "--bundle" => {
                    i += 1;
                    bundle = Some(
                        args.get(i)
                            .ok_or_else(|| {
                                Error::InvalidUsage("--bundle requires a value".to_string())
                            })?
                            .clone(),
                    );
                }
                "--out" => {
                    i += 1;
                    out = Some(
                        args.get(i)
                            .ok_or_else(|| {
                                Error::InvalidUsage("--out requires a value".to_string())
                            })?
                            .clone(),
                    );
                }
                other => {
                    return Err(Error::InvalidUsage(format!(
                        "unknown flag '{other}' for unpack"
                    )));
                }
            }
            i += 1;
        }

        Ok(CliCommand::Unpack {
            bundle: bundle
                .ok_or_else(|| Error::InvalidUsage("unpack requires --bundle".to_string()))?,
            out: out.ok_or_else(|| Error::InvalidUsage("unpack requires --out".to_string()))?,
        })
    }

    fn parse_verify(args: &[String]) -> Result<CliCommand, Error> {
        let mut bundle: Option<String> = None;
        let mut json = false;
        let mut i = 0;

        while i < args.len() {
            match args[i].as_str() {
                "--bundle" => {
                    i += 1;
                    bundle = Some(
                        args.get(i)
                            .ok_or_else(|| {
                                Error::InvalidUsage("--bundle requires a value".to_string())
                            })?
                            .clone(),
                    );
                }
                "--json" => {
                    json = true;
                }
                other => {
                    return Err(Error::InvalidUsage(format!(
                        "unknown flag '{other}' for verify"
                    )));
                }
            }
            i += 1;
        }

        Ok(CliCommand::Verify {
            bundle: bundle
                .ok_or_else(|| Error::InvalidUsage("verify requires --bundle".to_string()))?,
            json,
        })
    }
}
