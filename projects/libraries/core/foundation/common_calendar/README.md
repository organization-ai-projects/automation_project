# Common Calendar Library Documentation

This directory contains calendar utilities for the automation project.

## Role in the Project

This library is responsible for providing date and calendar utilities across the automation project. It includes leap year detection, month day counting, and date parsing functionality.

It interacts mainly with:

- Common time library - For timestamp operations
- Various products - For date handling and scheduling

## Directory Structure

```
common_calendar/
├── Cargo.toml          # Package configuration
├── README.md           # This file
├── documentation/      # Additional documentation
│   └── TOC.md
└── src/               # Source code
    ├── lib.rs
    └── ...
```

## Files

- `README.md`: This file.
- `Cargo.toml`: Package configuration.
- `documentation/`: Additional documentation.
- `src/`: Source code.


## Overview

This library provides date and calendar utilities including leap year detection, month day counting, and date parsing using chrono.

## Features

- **Leap Year Detection** - Check if a year is a leap year
- **Month Day Count** - Get the number of days in any month
- **Date Parsing** - Parse date strings in `YYYY-MM-DD` format

## Examples

### Leap Year Detection

```rust
use common_calendar::Calendar;
assert!(Calendar::is_leap_year(2024));
assert!(!Calendar::is_leap_year(2023));
```

### Days in Month

```rust
use common_calendar::Calendar;

assert_eq!(Calendar::days_in_month(2024, 2), Some(29)); // Leap year
assert_eq!(Calendar::days_in_month(2023, 2), Some(28)); // Normal year
```

### Date Parsing

```rust
use common_calendar::Calendar;

let date = Calendar::parse_date("2024-01-15");
assert!(date.is_some());
```

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
common_calendar = { path = "../common_calendar" }
```

## Usage

```rust
use common_calendar::Calendar;

// Check leap year
assert!(Calendar::is_leap_year(2024));
assert!(!Calendar::is_leap_year(2023));

// Get days in month
assert_eq!(Calendar::days_in_month(2024, 2), Some(29)); // Leap year
assert_eq!(Calendar::days_in_month(2023, 2), Some(28)); // Normal year
assert_eq!(Calendar::days_in_month(2024, 1), Some(31)); // January

// Parse date string
let date = Calendar::parse_date("2024-01-15");
assert!(date.is_some());
```

## License

This project is licensed under the MIT License. See [License](https://github.com/organization-ai-projects/automation_project/blob/main/LICENSE).

## Documentation

- [Documentation Index](https://github.com/organization-ai-projects/automation_project/blob/main/projects/libraries/core/foundation/common_calendar/documentation/TOC.md)

## Contributing

See the workspace README and contribution guide:

- [Workspace README](https://github.com/organization-ai-projects/automation_project/blob/main/README.md)
- [Contributing](https://github.com/organization-ai-projects/automation_project/blob/main/CONTRIBUTING.md)
