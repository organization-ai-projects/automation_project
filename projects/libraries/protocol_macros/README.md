# Protocol Macros Documentation

This directory contains a robust and powerful procedural macro library for generating enum constructors and Display implementations.

## Role in the Project

This library is responsible for providing procedural macros that generate constructor methods, Display, and as_str() implementations for enums across the automation project. It focuses on correctness, compile-time safety, and clean error messages.

It interacts mainly with:

- Protocol library - For protocol enums
- Various products - For enum convenience methods

## Directory Structure

```
protocol_macros/
├── Cargo.toml          # Package configuration
├── README.md           # This file
├── documentation/      # Additional documentation
│   ├── TOC.md
│   ├── implementation.md
│   ├── migration_guide.md
│   └── usage_examples.md
└── src/               # Source code
    ├── lib.rs
    └── ...
```

## Files

- `README.md`: This file.
- `Cargo.toml`: Package configuration.
- `documentation/`: Additional documentation.
- `src/`: Source code.
- `tests/`: Tests.


## Overview

The `protocol_macros` crate is a **procedural macro crate** that generates constructor methods, `Display`, and `as_str()` implementations for enums. The crate focuses on correctness, compile-time safety, and clean error messages, making it suitable for production APIs and internal tooling alike.

## Features

- 🚀 **Automatic Constructor Generation**
- 📝 **Smart Display Implementation**
- 🎯 **Full Variant Support**
- 🔍 **Debug Mode**
- ✨ **as_str() Method**
- 🔤 **Premium Snake Case**
- 🛡️ **Collision Detection**
- ⚡ **Zero Runtime Cost**
- 📚 **Well Documented**

## Quick Start

### Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
protocol_macros = { path = "../protocol_macros" }
```

### Basic Usage

```rust
use protocol_macros::EnumMethods;

#[derive(Debug, Clone, EnumMethods)]
enum Event {
    Ping,
    Created { id: String, data: String },
    Data(String, u32),
}

let ping = Event::ping();
let created = Event::created("id".to_string(), "data".to_string());
let data = Event::data("info".to_string(), 42);

assert_eq!(ping.to_string(), "ping");
assert_eq!(created.to_string(), "created { id=id, data=data }");
assert_eq!(data.to_string(), "data(arg0=info, arg1=42)");
```

### Debug Mode

```rust
#[derive(EnumMethods)]
#[enum_methods(mode = "debug")]
enum BinaryEvent {
    Data(Vec<u8>),
}
```

Debug mode uses `{:?}` formatting for all fields.

## Documentation

For more details, refer to the following documents:

- [Implementation Details](https://github.com/organization-ai-projects/automation_project/blob/main/projects/libraries/protocol_macros/documentation/implementation.md)
- [Migration Guide](https://github.com/organization-ai-projects/automation_project/blob/main/projects/libraries/protocol_macros/documentation/migration_guide.md)
- [Usage Examples](https://github.com/organization-ai-projects/automation_project/blob/main/projects/libraries/protocol_macros/documentation/usage_examples.md)

## Contribute

Contributions are welcome! Please open an issue or a pull request on the GitHub repository.

## License

This project is licensed under the MIT License. See [License](https://github.com/organization-ai-projects/automation_project/blob/main/LICENSE).

## Contributing

See the workspace README and contribution guide:

- [Workspace README](https://github.com/organization-ai-projects/automation_project/blob/main/README.md)
- [Contributing](https://github.com/organization-ai-projects/automation_project/blob/main/CONTRIBUTING.md)
