pub fn print_line(message: &str) {
    println!("{message}");
}

pub fn print_error_line(message: &str) {
    eprintln!("{message}");
}

pub fn print_usage() {
    println!("princeps — deterministic political campaign satire game");
    println!();
    println!("Commands:");
    println!("  run [--days N] [--seed S] [--json] [--replay-out <file>]");
    println!("  replay <replay_file.json> [--json]");
    println!("  export [--format markdown|json] [--seed S] [--days N]");
}
