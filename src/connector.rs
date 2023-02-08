use crate::config::ConnectorConfig;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror;

#[derive(thiserror::Error, Debug)]
pub enum ConnectorError {
    #[error("failed to load connector")]
    LoadConnectorError,
    #[error("failed to parse connector from json file")]
    ParseJSONError(#[from] serde_json::Error),
}

#[derive(Default, Debug, Clone)]
pub struct Connector {
    mu: f64,
    sigma: f64,
    scores: Vec<f64>,
    config: Option<ConnectorConfig>,
}

impl Connector {
    pub fn mu(&self) -> f64 {
        self.mu
    }

    pub fn sigma(&self) -> f64 {
        self.sigma
    }

    pub fn scores(&self) -> &Vec<f64> {
        self.scores.as_ref()
    }

    pub fn scores_mut(&mut self) -> &mut Vec<f64> {
        self.scores.as_mut()
    }

    pub fn config(&self) -> &ConnectorConfig {
        self.config.as_ref().expect("connector does not have a config")
    }

    pub fn config_mut(&mut self) -> &mut ConnectorConfig {
        self.config.as_mut().expect("connector does not have a config")
    }
    
    pub fn set_mu(&mut self, mu: f64) {
        self.mu = mu;
    }

    pub fn set_sigma(&mut self, sigma: f64) {
        self.sigma = sigma;
    }

    pub fn set_at(&mut self, val: f64, index: usize) {
        self.scores[index] = val;
    }

}


pub fn from_value(con: &Value, config: Option<&ConnectorConfig>) -> Result<Connector, ConnectorError> {
    let con = con.as_object().ok_or_else(|| ConnectorError::ParseJSONError(serde::de::Error::invalid_type( serde::de::Unexpected::Option, &"hi")))?;
    let mu = con["mu"].as_f64().ok_or_else(|| ConnectorError::ParseJSONError(serde::de::Error::invalid_type( serde::de::Unexpected::Option, &"hi")))?;
    let sigma = con["sigma"].as_f64().ok_or_else(|| ConnectorError::ParseJSONError(serde::de::Error::invalid_type( serde::de::Unexpected::Option, &"hi")))?;
    Ok(connector(mu, sigma, config.cloned()))
}

pub fn connector(mu: f64, sigma: f64, config: Option<ConnectorConfig>) -> Connector {
    Connector {
        mu,
        sigma,
        scores: Vec::new(),
        config,
    }
}
