#[cfg(test)]
mod tests {
    use crate::dsl::dsl_op::DslOp;
    use crate::dsl::dsl_parser::DslParser;

    #[test]
    fn parse_valid_ops() {
        let input = r#"[{"op":"ReplaceRange","file":"test.txt","start":0,"end":5,"text":"hello"}]"#;
        let ops = DslParser::parse(input).unwrap();
        assert_eq!(ops.len(), 1);
        assert!(matches!(&ops[0], DslOp::ReplaceRange { file, .. } if file == "test.txt"));
    }

    #[test]
    fn parse_empty_array_fails() {
        let input = "[]";
        assert!(DslParser::parse(input).is_err());
    }

    #[test]
    fn parse_invalid_json_fails() {
        let input = "not json";
        assert!(DslParser::parse(input).is_err());
    }

    #[test]
    fn parse_replace_range_start_gt_end_fails() {
        let input =
            r#"[{"op":"ReplaceRange","file":"test.txt","start":10,"end":5,"text":"hello"}]"#;
        assert!(DslParser::parse(input).is_err());
    }

    #[test]
    fn parse_empty_file_fails() {
        let input = r#"[{"op":"ReplaceFirst","file":"","pattern":"x","text":"y"}]"#;
        assert!(DslParser::parse(input).is_err());
    }
}
