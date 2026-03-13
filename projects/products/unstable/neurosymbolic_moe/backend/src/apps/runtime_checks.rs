//! projects/products/unstable/neurosymbolic_moe/backend/src/apps/runtime_checks.rs
use std::collections::HashMap;

use crate::{
    aggregator::AggregationStrategy,
    apps::DynError,
    dataset_engine::DatasetTrainingBuildOptions,
    echo_expert::EchoExpert,
    memory_engine::{MemoryEntry, MemoryType},
    moe_core::{ExpertCapability, Task, TaskType},
    orchestrator::{
        self, ArbitrationMode, ContinuousGovernancePolicy, GovernanceImportPolicy, MoePipeline,
        MoePipelineBuilder,
    },
    retrieval_engine::SimpleRetriever,
    router::HeuristicRouter,
};
pub(crate) fn run_runtime_persistence_checks() -> Result<MoePipeline, DynError> {
    let governance_policy =
        ContinuousGovernancePolicy::new(0.5, 0.5, 0.4, 0.2, false).with_auto_promote_on_pass(true);
    let import_policy = GovernanceImportPolicy {
        allow_schema_change: false,
        allow_version_regression: false,
        max_version_regression: Some(0),
        require_policy_match: false,
    };
    let mut runtime_pipeline: MoePipeline = MoePipelineBuilder::new()
        .with_router(Box::new(HeuristicRouter::new(2)))
        .with_aggregation_strategy(AggregationStrategy::HighestConfidence)
        .with_arbitration_mode(ArbitrationMode::Aggregation)
        .with_fallback_on_expert_error(true)
        .with_task_metadata_chain(true)
        .with_retriever(Box::new(SimpleRetriever::new()))
        .with_context_max_length(512)
        .with_continuous_governance_policy(governance_policy)
        .with_governance_import_policy(import_policy)
        .with_max_governance_audit_entries(16)
        .with_max_governance_state_snapshots(8)
        .with_max_traces(256)
        .build();
    runtime_pipeline.register_expert(Box::new(EchoExpert::new(
        "runtime_wired",
        "RuntimeWired",
        vec![ExpertCapability::CodeGeneration],
    )))?;
    runtime_pipeline.remember_short_term(MemoryEntry {
        id: "impl-stm".to_string(),
        content: "recent".to_string(),
        tags: vec!["impl-check".to_string()],
        created_at: 1,
        expires_at: None,
        memory_type: MemoryType::Short,
        relevance: 0.8,
        metadata: HashMap::new(),
    })?;
    runtime_pipeline.remember_long_term(MemoryEntry {
        id: "impl-ltm".to_string(),
        content: "historical".to_string(),
        tags: vec!["impl-check".to_string()],
        created_at: 2,
        expires_at: None,
        memory_type: MemoryType::Long,
        relevance: 0.7,
        metadata: HashMap::new(),
    })?;
    runtime_pipeline.put_session_buffer("impl-session", "note", "persist");
    let runtime_task = Task::new("impl-runtime-task", TaskType::CodeGeneration, "clean")
        .with_metadata("session_id", "impl-session")
        .with_metadata("expert_chain", "runtime_wired");
    let runtime_execution = runtime_pipeline.execute(runtime_task)?;
    runtime_pipeline.capture_evaluation_baseline();
    let has_baseline = runtime_pipeline.has_evaluation_baseline();
    let has_report = runtime_pipeline
        .last_continuous_improvement_report()
        .is_some();
    let human_approved = runtime_pipeline.approve_pending_human_review_and_promote();

    let mut state = runtime_pipeline.export_governance_state();
    state.ensure_checksum();
    let state_valid = state.verify_checksum();
    let state_json = runtime_pipeline.export_governance_state_json()?;
    let bundle = runtime_pipeline.export_governance_bundle();
    let bundle_json = runtime_pipeline.export_governance_bundle_json()?;
    let runtime_bundle = runtime_pipeline.export_runtime_bundle();
    let runtime_bundle_json = runtime_pipeline.export_runtime_bundle_json()?;
    let state_preview = runtime_pipeline.preview_governance_import_json(&state_json)?;
    let bundle_preview = runtime_pipeline.preview_governance_bundle_import_json(&bundle_json)?;
    let runtime_preview =
        runtime_pipeline.preview_runtime_bundle_import_json(&runtime_bundle_json)?;
    let state_diff = runtime_pipeline.diff_governance_state(&state);
    runtime_pipeline.import_governance_state(state.clone());

    let mut restore_pipeline = MoePipelineBuilder::new().build();
    roundtrip_restore_pipeline(
        &mut restore_pipeline,
        &state_json,
        &bundle,
        &bundle_json,
        &runtime_bundle,
        &runtime_bundle_json,
    )?;

    let trail = restore_pipeline.governance_audit_trail();
    let snapshots = restore_pipeline.governance_state_snapshots().len();
    if let Some(last) = trail.entries.last() {
        restore_pipeline.rollback_governance_state_to_version(last.version)?;
    }
    tracing::info!(
        "Runtime persistence wiring: outputs={} baseline={} report={} human_approved={} state_valid={} state_allowed={} bundle_allowed={} runtime_allowed={} drift={} trail={} snapshots={}",
        runtime_execution.outputs.len(),
        has_baseline,
        has_report,
        human_approved,
        state_valid,
        state_preview.allowed,
        bundle_preview.allowed,
        runtime_preview.allowed,
        state_diff.has_drift,
        trail.entries.len(),
        snapshots
    );

    Ok(restore_pipeline)
}

