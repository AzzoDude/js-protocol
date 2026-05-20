//! This domain is deprecated.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;

/// Description of the protocol domain.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Domain<'a> {
    /// Domain name.
    name: Cow<'a, str>,
    /// Domain version.
    version: Cow<'a, str>,
}

impl<'a> Domain<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `name`: Domain name.
    /// * `version`: Domain version.
    pub fn builder(name: impl Into<Cow<'a, str>>, version: impl Into<Cow<'a, str>>) -> DomainBuilder<'a> {
        DomainBuilder {
            name: name.into(),
            version: version.into(),
        }
    }
    /// Domain name.
    pub fn name(&self) -> &str { self.name.as_ref() }
    /// Domain version.
    pub fn version(&self) -> &str { self.version.as_ref() }
}


pub struct DomainBuilder<'a> {
    name: Cow<'a, str>,
    version: Cow<'a, str>,
}

impl<'a> DomainBuilder<'a> {
    pub fn build(self) -> Domain<'a> {
        Domain {
            name: self.name,
            version: self.version,
        }
    }
}

/// Returns supported domains.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetDomainsReturns<'a> {
    /// List of supported domains.
    domains: Vec<Domain<'a>>,
}

impl<'a> GetDomainsReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `domains`: List of supported domains.
    pub fn builder(domains: Vec<Domain<'a>>) -> GetDomainsReturnsBuilder<'a> {
        GetDomainsReturnsBuilder {
            domains: domains,
        }
    }
    /// List of supported domains.
    pub fn domains(&self) -> &[Domain<'a>] { &self.domains }
}


pub struct GetDomainsReturnsBuilder<'a> {
    domains: Vec<Domain<'a>>,
}

impl<'a> GetDomainsReturnsBuilder<'a> {
    pub fn build(self) -> GetDomainsReturns<'a> {
        GetDomainsReturns {
            domains: self.domains,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetDomainsParams {}

impl GetDomainsParams { pub const METHOD: &'static str = "Schema.getDomains"; }

impl<'a> crate::CdpCommand<'a> for GetDomainsParams {
    const METHOD: &'static str = "Schema.getDomains";
    type Response = GetDomainsReturns<'a>;
}
