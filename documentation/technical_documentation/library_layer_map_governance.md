# Library Layer Map Governance

## Purpose

Define governance rules for the canonical workspace `crate -> layer` mapping artifact used by strict layer checks.

## Canonical Artifact

- Primary artifact: `scripts/checks/layer_map.txt`
- Format: `crate_name=L0|L1|L2|L3|UNMAPPED`
- Scope: workspace crates under `projects/libraries/`

## Ownership

- Primary owners: maintainers responsible for architecture/layering checks.
- Contributors may propose updates through PRs, but ownership review is required.

## Update Policy

Any `layer_map.txt` change must be explicit and reviewable:

1. Explain the reason for each changed crate mapping.
2. Reference the issue/decision that justifies the change.
3. Keep one logical decision per commit when possible.
4. Avoid broad remapping without a migration plan.

## Validation Requirements

When updating the map:

1. Run analysis:

```bash
./scripts/checks/analyze_layer_anomalies.sh --map-file scripts/checks/layer_map.txt
```

1. Confirm map completeness (no missing workspace libraries).
2. Confirm no malformed entries (only `L0|L1|L2|L3|UNMAPPED`).
3. Capture key anomalies impacted by the mapping change in the PR description.

## UNMAPPED Policy

- `UNMAPPED` is allowed only as a temporary decision state.
- New crates must be mapped before strict enforcement is enabled for their path.
- `UNMAPPED` entries require a follow-up issue with owner and expected resolution window.

## Exception and Whitelist Alignment

- Map decisions do not replace whitelist governance.
- If a mapped crate still needs a forbidden edge temporarily, use a governed whitelist entry.
- Each whitelist exception must include reason, owner, and review/expiry date.

## Review Checklist

Before merging a map update, reviewers should confirm:

1. Mapping changes are justified and scoped.
2. Layer model remains consistent with `library_layer_taxonomy.md`.
3. Checker behavior assumptions in `library_layer_boundaries.md` still hold.
4. Related migration issues are linked and actionable.
