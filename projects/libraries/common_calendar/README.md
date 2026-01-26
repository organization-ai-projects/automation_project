# Common Calendar Library

Calendar utilities for `automation_project`.

## Overview

This library provides date and calendar utilities including leap year detection, month day counting, and date parsing using chrono.

## Features

- **Leap Year Detection** - Check if a year is a leap year
- **Month Day Count** - Get the number of days in any month
- **Date Parsing** - Parse date strings in `YYYY-MM-DD` format

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

- [Documentation Index](https://github.com/organization-ai-projects/automation_project/blob/main/projects/libraries/common_calendar/documentation/TOC.md)

## Contributing

See the workspace README and contribution guide:

- [Workspace README](https://github.com/organization-ai-projects/automation_project/blob/main/README.md)
- [Contributing](https://github.com/organization-ai-projects/automation_project/blob/main/CONTRIBUTING.md)
