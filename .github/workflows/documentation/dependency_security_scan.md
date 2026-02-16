# dependency_security_scan.yml Documentation

## Purpose

Defines dependency vulnerability scanning policy in CI using `cargo audit`.

## Triggers

- Pull requests and pushes that change dependency manifests/lockfiles.
- Weekly scheduled run.
- Manual trigger via `workflow_dispatch`.

## Policy

- **Tool**: `cargo audit`
- **Failure policy**: report-only (non-blocking)
- **Rationale**: provide visibility without blocking delivery while vulnerability triage policy is being stabilized.

## Operational Notes

- Warnings in this workflow should result in issue creation or update with remediation plan.
- Blocking policy can be introduced later once triage SLAs and ownership are formalized.
