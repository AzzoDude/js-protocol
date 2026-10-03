//! This domain is deprecated.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use crate::{CdpBuilder, CdpCommand, CdpEvent};

/// Description of the protocol domain.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Domain<'a> {
    /// Domain name.
    pub name: Cow<'a, str>,
    /// Domain version.
    pub version: Cow<'a, str>,
}
/// Returns supported domains.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Schema.getDomains", response = "GetDomainsReturns<'a>")]
pub struct GetDomainsParams {

}
/// Returns supported domains.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetDomainsReturns<'a> {
    /// List of supported domains.
    pub domains: Vec<Domain<'a>>,
}