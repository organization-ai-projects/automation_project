pub fn display(response_json: &str, json_mode: bool) {
    if json_mode {
        println!("{}", response_json);
    } else {
        println!("=== Stability Results ===");
        println!("{}", response_json);
    }
}
