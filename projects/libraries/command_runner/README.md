# Command Runner

`command_runner` is a Rust library designed to execute system commands in a robust and ergonomic way. It provides tools to handle errors, capture outputs, and log executions.

## Features

- **Strict or permissive execution**:
  - `run_cmd_ok`: Returns an error if the command fails.
  - `run_cmd_allow_failure`: Always returns the output, even in case of failure.
- **Error handling**:
  - Detailed error types (`CommandError`).
  - Logging of executed commands.
- **Safe truncation**:
  - Long outputs are truncated in a UTF-8 safe manner.

## Installation

Add the following dependency to your `Cargo.toml`:

```toml
[dependencies]
command_runner = "0.1.0"
```

## Usage

### Basic Example

```rust
use command_runner::{run_cmd_ok, CommandError};
use std::path::Path;

fn main() -> Result<(), CommandError> {
    let repo_path = Path::new("/path/to/repo");
    let mut logs = Vec::new();

    let output = run_cmd_ok(repo_path, "git", &["status"], &mut logs)?;

    println!("Status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    Ok(())
}
```

### Execution Modes

- **Strict**:
  - Use `run_cmd_ok` for commands where a non-zero exit code is considered an error.
- **Permissive**:
  - Use `run_cmd_allow_failure` to capture the output even if the command fails.

### Error Handling

Errors are encapsulated in the `CommandError` type:

- `InvalidInput`: Invalid input for the command.
- `Io`: Input/output error during execution.
- `NonZeroExit`: The command failed with a non-zero exit code.

### Logging

Logs of executed commands can be collected in a `Vec<String>`:

```rust
let mut logs = Vec::new();
run_cmd_ok(repo_path, "ls", &["-la"], &mut logs)?;
for log in logs {
    println!("{}", log);
}
```

## Contributing

Contributions are welcome! Please open an issue or pull request on the GitHub repository.

For more details on the Git/GitHub workflow used in this project, see the [versioning documentation](../../../docs/versioning/git-github.md).

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.
