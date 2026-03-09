use crate::protocol::console;

#[test]
fn console_functions_are_callable() {
    console::print_line("ok");
    console::print_error_line("ok");
    console::print_usage();
}
