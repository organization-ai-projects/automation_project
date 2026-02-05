# Products Directory Structure

This directory contains all product implementations organized into stable and unstable categories.

## Directory Structure

```
projects/products/
  stable/
    <product_x>/
      backend/
      ui/               (optional)
      metadata.ron      (optional)
      README.md
  unstable/
    <product_x>/
      backend/
      ui/               (optional)
      metadata.ron      (optional in phase 1)
      README.md
```

## Stable vs Unstable

### Stable Products

**Location**: `projects/products/stable/`

Stable products are production-ready implementations that follow all project principles and best practices:
- Follow the architecture patterns defined in `documentation/technical_documentation/ARCHITECTURE.md`
- Integrate with the Engine through the protocol
- Use the security and identity libraries for authentication and authorization
- Follow proper error handling and logging patterns
- Are thoroughly tested and documented
- Have well-defined APIs and contracts

Current stable products:
- `core/` - Core system binaries (engine, launcher, watcher, central UI)
- `accounts/` - User account management
- `varina/` - Varina product
- `code_agent_sandbox/` - Code agent sandbox environment

### Unstable Products

**Location**: `projects/products/unstable/`

Unstable products are MVP (Minimum Viable Product) implementations designed for rapid experimentation and iteration. They are allowed to take shortcuts and break architectural principles for speed of development, but must clearly document these violations.

## Rules

### Rule 1: Unstable Can Break Principles, But Must Declare It

Every unstable product **MUST** include a `README.md` with a **"Architectural Compliance"** section that explicitly declares:

1. **Allowed violations**: What principles or patterns are being violated and why
   - Examples: "Direct filesystem access instead of protocol", "No registry integration", "No permission checks"
   
2. **Exit criteria**: What needs to be done to promote to stable
   - Examples: "Implement protocol integration", "Add permission checks", "Refactor to use registry"
   
3. **Target date** (optional): When the product should be promoted or sunset

**Example compliance section:**

```markdown
## Architectural Compliance

### Current Violations

1. **Direct filesystem access**: Bypasses the Engine protocol for file operations
   - Reason: Faster MVP development for proof-of-concept
   - Impact: Limited to read-only operations in `/tmp/reports`

2. **No registry integration**: Product doesn't register with Engine
   - Reason: Testing UX patterns before protocol integration
   - Impact: Cannot be discovered by central UI

### Exit Criteria

- [ ] Implement protocol-based file access
- [ ] Integrate with Engine registry
- [ ] Add permission checks for file operations
- [ ] Complete end-to-end testing with Engine
- [ ] Document API in `documentation/`

### Target Promotion Date

2026-Q2
```

### Rule 2: No Contamination from Unstable to Stable

**Stable products MUST NOT depend on unstable products.**

This rule ensures that experimental, unvetted code doesn't leak into production systems. Dependencies flow in one direction only:

```
unstable → stable ✓ (allowed: unstable can use stable)
stable → unstable ✗ (blocked: stable cannot use unstable)
```

**Enforcement:**
- Manual code review (required)
- CI dependency checker (optional, see `.github/workflows/check_stable_deps.yml`)

**Exception handling:**
- If an unstable feature is needed by stable code, promote the feature to stable first
- Extract the needed functionality into a stable library if appropriate

### Rule 3: Unstable Has Standardized Output Boundary

Each unstable product must produce output in a standardized format to ensure observability and debugging:

**Required output format** (choose one):

1. **Structured JSON output**: `run_report.json` with standard fields
   ```json
   {
     "product": "example_mvp",
     "version": "0.1.0",
     "run_id": "uuid",
     "timestamp": "2026-02-05T22:00:00Z",
     "status": "success|failure",
     "output": { ... },
     "errors": [ ... ]
   }
   ```

2. **Artifact directory**: `artifacts/<run_id>/` with structured subdirectories
   ```
   artifacts/
     <run_id>/
       metadata.json
       outputs/
       logs/
   ```

3. **Structured stdout**: JSON lines format for streaming output
   ```json
   {"type": "start", "timestamp": "...", "product": "..."}
   {"type": "log", "level": "info", "message": "..."}
   {"type": "result", "data": {...}}
   {"type": "end", "status": "success"}
   ```

This standardization enables:
- Automated testing and validation
- Centralized log aggregation
- Debugging and troubleshooting
- Performance monitoring

## Promotion Path: Unstable → Stable

To promote an unstable product to stable:

1. **Complete exit criteria**: All items in the README must be addressed
2. **Code review**: Full architectural review by team
3. **Documentation**: Add comprehensive docs following patterns in `CONTRIBUTING.md`
4. **Testing**: Add integration tests and achieve target coverage
5. **Protocol integration**: Ensure Engine integration is complete
6. **Security review**: Verify security patterns are followed
7. **Move files**: Relocate from `unstable/` to `stable/`
8. **Update workspace**: Update `Cargo.toml` and dependencies
9. **CI update**: Ensure product is included in CI pipelines

## Creating a New Unstable Product

1. Create product directory: `projects/products/unstable/<product_name>/`
2. Add basic structure: `backend/`, `ui/` (if needed), `README.md`
3. Write `README.md` with Architectural Compliance section
4. Add to workspace: Update root `Cargo.toml` if using Rust
5. Start building!

## CI Checks

The repository includes optional CI checks to enforce these rules:

- **Dependency check**: Prevents stable products from depending on unstable
  - Workflow: `.github/workflows/check_stable_deps.yml`
  - Can be bypassed in emergencies with approval

## Questions?

For questions about product structure, architecture, or promotion process:
- See [CONTRIBUTING.md](../../CONTRIBUTING.md)
- See [ARCHITECTURE.md](../../documentation/technical_documentation/ARCHITECTURE.md)
- Open an issue for discussion
