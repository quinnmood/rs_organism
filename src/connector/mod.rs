use serde::{Deserialize, Serialize};

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
    mutate_probability_sigma: Option<f64>,
    mutate_probability_mu: Option<f64>,
    mutate_probability_swap: Option<f64>,
    mutate_variance_sigma: Option<f64>,
    mutate_variance_mu: Option<f64>,
    sigma_mutator: Option<String>,
    mu_mutator: Option<String>,
    expected_seq_length: Option<usize>,
}

impl ConnectorConfig {
    pub fn mutate_probability_sigma(&self) -> f64 {self.mutate_probability_sigma.unwrap()}
    pub fn mutate_probability_mu(&self) -> f64 {self.mutate_probability_mu.unwrap()}
    pub fn mutate_probability_swap(&self) -> f64 {self.mutate_probability_swap.unwrap()}
    pub fn mutate_variance_sigma(&self) -> f64 {self.mutate_variance_sigma.unwrap()}
    pub fn mutate_variance_mu(&self) -> f64 {self.mutate_variance_mu.unwrap()}
    pub fn sigma_mutator(&self) -> String {self.clone().sigma_mutator.unwrap()}
    pub fn mu_mutator(&self) -> String {self.clone().mu_mutator.unwrap()}
    pub fn expected_seq_length(&self) -> usize {self.expected_seq_length.unwrap()}

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

pub fn build_conn(mu: f64, sigma: f64, conf: Option<ConnectorConfig>) -> Connector {
    Connector {
        mu,
        sigma,
        conn_scores: Vec::new(),
        conf: if conf.is_some() { conf.expect("Failed to set connector config") } else { Default::default() },
    }
}


