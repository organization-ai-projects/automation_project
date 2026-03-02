use runtime_core::{
    DeterministicContext, Edge, EventLog, Graph, Node, RuntimeError, RuntimeId, Scheduler, Seed,
};

fn id(v: u64) -> RuntimeId {
    RuntimeId::new(v)
}

fn make_five_node_dag() -> Graph {
    // DAG: 1 -> 2 -> 4
    //      1 -> 3 -> 4
    //                4 -> 5
    Graph::new(
        vec![
            Node::new(id(1), "start"),
            Node::new(id(2), "branch_a"),
            Node::new(id(3), "branch_b"),
            Node::new(id(4), "merge"),
            Node::new(id(5), "end"),
        ],
        vec![
            Edge::new(id(1), id(2)),
            Edge::new(id(1), id(3)),
            Edge::new(id(2), id(4)),
            Edge::new(id(3), id(4)),
            Edge::new(id(4), id(5)),
        ],
    )
}

#[test]
fn five_node_dag_execute_and_replay() {
    let dag = make_five_node_dag();
    let mut ctx = DeterministicContext::new(Seed::new(42));
    let log = ctx.run(dag).expect("execution must succeed");

    assert_eq!(log.events().len(), 5, "all five nodes must be executed");

    // Serialize and deserialize
    let bytes = log.serialize().expect("serialization must succeed");
    let restored = EventLog::deserialize(&bytes).expect("deserialization must succeed");

    // Structural equivalence
    assert_eq!(restored.events(), log.events());

    // Replay returns same node order
    let original_replay = log.replay();
    let restored_replay = restored.replay();
    assert_eq!(original_replay, restored_replay);

    // First node executed must be the root (id=1)
    assert_eq!(original_replay[0], id(1));
    // Last node executed must be the sink (id=5)
    assert_eq!(*original_replay.last().unwrap(), id(5));
}

#[test]
fn cycle_detection_rejects_cyclic_graph() {
    let cyclic = Graph::new(
        vec![
            Node::new(id(1), "a"),
            Node::new(id(2), "b"),
            Node::new(id(3), "c"),
        ],
        vec![
            Edge::new(id(1), id(2)),
            Edge::new(id(2), id(3)),
            Edge::new(id(3), id(1)),
        ],
    );
    let mut ctx = DeterministicContext::new(Seed::new(0));
    let result = ctx.run(cyclic);
    assert!(matches!(result, Err(RuntimeError::CyclicGraph)));
}

#[test]
fn determinism_with_fixed_seed() {
    let build_dag = || {
        Graph::new(
            vec![
                Node::new(id(1), "a"),
                Node::new(id(2), "b"),
                Node::new(id(3), "c"),
            ],
            vec![Edge::new(id(1), id(2)), Edge::new(id(2), id(3))],
        )
    };

    let seed = Seed::new(7);
    let mut ctx_a = DeterministicContext::new(seed);
    let mut ctx_b = DeterministicContext::new(seed);

    let log_a = ctx_a.run(build_dag()).unwrap();
    let log_b = ctx_b.run(build_dag()).unwrap();

    assert_eq!(log_a.replay(), log_b.replay());
}

#[test]
fn scheduler_produces_valid_order_for_diamond_dag() {
    let dag = make_five_node_dag();
    let scheduler = Scheduler::new(dag);
    let jobs = scheduler.schedule().unwrap();
    let node_ids: Vec<RuntimeId> = jobs.iter().map(|j| j.node_id).collect();

    // 1 must appear before 2, 3 and 4; 4 must appear before 5
    let pos = |id: RuntimeId| node_ids.iter().position(|&x| x == id).unwrap();
    assert!(pos(id(1)) < pos(id(2)));
    assert!(pos(id(1)) < pos(id(3)));
    assert!(pos(id(2)) < pos(id(4)));
    assert!(pos(id(3)) < pos(id(4)));
    assert!(pos(id(4)) < pos(id(5)));
}
