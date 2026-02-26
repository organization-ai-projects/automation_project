// projects/products/unstable/autonomy_orchestrator_ai/src/validation_invocation_parser.rs
use crate::cli_value_parsers::parse_env_pair_cli;
use crate::pending_validation_invocation::PendingValidationInvocation;

pub fn parse_validation_pending_invocations(
    raw_args: &[String],
) -> Result<Vec<PendingValidationInvocation>, String> {
    let mut result: Vec<PendingValidationInvocation> = Vec::new();
    let mut i = 0usize;
    while i < raw_args.len() {
        match raw_args[i].as_str() {
            "--validation-bin" => {
                if i + 1 >= raw_args.len() {
                    return Err("--validation-bin requires a value".to_string());
                }
                result.push(PendingValidationInvocation {
                    command: raw_args[i + 1].clone(),
                    args: Vec::new(),
                    env: Vec::new(),
                });
                i += 2;
            }
            "--validation-arg" => {
                if i + 1 >= raw_args.len() {
                    return Err("--validation-arg requires a value".to_string());
                }
                let Some(last) = result.last_mut() else {
                    return Err(
                        "--validation-arg requires a preceding --validation-bin".to_string()
                    );
                };
                last.args.push(raw_args[i + 1].clone());
                i += 2;
            }
            "--validation-env" => {
                if i + 1 >= raw_args.len() {
                    return Err("--validation-env requires a value".to_string());
                }
                let Some(last) = result.last_mut() else {
                    return Err(
                        "--validation-env requires a preceding --validation-bin".to_string()
                    );
                };
                let env_pair = parse_env_pair_cli(&raw_args[i + 1])?;
                last.env.push(env_pair);
                i += 2;
            }
            _ => i += 1,
        }
    }
    Ok(result)
}
