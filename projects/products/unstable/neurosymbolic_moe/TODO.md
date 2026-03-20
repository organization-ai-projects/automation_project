# Neurosymbolic MoE Detailed TODO

## Purpose

This document defines what still needs to be built, clarified, or tightened for `neurosymbolic_moe` to become a real Mixture-of-Experts runtime rather than a well-structured platform skeleton.

The goal is not only to "have all the components." The goal is to ensure the components work together with real semantics:

- experts must be meaningfully different
- routing must make useful decisions
- aggregation must improve outcomes when multiple experts are used
- traces and datasets must represent real runtime behavior
- auto-improvement must change actual runtime outcomes
- governance and operational diagnostics must express trustworthy production semantics

This file should be treated as the top-level execution roadmap for the product and the backend.

---

## Repository Map

The roadmap below should be read against the current backend structure.

### Runtime and orchestration

- `backend/src/orchestrator/`
- `backend/src/orchestrator/pipeline_moe/`
- `backend/src/orchestrator/moe_pipeline_builder.rs`
- `backend/src/app.rs`
- `backend/src/apps/impl_check.rs`

### Experts, routing, and aggregation

- `backend/src/moe_core/`
- `backend/src/expert_registries/`
- `backend/src/router/`
- `backend/src/aggregator/`
- `backend/src/echo_expert.rs`

### Retrieval, memory, buffers, and context

- `backend/src/retrieval_engine/`
- `backend/src/memory_engine/`
- `backend/src/buffer_manager/`

### Datasets, evaluation, and feedback

- `backend/src/dataset_engine/`
- `backend/src/evaluations/`
- `backend/src/feedback_engine/`
- `backend/src/trace_logging/`

### Governance and persistence

- `backend/src/orchestrator/governance_*.rs`
- `backend/src/orchestrator/runtime_persistence_bundle.rs`
- `backend/src/apps/runtime_checks.rs`

### Why this section exists

The roadmap is intentionally product-level, but the work must land in real modules. This mapping makes it clear where each workstream is expected to touch code.

---

## Executive Summary

### What already exists

The project already contains most of the expected MoE platform building blocks:

- expert registration
- routing
- aggregation
- retrieval
- memory and buffers
- trace logging
- dataset conversion and storage
- evaluation and feedback handling
- training bundle generation and sharding
- auto-improvement policy and status tracking
- governance state diffing and import policy
- runtime persistence
- concurrent wrappers and operational check commands
- CLI and metrics endpoints

### What is still missing

The main missing part is not architecture. It is semantic maturity.

Today, too much of the runtime still proves that:

- the wiring compiles
- the components can be instantiated
- the pipeline can run end-to-end
- some artifacts can be produced

What still needs to be proven is that:

- different experts are genuinely useful for different work
- routing decisions are driven by meaningful inputs
- aggregation changes quality, robustness, or safety
- dataset artifacts come from realistic runtime diversity
- governance decisions protect real product rules
- `impl-check`, `status`, `metrics`, and `serve-metrics` represent actual operational guarantees

### Product-level definition of done

`neurosymbolic_moe` should only be considered a real MoE when all of the following are true:

- there are multiple experts with non-overlapping strengths
- the router can explain why it chose expert A instead of expert B
- some tasks benefit from one expert, some from multiple experts
- traces and dataset entries capture execution diversity rather than mostly placeholder flows
- the improvement loop affects promoted runtime behavior, not only stored metadata
- operational endpoints expose the health of actual MoE capabilities, not only process liveness

---

## Current State Assessment

### Strengths

- The codebase is already modular.
- Boundaries between routing, aggregation, dataset generation, governance, and observability are visible.
- There is already an implementation path for runtime persistence and bundle reconstruction.
- The project already thinks about concurrency, policy enforcement, and operational diagnostics.
- The governance layer now uses structured `Version` semantics instead of raw string comparison.

### Weaknesses

- Some runtime flows still rely on placeholder experts such as `EchoExpert`.
- The heuristic router is likely still closer to capability filtering than to true expert selection.
- Some diagnostics are still more "implementation exercises" than product capability checks.
- Dataset generation risks over-representing synthetic or repeated flows.
- The current `ProtocolId::default()` convention can blur the difference between:
  - one logical entity being updated repeatedly
  - many distinct runtime observations being stored
- Several operational paths still need a stronger definition of what they are guaranteeing.

### Main risk

The biggest near-term risk is a false sense of completeness:

- the project can look feature-rich
- many modules can compile and run
- many reports and artifacts can be emitted

