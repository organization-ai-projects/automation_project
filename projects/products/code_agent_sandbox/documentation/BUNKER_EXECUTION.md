# Documentation: Security and Execution in a Bunker

- [Back to Documentation Index](TOC.md)

## Context

For secure execution of actions, certain commands require an isolated environment, referred to as a "bunker," to mitigate risks associated with unsafe behaviors. Two main concepts are used to manage this logic:

- **`requires_bunker`**: A method that determines whether a specific command needs to be executed in a secure environment.
- **`run_in_bunker`**: A method that actually executes a command in this secure environment.

## How It Works

### `requires_bunker`

This method belongs to the `CommandRunner` structure and is used to analyze a given command (e.g., a `cargo` subcommand). It returns a boolean:

- `true`: The command must be executed in a bunker.
- `false`: The command can be executed normally.

#### Example Usage

```rust
if CommandRunner::requires_bunker(subcommand) {
    let mut argv = Vec::with_capacity(1 + args.len());
    argv.push(subcommand.to_string());
    argv.extend(args.iter().cloned());
    ctx.runner.run_in_bunker("cargo", &argv)
} else {
    ctx.runner.run_cargo(subcommand, args)
}
```

### `run_in_bunker`

This method belongs to the `CommandRunner` structure and allows executing a command in an isolated environment. This ensures that the command cannot access unauthorized resources or compromise the system.

## Why These Two Concepts?

- **Separation of Responsibilities**:
  - `requires_bunker` focuses on decision-making logic (command analysis).
  - `run_in_bunker` focuses on secure execution.
- **Flexibility**: This allows new rules to be easily added to `requires_bunker` without modifying the execution logic.

## Use Cases

- **Sensitive Commands**: Certain `cargo` subcommands (like `publish`) require a bunker to prevent unauthorized modifications.
- **Unsafe Actions**: Any command that could compromise system integrity.

## Conclusion

These mechanisms ensure secure and robust execution of actions while maintaining a clear and modular architecture.
