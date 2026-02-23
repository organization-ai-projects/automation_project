// projects/products/unstable/platform_versioning/backend/src/tests/integration.rs
//! Integration tests covering the full repo create → commit → browse → diff → merge flow.

use std::sync::atomic::{AtomicU64, Ordering};

use crate::auth::audit_entry::AuditEntry;
use crate::auth::{AuditOutcome, Permission, PermissionGrant, TokenClaims, TokenVerifier};
use crate::checkout::{Checkout, CheckoutPolicy};
use crate::diff::Diff;
use crate::history::HistoryWalker;
use crate::ids::RepoId;
use crate::index::Index;
use crate::merge::{Merge, MergeResult};
use crate::objects::{Blob, Object, ObjectStore};
use crate::pipeline::CommitBuilder;
use crate::refs_store::RefStore;
use crate::repos::RepoStore;
use crate::verify::Verification;

static COUNTER: AtomicU64 = AtomicU64::new(0);

fn unique_test_dir(tag: &str) -> std::path::PathBuf {
    let id = COUNTER.fetch_add(1, Ordering::SeqCst);
    let pid = std::process::id();
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.subsec_nanos())
        .unwrap_or(0);
    std::env::temp_dir().join(format!("pv_integ_{tag}_{pid}_{nanos}_{id}"))
}

fn make_stores(tag: &str) -> (ObjectStore, RefStore) {
    let dir = unique_test_dir(tag);
    (
        ObjectStore::open(&dir).unwrap(),
        RefStore::open(&dir).unwrap(),
    )
}

fn commit_files(
    files: &[(&str, &[u8])],
    ts: u64,
    obj: &ObjectStore,
    refs: &RefStore,
) -> crate::ids::CommitId {
    let mut idx = Index::new();
    for (path, content) in files {
        let blob = Blob::from_bytes(content.to_vec());
        idx.add(path.parse().unwrap(), blob.id.clone());
        obj.write(Object::Blob(blob)).unwrap();
    }
    CommitBuilder::new("user", "msg", ts)
        .commit(&idx, obj, refs)
        .unwrap()
        .commit_id
}

#[test]
fn full_flow_create_commit_browse_diff_merge() {
    let (obj, refs) = make_stores("full_flow");

    // === Create repo ===
    let repo_store_dir = unique_test_dir("full_flow_repos");
    let repo_store = RepoStore::open(&repo_store_dir).unwrap();
    let repo_id: RepoId = "test-repo".parse().unwrap();
    repo_store
        .create(repo_id.clone(), "Test Repo".to_string(), None, 1000)
        .unwrap();
    let repo = repo_store.get(&repo_id).unwrap();
    assert_eq!(repo.metadata.name, "Test Repo");

    // === Commit ===
    let c1 = commit_files(
        &[("readme.md", b"# Hello"), ("src/main.rs", b"fn main() {}")],
        1,
        &obj,
        &refs,
    );
    assert!(obj.exists(c1.as_object_id()));

    // === Browse (history) ===
    let c2 = commit_files(
        &[
            ("readme.md", b"# Updated"),
            ("src/main.rs", b"fn main() {}"),
        ],
        2,
        &obj,
        &refs,
    );
    let walker = HistoryWalker::new(&obj);
    let page = walker.page(&c2, 10).unwrap();
    assert_eq!(page.entries.len(), 2);

    // === Diff ===
    let diff = Diff::compute(&c1, &c2, &obj).unwrap();
    assert_eq!(diff.entries.len(), 1); // only readme.md changed

    // === Checkout ===
    let checkout_dir = unique_test_dir("full_flow_checkout");
    let mat =
        Checkout::materialize(&c1, &obj, &checkout_dir, &CheckoutPolicy::overwrite()).unwrap();
    assert_eq!(mat.files_written, 2);

    // === Merge attempt ===
    let (obj2, refs2) = make_stores("full_flow_merge");
    let base = commit_files(&[("shared.txt", b"shared content")], 1, &obj2, &refs2);
    let ours = commit_files(&[("ours.txt", b"ours")], 2, &obj2, &refs2);
    // Build theirs as a standalone commit from base (diverging branch).
    let theirs = {
        let blob = Blob::from_bytes(b"theirs".to_vec());
        let their_blob_id = blob.id.clone();
        obj2.write(Object::Blob(blob)).unwrap();

        let mut idx = Index::new();
        idx.add("theirs.txt".parse().unwrap(), their_blob_id);

        use crate::pipeline::Snapshot;
        use std::collections::BTreeMap;
        let map: BTreeMap<_, _> = idx.entries().map(|e| (e.path, e.blob_id)).collect();
        let tree_id = Snapshot::from_map(map).write_trees(&obj2).unwrap();

        let commit = crate::objects::Commit::new(
            tree_id,
            vec![base],
            "user".to_string(),
            "theirs".to_string(),
            3,
        );
        let id = commit.id.clone();
        obj2.write(crate::objects::Object::Commit(commit)).unwrap();
        id
    };
    let merge_result = Merge::perform(&ours, &theirs, "merger", "merge", 4, &obj2, &refs2).unwrap();
    assert!(matches!(merge_result, MergeResult::Clean { .. }));

    // === Verify ===
    let report = Verification::run(&obj, &refs).unwrap();
    assert!(report.is_healthy(), "{:?}", report.issues);
}

#[test]
fn auth_denied_without_token() {
    let verifier = TokenVerifier::new(b"a_very_secure_secret_key_here_!!" as &[u8]).unwrap();
    // A token with no grants should not have any permission.
    let claims = TokenClaims {
        subject: "anonymous".to_string(),
        grants: vec![],
        expires_at: None,
    };
    let repo_id: RepoId = "secret-repo".parse().unwrap();
    assert!(!claims.has_permission(&repo_id, Permission::Read));
    assert!(!claims.has_permission(&repo_id, Permission::Write));
    assert!(!claims.has_permission(&repo_id, Permission::Admin));

    // Issuing and verifying a token with a specific grant works.
    let claims_with_read = TokenClaims {
        subject: "alice".to_string(),
        grants: vec![PermissionGrant {
            repo_id: Some(repo_id.clone()),
            permission: Permission::Read,
        }],
        expires_at: None,
    };
    let token = verifier.issue(&claims_with_read).unwrap();
    let decoded = verifier.verify(&token).unwrap();
    assert!(decoded.has_permission(&repo_id, Permission::Read));
    assert!(!decoded.has_permission(&repo_id, Permission::Write));
}

#[test]
fn auth_enforced_via_audit_log() {
    use crate::auth::AuditLog;
    let log = AuditLog::new();

    // Record a denied action.
    log.record(AuditEntry {
        timestamp_secs: 100,
        subject: "mallory".to_string(),
        action: "repo.create".to_string(),
        repo_id: None,
        outcome: AuditOutcome::Denied,
    });

    // Record an allowed action.
    log.record(AuditEntry {
        timestamp_secs: 101,
        subject: "alice".to_string(),
        action: "repo.create".to_string(),
        repo_id: None,
        outcome: AuditOutcome::Allowed,
    });

    let snapshot = log.snapshot();
    assert_eq!(snapshot.len(), 2);
    assert_eq!(snapshot[0].outcome, AuditOutcome::Denied);
    assert_eq!(snapshot[1].outcome, AuditOutcome::Allowed);
}
