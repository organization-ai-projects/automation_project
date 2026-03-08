use crate::diagnostics::diplo_sim_error::DiploSimError;
use crate::map::map_loader::load_map_from_str;

#[test]
fn map_loader_parses_valid_map() {
    let json = r#"{
        "name":"tiny",
        "version":"1",
        "territories":[
            {"id":1,"name":"A"},
            {"id":2,"name":"B"}
        ],
        "adjacencies":[[1,2]],
        "starting_units":[{"faction_id":1,"territory_id":1}]
    }"#;

    let (graph, starting_units) = load_map_from_str(json).expect("valid map");
    assert_eq!(graph.name, "tiny");
    assert_eq!(graph.territory_count(), 2);
    assert_eq!(starting_units.len(), 1);
}

#[test]
fn map_loader_rejects_unknown_territory_in_adjacency() {
    let json = r#"{
        "name":"broken",
        "version":"1",
        "territories":[
            {"id":1,"name":"A"}
        ],
        "adjacencies":[[1,99]],
        "starting_units":[]
    }"#;

    let error = load_map_from_str(json).expect_err("invalid map must fail");
    assert!(
        matches!(error, DiploSimError::MapValidation(message) if message.contains("unknown territory"))
    );
}
