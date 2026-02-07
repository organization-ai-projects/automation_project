# Auto Manager AI (V0 - Safe-by-Default)

A V0 AI Automation Manager that analyzes repository and GitHub context and outputs strict, schema-validated JSON action plans. The AI **never executes privileged actions directly** - it only proposes actions that are evaluated by a policy/gatekeeper.

## Purpose

This unstable product implements a safe-by-default AI automation assistant for repository management. V0 focuses on:

- Read-only analysis of repository files and metadata
- Generating action plans (proposals only, never executed)
- Policy-based guardrails to prevent unsafe operations
- Schema-validated JSON outputs

## Architectural Compliance

### Current Violations

1. **Direct filesystem access**: Bypasses the Engine protocol for file operations
   - **Reason**: V0 MVP focused on proving the action plan concept
   - **Impact**: Limited to read-only repository analysis and write to `./out/` only
   - **Risk**: Low - enforced read-only operations with comprehensive tests

2. **No registry integration**: Product doesn't register with Engine
   - **Reason**: Standalone CLI tool for V0 to validate architecture
   - **Impact**: Cannot be discovered or managed by central UI
   - **Risk**: Low - intended for direct CLI usage in V0

3. **No permission checks**: No authentication or authorization
   - **Reason**: V0 focused on core planning and policy logic
   - **Impact**: Anyone with filesystem access can run analysis
   - **Risk**: Low for V0 (read-only), Medium for future versions with write capabilities

4. **Template-based planning**: Uses simple heuristics instead of AI/ML
   - **Reason**: V0 proves the action plan architecture and guardrails
   - **Impact**: Limited intelligence in action generation
   - **Risk**: Low - clearly documented as template-based

5. **No structured logging**: Uses println! for output
   - **Reason**: Simple debugging during V0 development
   - **Impact**: Logs not captured by central logging system
   - **Risk**: Low - easy to add in future versions

6. **Stub adapters**: GitHub and CI adapters are not fully implemented
   - **Reason**: V0 focuses on repository analysis only
   - **Impact**: Cannot analyze GitHub PRs/issues or CI failures
   - **Risk**: Low - clearly marked as stubs

### Exit Criteria

To promote this product to stable, the following must be completed:

- [ ] **Protocol integration**: Implement Engine protocol for file access
  - Use `protocol` library for message passing
  - Register with Engine on startup
  - Handle Engine shutdown signals

- [ ] **Permission system**: Integrate with identity/security libraries
  - Check user permissions before generating plans
  - Audit log all plan generation activities

- [ ] **AI/ML integration**: Replace template-based planner with actual AI
  - Integrate with `ai` library or external AI service
  - Implement context analysis and reasoning
  - Add learning from feedback

- [ ] **Complete adapters**: Fully implement GitHub and CI adapters
  - Use `gh --json` for structured GitHub data
  - Parse CI logs and failures
  - Support multiple CI systems

- [ ] **Structured logging**: Replace println! with tracing
  - Use workspace `tracing` dependency
  - Emit structured events for monitoring

- [ ] **Configuration system**: Support config files and environment
  - Use protocol for configuration
  - Support `config.toml`
  - Accept configuration from Engine

- [ ] **Testing**: Expand test coverage
  - Add integration tests with mock Engine
  - Test all adapter implementations
  - Target: >80% code coverage

- [ ] **Documentation**: Follow standardized docs pattern
  - Add `documentation/` directory
  - Document all action types
  - Create usage examples

- [ ] **Action execution**: Implement safe action execution (post-V0)
  - Add executor module with strict safety checks
  - Implement dry-run verification
  - Add rollback capabilities

### Target Promotion Date

**Q3 2026** - Contingent on AI integration and protocol stabilization

## Standardized Output

This product produces output in the **structured JSON file** format:

**Output files**:

- `./out/action_plan.json` - The generated action plan
- `./out/run_report.json` - Run report with policy decisions

**action_plan.json format**:

```json
{
  "version": "0.1.0",
  "generated_at": "timestamp",
  "actions": [
    {
      "id": "action_001",
      "action_type": "analyze_repository",
      "status": "proposed",
      "target": {
        "type": "repo",
        "ref": "owner/repo"
      },
      "justification": "...",
      "risk_level": "low",
      "required_checks": ["..."],
      "confidence": 0.95,
      "evidence": [
        {
          "source": "...",
          "pointer": "..."
        }
      ]
    }
  ],
  "summary": "..."
}
```

**run_report.json format**:

```json
{
  "product": "auto_manager_ai",
  "version": "0.1.0",
  "run_id": "run_123",
  "timestamp": "timestamp",
  "status": "success",
  "output": {
    "actions_proposed": 1,
    "actions_allowed": 1,
    "actions_denied": 0,
    "actions_needs_input": 0
  },
  "policy_decisions": [...],
  "errors": []
}
```

## V0 Safety Guarantees

### Hard Constraints

1. **No repository writes**: V0 cannot modify tracked files
   - Enforced by policy (all write actions denied)
   - Covered by unit tests in `src/domain/policy.rs` (test_policy_default_deny_writes, test_all_write_actions_denied)

2. **No GitHub writes**: V0 cannot create issues, PRs, or comments
   - Enforced by policy
   - Covered by unit tests in `src/domain/policy.rs` and `src/plan_evaluator.rs`

3. **Schema validation**: All outputs must be valid JSON
   - Covered by unit tests in `src/domain/action_plan.rs` (test_action_plan_serialization, test_action_plan_round_trip) and `src/domain/run_report.rs` (test_run_report_serialization)

