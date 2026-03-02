# meta_determinism_guard

A tool to detect and enforce determinism in build outputs and JSON serialization.

## Components

- **backend**: Core analysis engine (scan, canonical JSON, stability harness)
- **ui**: CLI frontend that spawns backend and displays results
- **tooling**: Ruleset validator and loader
