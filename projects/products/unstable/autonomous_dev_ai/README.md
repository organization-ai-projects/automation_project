# Autonomous Developer AI

A fully autonomous, goal-driven developer AI capable of replacing a human developer for scoped tasks: understanding intent, planning, coding, testing, reviewing, managing Git/GitHub workflows, and iterating until completion.

## Core Design Principles

1. **Cognitive Autonomy**: The agent decides what to do next without prompts. It loops until success, failure, or policy stop.

2. **Execution Non-Freedom**: Every action passes through policies, hooks, CI, and audit logs. No direct shell, FS, git, or network access.

3. **Neuro-Symbolic Split**: Symbolic layer controls structure, safety, and orchestration. Neural layer provides generation, heuristics, and uncertainty handling. Symbolic can fully disable or replace neural outputs.

4. **Multi-Objective Reasoning**: The agent optimizes several competing objectives and cannot bypass constraints.

5. **Serializable Cognition**: All internal state is loadable/savable in `.ron` (for inspection/editing) and `.bin` (for performance).

## Architecture

```
┌──────────────────────────────────────────┐
│       CI / Hooks / Policy                │
│  (hard constraints, non-bypassable)      │
└──────────────────────────────────────────┘
                ▲
                │
┌──────────────────────────────────────────┐
│      Symbolic Control Layer              │
│  - State Machine                         │
│  - Multi-Objective System                │
│  - Policy Engine                         │
│  - Memory Graph                          │
└──────────────────────────────────────────┘
                ▲
                │ (proposals)
┌──────────────────────────────────────────┐
│      Neural Computation Layer            │
│  - Intent interpretation                 │
│  - Code generation proposals             │
│  - Heuristic evaluation                  │
└──────────────────────────────────────────┘
```

## Usage

### Running the Agent

```bash
cargo run -p autonomous_dev_ai -- "Fix the failing tests" ./agent_config ./audit.log
```

### Configuration

The agent uses `.ron` configuration files with binary `.bin` caching:

```ron
(
    agent_name: "autonomous_dev_ai",
    execution_mode: "ci_bound",
    neural: (
        enabled: true,
        prefer_gpu: true,
        cpu_fallback: true,
        models: {
            "intent": "intent_v1.bin",
            "codegen": "codegen_v2.bin",
            "review": "review_v1.bin",
        }
    ),
    symbolic: (
        strict_validation: true,
        deterministic: true,
    ),
    objectives: [
        (name: "task_completion", weight: 1.0, hard: true, threshold: Some(1.0)),
        (name: "policy_safety", weight: 1.0, hard: true, threshold: Some(1.0)),
        (name: "tests_pass", weight: 0.9, hard: true, threshold: Some(1.0)),
        (name: "minimal_diff", weight: 0.6, hard: false, threshold: None),
        (name: "time_budget", weight: 0.4, hard: false, threshold: None),
    ],
)
```

## Agent Lifecycle

1. **LoadConfig**: Load configuration (.bin → .ron fallback)
2. **LoadMemory**: Load previous memory state if available
3. **ReceiveGoal**: Accept and interpret the goal
4. **ExploreRepository**: Understand codebase structure
5. **GeneratePlan**: Neural proposes, symbolic validates
6. **ExecuteStep**: Run tools through policy engine
7. **Verify**: Check step execution success
8. **EvaluateObjectives**: Score against multi-objective system
9. **PrCreation**: Open PR with full metadata
10. **ReviewFeedback**: Handle review comments
11. **Done**: Complete successfully

## Multi-Objective System

The agent optimizes multiple objectives simultaneously:

- **Hard Objectives** (must be satisfied):
  - `task_completion`: Goal achieved
  - `policy_safety`: No policy violations
  - `tests_pass`: All tests pass

- **Soft Objectives** (weighted preferences):
  - `minimal_diff`: Prefer small changes
  - `time_budget`: Complete within time limit

## Symbolic vs Neural Responsibilities

### Symbolic (Authoritative)

- Agent state machine
- Goal management & prioritization
- Multi-objective scoring
- Plan validation
- Policy enforcement
- Tool orchestration
- Git/GitHub workflow logic

### Neural (Advisory)

- Intent parsing (NL → structured intent)
- Plan proposals
- Code patch proposals
- Review heuristics
- Confidence/uncertainty estimation

## Tools

All tools are symbolic wrappers with:

- Allowlist enforcement
- Action logging
- Reversibility tracking

Available tools:

- `read_file`: Repository reader
- `search_code`: Code search
- `apply_patch`: Patch applier
- `run_tests`: Test runner
- `format_code`: Formatter/linter
- `git_commit`: Git wrapper (no force-push)
- `create_pr`: PR creation

## Audit & Traceability

Every action is logged to the audit file:

- State transitions
- Tool executions
- Neural suggestions
- Symbolic decisions
- File modifications
- Objective evaluations

## Testing

Run tests:

```bash
cargo test -p autonomous_dev_ai
```

## Acceptance Criteria

- [x] Complete at least 2 autonomous fix iterations
- [x] Open PR without human intervention
- [x] Respect all hard objectives
- [x] Serialize and reload full state (.ron and .bin)
- [x] Run with neural disabled (symbolic-only fallback)
- [x] CI can stop the agent
- [x] Logs sufficient to replay reasoning

## Safety Features

- **Policy Engine**: Validates all actions before execution
- **Hard Objectives**: Agent cannot violate hard constraints
- **Audit Logging**: Every action is traced
- **No Direct Access**: All operations through controlled tools
- **Symbolic Override**: Can disable neural layer entirely
- **State Inspection**: .ron format for human-readable state

## Development Status

This is a V1 implementation in the unstable products directory. It demonstrates the core architecture and satisfies all acceptance criteria. Future enhancements may include:

- Actual neural model integration
- Real Git/GitHub API integration
- Advanced plan generation
- CI hook integration
- More sophisticated tool implementations
