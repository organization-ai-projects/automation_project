use crate::scan::file_walker::walk_files_sorted;
use crate::scan::forbidden_rule::ForbiddenRule;
use crate::scan::scan_config::ScanConfig;
use anyhow::Result;
use std::collections::BTreeSet;

pub fn find_forbidden(root: &str, config: &ScanConfig) -> Result<Vec<String>> {
    let rules: Vec<ForbiddenRule> = config
        .forbidden_patterns
        .iter()
        .map(ForbiddenRule::new)
        .collect();

    let files = walk_files_sorted(root, &config.skip_dirs)?;
    let mut findings: BTreeSet<(String, usize)> = BTreeSet::new();

    for path in &files {
        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => continue,
        };
        for (line_no, line) in content.lines().enumerate() {
            for rule in &rules {
                if rule.matches(line) {
                    findings.insert((
                        format!("{}:{}: {}", path.display(), line_no + 1, rule.pattern),
                        line_no,
                    ));
                }
            }
        }
    }

    Ok(findings.into_iter().map(|(s, _)| s).collect())
}
