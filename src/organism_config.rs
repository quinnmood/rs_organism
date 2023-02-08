use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
#[serde(tag = "organism")]
#[serde(rename_all = "UPPERCASE")]
pub struct OrganismConfig {
    cumulative_fit_method: String,
    energy_threshold_method: String,
    energy_threshold_param: usize,
    insertion_method: String,
    deletion_method: String,
    mutate_probability_node_mutation: f64,
    mutate_probability_delete_recognizer: f64,
    mutate_probability_insert_recognizer: f64,
    mutate_probability_substitute_pssm: f64,
    min_nodes: usize,
    max_nodes: usize,
    precompute: bool,
}

impl OrganismConfig {
    pub fn cumulative_fit_method(&self) -> &str {
        &self.cumulative_fit_method
    }
    pub fn energy_threshold_method(&self) -> &str {
        &self.energy_threshold_method
    }
    pub fn energy_threshold_param(&self) -> usize {
        self.clone().energy_threshold_param
    }
    pub fn insertion_method(&self) -> &str {
        &self.insertion_method
    }
    pub fn deletion_method(&self) -> &str {
        &self.deletion_method
    }
    pub fn mutate_probability_node_mutation(&self) -> f64 {
        self.mutate_probability_node_mutation
    }
    pub fn mutate_probability_delete_recognizer(&self) -> f64 {
        self.mutate_probability_delete_recognizer
    }
    pub fn mutate_probability_insert_recognizer(&self) -> f64 {
        self.mutate_probability_insert_recognizer
    }
    pub fn mutate_probability_substitute_pssm(&self) -> f64 {
        self.mutate_probability_substitute_pssm
    }
    pub fn min_nodes(&self) -> usize {
        self.min_nodes
    }
    pub fn max_nodes(&self) -> usize {
        self.max_nodes
    }
    pub fn precompute(&self) -> bool {
        self.precompute
    }
}