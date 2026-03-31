    use std::collections::BTreeMap;

    use crate::apply::applier::Applier;
    use crate::dsl::dsl_op::DslOp;
    use crate::plan::plan_builder::PlanBuilder;

    fn files(pairs: &[(&str, &str)]) -> BTreeMap<String, String> {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    #[test]
    fn apply_replace_range() {
        let plan = PlanBuilder::build(vec![DslOp::ReplaceRange {
            file: "a.txt".into(),
            start: 0,
            end: 5,
            text: "HELLO".into(),
        }])
        .unwrap();
        let result = Applier::apply(&plan, &files(&[("a.txt", "hello world")])).unwrap();
        assert_eq!(result.files["a.txt"], "HELLO world");
    }

    #[test]
    fn apply_replace_first() {
        let plan = PlanBuilder::build(vec![DslOp::ReplaceFirst {
            file: "a.txt".into(),
            pattern: "old".into(),
            text: "new".into(),
        }])
        .unwrap();
        let result = Applier::apply(&plan, &files(&[("a.txt", "the old way")])).unwrap();
        assert_eq!(result.files["a.txt"], "the new way");
    }

    #[test]
    fn apply_insert_after() {
        let plan = PlanBuilder::build(vec![DslOp::InsertAfter {
            file: "a.txt".into(),
            pattern: "hello".into(),
            text: " world".into(),
        }])
        .unwrap();
        let result = Applier::apply(&plan, &files(&[("a.txt", "say hello!")])).unwrap();
        assert_eq!(result.files["a.txt"], "say hello world!");
    }

    #[test]
    fn apply_delete_range() {
        let plan = PlanBuilder::build(vec![DslOp::DeleteRange {
            file: "a.txt".into(),
            start: 3,
            end: 6,
        }])
        .unwrap();
        let result = Applier::apply(&plan, &files(&[("a.txt", "abcdefghi")])).unwrap();
        assert_eq!(result.files["a.txt"], "abcghi");
    }

    #[test]
    fn apply_deterministic() {
        let plan = PlanBuilder::build(vec![DslOp::ReplaceFirst {
            file: "a.txt".into(),
            pattern: "x".into(),
            text: "y".into(),
        }])
        .unwrap();
        let f = files(&[("a.txt", "x marks the spot")]);
        let r1 = Applier::apply(&plan, &f).unwrap();
        let r2 = Applier::apply(&plan, &f).unwrap();
        assert_eq!(r1, r2);
    }

    #[test]
    fn apply_idempotent_replace_range() {
        let plan = PlanBuilder::build(vec![DslOp::ReplaceRange {
            file: "a.txt".into(),
            start: 0,
            end: 5,
            text: "HELLO".into(),
        }])
        .unwrap();
        let initial = files(&[("a.txt", "hello world")]);
        let r1 = Applier::apply(&plan, &initial).unwrap();
        assert_eq!(r1.files["a.txt"], "HELLO world");
    }

    #[test]
    fn apply_file_not_found() {
        let plan = PlanBuilder::build(vec![DslOp::ReplaceFirst {
            file: "missing.txt".into(),
            pattern: "x".into(),
            text: "y".into(),
        }])
        .unwrap();
        assert!(Applier::apply(&plan, &BTreeMap::new()).is_err());
    }

    #[test]
    fn apply_range_out_of_bounds() {
        let plan = PlanBuilder::build(vec![DslOp::ReplaceRange {
            file: "a.txt".into(),
            start: 0,
            end: 100,
            text: "x".into(),
        }])
        .unwrap();
        assert!(Applier::apply(&plan, &files(&[("a.txt", "short")])).is_err());
    }
