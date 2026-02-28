use anyhow::Result;
use sha2::{Sha256, Digest};
use crate::stability::run_matrix::{RunResult, RunMatrix};
use crate::stability::stability_report::StabilityReport;
use crate::stability::repro_dumper;

pub fn run_stability(cmd: &str, runs: u32) -> Result<StabilityReport> {
    let mut results = Vec::new();

    for i in 0..runs {
        let output = std::process::Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()?;
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let mut sorted_lines: Vec<&str> = stdout.lines().collect();
        sorted_lines.sort();
        let normalized = sorted_lines.join("\n");

        let mut hasher = Sha256::new();
        hasher.update(normalized.as_bytes());
        let hash = hex::encode(hasher.finalize());

        results.push(RunResult { run_index: i, stdout, hash });
    }

    let matrix = RunMatrix::new(results);
    let stable = matrix.all_hashes_equal();
    let run_hashes = matrix.sorted_hashes();

    let diff = if !stable {
        let _ = repro_dumper::dump_failing_runs(&matrix);
        Some(build_diff(&matrix))
    } else {
        None
    };

    Ok(StabilityReport { stable, runs, run_hashes, diff })
}

fn build_diff(matrix: &RunMatrix) -> String {
    if matrix.results.len() < 2 {
        return String::from("(no diff available)");
    }
    let first = &matrix.results[0].stdout;
    let second = &matrix.results[1].stdout;
    crate::canon::canonical_diff::diff_strings(first, second)
}
