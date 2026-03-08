#[cfg(not(target_arch = "wasm32"))]
use crate::transport::protocol_codec;
#[cfg(not(target_arch = "wasm32"))]
use crate::transport::request::Request;
#[cfg(not(target_arch = "wasm32"))]
use crate::transport::response::Response;
#[cfg(not(target_arch = "wasm32"))]
use protocol::{Command, CommandType, Metadata, Payload};

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn request_round_trip_through_protocol_command_is_stable() {
    let original = Request::RunMatch {
        map_id: "tiny_triangle".to_string(),
        turns: 6,
        seed: 99,
        players: 2,
    };
    let command = protocol_codec::request_to_command(&original).expect("request->command");
    let decoded = protocol_codec::command_to_request(&command).expect("command->request");
    assert_eq!(decoded, original);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn response_round_trip_through_protocol_event_is_stable() {
    let original = Response::RunStatus {
        run_id: 7,
        replayed: true,
    };
    let event = protocol_codec::response_to_event(&original);
    let decoded = protocol_codec::event_to_response(&event).expect("event->response");
    assert_eq!(decoded, original);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn protocol_codec_rejects_unknown_action() {
    let command = Command {
        metadata: Metadata::now(),
        command_type: CommandType::Query,
        action: Some("diplo_sim.unknown".to_string()),
        payload: Some(Payload {
            payload_type: Some("diplo_sim/request".to_string()),
            payload: Some(common_json::pjson!({})),
        }),
    };

    let err = protocol_codec::command_to_request(&command).expect_err("unknown action");
    assert!(err.contains("unsupported protocol action"));
}

#[cfg(target_arch = "wasm32")]
#[test]
fn protocol_codec_related_types_are_available_on_wasm() {
    let response = crate::transport::response::Response::Ok;
    assert_eq!(response, crate::transport::response::Response::Ok);
}
