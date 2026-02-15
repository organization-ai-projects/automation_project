# Runtime Output Directory

This directory contains runtime outputs from the Code Agent Sandbox, including journals, logs, and run artifacts.

## Directory Structure

```
data/
└── runs/                   # All runtime outputs (ignored by git)
    ├── {RUN_ID}/          # Per-run directory (timestamp_UUID format)
    │   ├── journal.jsonl   # Action/result event log
    │   ├── agent_memory.jsonl  # Agent iteration tracking
    │   ├── policy_overrides.toml  # Run-specific policy config
    │   ├── worktree/       # Git repo copy (Learn mode only)
    │   └── generated_code.{ext}   # Generated code output
    ├── models/             # Neural models (shared across runs)
    └── replay.jsonl        # Training examples replay log
```

**Note**: The `runs` directory serves as the runs_root when invoking the sandbox.

## Versioning Policy

### Files Tracked in Version Control

- **README.md**: This documentation file

### Files Ignored by Version Control

All runtime outputs under `data/runs/` are excluded from version control to:
- Prevent repository bloat from large log files
- Protect sensitive information that may appear in runtime logs
- Avoid accidental commits of temporary execution artifacts
- Keep the repository focused on source code

**Ignored patterns:**
- `data/runs/` - All run outputs including journals, logs, generated code, models, and replay logs
- Any UUID-formatted directories (ProtocolId pattern)
- Any timestamped run directories
- Runtime artifacts: `*.jsonl`, `*.log` in run directories

## Privacy and Security

Runtime outputs may contain:
- Code snippets from processed repositories
- File paths and directory structures
- Command outputs and error messages
- Potentially sensitive information from analyzed codebases

**Important**: All outputs in this directory are considered potentially sensitive and must never be committed to version control or shared publicly without proper review.

## Team Workflow

### Local Development

When running the sandbox locally, specify the runs_root as `data/runs`:
```bash
./target/debug/code_agent_sandbox <repo_root> <sandbox_dir>/data/runs
```

Outputs will be written to `data/runs/{RUN_ID}/`. The `.gitignore` rules ensure these files are never accidentally committed.

### CI/CD

In CI/CD pipelines, the sandbox should be configured to write to temporary directories or use the `data/runs` structure with appropriate cleanup. Artifacts needed for analysis should be explicitly saved through CI artifact mechanisms, not through git commits.

### Cleanup

Old run directories can be safely deleted when no longer needed:
```bash
# Remove runs older than 7 days (excluding models/ and replay.jsonl)
find data/runs/ -maxdepth 1 -type d -name "20*" -mtime +7 -exec rm -rf {} +
```

## Usage

The sandbox automatically creates run directories when executed. No manual directory creation is required.

When invoking the sandbox from the repository root:
```bash
# Correct: uses data/runs as runs_root
./target/debug/code_agent_sandbox /path/to/repo /path/to/code_agent_sandbox/data/runs

# Example with relative paths
cd projects/products/stable/code_agent_sandbox
../../../../target/debug/code_agent_sandbox /path/to/repo ./data/runs
```
