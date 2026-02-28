#[cfg(test)]
mod tests {
    use crate::diagnostics::error::SpreadsheetError;
    use crate::engine::dependency_graph::DependencyGraph;
    use crate::formula::ast::{BinOpKind, Expr};
    use crate::formula::evaluator::Evaluator;
    use crate::formula::parser::{extract_deps, Parser};
    use crate::model::cell_id::CellId;
    use crate::model::cell_value::CellValue;
    use crate::model::sheet::Sheet;

    // --- Parser tests ---

    #[test]
    fn parse_number() {
        let expr = Parser::parse("=42").unwrap();
        assert_eq!(expr, Expr::Number(42.0));
    }

    #[test]
    fn parse_cell_ref() {
        let expr = Parser::parse("=A1").unwrap();
        assert_eq!(expr, Expr::CellRef(CellId::new(0, 0)));
    }

    #[test]
    fn parse_arithmetic() {
        let expr = Parser::parse("=A1+B1").unwrap();
        match expr {
            Expr::BinOp { op: BinOpKind::Add, .. } => {}
            _ => panic!("expected Add"),
        }
    }

    #[test]
    fn parse_sum_range() {
        let expr = Parser::parse("=SUM(A1:A3)").unwrap();
        match expr {
            Expr::FunctionCall { name, args } => {
                assert_eq!(name, "SUM");
                assert_eq!(args.len(), 1);
                match &args[0] {
                    Expr::RangeRef(from, to) => {
                        assert_eq!(*from, CellId::new(0, 0));
                        assert_eq!(*to, CellId::new(2, 0));
                    }
                    _ => panic!("expected range ref"),
                }
            }
            _ => panic!("expected FunctionCall"),
        }
    }

    #[test]
    fn parse_min_max() {
        let min = Parser::parse("=MIN(A1:B2)").unwrap();
        match min {
            Expr::FunctionCall { name, args } => {
                assert_eq!(name, "MIN");
                assert_eq!(args.len(), 1);
                assert!(matches!(&args[0], Expr::RangeRef(from, to) if *from == CellId::new(0, 0) && *to == CellId::new(1, 1)));
            }
            _ => panic!("expected FunctionCall MIN"),
        }
        let max = Parser::parse("=MAX(A1:B2)").unwrap();
        match max {
            Expr::FunctionCall { name, args } => {
                assert_eq!(name, "MAX");
                assert_eq!(args.len(), 1);
                assert!(matches!(&args[0], Expr::RangeRef(from, to) if *from == CellId::new(0, 0) && *to == CellId::new(1, 1)));
            }
            _ => panic!("expected FunctionCall MAX"),
        }
    }

    #[test]
    fn parse_parens() {
        let expr = Parser::parse("=(A1+B1)*2").unwrap();
        match expr {
            Expr::BinOp { op: BinOpKind::Mul, .. } => {}
            _ => panic!("expected Mul at top"),
        }
    }

    #[test]
    fn parse_no_equals_sign() {
        let err = Parser::parse("A1+B1").unwrap_err();
        assert!(matches!(err, SpreadsheetError::ParseError(_)));
    }

    #[test]
    fn parse_unclosed_paren() {
        let err = Parser::parse("=(A1+B1").unwrap_err();
        assert!(matches!(err, SpreadsheetError::ParseError(_)));
    }

    #[test]
    fn parse_unknown_function() {
        let err = Parser::parse("=FOOBAR(A1)").unwrap_err();
        assert!(matches!(err, SpreadsheetError::ParseError(_)));
    }

    // --- Evaluator tests ---

    #[test]
    fn eval_literal_number() {
        let sheet = Sheet::new();
        let ev = Evaluator::new(&sheet);
        let result = ev.eval(&Expr::Number(7.0)).unwrap();
        assert_eq!(result, CellValue::Number(7.0));
    }

    #[test]
    fn eval_cell_ref() {
        let mut sheet = Sheet::new();
        sheet.set_value(CellId::new(0, 0), CellValue::Number(5.0));
        let ev = Evaluator::new(&sheet);
        let result = ev.eval(&Expr::CellRef(CellId::new(0, 0))).unwrap();
        assert_eq!(result, CellValue::Number(5.0));
    }

    #[test]
    fn eval_a1_plus_b1() {
        let mut sheet = Sheet::new();
        sheet.set_value(CellId::from_a1("A1").unwrap(), CellValue::Number(3.0));
        sheet.set_value(CellId::from_a1("B1").unwrap(), CellValue::Number(4.0));
        let ev = Evaluator::new(&sheet);
        let expr = Parser::parse("=A1+B1").unwrap();
        let result = ev.eval(&expr).unwrap();
        assert_eq!(result, CellValue::Number(7.0));
    }

    #[test]
    fn eval_sum_range() {
        let mut sheet = Sheet::new();
        sheet.set_value(CellId::from_a1("A1").unwrap(), CellValue::Number(1.0));
        sheet.set_value(CellId::from_a1("A2").unwrap(), CellValue::Number(2.0));
        sheet.set_value(CellId::from_a1("A3").unwrap(), CellValue::Number(3.0));
        let ev = Evaluator::new(&sheet);
        let expr = Parser::parse("=SUM(A1:A3)").unwrap();
        let result = ev.eval(&expr).unwrap();
        assert_eq!(result, CellValue::Number(6.0));
    }

    // --- DependencyGraph tests ---

    #[test]
    fn dep_graph_affected() {
        let mut graph = DependencyGraph::new();
        let a1 = CellId::from_a1("A1").unwrap();
        let b1 = CellId::from_a1("B1").unwrap();
        let c1 = CellId::from_a1("C1").unwrap();
        graph.set_deps(b1.clone(), vec![a1.clone()]);
        graph.set_deps(c1.clone(), vec![b1.clone()]);

        let mut affected = graph.affected(&a1);
        affected.sort();
        assert!(affected.contains(&b1));
        assert!(affected.contains(&c1));
    }

    #[test]
    fn dep_graph_no_cycle() {
        let mut graph = DependencyGraph::new();
        let a1 = CellId::from_a1("A1").unwrap();
        let b1 = CellId::from_a1("B1").unwrap();
        graph.set_deps(b1.clone(), vec![a1.clone()]);
        assert!(graph.check_cycles().is_ok());
    }

    #[test]
    fn dep_graph_cycle_detection() {
        let mut graph = DependencyGraph::new();
        let a1 = CellId::from_a1("A1").unwrap();
        let b1 = CellId::from_a1("B1").unwrap();
        graph.set_deps(a1.clone(), vec![b1.clone()]);
        graph.set_deps(b1.clone(), vec![a1.clone()]);
        assert!(matches!(graph.check_cycles(), Err(SpreadsheetError::CycleDetected)));
    }

    #[test]
    fn extract_deps_from_formula() {
        let expr = Parser::parse("=A1+B2").unwrap();
        let deps = extract_deps(&expr);
        assert!(deps.contains(&CellId::from_a1("A1").unwrap()));
        assert!(deps.contains(&CellId::from_a1("B2").unwrap()));
    }
}