while the actual MoE semantics remain too weak to justify the architecture.

---

## Immediate Product Questions That Must Be Answered

These questions are blockers because too many implementation details depend on them.

### 1. What is the primary product task of this MoE?

Possible directions include:

- code generation and refinement
- code transformation and validation
- retrieval-augmented reasoning
- policy-safe orchestration
- self-improving modular runtime

This must be explicit because expert specialization and evaluation depend on it.

### 2. Is the system expected to be mostly single-expert with occasional fan-out, or genuinely multi-expert by default?

This affects:

- router design
- aggregation complexity
- evaluation metrics
- trace verbosity

### 3. What should count as a "better" result?

Possible dimensions:

- quality
- safety
- determinism
- latency
- policy compliance
- recoverability

The system cannot optimize routing or improvement without a declared objective.

### 4. What does one dataset entry represent?

This is a major unresolved semantic point.

A dataset entry could represent:

- one runtime execution event
- one evolving logical artifact
- one post-correction training sample
- one diagnostic placeholder record

That decision affects `ProtocolId` semantics, concurrency semantics, and provenance.

### 5. Who is the operator of this system?

Possible operators:

- developer running CLI diagnostics
- CI pipeline
- service operator
- internal platform maintainer

Without this, `status`, `impl-check`, `/healthz`, `/readyz`, and `/metrics` remain underspecified.

---

## Workstream 1: Expert Model Must Become Real

### Objective

Turn the expert layer from "multiple pluggable executors exist" into "multiple experts provide meaningfully different value."

### Why this matters

Without real expert specialization, the rest of the MoE architecture becomes mostly ceremonial:

- routing has nothing meaningful to optimize
- aggregation has nothing meaningful to combine
- evaluation has nothing meaningful to compare
- datasets do not capture expert diversity

### Required decisions

- Define the target expert families for the product.
- Define the expected ownership of each expert.
- Define which expert types are allowed to overlap and which should remain disjoint.
- Define what makes an expert selection "good."

### Candidate expert families

- retrieval-context expert
- planning expert
- code generation expert
- code transformation expert
- validation expert
- policy/compliance expert
- evaluation/ranking expert
- fallback/generalist expert

### Concrete tasks

- Replace placeholder experts in runtime-facing flows where they currently hide missing behavior.
- Define a capability matrix for experts:
  - accepted task kinds
  - rejected task kinds
  - expected confidence semantics
  - expected output structure
  - safety and policy constraints
- Add expert metadata that can support routing:
  - specialization tags
  - supported task families
  - trust level
  - latency class
  - quality tier
  - version / rollout status
- Decide whether expert identity should encode product semantics beyond a plain registration name.

### Deliverables

- Expert catalog document
- Expert capability matrix
- Removal or isolation of placeholder experts from production-oriented flows
- At least one concrete runtime scenario per expert family
- Explicit list of placeholder-only experts versus production-grade experts

### Acceptance criteria

- Each expert has at least one scenario where it is clearly preferred over another expert.
- Each expert also has at least one scenario it should not handle.
- Expert metadata is useful to the router, not merely descriptive.

---

## Workstream 2: Router Must Encode Real Decision Logic

### Objective

Make routing an explicit, understandable, and product-driven decision system.

### Why this matters

The router is the point where a MoE system proves it is not just "calling random modules." If routing is shallow, the MoE is shallow.

### Current concern

The current heuristic router likely proves that routing exists, but not yet that routing is strategically useful.

### Inputs that routing should eventually consider

- task type
- task priority
- task context
- task metadata
- retrieved supporting evidence
- historical expert performance
- policy constraints
- expert availability / health
- expert version compatibility
- confidence calibration quality

### Concrete tasks

- Audit current router behavior and document the real decision rules it uses today.
- Identify which decision dimensions are missing.
- Define routing policies for:
  - single expert selection
  - top-k expert fan-out
  - fallback routing
  - policy-restricted routing
  - low-confidence routing
- Decide whether the project wants to remain heuristic-first or become score-driven.
- Add traceability so routing decisions are inspectable in logs and reports.

### Questions to answer

- When should routing select one expert only?
- When should routing fan out to multiple experts?
- When should a policy constraint veto a routing candidate?
- How should retrieval evidence influence routing?
- Should routing consider previous runtime failures?

### Deliverables

- Routing specification
- Routing trace schema
- Clear examples of task-to-expert decisions
- Updated router tests covering real decision branches
- A documented decision table mapping representative tasks to expected routing outcomes

### Acceptance criteria

