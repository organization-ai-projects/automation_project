# Common Time Library

Time and duration utilities for `automation_project`.

## Overview

This library provides time-related utilities including clocks, timeouts, backoff strategies, and duration constants.

## Features

- **Clock Abstraction** - Testable clock trait with system and fake implementations
- **Backoff** - Exponential backoff for retry logic
- **TimeSpan** - Duration wrapper with convenient operations
- **Timeout** - Async timeout utilities
- **Constants** - Pre-defined duration constants (ONE_MINUTE, ONE_HOUR, etc.)

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
common_time = { path = "../common_time" }
```

## Usage

### Exponential backoff

```rust
use common_time::{Backoff, TimeSpan};

let backoff = Backoff::default();

// Get delay for each retry attempt
let delay0 = backoff.for_attempt(0); // 1 minute
let delay1 = backoff.for_attempt(1); // 2 minutes
let delay2 = backoff.for_attempt(2); // 4 minutes
// ... exponential up to max (30 minutes)
```

### Clock abstraction (for testing)

```rust
use common_time::{Clock, SystemClock, FakeClock};

// Production: use real system time
let clock = SystemClock;
let now = clock.now();

// Testing: use controllable fake clock
let fake = FakeClock::new();
fake.advance(Duration::from_secs(60));
```

### Duration constants

```rust
use common_time::{ONE_MINUTE, ONE_HOUR, ONE_DAY, ONE_WEEK};

let timeout = ONE_MINUTE;
let cache_ttl = ONE_HOUR;
let retention = ONE_WEEK;
```

### Timeout wrapper

```rust
use common_time::with_timeout;
use std::time::Duration;

async fn example() {
    let result = with_timeout(Duration::from_secs(5), async {
        // async operation
    }).await;
}
```

## License

This project is licensed under the MIT License. See [License](https://github.com/organization-ai-projects/automation_project/blob/main/LICENSE).

## Documentation

- [Documentation Index](https://github.com/organization-ai-projects/automation_project/blob/main/projects/libraries/common_time/documentation/TOC.md)

## Contributing

See the workspace README and contribution guide:

- [Workspace README](https://github.com/organization-ai-projects/automation_project/blob/main/README.md)
- [Contributing](https://github.com/organization-ai-projects/automation_project/blob/main/CONTRIBUTING.md)
