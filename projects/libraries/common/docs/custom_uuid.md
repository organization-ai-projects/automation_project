# `Id128` Documentation

## Overview

`Id128` is a 128-bit identifier designed for efficient generation and uniqueness. It is suitable for distributed systems, ensuring that IDs are time-ordered and collision-resistant.

## Structure

An `Id128` consists of the following components:

- **48-bit Timestamp**: Represents the number of milliseconds since the Unix epoch.
- **16-bit Node ID**: A unique identifier for the machine or cluster.
- **16-bit Process ID**: A per-process identifier, either derived from the PID or randomly generated.
- **16-bit Boot ID**: Changes with each program start or run.
- **32-bit Sequence**: Ensures uniqueness within the same millisecond.

## Key Methods

### Creation

- `Id128::new(node_id, boot_id, process_id)`
  - Generates a new ID with optional `boot_id` and `process_id`. If not provided, these are randomly generated.

- `Id128::new_with_params(node_id, boot_id, process_id)`
  - Generates a new ID with explicit parameters.

### Conversion

- `to_hex(&self) -> String`
  - Converts the ID to a 32-character lowercase hexadecimal string.

- `from_hex(s: &str) -> Result<Id128, IdError>`
  - Parses a hexadecimal string into an `Id128`.

### Extraction

- `timestamp_ms(&self) -> u64`
  - Extracts the 48-bit timestamp in milliseconds.

- `node_id(&self) -> u16`
  - Retrieves the Node ID.

- `process_id(&self) -> u16`
  - Retrieves the Process ID.

- `boot_id(&self) -> u16`
  - Retrieves the Boot ID.

- `seq(&self) -> u32`
  - Retrieves the sequence number.

## Error Handling

`IdError` is used to handle errors during ID parsing:

- `InvalidLen`: The input string does not have the correct length.
- `InvalidHex`: The input string contains invalid hexadecimal characters.

## Thread Safety

`Id128` ensures thread safety for monotonic timestamp generation and sequence numbers using atomic operations and mutexes.

## Example

```rust
use common::custom_uuid::Id128;

fn main() {
    let id = Id128::new(42, None, None);
    println!("Generated ID: {}", id.to_hex());

    let parsed = Id128::from_hex(&id.to_hex()).expect("valid hex id");
    assert_eq!(id, parsed);

    println!("Timestamp: {}", id.timestamp_ms());
    println!("Node ID: {}", id.node_id());
    println!("Process ID: {}", id.process_id());
    println!("Boot ID: {}", id.boot_id());
    println!("Sequence: {}", id.seq());
}
```

## License

This module is licensed under the MIT License.