4. **Confidence threshold**: Actions below 0.6 confidence are denied
   - Enforced by policy
   - Covered by unit tests in `src/domain/policy.rs` (test_policy_confidence_threshold, test_confidence_threshold_default)

5. **Output isolation**: Only `./out/` directory can be written
   - Enforced by implementation
   - Covered by unit tests in `src/output_writer.rs` (test_write_outputs) and `src/plan_generator.rs`

### Action Lifecycle

All actions in V0 are **proposals only**:

- Actions are generated with status `proposed`, `needs_input`, or `blocked_by_policy`
- Policy evaluates each action and returns `allow`, `deny`, or `needs_input`
- **No actions are executed** - execution is out of scope for V0

### Anti-Hallucination Rules

1. **Allowed sources only**: Repository files (read-only), structured outputs only
2. **No guessing**: Missing data → `status = needs_input`
3. **Evidence required**: Every action must reference its sources
4. **Schema validation**: Invalid JSON plans are rejected
5. **Confidence threshold**: Actions with confidence < 0.6 are blocked

## Usage (Current V0)

### Basic Usage

```bash
cd projects/products/unstable/auto_manager_ai
cargo build --release

# Run on current directory
./target/release/auto_manager_ai

# Run on specific repository with custom output directory
./target/release/auto_manager_ai /path/to/repo /path/to/output
```

### Example Output

```
Auto Manager AI V0 (Safe-by-Default)
Repository: "/home/user/myrepo"
Output: "./out"

Generating action plan...
Generated 1 actions
Evaluating actions against policy...
Policy evaluation complete:
  Allowed: 1
  Denied: 0
  Needs input: 0

Writing outputs to "./out"...
Done! Outputs written to:
  - "./out/action_plan.json"
  - "./out/run_report.json"

V0 Note: All actions are proposals only. No mutations were performed.
```

## Testing

Run all unit tests:

```bash
cd projects/products/unstable/auto_manager_ai
cargo test

# Run tests for specific modules
cargo test domain::policy::tests
cargo test plan_generator::tests
cargo test output_writer::tests
```

All tests are unit tests embedded within modules using `#[cfg(test)]`. Key test modules:

- `src/domain/policy.rs`: Policy guardrails (6 tests)
- `src/domain/action_plan.rs`: Schema validation (3 tests)
- `src/domain/run_report.rs`: Report generation (4 tests)
- `src/plan_generator.rs`: Action plan generation (1 test)
- `src/plan_evaluator.rs`: Policy evaluation (1 test)
- `src/output_writer.rs`: Output writing (1 test)

All tests must pass to ensure V0 safety guarantees.

## Crate Structure

```
projects/products/unstable/auto_manager_ai/
  Cargo.toml
  README.md
  src/
    main.rs             # Binary CLI entrypoint (binary-only crate, no lib.rs)
    config.rs           # Config struct
    plan_generator.rs   # Action plan generation logic
    plan_evaluator.rs   # Policy evaluation logic
    output_writer.rs    # JSON output writing
    domain/
      mod.rs
      # One struct/enum per file pattern:
      action.rs              # Action struct
      action_plan.rs         # ActionPlan struct
      action_status.rs       # ActionStatus enum
      action_target.rs       # ActionTarget enum
      dry_run.rs             # DryRun struct
      dry_run_step.rs        # DryRunStep struct
      evidence.rs            # Evidence struct
      policy.rs              # Policy struct
      policy_decision.rs     # PolicyDecision struct
      policy_decision_type.rs # PolicyDecisionType enum
      risk_level.rs          # RiskLevel enum
      run_output.rs          # RunOutput struct
      run_report.rs          # RunReport struct
      run_status.rs          # RunStatus enum
    adapters/
      mod.rs
      # One struct/enum per file pattern:
      repo_context.rs    # RepoContext struct
      repo_adapter.rs    # RepoAdapter struct (read-only)
      gh_context.rs      # GhContext struct
      gh_adapter.rs      # GhAdapter struct (stub in V0)
      ci_context.rs      # CiContext struct
      ci_adapter.rs      # CiAdapter struct (stub in V0)
    ai/
      mod.rs
      planning_context.rs  # PlanningContext struct
      planner.rs           # Planner struct (template-based for V0)
  schemas/
    action_plan.schema.json
    run_report.schema.json
```

Note: This is a **binary-only crate** following the one-struct-per-file pattern. All tests are unit tests embedded within modules using `#[cfg(test)]`.

## Public API

```rust
// Generate an action plan
let plan = generate_action_plan(&config)?;

// Evaluate plan against policy
let decisions = evaluate_plan(&plan, &policy);

// Write outputs to directory
write_outputs(&plan, &report, &out_dir)?;
```

## Future (Stable)

Once promoted to stable, this product will:

- Be launched and managed by the Engine
- Integrate with the central UI
- Use actual AI/ML for intelligent planning
- Support safe action execution with dry-run verification
- Fully implement GitHub and CI adapters
- Follow all architectural patterns
- Support dynamic configuration
- Emit structured telemetry

## Contributing

See [CONTRIBUTING.md](../../../../CONTRIBUTING.md) for general guidelines.

## Schema Documentation

JSON schemas are available in the `schemas/` directory:

- `action_plan.schema.json` - Schema for action plans
- `run_report.schema.json` - Schema for run reports

These schemas define the contract for all outputs and enable validation by external tools.
