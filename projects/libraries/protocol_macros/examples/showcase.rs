//! Showcase of protocol_macros features
//!
//! Run with: cargo run --example showcase

use protocol_macros::EnumMethods;

// Example 1: Basic enum with all variant types
#[derive(Debug, Clone, EnumMethods)]
enum Event {
    Ping,
    Created { id: String, name: String },
    DataReceived(String, usize),
}

// Example 2: Enum with acronym handling
#[derive(Debug, Clone, EnumMethods)]
enum HTTPStatus {
    HTTPRequest,
    HTTPNotFound,
    HTTPServerError,
    XMLParseError,
    JSONDataReceived,
}

// Example 3: Enum with Debug mode (for non-Display types)
#[derive(Debug, Clone, EnumMethods)]
#[enum_methods(mode = "debug")]
enum BinaryEvent {
    Connected { peer: std::net::SocketAddr },
    DataPacket(Vec<u8>),
}

fn main() {
    println!("=== protocol_macros Showcase ===\n");

    // Feature 1: Snake_case constructors
    println!("1. Snake_case constructors:");
    let ping = Event::ping();
    let created = Event::created("user-123".into(), "Alice".into());
    let data = Event::data_received("Hello, World!".into(), 42);
    println!("   Created: {:?}", created);

    // Feature 2: as_str() const method
    println!("\n2. as_str() const method:");
    println!("   ping.as_str() = {}", ping.as_str());
    println!("   created.as_str() = {}", created.as_str());
    println!("   data.as_str() = {}", data.as_str());

    // Feature 3: Display implementation
    println!("\n3. Display implementation:");
    println!("   {}", ping);
    println!("   {}", created);
    println!("   {}", data);

    // Feature 4: Acronym handling
    println!("\n4. Acronym handling (HTTPStatus):");
    let http_req = HTTPStatus::http_request();
    let http_not_found = HTTPStatus::http_not_found();
    let http_server_error = HTTPStatus::http_server_error();
    let xml_err = HTTPStatus::xml_parse_error();
    let json_recv = HTTPStatus::json_data_received();
    println!("   HTTPRequest -> {}", http_req.as_str());
    println!("   HTTPNotFound -> {}", http_not_found.as_str());
    println!("   HTTPServerError -> {}", http_server_error.as_str());
    println!("   XMLParseError -> {}", xml_err.as_str());
    println!("   JSONDataReceived -> {}", json_recv.as_str());

    // Feature 5: Debug mode for non-Display types
    println!("\n5. Debug mode (for non-Display types):");
    let addr = "127.0.0.1:8080".parse().unwrap();
    let connected = BinaryEvent::connected(addr);
    let packet = BinaryEvent::data_packet(vec![0xFF, 0xAA, 0xBB]);
    println!("   {}", connected);
    println!("   {}", packet);

    // Feature 6: Routing pattern (real-world usage)
    println!("\n6. Real-world routing pattern:");
    fn route_event(event: &Event) -> &'static str {
        match event.as_str() {
            "ping" => "handle_ping()",
            "created" => "handle_created()",
            "data_received" => "handle_data_received()",
            _ => "unknown_handler()",
        }
    }
    println!("   Event::ping() routes to: {}", route_event(&ping));
    println!(
        "   Event::created(...) routes to: {}",
        route_event(&created)
    );

    println!("\n=== All features working perfectly! ===");
}
