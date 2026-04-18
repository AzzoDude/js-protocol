//! This domain is deprecated - use Runtime or Log instead.

use serde::{Serialize, Deserialize};

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
    pub line: Option<i32>,
    /// Column number in the resource that generated this message (1-based).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<i32>,
}
