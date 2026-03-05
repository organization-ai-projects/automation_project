use crate::application::simulation_artifacts::SimulationArtifacts;
use crate::application::snapshot_output::SnapshotOutput;
use crate::application::snapshot_state_dto::SnapshotStateDto;
use crate::buildings;
use crate::citizens;
use crate::config::sim_config::SimConfig;
use crate::diagnostics::city_builder_error::CityBuilderError;
use crate::economy;
use crate::events;
use crate::io::json_codec::JsonCodec;
use crate::map;
use crate::replay;
use crate::replay::replay_checkpoint::ReplayCheckpoint;
use crate::replay::replay_file::ReplayFile;
use crate::report::run_hash::RunHash;
use crate::report::sim_report::SimReport;
use crate::report::tick_report::TickReport;
use crate::scenarios;
use crate::scenarios::scenario::Scenario;
use crate::scenarios::scripted_action::ScriptedAction;
use crate::services;
use crate::snapshot;
use crate::time::tick_clock::TickClock;
use crate::traffic;
use crate::zoning;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

pub fn validate_cli(scenario_path: &Path) -> Result<(), CityBuilderError> {
    let scenario = scenarios::scenario_loader::ScenarioLoader::load(scenario_path)?;
    scenarios::scenario_loader::ScenarioLoader::validate(&scenario)
}

pub fn run_cli(
    ticks: u64,
    seed: u64,
    scenario_path: &Path,
    out_path: &Path,
    replay_out: Option<&Path>,
) -> Result<(), CityBuilderError> {
    let scenario = scenarios::scenario_loader::ScenarioLoader::load(scenario_path)?;
    let config = SimConfig {
        grid_width: scenario.grid_width,
        grid_height: scenario.grid_height,
        seed,
        total_ticks: ticks,
    };

    let artifacts = simulate(&scenario, &config, ticks)?;
    JsonCodec::write(&artifacts.report, out_path)?;

    if let Some(path) = replay_out {
        let replay = ReplayFile {
            scenario_path: scenario_path.display().to_string(),
            scenario_name: scenario.name,
            seed,
            total_ticks: ticks,
            expected_report: artifacts.report,
            checkpoints: artifacts.checkpoints,
        };
        replay::replay_codec::ReplayCodec::save(path, &replay)?;
    }

    Ok(())
}

pub fn replay_cli(replay_path: &Path, out_path: &Path) -> Result<(), CityBuilderError> {
    let replay_file = replay::replay_codec::ReplayCodec::load(replay_path)?;
    let scenario = scenarios::scenario_loader::ScenarioLoader::load(&PathBuf::from(
        &replay_file.scenario_path,
    ))?;
    let config = SimConfig {
        grid_width: scenario.grid_width,
        grid_height: scenario.grid_height,
        seed: replay_file.seed,
        total_ticks: replay_file.total_ticks,
    };
    let artifacts = simulate(&scenario, &config, replay_file.total_ticks)?;
    replay::replay_engine::ReplayEngine::verify_reports(
        &replay_file.expected_report,
        &artifacts.report,
    )?;
    JsonCodec::write(&artifacts.report, out_path)?;
    let written: SimReport = JsonCodec::read(out_path)?;
    replay::replay_engine::ReplayEngine::verify_reports(&replay_file.expected_report, &written)?;
    Ok(())
}

pub fn snapshot_cli(
    replay_path: &Path,
    at_tick: u64,
    out_path: &Path,
) -> Result<(), CityBuilderError> {
    let replay_file = replay::replay_codec::ReplayCodec::load(replay_path)?;
    if at_tick == 0 || at_tick > replay_file.total_ticks {
        return Err(CityBuilderError::InvalidConfig(format!(
            "--at-tick must be in 1..={}",
            replay_file.total_ticks
        )));
    }

    let scenario = scenarios::scenario_loader::ScenarioLoader::load(&PathBuf::from(
        &replay_file.scenario_path,
    ))?;
    let config = SimConfig {
        grid_width: scenario.grid_width,
        grid_height: scenario.grid_height,
        seed: replay_file.seed,
        total_ticks: replay_file.total_ticks,
    };

    let artifacts = simulate(&scenario, &config, at_tick)?;
    let hash = snapshot::snapshot_hash::SnapshotHash::compute(&artifacts.final_state);
    let output = SnapshotOutput {
        tick: at_tick,
        snapshot_hash: hash.value,
        state: SnapshotStateDto::from_state(&artifacts.final_state),
    };
    JsonCodec::write(&output, out_path)?;
    Ok(())
}

