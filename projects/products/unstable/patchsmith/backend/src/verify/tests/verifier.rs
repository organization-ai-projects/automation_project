#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::apply::applier::Applier;
    use crate::dsl::dsl_op::DslOp;
    use crate::plan::plan_builder::PlanBuilder;
    use crate::verify::verifier::Verifier;

    fn files(pairs: &[(&str, &str)]) -> BTreeMap<String, String> {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    #[test]
    fn verify_ok() {
        let plan = PlanBuilder::build(vec![DslOp::ReplaceFirst {
            file: "a.txt".into(),
            pattern: "old".into(),
            text: "new".into(),
        }])
        .unwrap();
        let result = Applier::apply(&plan, &files(&[("a.txt", "old text")])).unwrap();
        let verify = Verifier::verify(&plan, &result);
        assert!(verify.ok);
        assert_eq!(verify.file_count, 1);
    }

    #[test]
    fn verify_deterministic_hash() {
        let plan = PlanBuilder::build(vec![DslOp::ReplaceFirst {
            file: "a.txt".into(),
            pattern: "old".into(),
            text: "new".into(),
        }])
        .unwrap();
        let f = files(&[("a.txt", "old text")]);
        let r1 = Applier::apply(&plan, &f).unwrap();
        let r2 = Applier::apply(&plan, &f).unwrap();
        let v1 = Verifier::verify(&plan, &r1);
        let v2 = Verifier::verify(&plan, &r2);
        assert_eq!(v1.content_hash, v2.content_hash);
    }

    #[test]
    fn verify_against_matching() {
        let plan = PlanBuilder::build(vec![DslOp::ReplaceFirst {
            file: "a.txt".into(),
            pattern: "x".into(),
            text: "y".into(),
        }])
        .unwrap();
        let result = Applier::apply(&plan, &files(&[("a.txt", "x")])).unwrap();
        let v = Verifier::verify(&plan, &result);
        let v2 = Verifier::verify_against(&plan, &result, &v.content_hash);
        assert!(v2.ok);
    }

    #[test]
    fn verify_against_mismatch() {
        let plan = PlanBuilder::build(vec![DslOp::ReplaceFirst {
            file: "a.txt".into(),
            pattern: "x".into(),
            text: "y".into(),
        }])
        .unwrap();
        let result = Applier::apply(&plan, &files(&[("a.txt", "x")])).unwrap();
        let v = Verifier::verify_against(&plan, &result, "wrong_hash");
        assert!(!v.ok);
    }
}
