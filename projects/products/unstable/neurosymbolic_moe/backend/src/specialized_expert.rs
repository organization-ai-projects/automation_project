use std::collections::HashMap;

use protocol::ProtocolId;
use common::Id128;

use crate::moe_core::{
    ExecutionContext, Expert, ExpertCapability, ExpertError, ExpertId, ExpertMetadata,
    ExpertOutput, ExpertStatus, ExpertType, Task, TaskType,
};
use crate::orchestrator::Version;

#[derive(Clone, Copy)]
enum SpecializedExpertKind {
    Planning,
    CodeGeneration,
    CodeTransformation,
    Validation,
}

pub(crate) struct SpecializedExpert {
    metadata: ExpertMetadata,
    kind: SpecializedExpertKind,
}

impl SpecializedExpert {
    pub(crate) fn planning(name: &str) -> Self {
        Self::planning_with_id(ProtocolId::new(Id128::new(0, None, None)), name)
    }

    pub(crate) fn planning_with_id(id: ProtocolId, name: &str) -> Self {
        Self::new(
            id,
            name,
            SpecializedExpertKind::Planning,
            vec![
                ExpertCapability::IssuePlanning,
                ExpertCapability::Summarization,
            ],
            ExpertType::Symbolic,
        )
    }

    pub(crate) fn code_generation(name: &str) -> Self {
        Self::code_generation_with_id(ProtocolId::new(Id128::new(0, None, None)), name)
    }

    pub(crate) fn code_generation_with_id(id: ProtocolId, name: &str) -> Self {
        Self::new(
            id,
            name,
            SpecializedExpertKind::CodeGeneration,
            vec![ExpertCapability::CodeGeneration],
            ExpertType::Hybrid,
        )
    }

    pub(crate) fn code_transformation(name: &str) -> Self {
        Self::code_transformation_with_id(ProtocolId::new(Id128::new(0, None, None)), name)
    }

    pub(crate) fn code_transformation_with_id(id: ProtocolId, name: &str) -> Self {
        Self::new(
            id,
            name,
            SpecializedExpertKind::CodeTransformation,
            vec![
                ExpertCapability::CodeTransformation,
                ExpertCapability::StructureAnalysis,
            ],
            ExpertType::Symbolic,
        )
    }

    pub(crate) fn validation(name: &str) -> Self {
        Self::validation_with_id(ProtocolId::new(Id128::new(0, None, None)), name)
    }

    pub(crate) fn validation_with_id(id: ProtocolId, name: &str) -> Self {
        Self::new(
            id,
            name,
            SpecializedExpertKind::Validation,
            vec![
                ExpertCapability::Validation,
                ExpertCapability::Evaluation,
                ExpertCapability::StructureAnalysis,
            ],
            ExpertType::Symbolic,
        )
    }

    fn new(
        id: ProtocolId,
        name: &str,
        kind: SpecializedExpertKind,
        capabilities: Vec<ExpertCapability>,
        expert_type: ExpertType,
    ) -> Self {
        Self {
            metadata: ExpertMetadata {
                id: ExpertId::from_protocol_id(id),
                name: name.to_string(),
                version: Version::new(1, 0, 0),
                capabilities,
                status: ExpertStatus::Active,
                expert_type,
            },
            kind,
        }
    }

    fn task_mentions(task: &Task, needles: &[&str]) -> bool {
        let input = task.input().to_ascii_lowercase();
        let context = task
            .context()
            .map(str::to_ascii_lowercase)
            .unwrap_or_default();
        let metadata = task
            .metadata
            .values()
            .map(|value| value.to_ascii_lowercase())
            .collect::<Vec<_>>();
        needles.iter().any(|needle| {
            input.contains(needle)
                || context.contains(needle)
                || metadata.iter().any(|value| value.contains(needle))
        })
    }

