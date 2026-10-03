#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]

use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

pub use js_protocol_macros::{CdpBuilder, CdpCommand, CdpEvent};

/// Trait for CDP commands that associate parameters with a method name and response type.
pub trait CdpCommand<'a>: Serialize {
    const METHOD: &'static str;
    type Response: Deserialize<'a>;
}

/// Marker trait implemented by every typed CDP event.
pub trait CdpEvent {
    const METHOD: &'static str;
}

/// A generic CDP command envelope.
#[derive(Serialize)]
pub struct Command<'a, T: CdpCommand<'a>> {
    pub id: u64,
    pub method: &'static str,
    pub params: &'a T,
}

impl<'a, T: CdpCommand<'a>> Command<'a, T> {
    pub fn new(id: u64, params: &'a T) -> Self {
        Self { id, method: T::METHOD, params }
    }
}

/// A generic CDP response envelope.
#[derive(Deserialize, Debug)]
pub struct Response<T> {
    pub id: u64,
    pub result: T,
}

/// An empty response for commands that don't return anything.
#[derive(Deserialize, Debug, Clone, Default)]
pub struct EmptyReturns {}

/// A protocol-level error returned by the browser.
#[derive(Deserialize, Debug, Clone)]
pub struct CdpError {
    pub code: i64,
    pub message: String,
    pub data: Option<JsonValue>,
}

/// An error reply envelope: `{"id": N, "error": { ... }}`.
#[derive(Deserialize, Debug, Clone)]
pub struct ErrorResponse {
    pub id: u64,
    pub error: CdpError,
}

/// A reply that is either a typed result or a protocol error.
#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum CdpReply<T> {
    Ok(Response<T>),
    Err(ErrorResponse),
}

#[cfg(feature = "console")]
pub mod console;
#[cfg(feature = "debugger")]
pub mod debugger;
#[cfg(feature = "heapprofiler")]
pub mod heapprofiler;
#[cfg(feature = "profiler")]
pub mod profiler;
#[cfg(feature = "runtime")]
pub mod runtime;
#[cfg(feature = "schema")]
pub mod schema;