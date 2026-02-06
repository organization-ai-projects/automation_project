# Example Report Generator (MVP)

A minimal viable product demonstrating unstable product structure and compliance requirements.

## Purpose

This is an example MVP that generates simple reports from data files. It demonstrates:
- How to structure an unstable product
- How to document architectural violations
- How to define exit criteria for promotion to stable

## Architectural Compliance

### Current Violations

1. **Direct filesystem access**: Bypasses the Engine protocol for file operations
   - **Reason**: Faster MVP development for proof-of-concept
   - **Impact**: Limited to read-only operations in `/tmp/reports_input` and write to `/tmp/reports_output`
   - **Risk**: Low - isolated to dedicated directories with no sensitive data

2. **No registry integration**: Product doesn't register with Engine
   - **Reason**: Testing report generation logic before protocol integration
   - **Impact**: Cannot be discovered or managed by central UI
   - **Risk**: Medium - requires manual execution

3. **No permission checks**: No authentication or authorization
   - **Reason**: MVP focused on core functionality only
   - **Impact**: Anyone with filesystem access can run reports
   - **Risk**: Medium - should add before production use

4. **No structured logging**: Uses println! for debug output
   - **Reason**: Simple debugging during development
   - **Impact**: Logs not captured by central logging system
   - **Risk**: Low - easy to fix

### Exit Criteria

To promote this product to stable, the following must be completed:

- [ ] **Protocol integration**: Implement Engine protocol for file access
  - Use `protocol` library for message passing
  - Register with Engine on startup
  - Handle Engine shutdown signals

- [ ] **Permission system**: Integrate with identity/security libraries
  - Check user permissions before generating reports
  - Audit log all report generation activities

- [ ] **Structured logging**: Replace println! with tracing
  - Use workspace `tracing` dependency
  - Emit structured events for monitoring

- [ ] **Configuration**: Use protocol for configuration instead of direct file reads
  - Remove hardcoded paths
  - Accept configuration from Engine

- [ ] **Testing**: Add comprehensive test suite
  - Unit tests for report generation logic
  - Integration tests with mock Engine
  - Target: >80% code coverage

- [ ] **Documentation**: Follow standardized docs pattern
  - Add `documentation/TOC.md`
  - Add `documentation/usage.md`
  - Update this README to stable format

- [ ] **Error handling**: Improve error handling and recovery
  - Use `thiserror` for structured errors
  - Handle all error cases gracefully
  - Return errors through protocol

### Target Promotion Date

**Q2 2026** - Contingent on core protocol stabilization

## Standardized Output

This product produces output in the **structured JSON file** format:

**Output file**: `/tmp/reports_output/run_report.json`

**Format**:
```json
{
  "product": "example_report_generator",
  "version": "0.1.0",
  "run_id": "uuid-v4",
  "timestamp": "2026-02-05T22:00:00Z",
  "status": "success|failure",
  "output": {
    "reports_generated": 3,
    "files": [
      "/tmp/reports_output/report1.json",
      "/tmp/reports_output/report2.json",
      "/tmp/reports_output/report3.json"
    ]
  },
  "errors": []
}
```

## Usage (Current MVP)

Since this is an unstable MVP, it's run directly from the command line:

```bash
cd projects/products/unstable/example_report_generator/backend
cargo run -- --input /tmp/reports_input --output /tmp/reports_output
```

## Future (Stable)

Once promoted to stable, this product will:
- Be launched and managed by the Engine
- Integrate with the central UI
- Follow all architectural patterns
- Support dynamic configuration
- Emit structured telemetry

## Development

This is a minimal example. For a real unstable product:
1. Copy this structure
2. Update the README with your specific violations and exit criteria
3. Implement your MVP functionality
4. Follow the promotion path when ready

## Contributing

See [CONTRIBUTING.md](../../../../CONTRIBUTING.md) for general guidelines.
