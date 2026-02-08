// Example: Using ast_macros to build AST nodes
//
// This example demonstrates the basic usage of ast_macros
// to construct AST nodes with minimal boilerplate.

use ast_core::AstKind;
use ast_macros::{build_array, build_object, key, value};

fn main() {
    println!("=== ast_macros Examples ===\n");

    // Example 1: Simple values
    println!("1. Simple values:");
    let null = value!(null);
    let bool_val = value!(true);
    let num = value!(42);
    println!("   null: {:?}", null.kind);
    println!("   bool: {:?}", bool_val.kind);
    println!("   number: {:?}", num.kind);

    // Example 2: Arrays
    println!("\n2. Arrays:");
    let simple_array = build_array!([1, 2, 3]);
    println!("   Simple array: {:?}", simple_array.kind);

    let mixed_array = value!([null, true, 42, "text"]);
    println!("   Mixed array: {:?}", mixed_array.kind);

    // Example 3: Objects
    println!("\n3. Objects:");
    let simple_obj = build_object!({
        name: "example",
        count: 10
    });
    if let AstKind::Object(fields) = &simple_obj.kind {
        println!("   Object with {} fields", fields.len());
    }

    // Example 4: Nested structures
    println!("\n4. Nested structures:");
    let _nested = value!({
        user: {
            name: "Alice",
            age: 30,
            active: true
        },
        tags: ["admin", "user"],
        metadata: {
            created: 1234567890,
            version: 1
        }
    });
    println!("   Complex nested structure created successfully");

    // Example 5: Keys
    println!("\n5. Keys:");
    let _ident_key = key!(field_name);
    let _string_key = key!("string-key");
    let dynamic = "dynamic";
    let _expr_key = key!((dynamic));
    println!("   Created 3 different types of keys");

    println!("\n=== All examples completed successfully! ===");
}
