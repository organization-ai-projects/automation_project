use crate::config::StoryConfig;
use crate::dsl::ScriptParser;
use crate::engine::NarrativeEngine;
use crate::io::JsonCodec;
use crate::replay::{ReplayCodec, ReplayEngine};

fn sample_script_json() -> String {
    r#"{
  "title": "Replay Test",
  "initial_state": {
    "location": { "Text": "village" },
    "health": { "Number": 100 },
    "has_sword": { "Flag": false }
  },
  "rules": [
    {
      "id": "enter_forest",
      "description": "Enter the dark forest",
      "conditions": [
        { "Equals": { "variable": "location", "value": { "Text": "village" } } }
      ],
      "effects": [
        { "Set": { "variable": "location", "value": { "Text": "forest" } } },
        { "Log": { "message": "You step into the dark forest." } }
      ],
      "weight": 10
    },
    {
      "id": "find_sword",
      "description": "Find a sword",
      "conditions": [
        { "Equals": { "variable": "location", "value": { "Text": "forest" } } },
        { "Equals": { "variable": "has_sword", "value": { "Flag": false } } }
      ],
      "effects": [
        { "Set": { "variable": "has_sword", "value": { "Flag": true } } },
        { "Log": { "message": "You find a gleaming sword." } }
      ],
      "weight": 5
    },
    {
      "id": "take_damage",
      "description": "Take damage in the forest",
      "conditions": [
        { "Equals": { "variable": "location", "value": { "Text": "forest" } } },
        { "GreaterThan": { "variable": "health", "value": 0 } }
      ],
      "effects": [
        { "Add": { "variable": "health", "amount": -10 } },
        { "Log": { "message": "A branch scratches you." } }
      ],
      "weight": 8
    }
  ],
  "max_steps": 20
}"#
    .to_string()
}

#[test]
fn run_and_replay_produce_same_hash() {
    let script = ScriptParser::parse(&sample_script_json()).unwrap();
    let config = StoryConfig {
        seed: 42,
        max_steps: script.max_steps,
    };

    let (report1, replay_data) = NarrativeEngine::run(&script, &config).unwrap();
    let report2 = ReplayEngine::replay(&replay_data).unwrap();

    assert_eq!(report1.run_hash, report2.run_hash);
    assert_eq!(report1.event_count, report2.event_count);
    assert_eq!(report1.snapshot_hash, report2.snapshot_hash);
}

#[test]
fn replay_codec_roundtrip() {
    let script = ScriptParser::parse(&sample_script_json()).unwrap();
    let config = StoryConfig {
        seed: 42,
        max_steps: script.max_steps,
    };

    let (_, replay_data) = NarrativeEngine::run(&script, &config).unwrap();
    let encoded = ReplayCodec::encode(&replay_data).unwrap();
    let decoded = ReplayCodec::decode(&encoded).unwrap();

    assert_eq!(decoded.seed, replay_data.seed);
    assert_eq!(decoded.events.len(), replay_data.events.len());
}

#[test]
fn report_serializes_to_stable_json() {
    let script = ScriptParser::parse(&sample_script_json()).unwrap();
    let config = StoryConfig {
        seed: 42,
        max_steps: script.max_steps,
    };

    let (report, _) = NarrativeEngine::run(&script, &config).unwrap();
    let json = JsonCodec::encode(&report).unwrap();

    let decoded: crate::report::StoryReport = JsonCodec::decode(&json).unwrap();
    assert_eq!(decoded.run_hash, report.run_hash);
    assert_eq!(decoded.seed, report.seed);
    assert_eq!(decoded.steps_taken, report.steps_taken);
    assert_eq!(decoded.event_count, report.event_count);
    assert_eq!(decoded.snapshot_hash, report.snapshot_hash);
}

#[test]
fn replay_from_decoded_file_matches_original() {
    let script = ScriptParser::parse(&sample_script_json()).unwrap();
    let config = StoryConfig {
        seed: 77,
        max_steps: script.max_steps,
    };

    let (original_report, replay_data) = NarrativeEngine::run(&script, &config).unwrap();

    let encoded = ReplayCodec::encode(&replay_data).unwrap();
    let decoded = ReplayCodec::decode(&encoded).unwrap();
    let replayed_report = ReplayEngine::replay(&decoded).unwrap();

    assert_eq!(original_report.run_hash, replayed_report.run_hash);
    assert_eq!(original_report.snapshot_hash, replayed_report.snapshot_hash);
    assert_eq!(original_report.event_count, replayed_report.event_count);
}
