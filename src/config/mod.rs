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


#[derive(Deserialize, Serialize, Debug, Default, Clone)]
#[serde(tag = "recognizer")]
#[serde(rename_all = "UPPERCASE")]
pub struct RecognizerConfig {
    mutate_probability_random_col: f64,
    mutate_probability_mutate_col: f64,
    mutate_probability_flip_col: f64,
    mutate_probability_flip_row: f64,
    mutate_probability_shift_left: f64,
    mutate_probability_shift_right: f64,
    mutate_probability_increase_pwm: f64,
    mutate_probability_decrease_pwm: f64,
    min_columns: usize,
    max_columns: usize,
    upper_print_probability: f64,
    pseudo_count: f64,
    scan_reverse_complement: bool,
}

impl RecognizerConfig {
    pub fn mutate_probability_random_col(&self) -> f64 {
        self.mutate_probability_random_col
    }

    pub fn mutate_probability_mutate_col(&self) -> f64 {
        self.mutate_probability_mutate_col
    }

    pub fn mutate_probability_flip_col(&self) -> f64 {
        self.mutate_probability_flip_col
    }

    pub fn mutate_probability_flip_row(&self) -> f64 {
        self.mutate_probability_flip_row
    }

    pub fn mutate_probability_shift_left(&self) -> f64 {
        self.mutate_probability_shift_left
    }

    pub fn mutate_probability_shift_right(&self) -> f64 {
        self.mutate_probability_shift_right
    }

    pub fn mutate_probability_increase_pwm(&self) -> f64 {
        self.mutate_probability_increase_pwm
    }

    pub fn mutate_probability_decrease_pwm(&self) -> f64 {
        self.mutate_probability_decrease_pwm
    }

    pub fn min_columns(&self) -> usize {
        self.min_columns
    }

    pub fn max_columns(&self) -> usize {
        self.max_columns
    }

    pub fn upper_print_probability(&self) -> f64 {
        self.upper_print_probability
    }

    pub fn pseudo_count(&self) -> f64 {
        self.pseudo_count
    }

    pub fn scan_reverse_complement(&self) -> bool {
        self.scan_reverse_complement
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "connector")]
#[serde(rename_all = "UPPERCASE")]
pub struct ConnectorConfig {
    mutate_probability_sigma: f64,
    mutate_probability_mu: f64,
    mutate_probability_swap: f64,
    mutate_variance_sigma: f64,
    mutate_variance_mu: f64,
    sigma_mutator: String,
    mu_mutator: String,
    expected_seq_length: usize,
}

impl ConnectorConfig {
    pub fn mutate_probability_sigma(&self) -> f64 {
        self.mutate_probability_sigma
    }
    pub fn mutate_probability_mu(&self) -> f64 {
        self.mutate_probability_mu
    }
    pub fn mutate_probability_swap(&self) -> f64 {
        self.mutate_probability_swap
    }
    pub fn mutate_variance_sigma(&self) -> f64 {
        self.mutate_variance_sigma
    }
    pub fn mutate_variance_mu(&self) -> f64 {
        self.mutate_variance_mu
    }
    pub fn sigma_mutator(&self) -> &str {
        &self.sigma_mutator
    }
    pub fn mu_mutator(&self) -> &str {
        &self.mu_mutator
    }
    pub fn expected_seq_length(&self) -> usize {
        self.expected_seq_length
    }
}
