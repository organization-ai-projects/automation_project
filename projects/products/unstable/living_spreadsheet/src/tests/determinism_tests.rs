#[cfg(test)]
mod tests {
    use runtime_core::public_api::Seed;
    use crate::engine::recompute_engine::RecomputeEngine;
    use crate::model::cell_id::CellId;
    use crate::model::cell_value::CellValue;

    fn build_engine(seed: Seed) -> RecomputeEngine {
        let mut engine = RecomputeEngine::new(seed);
        engine.set_value(CellId::from_a1("A1").unwrap(), CellValue::Number(5.0)).unwrap();
        engine.set_value(CellId::from_a1("B1").unwrap(), CellValue::Number(3.0)).unwrap();
        engine.set_formula(CellId::from_a1("C1").unwrap(), "=A1+B1".to_string()).unwrap();
        engine.set_formula(CellId::from_a1("D1").unwrap(), "=C1*2".to_string()).unwrap();
        engine.recompute_all().unwrap();
        engine
    }

    #[test]
    fn same_seed_same_results() {
        let engine1 = build_engine(Seed::new(42));
        let engine2 = build_engine(Seed::new(42));

        let c1 = CellId::from_a1("C1").unwrap();
        let d1 = CellId::from_a1("D1").unwrap();

        assert_eq!(engine1.get_value(&c1), engine2.get_value(&c1));
        assert_eq!(engine1.get_value(&d1), engine2.get_value(&d1));
    }

    #[test]
    fn same_seed_same_trace() {
        let engine1 = build_engine(Seed::new(7));
        let engine2 = build_engine(Seed::new(7));

        let c1 = CellId::from_a1("C1").unwrap();
        let trace1 = engine1.trace(&c1).unwrap();
        let trace2 = engine2.trace(&c1).unwrap();

        assert_eq!(trace1, trace2);
    }

    #[test]
    fn same_seed_same_event_log() {
        let engine1 = build_engine(Seed::new(99));
        let engine2 = build_engine(Seed::new(99));

        let log1 = engine1.event_log().events();
        let log2 = engine2.event_log().events();

        assert_eq!(log1.len(), log2.len());
        for (e1, e2) in log1.iter().zip(log2.iter()) {
            assert_eq!(e1.node_id, e2.node_id);
        }
    }
}
