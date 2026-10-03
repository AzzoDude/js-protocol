//! Smoke tests for the patterns a production CDP client relies on:
//! generic command encoding, typed decoding, typed events, and error replies.

use js_protocol::runtime::{
    ConsoleAPICalled, EnableParams, EvaluateParams, ExecutionContextsCleared, GetIsolateIdParams,
};
use js_protocol::{CdpCommand, CdpEvent, CdpReply, Command, EmptyReturns, Response};
use serde::Serialize;
use serde::de::DeserializeOwned;

/// Generic request encoder: `CdpCommand` provides the wire method, `Serialize` the body.
fn encode<'a, P>(id: u64, params: &'a P) -> String
where
    P: CdpCommand<'a> + Serialize,
{
    serde_json::to_string(&Command::new(id, params)).unwrap()
}

/// Generic response decoder: correlates by id and yields the typed `Response`.
fn decode<'a, P>(json: &'a str) -> Response<P::Response>
where
    P: CdpCommand<'a>,
    P::Response: DeserializeOwned,
{
    serde_json::from_str(json).unwrap()
}

#[test]
fn command_encodes_to_wire_format() {
    let params = EvaluateParams::builder("1 + 1").silent(true).build();

    assert_eq!(
        encode(7, &params),
        r#"{"id":7,"method":"Runtime.evaluate","params":{"expression":"1 + 1","silent":true}}"#
    );
}

#[test]
fn response_decodes_to_typed_result() {
    let response = decode::<GetIsolateIdParams>(r#"{"id":7,"result":{"id":"isolate-1"}}"#);

    assert_eq!(response.id, 7);
    assert_eq!(response.result.id(), "isolate-1");
}

#[test]
fn empty_returns_round_trips() {
    let params = EnableParams::default();
    assert_eq!(
        encode(1, &params),
        r#"{"id":1,"method":"Runtime.enable","params":{}}"#
    );
}

#[test]
fn events_are_typed_and_carry_method() {
    assert_eq!(ConsoleAPICalled::METHOD, "Runtime.consoleAPICalled");
    assert_eq!(
        ExecutionContextsCleared::METHOD,
        "Runtime.executionContextsCleared"
    );

    // The `CdpEvent` trait makes events usable generically.
    fn method_of<E: CdpEvent>() -> &'static str {
        E::METHOD
    }
    assert_eq!(
        method_of::<ExecutionContextsCleared>(),
        "Runtime.executionContextsCleared"
    );

    let event: ExecutionContextsCleared = serde_json::from_str("{}").unwrap();
    let _ = event;
}

#[test]
fn reply_decodes_success_and_error() {
    let ok: CdpReply<EmptyReturns> = serde_json::from_str(r#"{"id":1,"result":{}}"#).unwrap();
    match ok {
        CdpReply::Ok(response) => assert_eq!(response.id, 1),
        CdpReply::Err(_) => panic!("expected Ok"),
    }

    let err: CdpReply<EmptyReturns> =
        serde_json::from_str(r#"{"id":2,"error":{"code":-32000,"message":"boom"}}"#).unwrap();
    match err {
        CdpReply::Err(response) => {
            assert_eq!(response.id, 2);
            assert_eq!(response.error.code, -32000);
            assert_eq!(response.error.message, "boom");
        }
        CdpReply::Ok(_) => panic!("expected Err"),
    }
}
