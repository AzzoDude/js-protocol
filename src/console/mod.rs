//! This domain is deprecated - use Runtime or Log instead.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;

/// Console message.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ConsoleMessage<'a> {
    /// Message source.
    source: Cow<'a, str>,
    /// Message severity.
    level: Cow<'a, str>,
    /// Message text.
    text: Cow<'a, str>,
    /// URL of the message origin.
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<Cow<'a, str>>,
    /// Line number in the resource that generated this message (1-based).
    #[serde(skip_serializing_if = "Option::is_none")]
    line: Option<i64>,
    /// Column number in the resource that generated this message (1-based).
    #[serde(skip_serializing_if = "Option::is_none")]
    column: Option<i64>,
}

impl<'a> ConsoleMessage<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `source`: Message source.
    /// * `level`: Message severity.
    /// * `text`: Message text.
    pub fn builder(source: impl Into<Cow<'a, str>>, level: impl Into<Cow<'a, str>>, text: impl Into<Cow<'a, str>>) -> ConsoleMessageBuilder<'a> {
        ConsoleMessageBuilder {
            source: source.into(),
            level: level.into(),
            text: text.into(),
            url: None,
            line: None,
            column: None,
        }
    }
    /// Message source.
    pub fn source(&self) -> &str { self.source.as_ref() }
    /// Message severity.
    pub fn level(&self) -> &str { self.level.as_ref() }
    /// Message text.
    pub fn text(&self) -> &str { self.text.as_ref() }
    /// URL of the message origin.
    pub fn url(&self) -> Option<&str> { self.url.as_deref() }
    /// Line number in the resource that generated this message (1-based).
    pub fn line(&self) -> Option<i64> { self.line }
    /// Column number in the resource that generated this message (1-based).
    pub fn column(&self) -> Option<i64> { self.column }
}


pub struct ConsoleMessageBuilder<'a> {
    source: Cow<'a, str>,
    level: Cow<'a, str>,
    text: Cow<'a, str>,
    url: Option<Cow<'a, str>>,
    line: Option<i64>,
    column: Option<i64>,
}

impl<'a> ConsoleMessageBuilder<'a> {
    /// URL of the message origin.
    pub fn url(mut self, url: impl Into<Cow<'a, str>>) -> Self { self.url = Some(url.into()); self }
    /// Line number in the resource that generated this message (1-based).
    pub fn line(mut self, line: i64) -> Self { self.line = Some(line); self }
    /// Column number in the resource that generated this message (1-based).
    pub fn column(mut self, column: i64) -> Self { self.column = Some(column); self }
    pub fn build(self) -> ConsoleMessage<'a> {
        ConsoleMessage {
            source: self.source,
            level: self.level,
            text: self.text,
            url: self.url,
            line: self.line,
            column: self.column,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClearMessagesParams {}

impl ClearMessagesParams { pub const METHOD: &'static str = "Console.clearMessages"; }

impl<'a> crate::CdpCommand<'a> for ClearMessagesParams {
    const METHOD: &'static str = "Console.clearMessages";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisableParams {}

impl DisableParams { pub const METHOD: &'static str = "Console.disable"; }

impl<'a> crate::CdpCommand<'a> for DisableParams {
    const METHOD: &'static str = "Console.disable";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnableParams {}

impl EnableParams { pub const METHOD: &'static str = "Console.enable"; }

impl<'a> crate::CdpCommand<'a> for EnableParams {
    const METHOD: &'static str = "Console.enable";
    type Response = crate::EmptyReturns;
}
