use super::request::Request;
use super::response::Response;
use std::collections::BTreeMap;
use std::sync::Mutex;

type RunStorage = BTreeMap<u64, (String, u32, u64, u32)>;

#[derive(Debug, Default)]
pub struct IpcClient {
    state: Mutex<(u64, RunStorage)>,
}

impl IpcClient {
    pub fn new() -> Self {
        Self {
            state: Mutex::new((1, BTreeMap::new())),
        }
    }

    pub fn send(&self, request: Request) -> Response {
        let mut state = match self.state.lock() {
            Ok(guard) => guard,
            Err(_) => return Response::Error("ipc state lock poisoned".to_string()),
        };
        match request {
            Request::Health => Response::Ok,
            Request::RunMatch {
                map_id,
                turns,
                seed,
                players,
            } => {
                if map_id.trim().is_empty() {
                    return Response::Error("map_id must not be empty".to_string());
                }
                if turns == 0 {
                    return Response::Error("turns must be greater than zero".to_string());
                }
                if players == 0 {
                    return Response::Error("players must be greater than zero".to_string());
                }

                let run_id = state.0;
                state.0 += 1;
                state.1.insert(run_id, (map_id, turns, seed, players));
                Response::MatchRun { run_id }
            }
            Request::ReplayMatch { run_id } => match state.1.get(&run_id) {
                Some(_) => Response::ReplayReady { run_id },
                None => Response::Error(
                    "replay requires a prior run; no run has been executed yet".to_string(),
                ),
            },
        }
    }
}
