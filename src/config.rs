use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufReader, error::Error};

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    #[serde(rename = "main")]
    pub main_config: MainConfig,

    #[serde(rename = "organism")]
    pub organism_config: OrganismConfig,

    #[serde(rename = "organismFactory")]
    pub organism_factory_config: OrganismFactoryConfig,

    #[serde(rename = "connector")]
    pub connector_config: ConnectorConfig,

    #[serde(rename = "pssm")]
    pub pssm_config: PssmConfig,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "main")]
#[serde(rename_all = "UPPERCASE")]
pub struct MainConfig {
    pub run_mode: String,
    pub population_length: usize,
    pub population_origin: String,
    pub population_fill_type: String,
    pub dataset_base_path_dir: String,
    pub result_base_path_dir: String,
    pub result_test_base_path_dir: String,
    pub positive_filename: String,
    pub negative_filename: String,
    pub generated_neg_set_size: usize,
    pub generated_neg_set_kmer_len: usize,
    pub input_filename: String,
    pub output_filename: String,
    pub max_sequences_to_fit_pos: usize,
    pub max_sequences_to_fit_neg: usize,
    pub random_shuffle_sampling_pos: bool,
    pub random_shuffle_sampling_neg: bool,
    pub fitness_function: String,
    pub genome_length: usize,
    pub end_while_method: String,
    pub min_iterations: usize,
    pub min_fitness: usize,
    pub threshold: f64,
    pub periodic_org_export: usize,
    pub periodic_pop_export: usize,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "organism")]
#[serde(rename_all = "UPPERCASE")]
pub struct OrganismConfig {
    pub cumulative_fit_method: String,
    pub energy_threshold_method: String,
    pub energy_threshold_param: usize,
    pub insertion_method: String,
    pub deletion_method: String,
    pub mutate_probability_node_mutation: f64,
    pub mutate_probability_delete_recognizer: f64,
    pub mutate_probability_insert_recognizer: f64,
    pub mutate_probability_substitute_pssm: f64,
    pub min_nodes: usize,
    pub max_nodes: usize,
    pub precompute: bool,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "organismFactory")]
#[serde(rename_all = "UPPERCASE")]
pub struct OrganismFactoryConfig {
    pub recombination_probability: f64,
    pub num_recognizers_lambda_param: f64,
    pub min_mu: f64,
    pub max_mu: f64,
    pub min_sigma: f64,
    pub max_sigma: f64,
    pub pwm_length: usize,
    pub pwm_num_of_binding_sites: usize,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "connector")]
#[serde(rename_all = "UPPERCASE")]
pub struct ConnectorConfig {
    pub mutate_probability_sigma: f64,
    pub mutate_probability_mu: f64,
    pub mutate_probability_swap: f64,
    pub mutate_variance_sigma: f64,
    pub mutate_variance_mu: f64,
    pub sigma_mutator: String,
    pub mu_mutator: String,
    pub expected_seq_length: usize

}

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "pssm")]
#[serde(rename_all = "UPPERCASE")]
pub struct PssmConfig {
    pub mutate_probability_random_col: f64,
    pub mutate_probability_mutate_col: f64,
    pub mutate_probability_flip_col: f64,
    pub mutate_probability_flip_row: f64,
    pub mutate_probability_shift_left: f64,
    pub mutate_probability_shift_right: f64,
    pub mutate_probability_increase_pwm: f64,
    pub mutate_probability_decrease_pwm: f64,
    pub min_columns: usize,
    pub max_columns: usize,
    pub upper_print_probability: f64,
    pub pseudo_count: f64,
    pub scan_reverse_complement: bool,
}

pub fn load_config(filename: &str) -> Result<Config, Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let conf: Config = serde_json::from_reader(reader)?;
    Ok(conf)
}
