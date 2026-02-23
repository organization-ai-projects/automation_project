use crate::auth::{
    AuditEntry, AuditLog, AuditOutcome, AuthToken, Permission, PermissionGrant, TokenClaims,
    TokenVerifier,
};
use crate::checkouts::{Checkout, CheckoutPolicy, Materialized};
use crate::diffs::{ContentClass, Diff, DiffEntry, DiffKind};
use crate::errors::PvError;
use crate::history::{HistoryEntry, HistoryPage, HistoryWalker};
use crate::http::{ApiError, ApiVersion, RequestEnvelope, ResponseEnvelope};
use crate::ids::{BlobId, CommitId, ObjectId, RefId, RepoId, TreeId};
use crate::indexes::{Index, IndexEntry, SafePath};
use crate::merges::{Conflict, ConflictKind, Merge, MergeResult};
use crate::objects::{
    Blob, Commit, Object, ObjectKind, ObjectStore, Tree, TreeEntry, TreeEntryKind,
};
use crate::pipeline::{CommitBuilder, CommitResult, Snapshot, SnapshotEntry};
use crate::refs_store::{HeadState, RefKind, RefName, RefStore, RefTarget};
use crate::repos::{Repo, RepoMetadata, RepoStore};
use crate::sync::{FetchRequest, Negotiation, RefUpdate, RefUpdatePolicy, UploadRequest};
use crate::verify::{IntegrityIssue, IntegrityReport, Verification};

type WriteRefFn =
    fn(&RefStore, &RefName, &RefTarget, bool, Option<&ObjectStore>) -> Result<(), PvError>;
type RepoCreateFn = fn(&RepoStore, RepoId, String, Option<String>, u64) -> Result<Repo, PvError>;
type RepoUpdateFn = fn(
    &RepoStore,
    &RepoId,
    Option<String>,
    Option<Option<String>>,
    u64,
) -> Result<RepoMetadata, PvError>;
type MergePerformFn = fn(
    &CommitId,
    &CommitId,
    &str,
    &str,
    u64,
    &ObjectStore,
    &RefStore,
) -> Result<MergeResult, PvError>;

