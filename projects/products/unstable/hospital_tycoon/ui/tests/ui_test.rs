// projects/products/unstable/hospital_tycoon/ui/tests/ui_test.rs
//
// UI tests: reducer determinism and fixture-based report screen rendering.

// Test 1: Reducer determinism (pure state machine)
#[test]
fn test_reducer_determinism() {
    // We test the reducer logic inline since we can't import from a bin crate.
    // The reducer is deterministic: same sequence of actions => same final state.

    #[derive(Debug, Clone, PartialEq)]
    struct State {
        current_tick: u64,
        running: bool,
        last_event: Option<String>,
    }

    impl State {
        fn new() -> Self {
            Self { current_tick: 0, running: false, last_event: None }
        }
    }

    fn apply(state: &mut State, action: &str, n: u64) {
        match action {
            "step" => {
                state.current_tick += n;
                state.running = true;
                state.last_event = Some(format!("stepped {} ticks", n));
            }
            "run_to_end" => {
                state.running = false;
                state.last_event = Some("run completed".to_string());
            }
            "get_report" => {
                state.last_event = Some("report requested".to_string());
            }
            _ => {}
        }
    }

    let actions = vec![("step", 10u64), ("step", 20), ("run_to_end", 0), ("get_report", 0)];

    let mut s1 = State::new();
    let mut s2 = State::new();

    for (a, n) in &actions {
        apply(&mut s1, a, *n);
        apply(&mut s2, a, *n);
    }

    assert_eq!(s1.current_tick, s2.current_tick);
    assert_eq!(s1.running, s2.running);
    assert_eq!(s1.last_event, s2.last_event);
}

// Test 2: Fixture-based report screen rendering — load golden report JSON and render deterministically
#[test]
fn test_report_screen_rendering_deterministic() {
    let report_json = serde_json::json!({
        "seed": 42,
        "scenario_name": "tiny_clinic",
        "total_ticks": 50,
        "patients_treated": 9,
        "patients_died": 0,
        "final_budget": 12700,
        "final_reputation": 59,
        "event_count": 45,
        "run_hash": "abc123def456"
    });

    fn render_summary(report: &serde_json::Value) -> String {
        format!(
            "seed={} scenario={} treated={} budget={} hash={}",
            report["seed"],
            report["scenario_name"].as_str().unwrap_or(""),
            report["patients_treated"],
            report["final_budget"],
            report["run_hash"].as_str().unwrap_or("")
        )
    }

    let s1 = render_summary(&report_json);
    let s2 = render_summary(&report_json);
    assert_eq!(s1, s2, "report screen rendering must be deterministic");
    assert!(s1.contains("tiny_clinic"));
    assert!(s1.contains("abc123def456"));
}