- Routing choices differ across task classes for meaningful reasons.
- Routing decisions can be explained from runtime data.
- At least one routing case depends on more than static capability presence.

---

## Workstream 3: Aggregation Must Demonstrate Real Value

### Objective

Ensure that aggregation is not merely a technical post-processing step but a source of measurable MoE value.

### Why this matters

A MoE that routes to multiple experts but always ends up behaving like a single winner-takes-all executor is not using its architecture well.

### Aggregation outcomes that should exist

- best-answer selection
- confidence-weighted merge
- policy-safe merge
- consensus validation
- expert disagreement detection
- structured fallback when one expert fails

### Concrete tasks

- Audit existing aggregation strategies and document what they truly optimize.
- Define when aggregation should happen and when it should not.
- Decide whether aggregation should be content-aware, policy-aware, or both.
- Identify tasks where merged output is better than the best single output.
- Identify tasks where merging is actively harmful and should be blocked.

### Deliverables

- Aggregation strategy matrix
- Cases where aggregation improves quality
- Cases where aggregation improves robustness
- Cases where aggregation must be rejected

### Acceptance criteria

- There is at least one scenario where multi-expert aggregation materially improves the result.
- Aggregation is not used by default without a reason.
- Policy validation can reject an aggregated result if the merge introduced risk.

---

## Workstream 4: Dataset Generation Must Reflect Real Runtime Diversity

### Objective

Make dataset generation represent actual MoE runtime behavior rather than mostly synthetic plumbing.

### Why this matters

The dataset is the memory of the system. If the dataset is unrealistic, the improvement loop will reinforce unrealistic patterns.

### Pipeline to audit

The following path must be treated as one coherent product flow:

`trace -> dataset entry -> correction -> quality report -> training bundle -> shards -> rebuild`

### Concrete tasks

- For each step above, classify current behavior as:
  - real runtime behavior
  - diagnostic behavior
  - bootstrap behavior
  - synthetic placeholder behavior
- Measure how diverse dataset entries are:
  - task types
  - experts involved
  - success/failure outcomes
  - correction types
  - metadata coverage
- Verify whether traces preserve enough context to make later training useful.
- Verify whether corrections are actually represented downstream in training bundles.
- Verify whether bundle provenance remains understandable for operators.

### Protocol ID concern

`ProtocolId::default()` is currently used broadly in this backend. That may be acceptable in some diagnostic or bootstrap paths, but it creates ambiguity for dataset semantics.

This needs an explicit product decision:

- If a path is meant to represent updates to one logical entity, repeated `ProtocolId::default()` may be acceptable.
- If a path is meant to represent distinct runtime observations, reusing `ProtocolId::default()` obscures cardinality and data semantics.

### Required action

- Audit every dataset-related path and label whether it represents:
  - one logical evolving record
  - one runtime event among many
  - one synthetic placeholder artifact
- Isolate synthetic or diagnostic flows from real dataset flows where necessary.

### Deliverables

- Dataset semantics inventory
- Provenance rules
- Runtime-to-dataset mapping document
- Updated tests that reflect the chosen dataset semantics
- Explicit distinction between runtime, diagnostic, bootstrap, and synthetic dataset paths

### Acceptance criteria

- A runtime observer can explain why a dataset entry exists.
- Distinct runtime observations are not accidentally conflated unless that is the intentional product rule.
- Training bundles preserve enough diversity to be useful.

---

## Workstream 5: Auto-Improvement Must Change Real Runtime Outcomes

### Objective

Ensure the improvement loop is product-real, not just artifact-generating.

### Why this matters

It is not enough to produce bundles, counters, and reports. The system must actually improve something operators and users care about.

### Concrete tasks

- Audit `AutoImprovementPolicy` defaults and thresholds:
  - minimum dataset entries
  - success ratio thresholds
  - average score thresholds
  - inclusion/exclusion rules for failure and partial entries
- Verify that quality reports genuinely affect promotion decisions.
- Verify that promoted artifacts change runtime behavior in a visible way.
- Decide what "improvement" means:
  - quality increase
  - lower failure rate
  - better policy compliance
  - lower routing waste
  - better aggregation selection
- Ensure skip reasons and failure reasons are operationally actionable.

### Deliverables

- Auto-improvement success definition
- Promotion and rollback decision rules
- Runtime-visible effect of promoted versions
- Operator-readable reports

### Acceptance criteria

- At least one end-to-end flow demonstrates that runtime behavior changes after improvement.
- Operators can explain why a promotion happened or was skipped.
- Improvement status is not only a set of counters; it is tied to user-visible behavior.

