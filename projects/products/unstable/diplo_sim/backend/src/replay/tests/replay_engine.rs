use crate::replay::event_log::EventLog;
use crate::replay::replay_engine::replay;
use crate::replay::replay_file::ReplayFile;

#[test]
fn replay_engine_handles_empty_event_log() {
    let replay_file = ReplayFile {
        map_hash: "dummy".to_string(),
        map_name: "tiny".to_string(),
        map_json: r#"{
            "name":"tiny",
            "version":"1",
            "territories":[
                {"id":1,"name":"A"}
            ],
            "adjacencies":[],
            "starting_units":[
                {"faction_id":1,"territory_id":1}
            ]
        }"#
        .to_string(),
        seed: 42,
        num_factions: 1,
        event_log: EventLog::new(),
    };

    let report = replay(&replay_file).expect("replay should succeed");
    assert_eq!(report.turns.len(), 0);
    assert_eq!(report.map_name, "tiny");
}
