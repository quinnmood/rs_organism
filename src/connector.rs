use crate::config::ConnectorConfig;
use crate::error::ConnectorError;
use statrs::distribution::Normal;
use serde_json::Value;

#[derive(Default, Debug, Clone)]
pub struct Connector {
    mu: f64,
    sigma: f64,
    alt: Normal,
    config: Option<ConnectorConfig>,
}

impl Connector {
    pub fn mu(&self) -> f64 {
        self.mu
    }

    pub fn sigma(&self) -> f64 {
        self.sigma
    }

    pub fn alt(&self) -> &Vec<f64> {
        self.alt.as_ref()
    }

    pub fn scores_mut(&mut self) -> &mut Vec<f64> {
        self.alt.as_mut()
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
        self.alt[index] = val;
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
        alt: Vec::new(),
        config,
    }
}
