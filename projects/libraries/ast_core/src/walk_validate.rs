// projects/libraries/ast_core/src/walk_validate.rs
use crate::{
    AstErrorKind, AstKind, AstNode, AstPath, AstValidationError, PathItem, ValidateLimits,
};

enum Frame<'a> {
    Enter {
        node: &'a AstNode,
        depth: usize,
    },
    ArrayNext {
        node: &'a AstNode,
        depth: usize,
        idx: usize,
    },
    ObjectNext {
        node: &'a AstNode,
        depth: usize,
        idx: usize,
    },
    PopPath,
}

pub fn validate_iterative(
    root: &AstNode,
    limits: &ValidateLimits,
) -> Result<(), AstValidationError> {
    let mut path = AstPath::default();
    let mut stack: Vec<Frame<'_>> = Vec::with_capacity(256);

    stack.push(Frame::Enter {
        node: root,
        depth: 1,
    });

    while let Some(frame) = stack.pop() {
        match frame {
            Frame::PopPath => {
                path.0.pop();
            }

            Frame::Enter { node, depth } => {
                if depth > limits.max_depth {
                    return Err(AstValidationError {
                        path: path.clone(),
                        kind: AstErrorKind::MaxDepth {
                            max: limits.max_depth,
                            got: depth,
                        },
                    });
                }

                match &node.kind {
                    AstKind::Array(items) => {
                        if items.len() > limits.max_size {
                            return Err(AstValidationError {
                                path: path.clone(),
                                kind: AstErrorKind::MaxSize {
                                    kind: "array",
                                    max: limits.max_size,
                                },
                            });
                        }
                        stack.push(Frame::ArrayNext {
                            node,
                            depth,
                            idx: 0,
                        });
                    }
                    AstKind::Object(fields) => {
                        if fields.len() > limits.max_size {
                            return Err(AstValidationError {
                                path: path.clone(),
                                kind: AstErrorKind::MaxSize {
                                    kind: "object",
                                    max: limits.max_size,
                                },
                            });
                        }

                        let mut seen = std::collections::BTreeSet::<&str>::new();
                        for (k, _) in fields.iter() {
                            let ks = k.as_str();
                            if !seen.insert(ks) {
                                return Err(AstValidationError {
                                    path: path.clone(),
                                    kind: AstErrorKind::DuplicateKey {
                                        key: ks.to_string(),
                                    },
                                });
                            }
                        }

                        stack.push(Frame::ObjectNext {
                            node,
                            depth,
                            idx: 0,
                        });
                    }
                    _ => {}
                }
            }

            Frame::ArrayNext { node, depth, idx } => {
                let AstKind::Array(items) = &node.kind else {
                    continue;
                };
                if idx >= items.len() {
                    continue;
                }

                stack.push(Frame::ArrayNext {
                    node,
                    depth,
                    idx: idx + 1,
                });

                path.0.push(PathItem::Index(idx));
                stack.push(Frame::PopPath);
                stack.push(Frame::Enter {
                    node: &items[idx],
                    depth: depth + 1,
                });
            }

            Frame::ObjectNext { node, depth, idx } => {
                let AstKind::Object(fields) = &node.kind else {
                    continue;
                };
                if idx >= fields.len() {
                    continue;
                }

                stack.push(Frame::ObjectNext {
                    node,
                    depth,
                    idx: idx + 1,
                });

                let (k, v) = &fields[idx];
                path.0.push(PathItem::Key(k.as_str().to_string()));
                stack.push(Frame::PopPath);
                stack.push(Frame::Enter {
                    node: v,
                    depth: depth + 1,
                });
            }
        }
    }

    Ok(())
}