---

## Workstream 6: Governance Must Protect Real Product Semantics

### Objective

Make governance rules represent actual product protection boundaries instead of generic metadata drift checks.

### Why this matters

Governance exists to block unsafe or incoherent state transitions. If the rules are too abstract, the system will accept bad runtime states or reject valid ones for the wrong reasons.

### Current positive state

The project now uses structured `Version` and richer `VersionDelta` semantics. That is a good foundation.

### Remaining work

- Define what version change types mean operationally:
  - patch upgrade
  - minor upgrade
  - major upgrade
  - patch regression
  - minor regression
  - major regression
- Define when a regression is acceptable and why.
- Define how schema version, checksum, policy, baseline, and report drift interact.
- Ensure rejection reasons are readable from logs and reports.
- Decide whether governance output needs:
  - human-readable summaries
  - CLI-friendly explanations
  - audit serialization guarantees

### Deliverables

- Governance rulebook
- Import decision matrix
- Operator-facing rejection reason catalog

### Acceptance criteria

- Every governance rejection can be understood without reading source code.
- Version semantics are tied to actual release and rollback policy.
- Governance decisions match real product risk, not just structure diffs.

---

## Workstream 7: `cmd_impl_check` Must Become A Production Capability Diagnostic

### Objective

Turn `cmd_impl_check` into a trustworthy production-oriented diagnostic that validates MoE capabilities, not just component presence.

### Why this matters

This command is one of the most visible operational entry points. If its semantics are vague, operators will draw the wrong conclusions from it.

### Required mindset

`cmd_impl_check` must not behave like a hidden integration test dump.
It must answer:

- what capability is being checked
- why that capability matters to the MoE
- what operational risk a failure would signal

### Recommended structure

Split the command into explicit phases such as:

- core runtime initialization
- expert registry and routing
- trace logging and observability
- dataset generation and correction flow
- training bundle generation and reconstruction
- concurrency and contention behavior
- governance and persistence
- report emission and summary

### Concurrent dataset block

This block needs a clear semantic decision:

- If it validates concurrent updates to the same logical entry, say so explicitly.
- If it validates concurrent ingestion of multiple runtime observations, current `ProtocolId::default()` behavior is not semantically expressive enough.

### Concrete tasks

- Rename phases and logs around capability checks.
- Remove any wording that sounds like a test harness.
- Add explicit phase results.
- Decide whether the command should fail fast or produce a phase-by-phase report.
- Decide whether this command is CI-facing, operator-facing, or both.

### Deliverables

- `impl-check` phase map
- capability-oriented logs
- explicit failure semantics
- diagnostic summary format
- documented explanation of what `impl-check` proves and what it does not prove

### Acceptance criteria

- An operator can read the output and know which MoE capability failed.
- The command does not over-claim what it has proven.
- The command checks runtime semantics, not just module existence.

---

## Workstream 8: Operational CLI And Endpoints Must Express Guarantees

### Objective

Define what each operational surface actually guarantees.

### Surfaces to define

- `status`
- `impl-check`
- `metrics`
- `serve-metrics`
- `trainer-events`
- trace CLI paths

### Questions to answer

- Is `status` only inventory, or does it mean health?
- Is `/healthz` shallow or deep?
- Is `/readyz` allowed to be expensive?
- What does `/metrics` promise about freshness?
- Should profile switching use cached health or fresh health?
- Are trainer event operations observability tools, maintenance tools, or real operator workflows?

### Concrete tasks

- Write guarantee statements for each endpoint/command.
- Separate cheap liveness checks from expensive correctness checks.
- Ensure cache behavior is intentional and documented.
- Decide which endpoints are safe for high-frequency scraping.
- Tighten admin endpoint semantics and failure messaging.

### Deliverables

- Operational semantics document
- endpoint guarantee table
- cache and freshness policy
- admin workflow policy

### Acceptance criteria

- Operators know which endpoint to trust for which question.
- The system does not present expensive deep diagnostics as cheap health checks.
- Admin actions have auditable, understandable behavior.

---

## Workstream 9: Concurrency Semantics Must Be Explicit

### Objective

Define concurrency behavior at the product level, not just at the mutex/RwLock level.

### Why this matters

Thread safety alone is not enough. The product needs stable semantics for concurrent updates.

### Questions to answer

- What is last-write-wins and where is it acceptable?
- What is idempotent and where is it required?
- Which objects can tolerate duplicate writes?
- Which paths must preserve ordering?
- Which flows represent contention on one logical object?
- Which flows represent parallel creation of many objects?

