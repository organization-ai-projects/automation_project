use super::request::Request;
use super::response::Response;
use protocol::{Command, CommandType, Event, EventType, Metadata, Payload};

const ACTION_HEALTH: &str = "diplo_sim.health";
const ACTION_LIST_MAPS: &str = "diplo_sim.list_maps";
const ACTION_GET_MAP_INFO: &str = "diplo_sim.get_map_info";
const ACTION_RUN_MATCH: &str = "diplo_sim.run_match";
const ACTION_GET_RUN_STATUS: &str = "diplo_sim.get_run_status";
const ACTION_REPLAY_MATCH: &str = "diplo_sim.replay_match";

pub fn request_to_command(request: &Request) -> Result<Command, String> {
    match request {
        Request::Health => Ok(make_command(
            CommandType::Query,
            ACTION_HEALTH,
            common_json::pjson!({}),
        )),
        Request::ListMaps => Ok(make_command(
            CommandType::Query,
            ACTION_LIST_MAPS,
            common_json::pjson!({}),
        )),
        Request::GetMapInfo { map_id } => Ok(make_command(
            CommandType::Query,
            ACTION_GET_MAP_INFO,
            common_json::pjson!({ "map_id": map_id.clone() }),
        )),
        Request::RunMatch {
            map_id,
            turns,
            seed,
            players,
        } => Ok(make_command(
            CommandType::Execute,
            ACTION_RUN_MATCH,
            common_json::pjson!({
                "map_id": map_id,
                "turns": *turns,
                "seed": *seed,
                "players": *players
            }),
        )),
        Request::GetRunStatus { run_id } => Ok(make_command(
            CommandType::Query,
            ACTION_GET_RUN_STATUS,
            common_json::pjson!({ "run_id": *run_id }),
        )),
        Request::ReplayMatch { run_id } => Ok(make_command(
            CommandType::Execute,
            ACTION_REPLAY_MATCH,
            common_json::pjson!({ "run_id": *run_id }),
        )),
    }
}

pub fn command_to_request(command: &Command) -> Result<Request, String> {
    let action = command
        .action
        .as_deref()
        .ok_or_else(|| "missing protocol action".to_string())?;
    let payload = command
        .payload
        .as_ref()
        .and_then(|p| p.payload.as_ref())
        .ok_or_else(|| "missing protocol payload".to_string())?;

    match action {
        ACTION_HEALTH => Ok(Request::Health),
        ACTION_LIST_MAPS => Ok(Request::ListMaps),
        ACTION_GET_MAP_INFO => {
            #[derive(serde::Deserialize)]
            struct Body {
                map_id: String,
            }
            let body: Body = common_json::from_json(payload)
                .map_err(|e| format!("invalid map info payload: {e}"))?;
            Ok(Request::GetMapInfo {
                map_id: body.map_id,
            })
        }
        ACTION_RUN_MATCH => {
            #[derive(serde::Deserialize)]
            struct Body {
                map_id: String,
                turns: u32,
                seed: u64,
                players: u32,
            }
            let body: Body =
                common_json::from_json(payload).map_err(|e| format!("invalid run payload: {e}"))?;
            Ok(Request::RunMatch {
                map_id: body.map_id,
                turns: body.turns,
                seed: body.seed,
                players: body.players,
            })
        }
        ACTION_GET_RUN_STATUS => {
            #[derive(serde::Deserialize)]
            struct Body {
                run_id: u64,
            }
            let body: Body = common_json::from_json(payload)
                .map_err(|e| format!("invalid run status payload: {e}"))?;
            Ok(Request::GetRunStatus {
                run_id: body.run_id,
            })
        }
        ACTION_REPLAY_MATCH => {
            #[derive(serde::Deserialize)]
            struct Body {
                run_id: u64,
            }
            let body: Body = common_json::from_json(payload)
                .map_err(|e| format!("invalid replay payload: {e}"))?;
            Ok(Request::ReplayMatch {
                run_id: body.run_id,
            })
        }
        _ => Err(format!("unsupported protocol action '{action}'")),
    }
}

