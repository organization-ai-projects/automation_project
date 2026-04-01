/// Result of scope resolution.
pub struct ScopeInfo {
    pub balanced: bool,
    pub mismatch_line: Option<usize>,
    pub max_depth: usize,
}

/// Resolves scope boundaries (brace matching) in source text.
pub struct ScopeResolver;

impl ScopeResolver {
    pub fn resolve(source: &str) -> ScopeInfo {
        let mut depth: i32 = 0;
        let mut max_depth: i32 = 0;
        let mut mismatch_line: Option<usize> = None;

        for (idx, line) in source.lines().enumerate() {
            let line_num = idx + 1;
            for ch in line.chars() {
                match ch {
                    '{' => {
                        depth += 1;
                        if depth > max_depth {
                            max_depth = depth;
                        }
                    }
                    '}' => {
                        depth -= 1;
                        if depth < 0 && mismatch_line.is_none() {
                            mismatch_line = Some(line_num);
                        }
                    }
                    _ => {}
                }
            }
        }

        let balanced = depth == 0 && mismatch_line.is_none();
        ScopeInfo {
            balanced,
            mismatch_line,
            max_depth: max_depth.unsigned_abs() as usize,
        }
    }
}
