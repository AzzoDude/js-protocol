//! This domain is deprecated - use Runtime or Log instead.
use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

/// Console message.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ConsoleMessage {
    /// Message source.

    pub source: String,
    /// Message severity.

    pub level: String,
    /// Message text.

    pub text: String,
    /// URL of the message origin.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Line number in the resource that generated this message (1-based).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<i64>,
    /// Column number in the resource that generated this message (1-based).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClearMessagesParams {}

impl ClearMessagesParams { pub const METHOD: &'static str = "Console.clearMessages"; }

impl crate::CdpCommand for ClearMessagesParams {
    const METHOD: &'static str = "Console.clearMessages";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisableParams {}

impl DisableParams { pub const METHOD: &'static str = "Console.disable"; }

impl crate::CdpCommand for DisableParams {
    const METHOD: &'static str = "Console.disable";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnableParams {}

impl EnableParams { pub const METHOD: &'static str = "Console.enable"; }

impl crate::CdpCommand for EnableParams {
    const METHOD: &'static str = "Console.enable";
    type Response = crate::EmptyReturns;
}
