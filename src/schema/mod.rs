//! This domain is deprecated.

use serde::{Serialize, Deserialize};

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
