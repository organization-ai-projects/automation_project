use crate::app::{Action, AnalyticsView, AppState, CatalogEntry, Controller, PlaybackView, reduce};
use crate::fixtures::FixtureLoader;
use crate::screens::{AnalyticsScreen, CatalogScreen, PlaybackScreen, TitleScreen};
use crate::transport::{BackendProcess, IpcClient};
use common_json::{Json, JsonAccess, pjson};
use std::io::{BufReader, BufWriter};

pub struct PublicApi;

impl PublicApi {
    pub fn run(args: &[String]) {
        if args.len() < 2 {
            usage();
        }
        match args[1].as_str() {
            "catalog" => cmd_catalog(&args[2..]),
            "play" => cmd_play(&args[2..]),
            "analytics" => cmd_analytics(&args[2..]),
            "recommend" => cmd_recommend(&args[2..]),
            _ => usage(),
        }
    }
}

fn usage() -> ! {
    eprintln!("Usage:");
    eprintln!("  vod_forge_ui catalog --import-fixture <dir>");
    eprintln!("  vod_forge_ui play --profile <name> --episode <id> --steps N");
    eprintln!("  vod_forge_ui analytics --profile <name> [--json]");
    eprintln!("  vod_forge_ui recommend --profile <name> [--all] [--json]");
    std::process::exit(2);
}

fn backend_binary() -> String {
    std::env::var("VOD_FORGE_BACKEND").unwrap_or_else(|_| "vod_forge_backend".to_string())
}

fn cmd_catalog(args: &[String]) {
    let mut import_fixture: Option<String> = None;
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--import-fixture" {
            i += 1;
            import_fixture = args.get(i).cloned();
        }
        i += 1;
    }
    let fixture_dir = import_fixture.unwrap_or_else(|| usage());
    let catalog_path = format!("{}/small_catalog.json", fixture_dir);
    let entries = FixtureLoader::load_catalog_json(&catalog_path).unwrap_or_else(|e| {
        let state = reduce(AppState::default(), Action::ErrorOccurred(e.to_string()));
        eprintln!("fixture error: {}", state.last_error.unwrap_or_default());
        std::process::exit(4);
    });

    let mut process = BackendProcess::spawn(&backend_binary()).unwrap_or_else(|e| {
        eprintln!("ipc error: {}", e);
        std::process::exit(3);
    });

    let mut writer = BufWriter::new(&mut process.stdin);
    let mut reader = BufReader::new(&mut process.stdout);
    let mut client = IpcClient::new();
    for entry in &entries {
        let req = pjson!({
            "type": "CatalogAddTitle",
            "title_id": entry.id.clone(),
            "name": entry.name.clone(),
            "year": entry.year,
        });
        let resp: Json = client
            .send_request(&mut writer, &mut reader, &req)
            .unwrap_or_else(|e| {
                eprintln!("ipc error: {}", e);
                std::process::exit(3);
            });
        validate_ok_response(&resp).unwrap_or_else(|e| {
            eprintln!("ipc error: {e}");
            std::process::exit(3);
        });
    }

    let mut controller = Controller { ipc: client };
    let catalog_titles: Vec<CatalogEntry> = controller
        .catalog_list(&mut writer, &mut reader)
        .unwrap_or_else(|e| {
            eprintln!("ipc error: {e}");
            std::process::exit(3);
        });

    let mut state = reduce(AppState::default(), Action::Reset);
    state = reduce(state, Action::CatalogLoaded(catalog_titles));
    println!("{}", CatalogScreen::render(&state));
    if let Some(first) = state.catalog_titles.first() {
        println!("{}", TitleScreen::render(&state, &first.id));
    }
    let _ = process.child.kill();
}