    fn confidence(&self, task: &Task, context: &ExecutionContext) -> f64 {
        let context_bonus = if context.retrieved_context.is_empty() {
            0.0_f64
        } else {
            0.03_f64
        };
        let memory_bonus = if context.memory_entries.is_empty() {
            0.0_f64
        } else {
            0.02_f64
        };
        let base = match self.kind {
            SpecializedExpertKind::Planning => {
                if matches!(task.task_type(), TaskType::Planning) {
                    0.94
                } else {
                    0.72
                }
            }
            SpecializedExpertKind::CodeGeneration => {
                if matches!(task.task_type(), TaskType::CodeGeneration) {
                    0.93
                } else {
                    0.7
                }
            }
            SpecializedExpertKind::CodeTransformation => {
                if matches!(
                    task.task_type(),
                    TaskType::CodeTransformation | TaskType::Refactoring
                ) {
                    0.92
                } else {
                    0.69
                }
            }
            SpecializedExpertKind::Validation => {
                if matches!(
                    task.task_type(),
                    TaskType::Validation | TaskType::CodeAnalysis | TaskType::Evaluation
                ) {
                    0.95
                } else {
                    0.71
                }
            }
        };
        (base + context_bonus + memory_bonus).min(0.99)
    }

    fn output_content(&self, task: &Task, context: &ExecutionContext) -> String {
        match self.kind {
            SpecializedExpertKind::Planning => format!(
                "Plan for '{}':\n1. Clarify scope and constraints.\n2. Use {} retrieved context item(s) and {} memory item(s).\n3. Prepare an implementation sequence.\n4. Hand off to generation or transformation once validated.",
                task.input(),
                context.retrieved_context.len(),
                context.memory_entries.len()
            ),
            SpecializedExpertKind::CodeGeneration => format!(
                "Implementation draft for '{}':\n- preserve task intent\n- incorporate {} retrieved context item(s)\n- keep {} buffer hint(s)\n- emit a concrete code-oriented answer",
                task.input(),
                context.retrieved_context.len(),
                context.buffer_data.len()
            ),
            SpecializedExpertKind::CodeTransformation => format!(
                "Transformation plan for '{}':\n- preserve current behavior\n- reshape structure or naming\n- review {} memory hint(s)\n- prepare a safer refactoring path",
                task.input(),
                context.memory_entries.len()
            ),
            SpecializedExpertKind::Validation => format!(
                "Validation report for '{}':\n- review correctness assumptions\n- inspect {} retrieved context item(s)\n- inspect {} memory item(s)\n- flag policy or consistency risks before acceptance",
                task.input(),
                context.retrieved_context.len(),
                context.memory_entries.len()
            ),
        }
    }
}

impl Expert for SpecializedExpert {
    fn id(&self) -> &ExpertId {
        &self.metadata.id
    }

    fn metadata(&self) -> &ExpertMetadata {
        &self.metadata
    }

    fn can_handle(&self, task: &Task) -> bool {
        match self.kind {
            SpecializedExpertKind::Planning => {
                matches!(
                    task.task_type(),
                    TaskType::Planning | TaskType::Documentation
                ) || Self::task_mentions(task, &["plan", "roadmap", "strategy", "steps"])
            }
            SpecializedExpertKind::CodeGeneration => {
                matches!(task.task_type(), TaskType::CodeGeneration)
                    || Self::task_mentions(task, &["implement", "generate", "build", "write"])
            }
            SpecializedExpertKind::CodeTransformation => {
                matches!(
                    task.task_type(),
                    TaskType::CodeTransformation | TaskType::Refactoring
                ) || Self::task_mentions(task, &["refactor", "transform", "rewrite", "rename"])
            }
            SpecializedExpertKind::Validation => {
                matches!(
                    task.task_type(),
                    TaskType::Validation | TaskType::CodeAnalysis | TaskType::Evaluation
                ) || Self::task_mentions(task, &["validate", "review", "check", "analyze"])
            }
        }
    }

    fn execute(
        &self,
        task: &Task,
        context: &ExecutionContext,
    ) -> Result<ExpertOutput, ExpertError> {
        let mut metadata = HashMap::new();
        metadata.insert("expert_role".to_string(), self.metadata.name.clone());
        metadata.insert(
            "retrieved_context_count".to_string(),
            context.retrieved_context.len().to_string(),
        );
        metadata.insert(
            "memory_entries_count".to_string(),
            context.memory_entries.len().to_string(),
        );
        Ok(ExpertOutput {
            expert_id: self.metadata.id.clone(),
            content: self.output_content(task, context),
            confidence: self.confidence(task, context),
            metadata,
            trace: vec![format!(
                "specialized expert '{}' handled {:?}",
                self.metadata.name,
                task.task_type()
            )],
        })
    }
}