pub fn touch_api_surface() {
    let _ = std::mem::size_of::<AuditEntry>();
    let _ = std::mem::size_of::<AuditLog>();
    let _ = std::mem::size_of::<AuditOutcome>();
    let _ = std::mem::size_of::<AuthToken>();
    let _ = std::mem::size_of::<Permission>();
    let _ = std::mem::size_of::<PermissionGrant>();
    let _ = std::mem::size_of::<TokenClaims>();
    let _ = std::mem::size_of::<TokenVerifier>();
    let _ = std::mem::size_of::<Checkout>();
    let _ = std::mem::size_of::<CheckoutPolicy>();
    let _ = std::mem::size_of::<Materialized>();
    let _ = std::mem::size_of::<Diff>();
    let _ = std::mem::size_of::<DiffEntry>();
    let _ = std::mem::size_of::<DiffKind>();
    let _ = std::mem::size_of::<ContentClass>();
    let _ = std::mem::size_of::<HistoryEntry>();
    let _ = std::mem::size_of::<HistoryPage>();
    let _ = std::mem::size_of::<HistoryWalker<'static>>();
    let _ = std::mem::size_of::<ApiError>();
    let _ = std::mem::size_of::<ApiVersion>();
    let _ = std::mem::size_of::<RequestEnvelope>();
    let _ = std::mem::size_of::<ResponseEnvelope<String>>();
    let _ = std::mem::size_of::<BlobId>();
    let _ = std::mem::size_of::<CommitId>();
    let _ = std::mem::size_of::<ObjectId>();
    let _ = std::mem::size_of::<RefId>();
    let _ = std::mem::size_of::<RepoId>();
    let _ = std::mem::size_of::<TreeId>();
    let _ = std::mem::size_of::<Index>();
    let _ = std::mem::size_of::<IndexEntry>();
    let _ = std::mem::size_of::<SafePath>();
    let _ = std::mem::size_of::<Conflict>();
    let _ = std::mem::size_of::<ConflictKind>();
    let _ = std::mem::size_of::<Merge>();
    let _ = std::mem::size_of::<MergeResult>();
    let _ = std::mem::size_of::<Blob>();
    let _ = std::mem::size_of::<Commit>();
    let _ = std::mem::size_of::<Object>();
    let _ = std::mem::size_of::<ObjectKind>();
    let _ = std::mem::size_of::<ObjectStore>();
    let _ = std::mem::size_of::<Tree>();
    let _ = std::mem::size_of::<TreeEntry>();
    let _ = std::mem::size_of::<TreeEntryKind>();
    let _ = std::mem::size_of::<CommitBuilder>();
    let _ = std::mem::size_of::<CommitResult>();
    let _ = std::mem::size_of::<Snapshot>();
    let _ = std::mem::size_of::<SnapshotEntry>();
    let _ = std::mem::size_of::<HeadState>();
    let _ = std::mem::size_of::<RefKind>();
    let _ = std::mem::size_of::<RefName>();
    let _ = std::mem::size_of::<RefStore>();
    let _ = std::mem::size_of::<RefTarget>();
    let _ = std::mem::size_of::<Repo>();
    let _ = std::mem::size_of::<RepoMetadata>();
    let _ = std::mem::size_of::<RepoStore>();
    let _ = std::mem::size_of::<FetchRequest>();
    let _ = std::mem::size_of::<Negotiation>();
    let _ = std::mem::size_of::<RefUpdate>();
    let _ = std::mem::size_of::<RefUpdatePolicy>();
    let _ = std::mem::size_of::<UploadRequest>();
    let _ = std::mem::size_of::<IntegrityIssue>();
    let _ = std::mem::size_of::<IntegrityReport>();
    let _ = std::mem::size_of::<Verification>();

    let _ = PvError::MergeConflict(String::new());
    let _ = PvError::PermissionDenied(String::new());

    let _ = TokenVerifier::new(vec![0u8; 32]);
    let _: fn(&TokenVerifier, &TokenClaims) -> Result<AuthToken, PvError> = TokenVerifier::issue;
    let _: fn(&TokenVerifier, &AuthToken) -> Result<TokenClaims, PvError> = TokenVerifier::verify;
    let _: fn(&TokenClaims, &RepoId, Permission) -> bool = TokenClaims::has_permission;
    let _: fn(&TokenClaims, u64) -> bool = TokenClaims::is_valid_at;
    let _: fn() -> AuditLog = AuditLog::new;
    let _: fn(&AuditLog, AuditEntry) = AuditLog::record;
    let _: fn(&AuditLog) -> Vec<AuditEntry> = AuditLog::snapshot;

    let _: fn() -> CheckoutPolicy = CheckoutPolicy::overwrite;
    let _: fn() -> CheckoutPolicy = CheckoutPolicy::clean;
    let _: fn() -> CheckoutPolicy = CheckoutPolicy::safe;
    let _: fn(
        &CommitId,
        &ObjectStore,
        &std::path::Path,
        &CheckoutPolicy,
    ) -> Result<Materialized, PvError> = Checkout::materialize;
    let _: fn(&CommitId, &CommitId, &ObjectStore) -> Result<Diff, PvError> = Diff::compute;
    if let Ok(store) = ObjectStore::open(std::env::temp_dir().join("pv_clippy_touch_history")) {
        let walker = HistoryWalker::new(&store);
        let commit = CommitId::from_bytes(&[1u8; 32]);
        let _ = walker.page(&commit, 1);
    }

    let _: fn(&ApiVersion) -> &'static str = ApiVersion::path_prefix;
    let _: fn(PvError) -> ApiError = ApiError::from;
    let _: fn(&PvError) -> u16 = crate::http::api_error::http_status_for;
    let _: fn(String) -> ResponseEnvelope<String> = ResponseEnvelope::<String>::ok;
    let _: fn(ApiError) -> ResponseEnvelope<String> = ResponseEnvelope::<String>::err;

    let _: fn(&[u8; 32]) -> BlobId = BlobId::from_bytes;
    let _: fn(&[u8; 32]) -> CommitId = CommitId::from_bytes;
    let _: fn(&[u8; 32]) -> TreeId = TreeId::from_bytes;
    let _: fn(&BlobId) -> &str = BlobId::as_str;
    let _: fn(&CommitId) -> &str = CommitId::as_str;
    let _: fn(&TreeId) -> &str = TreeId::as_str;
    let oid = ObjectId::from_bytes(&[9u8; 32]);
    let _ = oid.to_bytes();
    let _: fn(&RefId) -> &str = RefId::as_str;
    let _: fn(&RepoId) -> &str = RepoId::as_str;
    let _: fn(&RefName) -> &str = RefName::as_str;
    let _: fn(&RefName) -> RefKind = RefName::kind;
    let _: fn(&RefName) -> &str = RefName::short_name;

    let _: fn() -> Index = Index::new;
    let _: fn(&Index) -> Result<(), PvError> = Index::check_version;
    let _: fn(&mut Index, SafePath, BlobId) -> Option<BlobId> = Index::add;
    let _: fn(&mut Index, &SafePath) -> Option<BlobId> = Index::remove;
    let mut idx = Index::new();
    if let Ok(path) = "a.txt".parse::<SafePath>() {
        let blob = BlobId::from_bytes(&[2u8; 32]);
        let _ = idx.add(path.clone(), blob);
        let _ = idx.get(&path);
    }
    let _: fn(&Index) -> usize = Index::len;
    let _: fn(&Index) -> bool = Index::is_empty;

    let _ = Blob::from_bytes(vec![1, 2, 3]);
    let _ = crate::objects::HashDigest::compute(b"x");
    let _ = Commit::new(
        TreeId::from_bytes(&[3u8; 32]),
        Vec::new(),
        "a".to_string(),
        "m".to_string(),
        0,
    );
    let _: fn(Vec<TreeEntry>) -> Tree = Tree::from_entries;
    let _: fn(&[TreeEntry]) -> TreeId = Tree::compute_id;
    let _: fn(&Tree) -> bool = Tree::verify;
    let _: fn(&Object) -> ObjectKind = Object::kind;
    let _: fn(&Object) -> bool = Object::verify;
    let _ = ObjectStore::open(std::path::PathBuf::from("."));
    let _: fn(&ObjectStore, Object) -> Result<ObjectId, PvError> = ObjectStore::write;
    let _: fn(&ObjectStore, &ObjectId) -> Result<Object, PvError> = ObjectStore::read;
    let _: fn(&ObjectStore, &ObjectId) -> bool = ObjectStore::exists;

    let _ = CommitBuilder::new("author", "message", 0);
    let _: fn(CommitBuilder, CommitId) -> CommitBuilder = CommitBuilder::with_parent;
    let _: fn(CommitBuilder, &Index, &ObjectStore, &RefStore) -> Result<CommitResult, PvError> =
        CommitBuilder::commit;
    let _: fn(std::collections::BTreeMap<SafePath, BlobId>) -> Snapshot = Snapshot::from_map;
    let _: fn(&Snapshot, &ObjectStore) -> Result<TreeId, PvError> = Snapshot::write_trees;

    let _ = RefStore::open(std::path::PathBuf::from("."));
    let _: fn(&RefStore) -> Result<HeadState, PvError> = RefStore::read_head;
    let _: fn(&RefStore, &HeadState) -> Result<(), PvError> = RefStore::write_head;
    let _: fn(&RefStore, &RefName) -> Result<RefTarget, PvError> = RefStore::read_ref;
    let _: WriteRefFn = RefStore::write_ref;
    let _: fn(&RefStore) -> Result<std::collections::HashMap<RefName, RefTarget>, PvError> =
        RefStore::list_refs;
    let _: fn(&RefTarget) -> &CommitId = RefTarget::commit_id;

    let _ = RepoStore::open(std::path::PathBuf::from("."));
    let _: RepoCreateFn = RepoStore::create;
    let _: fn(&RepoStore, &RepoId) -> Result<Repo, PvError> = RepoStore::get;
    let _: RepoUpdateFn = RepoStore::update_metadata;
    let _: fn(&RepoStore) -> Result<Vec<RepoId>, PvError> = RepoStore::list;
    let _: fn(&RepoStore, &RepoId) -> bool = RepoStore::exists;
    let _: fn(&Repo) -> &RepoId = Repo::id;

    let _: MergePerformFn = Merge::perform;
    let _: fn(&FetchRequest, &ObjectStore, &RefStore) -> Result<Vec<Object>, PvError> =
        Negotiation::collect;
    let _: fn(&ObjectStore, &RefStore) -> Result<IntegrityReport, PvError> = Verification::run;
    let _: fn(&IntegrityReport) -> bool = IntegrityReport::is_healthy;
}
