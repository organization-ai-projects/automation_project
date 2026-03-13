use crate::pr::commands::pr_pr_state_options::PrPrStateOptions;
use crate::pr::contracts::github::pr_state_snapshot::PrStateSnapshot;
use crate::pr::gh_cli::gh_output_trim_end_newline;
use crate::repo_name::resolve_repo_name;

pub(crate) fn run_pr_state(opts: PrPrStateOptions) -> i32 {
    let Ok(repo_name) = resolve_repo_name(opts.repo) else {
        println!();
        return 0;
    };

    match fetch_pr_state(&opts.pr_number, &repo_name) {
        Ok(state) => println!("{state}"),
        Err(_) => println!(),
    }
    0
}

fn fetch_pr_state(pr_number: &str, repo_name: &str) -> Result<String, String> {
    let json = gh_output_trim_end_newline(
        "pr",
        &["view", pr_number, "-R", repo_name, "--json", "state"],
    )?;
    let snapshot = common_json::from_json_str::<PrStateSnapshot>(&json)
        .map_err(|err| format!("Error: invalid gh PR state payload: {err}"))?;
    Ok(snapshot.state)
}
