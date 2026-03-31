#[cfg(test)]
mod tests {
    use crate::dsl::dsl_op::DslOp;
    use crate::plan::patch_plan::PatchPlan;

    #[test]
    fn plan_roundtrip_json() {
        let plan = PatchPlan {
            ops: vec![DslOp::ReplaceRange {
                file: "a.txt".into(),
                start: 0,
                end: 3,
                text: "hi".into(),
            }],
            plan_hash: "abc123".into(),
        };
        let json = common_json::to_string(&plan).unwrap();
        let decoded: PatchPlan = common_json::from_json_str(&json).unwrap();
        assert_eq!(plan, decoded);
    }
}
