use crate::pr::commands::pr_body_context_options::PrBodyContextOptions;
use crate::pr_remote_snapshot::load_pr_remote_snapshot;
use crate::repo_name::resolve_repo_name;

pub(crate) fn run_body_context(opts: PrBodyContextOptions) -> i32 {
    let Ok(repo_name) = resolve_repo_name(opts.repo) else {
        return 0;
    };

    let Ok(snapshot) = load_pr_remote_snapshot(&opts.pr_number, &repo_name) else {
        return 0;
    };

    let labels_raw = snapshot
        .labels
        .iter()
        .map(|label| label.name.clone())
        .filter(|name: &String| !name.trim().is_empty())
        .collect::<Vec<_>>()
        .join("||");
    println!("{}\x1f{}\x1f{}", snapshot.title, snapshot.body, labels_raw);
    0
}
