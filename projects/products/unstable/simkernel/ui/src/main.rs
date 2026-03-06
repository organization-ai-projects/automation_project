mod app;
mod diagnostics;
mod fixtures;
mod public_api;
mod screens;
mod transport;
mod widgets;

use crate::public_api::Controller;
use crate::screens::home_screen::HomeScreen;
use crate::screens::inspect_screen::InspectScreen;
use crate::screens::pack_screen::PackScreen;
use crate::screens::replay_screen::ReplayScreen;
use crate::screens::report_screen::ReportScreen;
use crate::screens::run_screen::RunScreen;
use crate::screens::scenario_screen::ScenarioScreen;
use crate::widgets::graph_widget::GraphWidget;
use crate::widgets::log_widget::LogWidget;
use crate::widgets::table_widget::TableWidget;

fn main() {
    tracing_subscriber::fmt::init();
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_usage();
        std::process::exit(2);
    }

    match args[1].as_str() {
        "run" => {
            let mut pack = String::new();
            let mut seed: u64 = 0;
            let mut ticks: u64 = 100;
            let mut out = String::from("report.json");
            let mut replay_out: Option<String> = None;
            let mut i = 2;
            while i < args.len() {
                match args[i].as_str() {
                    "--pack" => {
                        i += 1;
                        if i < args.len() {
                            pack = args[i].clone();
                        }
                    }
                    "--seed" => {
                        i += 1;
                        if i < args.len() {
                            seed = args[i].parse().unwrap_or(0);
                        }
                    }
                    "--ticks" => {
                        i += 1;
                        if i < args.len() {
                            ticks = args[i].parse().unwrap_or(100);
                        }
                    }
                    "--out" => {
                        i += 1;
                        if i < args.len() {
                            out = args[i].clone();
                        }
                    }
                    "--replay-out" => {
                        i += 1;
                        if i < args.len() {
                            replay_out = Some(args[i].clone());
                        }
                    }
                    _ => {}
                }
                i += 1;
            }
            let mut controller = Controller::new();
            match replay_out.as_deref() {
                Some(path) => {
                    if let Err(e) =
                        controller.run_pack_with_replay(&pack, seed, ticks, &out, Some(path))
                    {
                        eprintln!("Error: {}", e);
                        std::process::exit(5);
                    }
                }
                None => {
                    if let Err(e) = controller.run_pack(&pack, seed, ticks, &out) {
                        eprintln!("Error: {}", e);
                        std::process::exit(5);
                    }
                }
            }
            let home = HomeScreen;
            let pack_screen = PackScreen {
                packs: vec![pack.clone()],
            };
            let scenario_screen = ScenarioScreen {
                scenario_name: "default".to_string(),
            };
            let run_screen = RunScreen {
                tick: ticks,
                pack_kind: pack.clone(),
            };
            let report_preview = controller
                .state()
                .last_report
                .clone()
                .unwrap_or_else(|| "report-written".to_string());
            let report_screen = ReportScreen {
                report_json: report_preview.clone(),
            };
            let table = TableWidget {
                headers: vec!["field".to_string(), "value".to_string()],
                rows: vec![
                    vec!["pack".to_string(), pack.clone()],
                    vec!["seed".to_string(), seed.to_string()],
                    vec!["ticks".to_string(), ticks.to_string()],
                ],
            };
            let graph = GraphWidget {
                values: vec![ticks as i64],
                label: "run_ticks".to_string(),
            };
            let log = LogWidget {
                entries: vec![
                    home.render(),
                    pack_screen.render(),
                    scenario_screen.render(),
                    run_screen.render(),
                    report_screen.render(),
                    table.render(),
                    graph.render(),
                ],
            };
            println!("{}", log.render());
            let state = controller.state();
            println!(
                "State pack={:?} seed={} ticks={} last_error={:?}",
                state.pack_kind, state.seed, state.ticks, state.last_error
            );
            if let Some(replay_path) = replay_out {
                let replay_screen = ReplayScreen {
                    replay_path: replay_path.clone(),
                };
                println!("{}", replay_screen.render());
            }
        }
        "replay" => {
            let mut replay = String::new();
            let mut out = String::from("report.json");
            let mut i = 2;
            while i < args.len() {
                match args[i].as_str() {
                    "--replay" => {
                        i += 1;
                        if i < args.len() {
                            replay = args[i].clone();
                        }
                    }
                    "--out" => {
                        i += 1;
                        if i < args.len() {
                            out = args[i].clone();
                        }
                    }
                    _ => {}
                }
                i += 1;
            }
            if replay.is_empty() {
                eprintln!("Missing --replay <file>");
                std::process::exit(2);
            }
            let mut controller = Controller::new();
            if let Err(e) = controller.replay_to_report(&replay, &out) {
                eprintln!("Error: {}", e);
                std::process::exit(4);
            }
            let replay_screen = ReplayScreen {
                replay_path: replay.clone(),
            };
            println!("{}", replay_screen.render());
        }
        "inspect" => {
            let mut replay = String::new();
            let mut query = String::new();
            let mut as_json = false;
            let mut i = 2;
            while i < args.len() {
                match args[i].as_str() {
                    "--replay" => {
                        i += 1;
                        if i < args.len() {
                            replay = args[i].clone();
                        }
                    }
                    "--query" => {
                        i += 1;
                        if i < args.len() {
                            query = args[i].clone();
                        }
                    }
                    "--json" => {
                        as_json = true;
                    }
                    _ => {}
                }
                i += 1;
            }
            if replay.is_empty() || query.is_empty() {
                eprintln!("Missing --replay <file> or --query <q>");
                std::process::exit(2);
            }
            let mut controller = Controller::new();
            match controller.inspect_replay(&replay, &query) {
                Ok(result) => {
                    let inspect_screen = InspectScreen {
                        query_result: result.clone(),
                    };
                    let log = LogWidget {
                        entries: vec![inspect_screen.render()],
                    };
                    if as_json {
                        println!("{}", result);
                    } else {
                        println!("{}", log.render());
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(5);
                }
            }
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            std::process::exit(2);
        }
    }
}

fn print_usage() {
    println!("simkernel_ui - deterministic simulation UI client");
    println!();
    println!("Commands:");
    println!("  run --pack <pack_kind> --seed <n> --ticks <n> --out <file> [--replay-out <file>]");
    println!("  replay --replay <replay.bin> --out <report.json>");
    println!("  inspect --replay <replay.bin> --query <q> --json");
}
