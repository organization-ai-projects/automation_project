# GitHub Templates Documentation

This directory contains documentation for issue and pull request templates defined in `.github/`.

## Role in the Project

This directory documents template conventions and maintenance rules for GitHub contribution flows.
It interacts mainly with:

- `.github/PULL_REQUEST_TEMPLATE/`: Pull request templates used by GitHub
- `.github/ISSUE_TEMPLATE/`: Issue templates and template chooser configuration
- `.github/documentation/`: Parent documentation for GitHub-specific conventions

## Directory Structure

```plaintext
.github/documentation/templates/
├── README.md                            # This file
├── TOC.md                               # Templates documentation index
├── overview.md                          # How templates and generator work together
├── pr_template.md                       # Documentation for PR template
├── issue_template_direct_issue.md       # Documentation for direct issue template
├── issue_template_review_followup.md    # Documentation for review follow-up template
└── issue_template_config.md             # Documentation for issue template config
```

## Files

- `README.md`: Main documentation for templates and conventions.
- `TOC.md`: Documentation index for templates.
- `overview.md`: Global articulation between templates and PR generator behavior.
- `pr_template.md`: Pull request template purpose and conventions.
- `issue_template_direct_issue.md`: Direct issue template purpose and conventions.
- `issue_template_review_followup.md`: Review follow-up template purpose and conventions.
- `issue_template_config.md`: Issue template chooser configuration and expected behavior.

## Source of Truth

Template definitions are maintained in:

- `.github/PULL_REQUEST_TEMPLATE/default.md`
- `.github/ISSUE_TEMPLATE/direct_issue.md`
- `.github/ISSUE_TEMPLATE/review_followup.md`
- `.github/ISSUE_TEMPLATE/config.yml`

This directory documents how those templates are organized and used.

For issue decomposition strategy (parent/sub-issues vs standalone issues), see:

- `.github/documentation/issue_decomposition_conventions.md`

## Conventions

- PR template placeholders use editable `<...>` syntax.
- Generated PR bodies use rendered values with backticks.
- Expected contract is section and semantic alignment between template and generator, not identical placeholder syntax.

## Documentation

- See [Templates TOC](TOC.md) for the documentation index.
