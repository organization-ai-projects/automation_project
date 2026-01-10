//! Basic usage example of the EnumMethods derive macro
//!
//! Run this example with:
//! cargo run --example basic_usage -p protocol_macros

use protocol_macros::EnumMethods;

/// A simple event enum demonstrating all variant types
#[derive(Debug, Clone, EnumMethods)]
enum Event {
    /// Unit variant - simple state
    Ping,
    Pong,

    /// Struct variant - with named fields
    UserCreated {
        id: String,
        username: String,
        email: String,
    },

    /// Tuple variant - with positional fields
    MessageReceived(String, u64),

    /// Complex struct variant
    FileUploaded {
        file_id: String,
        filename: String,
        size_bytes: u64,
        uploaded_by: String,
    },
}

/// Example using debug mode for types without Display
#[derive(Debug, Clone, EnumMethods)]
#[enum_methods(mode = "debug")]
enum BinaryData {
    Raw(Vec<u8>),
    Encoded { algorithm: String, data: Vec<u8> },
    Empty,
}

fn main() {
    println!("=== EnumMethods Derive Macro Examples ===\n");

    // Example 1: Unit variants
    println!("1. Unit Variants:");
    let ping = Event::ping();
    let pong = Event::pong();
    println!("   ping: {}", ping);
    println!("   pong: {}", pong);
    println!();

    // Example 2: Struct variants
    println!("2. Struct Variants:");
    let user = Event::user_created(
        "usr_123".to_string(),
        "alice".to_string(),
        "alice@example.com".to_string(),
    );
    println!("   {}", user);
    println!();

    // Example 3: Tuple variants
    println!("3. Tuple Variants:");
    let message = Event::message_received("Hello, World!".to_string(), 1234567890);
    println!("   {}", message);
    println!();

    // Example 4: Complex struct variant
    println!("4. Complex Struct Variant:");
    let upload = Event::file_uploaded(
        "file_789".to_string(),
        "document.pdf".to_string(),
        1024 * 1024, // 1MB
        "alice".to_string(),
    );
    println!("   {}", upload);
    println!();

    // Example 5: Debug mode with Vec<u8>
    println!("5. Debug Mode (for Vec<u8>):");
    let raw = BinaryData::raw(vec![0xDE, 0xAD, 0xBE, 0xEF]);
    println!("   {}", raw);

    let encoded = BinaryData::encoded(
        "base64".to_string(),
        vec![0x48, 0x65, 0x6C, 0x6C, 0x6F], // "Hello"
    );
    println!("   {}", encoded);

    let empty = BinaryData::empty();
    println!("   {}", empty);
    println!();

    // Example 6: Working with collections
    println!("6. Collections:");
    let events = [
        Event::ping(),
        Event::pong(),
        Event::user_created("usr_456".into(), "bob".into(), "bob@example.com".into()),
        Event::message_received("Test message".into(), 9876543210),
    ];

    for (i, event) in events.iter().enumerate() {
        println!("   Event {}: {}", i + 1, event);
    }
    println!();

    // Example 7: Pattern matching still works!
    println!("7. Pattern Matching:");
    match &user {
        Event::UserCreated { username, .. } => {
            println!("   Matched user creation for: {}", username);
        }
        _ => println!("   Not a user creation event"),
    }
    println!();

    // Example 8: Clone support
    println!("8. Clone Support:");
    let original = Event::ping();
    let cloned = original.clone();
    println!("   Original: {}", original);
    println!("   Cloned:   {}", cloned);
    println!();

    // Example 9: Generic usage
    println!("9. Generic Usage:");
    print_any_display(&ping);
    print_any_display(&user);
    print_any_display(&message);
    println!();

    println!("=== All examples completed successfully! ===");
}

/// Demonstrates that generated code works with generics
fn print_any_display<T: std::fmt::Display>(value: &T) {
    println!("   Display: {}", value);
}
