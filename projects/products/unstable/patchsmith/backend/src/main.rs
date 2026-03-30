mod cli;
mod dsl;
mod plan;
mod apply;
mod verify;
mod report;
mod diagnostics;
mod transport;

use std::collections::BTreeMap;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let code = match run(args) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("error: {e}");
            e.exit_code()
        }
    };
    std::process::exit(code);
}

fn run(args: Vec<String>) -> Result<i32, diagnostics::error::PatchsmithError> {
    let cmd = cli::cli_args::parse_args(&args)
        .map_err(|e| diagnostics::error::PatchsmithError::Parse(e))?;

    match cmd {
        cli::cli_args::CliCommand::Plan { dsl_file, json: _ } => {
            let dsl_text = std::fs::read_to_string(&dsl_file)?;
            let ops = dsl::dsl_parser::DslParser::parse(&dsl_text)?;
            let plan = plan::plan_builder::PlanBuilder::build(ops)?;
            let output = common_json::to_string(&plan)
                .map_err(|e| diagnostics::error::PatchsmithError::Internal(e.to_string()))?;
            println!("{output}");
            Ok(0)
        }
        cli::cli_args::CliCommand::Apply { plan_file } => {
            let plan_text = std::fs::read_to_string(&plan_file)?;
            let plan: plan::patch_plan::PatchPlan = common_json::from_json_str(&plan_text)
                .map_err(|e| diagnostics::error::PatchsmithError::Parse(e.to_string()))?;
            let mut file_contents = BTreeMap::new();
            for op in &plan.ops {
                let file_path = match op {
                    dsl::dsl_op::DslOp::ReplaceRange { file, .. } => file,
                    dsl::dsl_op::DslOp::ReplaceFirst { file, .. } => file,
                    dsl::dsl_op::DslOp::InsertAfter { file, .. } => file,
                    dsl::dsl_op::DslOp::DeleteRange { file, .. } => file,
                };
                if !file_contents.contains_key(file_path) {
                    let content = std::fs::read_to_string(file_path)?;
                    file_contents.insert(file_path.clone(), content);
                }
            }
            let result = apply::applier::Applier::apply(&plan, &file_contents)?;
            for (path, content) in &result.files {
                std::fs::write(path, content)?;
            }
            let verify = verify::verifier::Verifier::verify(&plan, &result);
            let report = report::patch_report::PatchReport::from_verify(&verify, plan.ops.len());
            let output = report.to_json()?;
            println!("{output}");
            Ok(0)
        }
        cli::cli_args::CliCommand::Verify { plan_file, json: _ } => {
            let plan_text = std::fs::read_to_string(&plan_file)?;
            let plan: plan::patch_plan::PatchPlan = common_json::from_json_str(&plan_text)
                .map_err(|e| diagnostics::error::PatchsmithError::Parse(e.to_string()))?;
            let mut file_contents = BTreeMap::new();
            for op in &plan.ops {
                let file_path = match op {
                    dsl::dsl_op::DslOp::ReplaceRange { file, .. } => file,
                    dsl::dsl_op::DslOp::ReplaceFirst { file, .. } => file,
                    dsl::dsl_op::DslOp::InsertAfter { file, .. } => file,
                    dsl::dsl_op::DslOp::DeleteRange { file, .. } => file,
                };
                if !file_contents.contains_key(file_path) {
                    let content = std::fs::read_to_string(file_path)?;
                    file_contents.insert(file_path.clone(), content);
                }
            }
            let result = apply::applier::Applier::apply(&plan, &file_contents)?;
            let verify = verify::verifier::Verifier::verify(&plan, &result);
            let report = report::patch_report::PatchReport::from_verify(&verify, plan.ops.len());
            let output = report.to_json()?;
            println!("{output}");
            if verify.ok { Ok(0) } else { Ok(1) }
        }
        cli::cli_args::CliCommand::Serve => {
            transport::server::run()
                .map_err(|e| diagnostics::error::PatchsmithError::Internal(e.to_string()))?;
            Ok(0)
        }
    }
}
