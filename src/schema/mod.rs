//! This domain is deprecated.
use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

/// Description of the protocol domain.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Domain {
    /// Domain name.

    pub name: String,
    /// Domain version.

    pub version: String,
}

/// Returns supported domains.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetDomainsReturns {
    /// List of supported domains.

    pub domains: Vec<Domain>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetDomainsParams {}

impl GetDomainsParams { pub const METHOD: &'static str = "Schema.getDomains"; }

impl crate::CdpCommand for GetDomainsParams {
    const METHOD: &'static str = "Schema.getDomains";
    type Response = GetDomainsReturns;
}
