#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]

use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

/// Trait for CDP commands that associate parameters with a method name and response type.
pub trait CdpCommand: Serialize {
    const METHOD: &'static str;
    type Response: for<'de> Deserialize<'de>;
}

/// A generic CDP command envelope.
#[derive(Serialize)]
pub struct Command<'a, T: CdpCommand> {
    pub id: u64,
    pub method: &'static str,
    pub params: &'a T,
}

impl<'a, T: CdpCommand> Command<'a, T> {
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