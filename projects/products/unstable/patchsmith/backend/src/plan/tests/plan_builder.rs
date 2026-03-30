#[cfg(test)]
mod tests {
    use crate::dsl::dsl_op::DslOp;
    use crate::plan::plan_builder::PlanBuilder;

    #[test]
    fn build_deterministic() {
        let ops = vec![DslOp::ReplaceFirst {
            file: "a.txt".into(),
            pattern: "old".into(),
            text: "new".into(),
        }];
        let plan1 = PlanBuilder::build(ops.clone()).unwrap();
        let plan2 = PlanBuilder::build(ops).unwrap();
        assert_eq!(plan1.plan_hash, plan2.plan_hash);
    }

    #[test]
    fn build_empty_fails() {
        assert!(PlanBuilder::build(vec![]).is_err());
    }

    #[test]
    fn different_ops_different_hash() {
        let plan1 = PlanBuilder::build(vec![DslOp::ReplaceFirst {
            file: "a.txt".into(),
            pattern: "old".into(),
            text: "new".into(),
        }])
        .unwrap();
        let plan2 = PlanBuilder::build(vec![DslOp::ReplaceFirst {
            file: "b.txt".into(),
            pattern: "old".into(),
            text: "new".into(),
        }])
        .unwrap();
        assert_ne!(plan1.plan_hash, plan2.plan_hash);
    }
}
