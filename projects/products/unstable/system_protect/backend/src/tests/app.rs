#[test]
fn app_run_symbol_is_wired() {
    let run_fn: fn() -> Result<(), crate::diagnostics::error::Error> = crate::app::run;
    let _ = run_fn;
}
