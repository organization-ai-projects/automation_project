use crate::dsl::{Script, ScriptParser};
use crate::state::StateValue;

fn sample_script_json() -> String {
    r#"{
  "title": "Test Story",
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
      "description": "Find a sword in the forest",
      "conditions": [
        { "Equals": { "variable": "location", "value": { "Text": "forest" } } },
        { "Equals": { "variable": "has_sword", "value": { "Flag": false } } }
      ],
      "effects": [
        { "Set": { "variable": "has_sword", "value": { "Flag": true } } },
        { "Log": { "message": "You find a gleaming sword." } }
      ],
      "weight": 5
    }
  ],
  "max_steps": 20
}"#
    .to_string()
}

#[test]
fn parse_script_from_json() {
    let json = sample_script_json();
    let script = ScriptParser::parse(&json).unwrap();
    assert_eq!(script.title, "Test Story");
    assert_eq!(script.rules.len(), 2);
    assert_eq!(script.max_steps, 20);
}

#[test]
fn parse_script_initial_state() {
    let json = sample_script_json();
    let script = ScriptParser::parse(&json).unwrap();
    assert_eq!(
        script.initial_state.get("location"),
        Some(&StateValue::Text("village".to_string()))
    );
    assert_eq!(
        script.initial_state.get("health"),
        Some(&StateValue::Number(100))
    );
    assert_eq!(
        script.initial_state.get("has_sword"),
        Some(&StateValue::Flag(false))
    );
}

#[test]
fn parse_script_rules() {
    let json = sample_script_json();
    let script = ScriptParser::parse(&json).unwrap();

    let rule = &script.rules[0];
    assert_eq!(rule.id, "enter_forest");
    assert_eq!(rule.conditions.len(), 1);
    assert_eq!(rule.effects.len(), 2);
    assert_eq!(rule.weight, 10);
}

#[test]
fn parse_invalid_json_returns_error() {
    let result = ScriptParser::parse("not valid json");
    assert!(result.is_err());
}

#[test]
fn script_roundtrip_serialization() {
    let json = sample_script_json();
    let script = ScriptParser::parse(&json).unwrap();
    let re_encoded = crate::io::JsonCodec::encode(&script).unwrap();
    let re_decoded: Script = crate::io::JsonCodec::decode(&re_encoded).unwrap();
    assert_eq!(re_decoded.title, script.title);
    assert_eq!(re_decoded.rules.len(), script.rules.len());
    assert_eq!(re_decoded.max_steps, script.max_steps);
}
