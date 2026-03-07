use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ViolationCode {
    StructInvalidWorkspaceMembers,
    StructMissingBackendOrUi,
    StructThirdCrateDetected,
    StructForbiddenFolderName,
    StructMissingReadme,
    CrateNotBinOnly,
    CrateMissingMain,
    CratePrimaryItemContractViolation,
    NameProductMismatch,
    NameCrateMismatch,
    LayerUiImportsBackend,
    LayerUiSuspectDomainLogic,
    DetWallClockUsage,
    DetForbiddenTimeDep,
    DetStdoutUsage,
    DetStdioUsage,
    DetNondeterministicRngHeuristic,
    DetUnwrapRisk,
    DetPanicRisk,
    DetUnsafeUsage,
    DetUnderscoreUnusedMasking,
    StructMissingProductMetadata,
    StructMissingCrateManifest,
    StructShellRunMissingStrictMode,
    StructShellLoadHasFunctionDefinition,
}
