#[cfg(test)]
mod tests {
    use crate::dsl::dsl_op::DslOp;

    #[test]
    fn serialize_replace_range() {
        let op = DslOp::ReplaceRange {
            file: "test.txt".into(),
            start: 0,
            end: 5,
            text: "hello".into(),
        };
        let json = common_json::to_string(&op).unwrap();
        let decoded: DslOp = common_json::from_json_str(&json).unwrap();
        assert_eq!(op, decoded);
    }

    #[test]
    fn serialize_all_ops() {
        let ops = vec![
            DslOp::ReplaceRange {
                file: "a.txt".into(),
                start: 0,
                end: 3,
                text: "xyz".into(),
            },
            DslOp::ReplaceFirst {
                file: "b.txt".into(),
                pattern: "old".into(),
                text: "new".into(),
            },
            DslOp::InsertAfter {
                file: "c.txt".into(),
                pattern: "marker".into(),
                text: " added".into(),
            },
            DslOp::DeleteRange {
                file: "d.txt".into(),
                start: 1,
                end: 4,
            },
        ];
        let json = common_json::to_string(&ops).unwrap();
        let decoded: Vec<DslOp> = common_json::from_json_str(&json).unwrap();
        assert_eq!(ops, decoded);
    }
}
