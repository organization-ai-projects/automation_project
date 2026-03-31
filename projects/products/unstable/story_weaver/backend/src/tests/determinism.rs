use crate::config::StoryConfig;
use crate::dsl::ScriptParser;
use crate::engine::NarrativeEngine;

fn sample_script_json() -> String {
    r#"{
  "title": "Determinism Test",
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
fn same_seed_produces_identical_output() {
    let script = ScriptParser::parse(&sample_script_json()).unwrap();
    let config = StoryConfig {
        seed: 42,
        max_steps: script.max_steps,
    };

    let (report1, _) = NarrativeEngine::run(&script, &config).unwrap();
    let (report2, _) = NarrativeEngine::run(&script, &config).unwrap();

    assert_eq!(report1.run_hash, report2.run_hash);
    assert_eq!(report1.event_count, report2.event_count);
    assert_eq!(report1.snapshot_hash, report2.snapshot_hash);
    assert_eq!(report1.steps_taken, report2.steps_taken);
}

#[test]
fn different_seeds_produce_different_output() {
    let script = ScriptParser::parse(&sample_script_json()).unwrap();

    let config1 = StoryConfig {
        seed: 1,
        max_steps: script.max_steps,
    };
    let config2 = StoryConfig {
        seed: 999,
        max_steps: script.max_steps,
    };

    let (report1, _) = NarrativeEngine::run(&script, &config1).unwrap();
    let (report2, _) = NarrativeEngine::run(&script, &config2).unwrap();

    assert_ne!(report1.run_hash, report2.run_hash);
}

#[test]
fn engine_produces_events() {
    let script = ScriptParser::parse(&sample_script_json()).unwrap();
    let config = StoryConfig {
        seed: 42,
        max_steps: script.max_steps,
    };

    let (report, replay) = NarrativeEngine::run(&script, &config).unwrap();

    assert!(report.event_count > 0);
    assert!(report.steps_taken > 0);
    assert!(!replay.events.is_empty());
}

#[test]
fn engine_stops_when_no_rules_apply() {
    let json = r#"{
  "title": "Short Story",
  "initial_state": {
    "done": { "Flag": false }
  },
  "rules": [
    {
      "id": "finish",
      "description": "Mark as done",
      "conditions": [
        { "Equals": { "variable": "done", "value": { "Flag": false } } }
      ],
      "effects": [
        { "Set": { "variable": "done", "value": { "Flag": true } } }
      ],
      "weight": 1
    }
  ],
  "max_steps": 100
}"#;

    let script = ScriptParser::parse(json).unwrap();
    let config = StoryConfig {
        seed: 42,
        max_steps: script.max_steps,
    };

    let (report, _) = NarrativeEngine::run(&script, &config).unwrap();
    assert_eq!(report.steps_taken, 2);
}
