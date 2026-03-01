mod app;
mod diagnostics;
mod fixtures;
mod screens;
mod transport;
mod widgets;

use app::{Action, AppState, Controller, reduce};
use fixtures::FixtureLoader;
use screens::{AnalyticsScreen, CatalogScreen, PlaybackScreen};
use serde::{Deserialize, Serialize};
use std::io::{BufReader, BufWriter};
use transport::{BackendProcess, IpcClient};

fn usage() -> ! {
    eprintln!("Usage:");
    eprintln!("  vod_forge_ui catalog --import-fixture <dir>");
    eprintln!("  vod_forge_ui play --profile <name> --episode <id> --steps N");
    eprintln!("  vod_forge_ui analytics --profile <name> [--json]");
    std::process::exit(2);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        usage();
    }
    match args[1].as_str() {
        "catalog" => cmd_catalog(&args[2..]),
        "play" => cmd_play(&args[2..]),
        "analytics" => cmd_analytics(&args[2..]),
        _ => usage(),
    }
}

fn backend_binary() -> String {
    std::env::var("VOD_FORGE_BACKEND").unwrap_or_else(|_| "vod_forge_backend".to_string())
}

#[derive(Serialize)]
#[serde(tag = "type")]
enum Req {
    CatalogAddTitle { title_id: String, name: String, year: u16 },
}

#[derive(Deserialize)]
struct AnyResp {
    #[allow(dead_code)]
    pub id: u64,
    pub payload: serde::de::IgnoredAny,
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
        eprintln!("fixture error: {}", e);
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
        let req = Req::CatalogAddTitle {
            title_id: entry.id.clone(),
            name: entry.name.clone(),
            year: entry.year,
        };
        let _resp: AnyResp = client.send_request(&mut writer, &mut reader, &req).unwrap_or_else(|e| {
            eprintln!("ipc error: {}", e);
            std::process::exit(3);
        });
        println!("added title: {} ({})", entry.name, entry.year);
    }

    println!("{}", CatalogScreen::render(&AppState {
        catalog_titles: entries,
        ..Default::default()
    }));
}

fn cmd_play(args: &[String]) {
    let mut profile: Option<String> = None;
    let mut episode: Option<String> = None;
    let mut steps: u32 = 10;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--profile" => { i += 1; profile = args.get(i).cloned(); }
            "--episode" => { i += 1; episode = args.get(i).cloned(); }
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

    let pv = controller
        .playback_start(&mut writer, &mut reader, &profile, &episode_id)
        .unwrap_or_else(|e| { eprintln!("ipc error: {}", e); std::process::exit(3); });

    let session_id = pv.session_id.clone();
    let mut state = reduce(AppState::default(), Action::PlaybackUpdated(pv));

    for _ in 0..steps {
        let pv = controller
            .playback_step(&mut writer, &mut reader, &session_id, 1)
            .unwrap_or_else(|e| { eprintln!("ipc error: {}", e); std::process::exit(3); });
        let done = pv.done;
        state = reduce(state, Action::PlaybackUpdated(pv));
        if done { break; }
    }

    println!("{}", PlaybackScreen::render(&state));
}

fn cmd_analytics(args: &[String]) {
    let mut profile: Option<String> = None;
    let mut json_output = false;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--profile" => { i += 1; profile = args.get(i).cloned(); }
            "--json" => { json_output = true; }
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

    let av = controller
        .analytics_report(&mut writer, &mut reader, &profile)
        .unwrap_or_else(|e| { eprintln!("ipc error: {}", e); std::process::exit(3); });

    let state = reduce(AppState::default(), Action::AnalyticsLoaded(av.clone()));

    if json_output {
        let json = common_json::to_string(&av).unwrap_or_else(|e| {
            eprintln!("json error: {}", e);
            std::process::exit(5);
        });
        println!("{}", json);
    } else {
        println!("{}", AnalyticsScreen::render(&state));
    }
}
