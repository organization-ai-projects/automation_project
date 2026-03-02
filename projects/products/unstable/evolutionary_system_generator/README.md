# evolutionary_system_generator

A 3-binary Rust project implementing an evolutionary search engine with terminal UI and validation tooling.

## Binaries

- **evo-backend** (`backend/`): Evolutionary search engine. Reads JSON requests from stdin, writes JSON responses to stdout. Fully deterministic given the same seed.
- **evo-ui** (`ui/`): Terminal UI that spawns and communicates with the backend via IPC.
- **evo-tooling** (`tooling/`): Validation CLI tools for determinism and replay checks.

## Architecture

### Backend

The backend implements a genetic algorithm with the following components:

- **Seed / XorShift64**: Seeded deterministic PRNG for reproducible results.
- **Genome**: A list of named rules with integer weights.
- **Mutation**: Three mutation operators (TweakWeight, SwapWeights, ZeroRule).
- **Crossover**: Uniform crossover between two parent genomes.
- **Constraints**: MinActiveRules, MaxTotalWeight, RequiredRule.
- **Evaluator**: Computes fitness from rule score, constraint satisfaction, and diversity.
- **EvolutionEngine**: Manages population, selection, evolution, and event logging.
- **ReplayEngine**: Replays a search from a saved event log to verify determinism.
- **CandidateManifest**: Top-N genomes with a SHA-256 hash for verification.

### Protocol (stdin/stdout JSON)

Request types (send as a single JSON line):

```json
{"type":"NewSearch","seed":42,"population_size":10,"max_generations":5,"rule_pool":["rule_a","rule_b"],"constraints":[]}
{"type":"StepGen"}
{"type":"RunToEnd"}
{"type":"GetCandidates","top_n":5}
{"type":"SaveReplay","path":"/tmp/replay.json"}
{"type":"LoadReplay","path":"/tmp/replay.json","rule_pool":["rule_a","rule_b"],"constraints":[]}
{"type":"ReplayToEnd"}
```

Response types:

```json
{"type":"Ok"}
{"type":"Error","message":"..."}
{"type":"Report","generation":3,"best_fitness":0.85,"population_size":10,"done":true}
{"type":"Candidates","manifest":{...}}
```

## Usage

### Run the backend interactively

```bash
cargo run -p evolutionary_system_generator_backend
```

### Run the UI

```bash
cargo run -p evolutionary_system_generator_ui
```

### Validate determinism

```bash
cargo run -p evolutionary_system_generator_tooling -- validate-determinism --seed 42 --generations 5
```

### Validate replay

```bash
cargo run -p evolutionary_system_generator_tooling -- validate-replay --seed 42 --generations 5 --replay-path /tmp/replay.json
```

## Testing

```bash
cargo test -p evolutionary_system_generator_backend -p evolutionary_system_generator_tooling
```