### Concrete tasks

- Audit concurrent stores and wrappers.
- Label each path with its expected semantics:
  - safe concurrent read/write
  - deterministic overwrite
  - merge-on-write
  - import de-duplication
  - conflict rejection
- Align runtime checks with actual production risks:
  - lock poisoning
  - timeout behavior
  - contention spikes
  - duplicate identifiers
  - partial persistence failure

### Deliverables

- concurrency semantics matrix
- path-by-path write behavior inventory
- clarified contention policy

### Acceptance criteria

- Each concurrent path has a declared semantic model.
- Diagnostics and metrics reflect the declared model.
- Operators can distinguish data contention from healthy parallelism.

---

## Workstream 10: Evaluation Must Measure MoE Usefulness

### Objective

Make the evaluation layer measure whether the MoE architecture is helping.

### Why this matters

If metrics only measure component activity, they will not prove product value.

### Metrics that should exist

- routing accuracy or routing usefulness
- expert win rate by task family
- aggregation uplift
- fallback rate
- policy rejection rate
- correction rate
- training bundle acceptance rate
- governance rejection rate
- improvement promotion rate

### Concrete tasks

- Audit existing evaluation metrics.
- Separate infrastructure metrics from MoE value metrics.
- Add metrics for expert specialization effectiveness.
- Add metrics for aggregation benefit.
- Add metrics for skipped improvement and skipped promotion reasons.

### Deliverables

- MoE metrics inventory
- dashboard-worthy metric set
- report output that distinguishes activity from usefulness
- explicit relationship between evaluation signals and routing/improvement decisions

### Acceptance criteria

- The project can answer whether the MoE is helping, not only whether it is running.
- Metrics can identify underperforming experts or useless routing branches.

---

## Workstream 11: Observability Must Explain Behavior, Not Only Events

### Objective

Improve logs, traces, and reports so they explain why the MoE behaved the way it did.

### Required visibility

- selected experts
- rejected experts and why
- routing rationale
- aggregation rationale
- policy blocks
- correction insertion
- bundle generation decisions
- auto-improvement skip reasons
- governance rejection reasons

### Concrete tasks

- Review current trace fields and logs.
- Ensure trace records are rich enough for later dataset and debugging use.
- Add operator-friendly summaries where raw internal structures are too noisy.
- Align reporting vocabulary across runtime, governance, and diagnostics.

### Deliverables

- observability vocabulary
- trace schema review
- report summary fields

### Acceptance criteria

- A trace can be used to reconstruct the decision flow of one MoE execution.
- Logs explain product behavior instead of only listing internal events.

---

## Workstream 12: Testing Strategy Must Follow Product Semantics

### Objective

Make tests protect meaningful product behavior rather than stale abstractions.

### Current direction that should remain

- trait-only tests should usually be removed unless they protect a meaningful contract
- tests should follow the current `ProtocolId` convention
- tests should target concrete behavior, not ornamental abstraction

### Concrete tasks

- Audit tests that still reflect outdated string-ID assumptions.
- Audit tests that mostly prove compile-time trait usability instead of behavior.
- Add end-to-end MoE scenarios covering:
  - specialized expert routing
  - multi-expert aggregation
  - trace to dataset conversion
  - correction to bundle propagation
  - governance protection
  - persistence round-trip
  - concurrency semantics
- Separate tests for:
  - production semantics
  - diagnostic semantics
  - bootstrap/synthetic semantics

### Deliverables

- testing strategy note
- updated semantic test suites
- end-to-end MoE flow coverage

### Acceptance criteria

- Tests fail when a real MoE behavior regresses.
- Tests do not overfit placeholder implementation details.

---

## Concrete Backlog By Module

This section converts the product workstreams into an implementation-oriented backlog.

### `backend/src/echo_expert.rs`

- Decide whether `EchoExpert` remains:
  - demo-only
  - test-only
  - fallback-only
- If it stays, document its scope explicitly.
- Ensure production-facing paths do not silently rely on it unless that is intentional.

### `backend/src/router/`

- Audit the real behavior of `HeuristicRouter`.
- Document the exact decision inputs it uses today.
- Expand routing traces to expose real decision rationale.
- Decide whether router outputs should carry richer scoring details.

### `backend/src/aggregator/`

- Identify which aggregation strategies are truly meaningful.
- Add scenarios that prove aggregation uplift.
- Prevent aggregation from hiding weak routing choices.

