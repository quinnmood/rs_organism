use serde_json::{Result, Value};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufReader, error::Error};

#[derive(Clone)]
pub struct Connector {
    pub mu: f64,
    pub sigma: f64,
    pub conn_scores: Vec<f64>,
    pub conf: ConnectorConfig,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "connector")]
#[serde(rename_all = "UPPERCASE")]
pub struct ConnectorConfig {
    pub mutate_probability_sigma: Option<f64>,
    pub mutate_probability_mu: Option<f64>,
    pub mutate_probability_swap: Option<f64>,
    pub mutate_variance_sigma: Option<f64>,
    pub mutate_variance_mu: Option<f64>,
    pub sigma_mutator: Option<String>,
    pub mu_mutator: Option<String>,
    pub expected_seq_length: Option<usize>,
}

impl Default for ConnectorConfig {
    fn default() -> ConnectorConfig {
        ConnectorConfig {
            mutate_probability_sigma: Default::default(),
            mutate_probability_mu: Default::default(),
            mutate_probability_swap: Default::default(),
            mutate_variance_sigma: Default::default(),
            mutate_variance_mu: Default::default(),
            sigma_mutator: Default::default(),
            mu_mutator: Default::default(),
            expected_seq_length: Default::default(),
        }
    }
}

pub fn build_conn(mu: f64, sigma: f64) -> Connector {
    Connector {
        mu,
        sigma,
        conn_scores: Vec::new(),
        conf: Default::default(),
    }
}