fn cmd_play(args: &[String]) {
    let mut profile: Option<String> = None;
    let mut episode: Option<String> = None;
    let mut steps: u32 = 10;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--profile" => {
                i += 1;
                profile = args.get(i).cloned();
            }
            "--episode" => {
                i += 1;
                episode = args.get(i).cloned();
            }
            "--steps" => {
                i += 1;
                steps = args.get(i).and_then(|s| s.parse().ok()).unwrap_or(10);
            }
            _ => {}
        }
        i += 1;
    }
    let profile = profile.unwrap_or_else(|| usage());
    let episode_id = episode.unwrap_or_else(|| usage());

    let mut process = BackendProcess::spawn(&backend_binary()).unwrap_or_else(|e| {
        eprintln!("ipc error: {}", e);
        std::process::exit(3);
    });
    let mut writer = BufWriter::new(&mut process.stdin);
    let mut reader = BufReader::new(&mut process.stdout);
    let mut controller = Controller::new();

    let pv: PlaybackView = controller
        .playback_start(&mut writer, &mut reader, &profile, &episode_id)
        .unwrap_or_else(|e| {
            eprintln!("ipc error: {}", e);
            std::process::exit(3);
        });

    let session_id = pv.session_id.clone();
    let mut state = reduce(AppState::default(), Action::PlaybackUpdated(pv));

    for _ in 0..steps {
        let pv = controller
            .playback_step(&mut writer, &mut reader, &session_id, 1)
            .unwrap_or_else(|e| {
                eprintln!("ipc error: {}", e);
                std::process::exit(3);
            });
        let done = pv.done;
        state = reduce(state, Action::PlaybackUpdated(pv));
        if done {
            break;
        }
    }

    println!("{}", PlaybackScreen::render(&state));
    let _ = process.child.kill();
}

fn cmd_analytics(args: &[String]) {
    let mut profile: Option<String> = None;
    let mut json_output = false;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--profile" => {
                i += 1;
                profile = args.get(i).cloned();
            }
            "--json" => {
                json_output = true;
            }
            _ => {}
        }
        i += 1;
    }
    let profile = profile.unwrap_or_else(|| usage());

    let mut process = BackendProcess::spawn(&backend_binary()).unwrap_or_else(|e| {
        eprintln!("ipc error: {}", e);
        std::process::exit(3);
    });
    let mut writer = BufWriter::new(&mut process.stdin);
    let mut reader = BufReader::new(&mut process.stdout);
    let mut controller = Controller::new();

    let av: AnalyticsView = controller
        .analytics_report(&mut writer, &mut reader, &profile)
        .unwrap_or_else(|e| {
            eprintln!("ipc error: {}", e);
            std::process::exit(3);
        });

    let state = reduce(AppState::default(), Action::AnalyticsLoaded(av.clone()));
    if json_output {
        let json = common_json::to_string(&av).unwrap_or_else(|e| {
            eprintln!("json error: {e}");
            std::process::exit(5);
        });
        println!("{json}");
    } else {
        println!("{}", AnalyticsScreen::render(&state));
    }
    let _ = process.child.kill();
}

fn cmd_recommend(args: &[String]) {
    let mut profile: Option<String> = None;
    let mut include_watched = false;
    let mut json_output = false;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--profile" => {
                i += 1;
                profile = args.get(i).cloned();
            }
            "--all" => {
                include_watched = true;
            }
            "--json" => {
                json_output = true;
            }
            _ => {}
        }
        i += 1;
    }
    let profile = profile.unwrap_or_else(|| usage());

    let mut process = BackendProcess::spawn(&backend_binary()).unwrap_or_else(|e| {
        eprintln!("ipc error: {}", e);
        std::process::exit(3);
    });
    let mut writer = BufWriter::new(&mut process.stdin);
    let mut reader = BufReader::new(&mut process.stdout);
    let mut controller = Controller::new();

    let recommendations = controller
        .recommend(&mut writer, &mut reader, &profile, !include_watched)
        .unwrap_or_else(|e| {
            eprintln!("ipc error: {}", e);
            std::process::exit(3);
        });

    if json_output {
        let json = common_json::to_string(&recommendations).unwrap_or_else(|e| {
            eprintln!("json error: {e}");
            std::process::exit(5);
        });
        println!("{json}");
        let _ = process.child.kill();
        return;
    }

    println!("=== Recommendations ===");
    if recommendations.is_empty() {
        println!("(no recommendations)");
    } else {
        for episode in recommendations {
            println!("- {episode}");
        }
    }
    let _ = process.child.kill();
}

fn validate_ok_response(resp: &Json) -> Result<(), String> {
    let id = resp
        .get_field("id")
        .and_then(|value| value.as_u64_strict())
        .map_err(|_| "invalid response id".to_string())?;
    if id == 0 {
        return Err("invalid response id".to_string());
    }

    let payload = resp
        .get_field("payload")
        .map_err(|_| "missing response payload".to_string())?;
    let payload_type = payload
        .get_field("type")
        .and_then(|value| value.as_str_strict())
        .map_err(|_| "missing response payload type".to_string())?;

    match payload_type {
        "Ok" => Ok(()),
        "Error" => {
            let message = payload
                .get_field("message")
                .and_then(|value| value.as_str_strict())
                .unwrap_or("backend error");
            Err(message.to_string())
        }
        other => Err(format!("unexpected response payload type: {other}")),
    }
}
