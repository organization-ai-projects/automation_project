use crate::stability::run_matrix::RunMatrix;
use anyhow::Result;
use std::io::Write;

pub fn dump_failing_runs(matrix: &RunMatrix) -> Result<std::path::PathBuf> {
    let dir = std::env::temp_dir().join("meta_determinism_guard_repro");
    std::fs::create_dir_all(&dir)?;

    for result in &matrix.results {
        let file_path = dir.join(format!("run_{}.txt", result.run_index));
        let mut f = std::fs::File::create(&file_path)?;
        writeln!(f, "hash: {}", result.hash)?;
        write!(f, "{}", result.stdout)?;
    }

    Ok(dir)
}
