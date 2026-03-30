#[cfg(test)]
mod tests {
    use crate::report::patch_report::PatchReport;
    use crate::verify::verifier::VerifyResult;

    #[test]
    fn report_canonical_json() {
        let verify = VerifyResult {
            ok: true,
            plan_hash: "abc".into(),
            content_hash: "def".into(),
            file_count: 2,
        };
        let report = PatchReport::from_verify(&verify, 3);
        let json1 = report.to_json().unwrap();
        let json2 = report.to_json().unwrap();
        assert_eq!(json1, json2);
        assert_eq!(report.op_count, 3);
        assert!(report.verified);
    }

    #[test]
    fn report_roundtrip() {
        let verify = VerifyResult {
            ok: true,
            plan_hash: "hash1".into(),
            content_hash: "hash2".into(),
            file_count: 1,
        };
        let report = PatchReport::from_verify(&verify, 5);
        let json = report.to_json().unwrap();
        let decoded: PatchReport = common_json::from_json_str(&json).unwrap();
        assert_eq!(report, decoded);
    }
}
