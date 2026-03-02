pub fn display(response_json: &str, json_mode: bool) {
    if json_mode {
        println!("{}", response_json);
    } else {
        println!("=== Scan Results ===");
        println!("{}", response_json);
    }
}
