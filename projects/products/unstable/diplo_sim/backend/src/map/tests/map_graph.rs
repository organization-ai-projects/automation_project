use crate::map::map_graph::MapGraph;
use crate::map::territory::Territory;
use crate::map::territory_id::TerritoryId;

#[test]
fn map_graph_reports_neighbors_and_adjacency() {
    let graph = MapGraph {
        name: "tiny".to_string(),
        version: "1".to_string(),
        territories: vec![
            Territory {
                id: TerritoryId(1),
                name: "A".to_string(),
            },
            Territory {
                id: TerritoryId(2),
                name: "B".to_string(),
            },
            Territory {
                id: TerritoryId(3),
                name: "C".to_string(),
            },
        ],
        adjacencies: vec![
            [TerritoryId(1), TerritoryId(2)],
            [TerritoryId(2), TerritoryId(3)],
        ],
    };

    assert!(graph.is_adjacent(TerritoryId(1), TerritoryId(2)));
    assert!(!graph.is_adjacent(TerritoryId(1), TerritoryId(3)));
    assert_eq!(
        graph.neighbors(TerritoryId(2)),
        vec![TerritoryId(1), TerritoryId(3)]
    );
    assert_eq!(graph.territory_count(), 3);
}