pub(crate) fn run_training_and_cas_checks(
    restore_pipeline: &mut MoePipeline,
) -> Result<(), DynError> {
    let training_bundle =
        restore_pipeline.export_training_dataset_bundle(&impl_check_training_options(1))?;
    let training_bundle_json =
        restore_pipeline.export_training_dataset_bundle_json(&impl_check_training_options(2))?;
    let training_shards =
        restore_pipeline.export_training_dataset_shards(&impl_check_training_options(3), 128)?;
    let training_shards_json = restore_pipeline
        .export_training_dataset_shards_json(&impl_check_training_options(4), 128)?;
    let rebuilt_training_bundle =
        restore_pipeline.rebuild_training_dataset_bundle_from_shards(&training_shards)?;
    let rebuilt_training_bundle_from_json =
        restore_pipeline.rebuild_training_dataset_bundle_from_shards_json(&training_shards_json)?;
    let preview_training_bundle =
        restore_pipeline.preview_training_dataset_bundle_json(&training_bundle_json)?;
    let preview_training_bundle_from_shards =
        restore_pipeline.preview_training_dataset_shards_json(&training_shards_json)?;
    let (cas_runtime_payload, cas_governance_payload, cas_governance_state_payload) =
        compare_and_import_pipeline_payloads_with_checksum(restore_pipeline)?;
    tracing::info!(
        "Training dataset bundle: total={} included={} train={} valid={} json_bytes={} shards={} shards_json_bytes={} rebuilt={} rebuilt_json={} preview={} preview_shards={} cas_runtime_bytes={} cas_governance_bytes={} cas_governance_state_bytes={}",
        training_bundle.total_entries,
        training_bundle.included_entries,
        training_bundle.train_samples.len(),
        training_bundle.validation_samples.len(),
        training_bundle_json.len(),
        training_shards.len(),
        training_shards_json.len(),
        rebuilt_training_bundle.included_entries,
        rebuilt_training_bundle_from_json.included_entries,
        preview_training_bundle.included_entries,
        preview_training_bundle_from_shards.included_entries,
        cas_runtime_payload.len(),
        cas_governance_payload.len(),
        cas_governance_state_payload.len(),
    );
    Ok(())
}

fn impl_check_training_options(generated_at: u64) -> DatasetTrainingBuildOptions {
    DatasetTrainingBuildOptions {
        generated_at,
        validation_ratio: 0.2,
        min_score: None,
        include_failure_entries: true,
        include_partial_entries: true,
        include_unknown_entries: false,
        require_correction_for_failure: false,
        split_seed: 7,
    }
}

fn governance_checksum_for_pipeline(pipeline: &MoePipeline) -> String {
    pipeline
        .governance_audit_trail()
        .current_checksum
        .unwrap_or_else(|| pipeline.export_governance_state().state_checksum)
}

fn roundtrip_restore_pipeline(
    pipeline: &mut MoePipeline,
    state_json: &str,
    bundle: &orchestrator::GovernancePersistenceBundle,
    bundle_json: &str,
    runtime_bundle: &orchestrator::RuntimePersistenceBundle,
    runtime_bundle_json: &str,
) -> Result<(), DynError> {
    pipeline.import_governance_state_json(state_json)?;
    pipeline.try_import_governance_state_json(state_json)?;
    pipeline.import_governance_bundle(bundle.clone())?;
    pipeline.import_governance_bundle_json(bundle_json)?;
    pipeline.try_import_governance_bundle(bundle.clone())?;
    pipeline.try_import_governance_bundle_json(bundle_json)?;
    pipeline.import_runtime_bundle(runtime_bundle.clone())?;
    pipeline.import_runtime_bundle_json(runtime_bundle_json)?;
    pipeline.try_import_runtime_bundle(runtime_bundle.clone())?;
    pipeline.try_import_runtime_bundle_json(runtime_bundle_json)?;
    Ok(())
}

fn compare_and_import_pipeline_payloads_with_checksum(
    pipeline: &mut MoePipeline,
) -> Result<(String, String, String), DynError> {
    let runtime_payload = pipeline.export_runtime_bundle_json()?;
    let guard = pipeline.governance_audit_trail();
    let checksum = governance_checksum_for_pipeline(pipeline);
    pipeline.compare_and_import_runtime_bundle_json_with_checksum(
        guard.current_version,
        &checksum,
        &runtime_payload,
    )?;

    let governance_bundle_payload = pipeline.export_governance_bundle_json()?;
    let guard = pipeline.governance_audit_trail();
    let checksum = governance_checksum_for_pipeline(pipeline);
    pipeline.compare_and_import_governance_bundle_json_with_checksum(
        guard.current_version,
        &checksum,
        &governance_bundle_payload,
    )?;

    let governance_state_payload = pipeline.export_governance_state_json()?;
    let guard = pipeline.governance_audit_trail();
    let checksum = governance_checksum_for_pipeline(pipeline);
    pipeline.compare_and_import_governance_state_json_with_checksum(
        guard.current_version,
        &checksum,
        &governance_state_payload,
    )?;

    Ok((
        runtime_payload,
        governance_bundle_payload,
        governance_state_payload,
    ))
}
