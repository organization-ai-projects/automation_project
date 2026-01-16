# Protocol Macros

**A robust and powerful procedural macro library for generating enum constructors and Display implementations.**

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

- [Implementation Details](docs/implementation.md)
- [Migration Guide](docs/migration_guide.md)
- [Usage Examples](docs/usage_examples.md)
- [Contributing](docs/contributing.md)

## Contribuer

Les contributions sont les bienvenues ! Veuillez ouvrir une issue ou une pull request sur le dépôt GitHub.

Pour plus de détails sur le workflow Git/GitHub utilisé dans ce projet, consultez la [documentation sur le versioning](../../../docs/versioning/git-github.md).

## Licence

Part of the automation project workspace.