pub fn response_to_event(response: &Response) -> Event {
    let (event_type, payload) = match response {
        Response::Ok => (
            EventType::Acknowledgment,
            common_json::pjson!({ "kind": "ok" }),
        ),
        Response::Maps { map_ids } => (
            EventType::Payload,
            common_json::pjson!({ "kind": "maps", "map_ids": map_ids.clone() }),
        ),
        Response::MapInfo {
            map_id,
            territory_count,
            adjacency_count,
            starting_unit_count,
        } => (
            EventType::Payload,
            common_json::pjson!({
                "kind": "map_info",
                "map_id": map_id.clone(),
                "territory_count": *territory_count,
                "adjacency_count": *adjacency_count,
                "starting_unit_count": *starting_unit_count
            }),
        ),
        Response::MatchRun { run_id } => (
            EventType::Created,
            common_json::pjson!({ "kind": "match_run", "run_id": *run_id }),
        ),
        Response::RunStatus { run_id, replayed } => (
            EventType::StateChanged,
            common_json::pjson!({ "kind": "run_status", "run_id": *run_id, "replayed": *replayed }),
        ),
        Response::ReplayReady { run_id } => (
            EventType::Completed,
            common_json::pjson!({ "kind": "replay_ready", "run_id": *run_id }),
        ),
        Response::Error(message) => (
            EventType::Error,
            common_json::pjson!({ "kind": "error", "message": message.clone() }),
        ),
    };

    Event::with_payload(
        "diplo_sim.transport".to_string(),
        event_type,
        Metadata::now(),
        Payload {
            payload_type: Some("diplo_sim/response".to_string()),
            payload: Some(payload),
        },
    )
}

pub fn event_to_response(event: &Event) -> Result<Response, String> {
    let payload = event
        .payload
        .as_ref()
        .and_then(|p| p.payload.as_ref())
        .ok_or_else(|| "missing protocol event payload".to_string())?;

    #[derive(serde::Deserialize)]
    struct Envelope {
        kind: String,
        map_ids: Option<Vec<String>>,
        map_id: Option<String>,
        territory_count: Option<u32>,
        adjacency_count: Option<u32>,
        starting_unit_count: Option<u32>,
        run_id: Option<u64>,
        replayed: Option<bool>,
        message: Option<String>,
    }

    let envelope: Envelope =
        common_json::from_json(payload).map_err(|e| format!("invalid response payload: {e}"))?;
    match envelope.kind.as_str() {
        "ok" => Ok(Response::Ok),
        "maps" => Ok(Response::Maps {
            map_ids: envelope.map_ids.unwrap_or_default(),
        }),
        "map_info" => Ok(Response::MapInfo {
            map_id: envelope.map_id.unwrap_or_default(),
            territory_count: envelope.territory_count.unwrap_or_default(),
            adjacency_count: envelope.adjacency_count.unwrap_or_default(),
            starting_unit_count: envelope.starting_unit_count.unwrap_or_default(),
        }),
        "match_run" => Ok(Response::MatchRun {
            run_id: envelope.run_id.unwrap_or_default(),
        }),
        "run_status" => Ok(Response::RunStatus {
            run_id: envelope.run_id.unwrap_or_default(),
            replayed: envelope.replayed.unwrap_or(false),
        }),
        "replay_ready" => Ok(Response::ReplayReady {
            run_id: envelope.run_id.unwrap_or_default(),
        }),
        "error" => {
            Ok(Response::Error(envelope.message.unwrap_or_else(|| {
                "transport error without message".to_string()
            })))
        }
        other => Err(format!("unknown response kind '{other}'")),
    }
}

fn make_command(command_type: CommandType, action: &str, payload: common_json::Json) -> Command {
    Command {
        metadata: Metadata::now(),
        command_type,
        action: Some(action.to_string()),
        payload: Some(Payload {
            payload_type: Some("diplo_sim/request".to_string()),
            payload: Some(payload),
        }),
    }
}
