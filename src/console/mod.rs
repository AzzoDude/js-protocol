//! This domain is deprecated - use Runtime or Log instead.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use crate::{CdpBuilder, CdpCommand, CdpEvent};

/// Console message.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ConsoleMessage<'a> {
    /// Message source.
    pub source: Cow<'a, str>,
    /// Message severity.
    pub level: Cow<'a, str>,
    /// Message text.
    pub text: Cow<'a, str>,
    /// URL of the message origin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Cow<'a, str>>,
    /// Line number in the resource that generated this message (1-based).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<i64>,
    /// Column number in the resource that generated this message (1-based).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<i64>,
}
/// Does nothing.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Console.clearMessages")]
pub struct ClearMessagesParams {

}
/// Disables console domain, prevents further console messages from being reported to the client.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Console.disable")]
pub struct DisableParams {

}
/// Enables console domain, sends the messages collected so far to the client by means of the
/// 'messageAdded' notification.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Console.enable")]
pub struct EnableParams {

}
/// Issued when new console message is added.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Console.messageAdded")]
pub struct MessageAdded<'a> {
    /// Console message that has been added.
    pub message: ConsoleMessage<'a>,
}