fn simulate(
    scenario: &Scenario,
    config: &SimConfig,
    total_ticks: u64,
) -> Result<SimulationArtifacts, CityBuilderError> {
    if total_ticks > config.total_ticks {
        return Err(CityBuilderError::InvalidConfig(format!(
            "requested ticks {} exceed configured total_ticks {}",
            total_ticks, config.total_ticks
        )));
    }

    let mut state = snapshot::state_snapshot::StateSnapshot::from_scenario(scenario);
    let mut tick_reports: Vec<TickReport> = Vec::new();
    let mut checkpoints: Vec<ReplayCheckpoint> = Vec::new();
    let mut clock = TickClock::new();
    let mut event_log = events::event_log::EventLog::new();
    let mut budget = economy::budget::Budget::new();
    budget.balance = state.budget_balance;

    let mut actions = scenario.scripted_actions.clone();
    actions.sort_by_key(ScriptedAction::tick);
    let mut next_action = 0usize;

    let mut growth = buildings::growth_engine::GrowthEngine::new();
    let mut service = services::service_engine::ServiceEngine::new();
    let mut traffic = traffic::traffic_engine::TrafficEngine::new();
    let mut economy = economy::economy_engine::EconomyEngine::new();

    for t in 1..=total_ticks {
        clock.advance();
        let tick = clock.current;

        while next_action < actions.len() && actions[next_action].tick() == t {
            apply_scripted_action(&mut state, &actions[next_action])?;
            event_log.push(events::sim_event::SimEvent {
                tick,
                kind: "scripted_action".to_string(),
                data: format!("{:?}", actions[next_action]),
            });
            next_action += 1;
        }

        growth.tick(&mut state, tick, config);
        service.tick(&mut state);
        traffic.tick(&mut state);
        economy.tick(&mut state);

        budget.balance = state.budget_balance;

        let mut by_zone: BTreeMap<zoning::zone_kind::ZoneKind, (u64, i64)> = BTreeMap::new();
        for building in state.buildings.values_mut() {
            if building.population == 0 {
                continue;
            }
            let has_power = state
                .coverage
                .is_covered(services::service_kind::ServiceKind::Power, &building.tile);
            let has_water = state
                .coverage
                .is_covered(services::service_kind::ServiceKind::Water, &building.tile);
            let delta = if has_power && has_water { 1 } else { -1 };
            building.happiness = (building.happiness + delta).clamp(0, 100);

            let slot = by_zone.entry(building.zone).or_insert((0, 0));
            slot.0 += building.population;
            slot.1 += building.population as i64 * building.happiness as i64;
        }

        let mut groups = Vec::new();
        for (zone, (count, weighted_happiness)) in by_zone {
            let avg = if count == 0 {
                0
            } else {
                (weighted_happiness / count as i64) as i32
            };
            let happiness = citizens::happiness::Happiness::new(avg);
            groups.push(citizens::citizen_group::CitizenGroup {
                zone,
                count,
                happiness: happiness.value,
            });
        }

        let expected_demand: u64 = groups
            .iter()
            .map(|g| citizens::demand_model::DemandModel::demand(g.zone))
            .sum();
        if state.total_population() > expected_demand {
            event_log.push(events::sim_event::SimEvent {
                tick,
                kind: "demand_pressure".to_string(),
                data: format!(
                    "population={} demand={}",
                    state.total_population(),
                    expected_demand
                ),
            });
        }

        let hash = snapshot::snapshot_hash::SnapshotHash::compute(&state);
        let report = TickReport {
            tick: t,
            building_count: state.buildings.len() as u32,
            total_population: state.total_population(),
            budget_balance: budget.balance,
            snapshot_hash: hash.value.clone(),
        };
        tick_reports.push(report);
        checkpoints.push(ReplayCheckpoint {
            tick: t,
            snapshot_hash: hash.value.clone(),
        });

        for cp in &scenario.checkpoints {
            if cp.tick == t && cp.expected_hash != hash.value {
                return Err(CityBuilderError::ReplayMismatch(format!(
                    "Checkpoint at tick {t}: expected {}, got {}",
                    cp.expected_hash, hash.value
                )));
            }
        }
    }

    let run_hash = RunHash::compute(&scenario.name, config.seed, total_ticks, &tick_reports);
    let report = SimReport {
        scenario_name: scenario.name.clone(),
        seed: config.seed,
        total_ticks,
        tick_reports,
        run_hash: run_hash.value,
    };

    Ok(SimulationArtifacts {
        report,
        checkpoints,
        final_state: state,
    })
}

fn apply_scripted_action(
    state: &mut snapshot::state_snapshot::StateSnapshot,
    action: &ScriptedAction,
) -> Result<(), CityBuilderError> {
    match action {
        ScriptedAction::PlaceZone { x, y, kind, .. } => {
            let zone = zoning::zone::Zone {
                tile: map::tile_id::TileId { x: *x, y: *y },
                kind: *kind,
            };
            if let Some(tile) = state.grid.get_mut(&zone.tile) {
                tile.zone = zone.kind;
            } else {
                return Err(CityBuilderError::InvalidScenario(
                    "place_zone out of bounds".to_string(),
                ));
            }
        }
        ScriptedAction::PlaceRoad { x1, y1, x2, y2, .. } => {
            let road = map::road::Road {
                from: map::tile_id::TileId { x: *x1, y: *y1 },
                to: map::tile_id::TileId { x: *x2, y: *y2 },
            };
            if state.grid.get(&road.from).is_none() || state.grid.get(&road.to).is_none() {
                return Err(CityBuilderError::InvalidScenario(
                    "place_road out of bounds".to_string(),
                ));
            }
            state.road_graph.add_road(&road);
            if let Some(tile) = state.grid.get_mut(&road.from) {
                tile.has_road = true;
            }
            if let Some(tile) = state.grid.get_mut(&road.to) {
                tile.has_road = true;
            }
        }
        ScriptedAction::PlaceService {
            x,
            y,
            kind,
            coverage_radius,
            ..
        } => {
            let tile = map::tile_id::TileId { x: *x, y: *y };
            if state.grid.get(&tile).is_none() {
                return Err(CityBuilderError::InvalidScenario(
                    "place_service out of bounds".to_string(),
                ));
            }
            state.service_buildings.insert(
                tile,
                services::service_building::ServiceBuilding {
                    tile,
                    kind: *kind,
                    coverage_radius: *coverage_radius,
                },
            );
        }
        ScriptedAction::SetTax { percent, .. } => {
            state.budget_balance += *percent;
        }
    }
    Ok(())
}
