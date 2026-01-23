# Migration Guide

This document provides a step-by-step guide for migrating from the old `macro_rules!`-based implementation to the new `protocol_macros` procedural macro.

## Why Migrate?

The old `macro_rules!` implementation had several limitations:

- Could not handle struct or tuple variants.
- Poor error messages.
- Limited extensibility.

The new procedural macro resolves these issues and provides additional features like debug mode and generics support.

## Steps to Migrate

1. Add `protocol_macros` to your `Cargo.toml`:

   ```toml
   [dependencies]
   protocol_macros = { path = "../protocol_macros" }
   ```

2. Replace `macro_rules!` usage with `#[derive(EnumMethods)]`:

   **Before:**

   ```rust
   generate_enum_methods!(EventVariant,
       acknowledged => Acknowledged { id: String },
       created => Created { id: String, data: String },
   );
   ```

   **After:**

   ```rust
   use protocol_macros::EnumMethods;

   #[derive(EnumMethods)]
   enum EventVariant {
       Acknowledged { id: String },
       Created { id: String, data: String },
   }
   ```

3. Run `cargo build` to verify the changes.

4. Update any tests to use the new constructors and methods.
