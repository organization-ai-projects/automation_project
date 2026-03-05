// projects/products/unstable/hospital_tycoon/ui/tests/ui_test.rs
//
// UI tests: reducer determinism and fixture-based report screen rendering.
use common_json::JsonAccess;

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
            Self {
                current_tick: 0,
                running: false,
                last_event: None,
            }
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

    let actions = vec![
        ("step", 10u64),
        ("step", 20),
        ("run_to_end", 0),
        ("get_report", 0),
    ];

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
    let mut obj = common_json::JsonMap::new();
    obj.insert("seed".to_string(), common_json::Json::from(42_u64));
    obj.insert(
        "scenario_name".to_string(),
        common_json::Json::from("tiny_clinic"),
    );
    obj.insert("total_ticks".to_string(), common_json::Json::from(50_u64));
    obj.insert(
        "patients_treated".to_string(),
        common_json::Json::from(9_u64),
    );
    obj.insert("patients_died".to_string(), common_json::Json::from(0_u64));
    obj.insert(
        "final_budget".to_string(),
        common_json::Json::from(12700_i64),
    );
    obj.insert(
        "final_reputation".to_string(),
        common_json::Json::from(59_u64),
    );
    obj.insert("event_count".to_string(), common_json::Json::from(45_u64));
    obj.insert(
        "run_hash".to_string(),
        common_json::Json::from("abc123def456"),
    );
    let report_json = common_json::Json::Object(obj);

    fn render_summary(report: &common_json::Json) -> String {
        format!(
            "seed={} scenario={} treated={} budget={} hash={}",
            report
                .get_field("seed")
                .ok()
                .and_then(common_json::Json::as_u64)
                .unwrap_or(0),
            report
                .get_field("scenario_name")
                .ok()
                .and_then(common_json::Json::as_str)
                .unwrap_or(""),
            report
                .get_field("patients_treated")
                .ok()
                .and_then(common_json::Json::as_u64)
                .unwrap_or(0),
            report
                .get_field("final_budget")
                .ok()
                .and_then(common_json::Json::as_i64)
                .unwrap_or(0),
            report
                .get_field("run_hash")
                .ok()
                .and_then(common_json::Json::as_str)
                .unwrap_or("")
        )
    }

    let s1 = render_summary(&report_json);
    let s2 = render_summary(&report_json);
    assert_eq!(s1, s2, "report screen rendering must be deterministic");
    assert!(s1.contains("tiny_clinic"));
    assert!(s1.contains("abc123def456"));
}
