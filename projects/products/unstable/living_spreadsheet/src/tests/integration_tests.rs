#[cfg(test)]
mod tests {
    use runtime_core::public_api::Seed;
    use crate::engine::recompute_engine::RecomputeEngine;
    use crate::model::cell_id::CellId;
    use crate::model::cell_value::CellValue;

    fn seed() -> Seed {
        Seed::new(0)
    }

    #[test]
    fn basic_sheet_values_and_formulas() {
        let mut engine = RecomputeEngine::new(seed());
        let a1 = CellId::from_a1("A1").unwrap();
        let b1 = CellId::from_a1("B1").unwrap();
        let c1 = CellId::from_a1("C1").unwrap();

        engine.set_value(a1.clone(), CellValue::Number(3.0)).unwrap();
        engine.set_value(b1.clone(), CellValue::Number(4.0)).unwrap();
        engine.set_formula(c1.clone(), "=A1+B1".to_string()).unwrap();

        let recomputed = engine.recompute_all().unwrap();
        assert!(recomputed.contains(&c1));
        assert_eq!(engine.get_value(&c1), CellValue::Number(7.0));
    }

    #[test]
    fn recompute_from_only_affected() {
        let mut engine = RecomputeEngine::new(seed());
        let a1 = CellId::from_a1("A1").unwrap();
        let b1 = CellId::from_a1("B1").unwrap();
        let c1 = CellId::from_a1("C1").unwrap();
        let d1 = CellId::from_a1("D1").unwrap();

        engine.set_value(a1.clone(), CellValue::Number(1.0)).unwrap();
        engine.set_value(b1.clone(), CellValue::Number(2.0)).unwrap();
        engine.set_formula(c1.clone(), "=A1+B1".to_string()).unwrap();
        engine.set_formula(d1.clone(), "=B1*2".to_string()).unwrap();
        engine.recompute_all().unwrap();

        // Change A1, only C1 should be affected (not D1)
        engine.set_value(a1.clone(), CellValue::Number(10.0)).unwrap();
        let recomputed = engine.recompute_from(&a1).unwrap();
        assert!(recomputed.contains(&c1));
        assert!(!recomputed.contains(&d1));
        assert_eq!(engine.get_value(&c1), CellValue::Number(12.0));
    }

    #[test]
    fn sum_formula_integration() {
        let mut engine = RecomputeEngine::new(seed());
        engine.set_value(CellId::from_a1("A1").unwrap(), CellValue::Number(10.0)).unwrap();
        engine.set_value(CellId::from_a1("A2").unwrap(), CellValue::Number(20.0)).unwrap();
        engine.set_value(CellId::from_a1("A3").unwrap(), CellValue::Number(30.0)).unwrap();
        engine.set_formula(CellId::from_a1("A4").unwrap(), "=SUM(A1:A3)".to_string()).unwrap();
        engine.recompute_all().unwrap();
        assert_eq!(engine.get_value(&CellId::from_a1("A4").unwrap()), CellValue::Number(60.0));
    }

    #[test]
    fn event_log_records_recomputed_cells() {
        let mut engine = RecomputeEngine::new(seed());
        engine.set_value(CellId::from_a1("A1").unwrap(), CellValue::Number(1.0)).unwrap();
        engine.set_formula(CellId::from_a1("B1").unwrap(), "=A1*2".to_string()).unwrap();
        engine.recompute_all().unwrap();
        assert!(!engine.event_log().events().is_empty());
    }
}
