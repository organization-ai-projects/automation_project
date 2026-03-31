use crate::transport::request::{Message, Request};

#[test]
fn ping_message_roundtrip() {
    let msg = Message {
        id: 1,
        request: Request::Ping,
    };
    let json = common_json::to_string(&msg).unwrap();
    let decoded: Message = common_json::from_str(&json).unwrap();
    assert_eq!(decoded.id, 1);
}