### `backend/src/dataset_engine/`

- Audit what one `DatasetEntry` means product-wise.
- Clarify whether add/update behavior models ingestion or overwrite.
- Review correction propagation into training bundle generation.
- Review shard and rebuild semantics for provenance and reproducibility.

### `backend/src/trace_logging/`

- Verify that trace records capture enough decision context.
- Ensure traces can explain:
  - routing
  - expert execution
  - aggregation
  - policy rejection
  - improvement decisions

### `backend/src/evaluations/`

- Distinguish infrastructure metrics from MoE value metrics.
- Track routing usefulness, not only counts.
- Track expert usefulness by task family.

### `backend/src/orchestrator/`

- Verify that promoted or imported state affects runtime behavior.
- Ensure governance and auto-improvement semantics are aligned.
- Keep runtime persistence semantically honest: persisted data must represent real state transitions.

### `backend/src/apps/impl_check.rs`

- Rewrite the command around named capabilities and explicit failure meaning.
- Remove ambiguity between diagnostic semantics and runtime semantics.
- Decide what the concurrent dataset phase is actually validating.

### `backend/src/app.rs`

- Keep the CLI focused on operator semantics, not only technical access to features.
- Ensure each command communicates a guarantee, not just an action.

---

## 30-60-90 Execution Plan

### 30 days

- finalize expert catalog
- audit current router behavior
- classify dataset paths by semantics
- rewrite `impl-check` as a capability diagnostic
- define operational guarantees for `status`, `metrics`, `/healthz`, and `/readyz`

### 60 days

- replace or isolate placeholder experts in runtime-facing flows
- enrich routing traces and evaluation metrics
- tighten aggregation semantics
- align dataset provenance with actual runtime diversity
- ensure governance messages are operator-readable

### 90 days

- demonstrate at least one strong end-to-end MoE scenario
- show measurable difference between expert choices
- show meaningful aggregation uplift for some task family
- show auto-improvement influencing promoted runtime behavior
- expose trustworthy operational diagnostics and metrics

---

## Recommended Execution Order

### Phase 1: Define product semantics

- define expert families
- define routing goals
- define aggregation goals
- define dataset semantics
- define operational guarantees

### Phase 2: Upgrade runtime semantics

- replace or isolate placeholder experts
- strengthen router inputs and traces
- tighten aggregation behavior
- clarify concurrency semantics

### Phase 3: Upgrade data and improvement loop

- audit trace-to-dataset flow
- enforce dataset semantics
- validate bundle provenance
- ensure promotions affect runtime

### Phase 4: Upgrade diagnostics and operations

- refactor `cmd_impl_check` around capabilities
- tighten CLI guarantee semantics
- align health/ready/metrics meaning
- improve governance and audit messaging

### Phase 5: Lock it with evaluation and tests

- define MoE usefulness metrics
- add end-to-end product-semantic tests
- ensure observability and reports explain behavior

---

## Short-Term Next Actions

These are the highest-value next actions if work starts immediately:

1. Write an explicit expert catalog and assign target responsibility boundaries.
2. Audit the current router and document what it actually uses versus what it should use.
3. Review `cmd_impl_check` block by block and rewrite it as a capability diagnostic.
4. Audit dataset-generation paths and mark each one as real runtime, diagnostic, or synthetic.
5. Define whether repeated `ProtocolId::default()` use is acceptable on each dataset-related path.
6. Add at least one end-to-end scenario where multiple experts and aggregation matter.
7. Define what counts as a successful auto-improvement promotion in product terms.
8. Define operational guarantees for `status`, `metrics`, `/healthz`, and `/readyz`.

---

## Suggested First Real Deliverable

If only one thing is tackled first, it should be this:

- define the first real expert trio
- define the routing policy for that trio
- define one task family where the trio matters
- define how aggregation helps or when it must not be used
- run that flow through trace, dataset, evaluation, and improvement

This is the smallest slice that can prove the project is becoming a real MoE instead of a generic modular pipeline.

---

## Success Criteria

The project can be called a real MoE when the following become true at the same time:

- experts are specialized in a way that changes runtime behavior
- routing chooses experts for explainable product reasons
- aggregation improves at least some task classes
- datasets represent diverse real executions
- improvement affects promoted runtime behavior
- governance protects real release semantics
- `impl-check` validates real MoE capabilities
- operational endpoints expose trustworthy guarantees
- evaluation shows whether the MoE is useful, not only alive

Until then, the platform should be treated as a strong MoE foundation that still needs semantic hardening.
