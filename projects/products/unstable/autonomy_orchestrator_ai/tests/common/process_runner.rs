use std::path::Path;
use std::process::{Command, Output};

pub fn run_autonomy_orchestrator(out_dir: &Path, args: &[&str]) -> Output {
    let bin = env!("CARGO_BIN_EXE_autonomy_orchestrator_ai");
    let mut cmd = Command::new(bin);
    cmd.arg(out_dir);
    for arg in args {
        cmd.arg(arg);
    }
    cmd.output()
        .expect("failed to execute autonomy_orchestrator_ai binary")
}
