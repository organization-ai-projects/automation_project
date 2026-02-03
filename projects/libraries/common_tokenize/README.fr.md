# Common Tokenize Library

Text tokenization utilities for `automation_project`.

## Overview

This library provides text tokenization utilities for splitting text into tokens.

## Status

This library is under development. Basic whitespace tokenization is available.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
common_tokenize = { path = "../common_tokenize" }
```

## Usage

```rust
use common_tokenize::tokenize_example;

let tokens = tokenize_example("hello world foo bar");
// Returns: ["hello", "world", "foo", "bar"]
```

## Examples

### Basic Tokenization

```rust
use common_tokenize::tokenize_example;
let tokens = tokenize_example("hello world foo bar");
assert_eq!(tokens, vec!["hello", "world", "foo", "bar"]);
```

## License

This project is licensed under the MIT License. See [License](https://github.com/organization-ai-projects/automation_project/blob/main/LICENSE).

## Documentation

- [Index de Documentation](https://github.com/organization-ai-projects/automation_project/blob/main/projects/libraries/common_tokenize/documentation/en/TOC.md)

## Contributing

See the workspace README and contribution guide:

- [Workspace README](https://github.com/organization-ai-projects/automation_project/blob/main/README.md)
- [Contributing](https://github.com/organization-ai-projects/automation_project/blob/main/CONTRIBUTING.md)
