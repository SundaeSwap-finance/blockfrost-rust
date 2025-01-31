use std::{fmt, ops::FnOnce};

use crate::{CARDANO_MAINNET_NETWORK, CARDANO_TESTNET_NETWORK};

#[derive(Debug, Clone)]
pub struct Settings {
    pub(crate) network_endpoint: String,
    pub(crate) query_parameters: QueryParameters,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            network_endpoint: CARDANO_MAINNET_NETWORK.to_string(),
            query_parameters: QueryParameters::default(),
        }
    }

    pub fn use_mainnet(mut self) -> Self {
        self.network_endpoint = CARDANO_MAINNET_NETWORK.to_string();
        self
    }

    pub fn use_testnet(mut self) -> Self {
        self.network_endpoint = CARDANO_TESTNET_NETWORK.to_string();
        self
    }

    pub fn set_network(&mut self, network: impl AsRef<str>) {
        self.network_endpoint = network.as_ref().to_string();
    }

    pub fn current_network(&self) -> &str {
        &self.network_endpoint
    }

    pub fn query_parameters(&self) -> &QueryParameters {
        &self.query_parameters
    }

    pub fn configure<F>(mut self, function: F) -> Self
    where
        F: FnOnce(&mut QueryParameters),
    {
        function(&mut self.query_parameters);
        self
    }
}

#[derive(Debug, Clone, Default)]
pub struct QueryParameters {
    pub(crate) count: Option<u8>,
    pub(crate) page: Option<u64>,
    pub(crate) order: Option<QueryOrder>,
    pub(crate) from: Option<String>,
    pub(crate) to: Option<String>,
}

impl QueryParameters {
    pub fn set_count(&mut self, count: u8) -> &mut Self {
        self.count = Some(count);
        self
    }

    pub fn set_page(&mut self, page: u64) -> &mut Self {
        self.page = Some(page);
        self
    }

    pub fn set_order(&mut self, order: QueryOrder) -> &mut Self {
        self.order = Some(order);
        self
    }
    pub fn set_from(&mut self, from: String) -> &mut Self {
        self.from = Some(from);
        self
    }

    pub fn set_to(&mut self, to: String) -> &mut Self {
        self.to = Some(to);
        self
    }

    pub fn unset_count(&mut self) -> &mut Self {
        self.count = None;
        self
    }

    pub fn unset_page(&mut self) -> &mut Self {
        self.page = None;
        self
    }

    pub fn unset_order(&mut self) -> &mut Self {
        self.order = None;
        self
    }

    pub fn unset_from(&mut self) -> &mut Self {
        self.from = None;
        self
    }

    pub fn unset_to(&mut self) -> &mut Self {
        self.to = None;
        self
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub enum QueryOrder {
    Ascending,
    Descending,
}

impl Default for QueryOrder {
    fn default() -> Self {
        Self::Ascending
    }
}

impl fmt::Display for QueryOrder {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            QueryOrder::Ascending => write!(formatter, "asc"),
            QueryOrder::Descending => write!(formatter, "desc"),
        }
    }
}
