use crate::config::StoryConfig;
use crate::dsl::ScriptParser;
use crate::engine::NarrativeEngine;
use crate::export::{JsonExporter, MarkdownExporter};

fn sample_script_json() -> String {
    r#"{
  "title": "Golden Export Test",
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
  "max_steps": 10
}"#
    .to_string()
}

#[test]
fn json_export_is_stable() {
    let script = ScriptParser::parse(&sample_script_json()).unwrap();
    let config = StoryConfig {
        seed: 42,
        max_steps: script.max_steps,
    };

    let (report1, replay1) = NarrativeEngine::run(&script, &config).unwrap();
    let (report2, replay2) = NarrativeEngine::run(&script, &config).unwrap();

    let json1 = JsonExporter::export(&report1, &replay1.events).unwrap();
    let json2 = JsonExporter::export(&report2, &replay2.events).unwrap();

    assert_eq!(json1, json2);
}

#[test]
fn markdown_export_is_stable() {
    let script = ScriptParser::parse(&sample_script_json()).unwrap();
    let config = StoryConfig {
        seed: 42,
        max_steps: script.max_steps,
    };

    let (report1, replay1) = NarrativeEngine::run(&script, &config).unwrap();
    let (report2, replay2) = NarrativeEngine::run(&script, &config).unwrap();

    let md1 = MarkdownExporter::export(&report1, &replay1.events);
    let md2 = MarkdownExporter::export(&report2, &replay2.events);

    assert_eq!(md1, md2);
}

#[test]
fn json_export_contains_report_and_events() {
    let script = ScriptParser::parse(&sample_script_json()).unwrap();
    let config = StoryConfig {
        seed: 42,
        max_steps: script.max_steps,
    };

    let (report, replay) = NarrativeEngine::run(&script, &config).unwrap();
    let json = JsonExporter::export(&report, &replay.events).unwrap();

    assert!(json.contains("run_hash"));
    assert!(json.contains("snapshot_hash"));
    assert!(json.contains("events"));
    assert!(json.contains("Golden Export Test"));
}

#[test]
fn markdown_export_contains_structure() {
    let script = ScriptParser::parse(&sample_script_json()).unwrap();
    let config = StoryConfig {
        seed: 42,
        max_steps: script.max_steps,
    };

    let (report, replay) = NarrativeEngine::run(&script, &config).unwrap();
    let md = MarkdownExporter::export(&report, &replay.events);

    assert!(md.contains("# Golden Export Test"));
    assert!(md.contains("**Seed:** 42"));
    assert!(md.contains("## Events"));
    assert!(md.contains("## Summary"));
    assert!(md.contains("**Run Hash:**"));
}

#[test]
fn golden_json_export_matches_expected() {
    let script = ScriptParser::parse(&sample_script_json()).unwrap();
    let config = StoryConfig {
        seed: 42,
        max_steps: script.max_steps,
    };

    let (report, replay) = NarrativeEngine::run(&script, &config).unwrap();
    let json = JsonExporter::export(&report, &replay.events).unwrap();

    let golden = include_str!("golden/golden_export.json");
    assert_eq!(json.trim(), golden.trim());
}

#[test]
fn golden_markdown_export_matches_expected() {
    let script = ScriptParser::parse(&sample_script_json()).unwrap();
    let config = StoryConfig {
        seed: 42,
        max_steps: script.max_steps,
    };

    let (report, replay) = NarrativeEngine::run(&script, &config).unwrap();
    let md = MarkdownExporter::export(&report, &replay.events);

    let golden = include_str!("golden/golden_export.md");
    assert_eq!(md.trim(), golden.trim());
}
