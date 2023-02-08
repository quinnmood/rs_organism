pub mod connector;
pub mod placement;
pub mod recognizer;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{fs, io, cell};
use thiserror;

#[derive(thiserror::Error, Debug)]
pub enum OrganismError {
    #[error("failed to load organism")]
    LoadOrganismError,
    #[error("failed to parse organism or config from json file")]
    ParseJSONError(#[from] serde_json::Error),
    #[error("failed to open organism or config file")]
    IOError(#[from] std::io::Error),
    #[error("failed to parse recognizer object")]
    RecognizerError(#[from] recognizer::RecognizerError),
    #[error("failed to parse connector object")]
    ConnectorError(#[from] connector::ConnectorError),
}


#[derive(Clone, Debug, Default)]
pub struct Organism {
    recognizers: Vec<cell::RefCell<recognizer::Recognizer>>,
    connectors: Vec<cell::RefCell<connector::Connector>>,
    id: Option<usize>,
    config: Option<OrganismConfig>,
}

impl Organism {
    pub fn recs(&self) -> &Vec<cell::RefCell<recognizer::Recognizer>> {
        &self.recognizers
    }

    pub fn cons(&self) -> &Vec<cell::RefCell<connector::Connector>> {
        &self.connectors
    }

    pub fn id(&self) -> usize {
        self.id.expect("organism does not have an id")
    }

    pub fn config(&self) -> &OrganismConfig {
        self.config.as_ref().expect("organism does not have a config")
    }

    pub fn len(&self) -> usize {
        self.recognizers.len() + self.connectors.len()        
    }

    pub fn num_recs(&self) -> usize {
        self.recognizers.len()
    }

    pub fn num_cons(&self) -> usize {
        self.connectors.len()
    }

    pub fn len_recs(&self) -> usize {
        let mut sum: usize = 0;
        for i in 0..self.recognizers.len() {
            sum += &self.recognizers[i].borrow().len()
        }
        sum
    }

    pub fn rec_at(&self, index: usize) -> &cell::RefCell<recognizer::Recognizer> {
        &self.recognizers[index]
    }

    pub fn rec_at_mut(&mut self, index: usize) -> &mut cell::RefCell<recognizer::Recognizer> {
        &mut self.recognizers[index]
    }

    pub fn con_at(&self, index: usize) -> &cell::RefCell<connector::Connector> {
        &self.connectors[index]
    }

    pub fn con_at_mut(&mut self, index: usize) -> &mut cell::RefCell<connector::Connector> {
        &mut self.connectors[index]
    }

    pub fn swap_rec(&mut self, rec_a: usize, rec_b: usize) {
        self.recognizers.swap(rec_a, rec_b);
    }

    pub fn swap_con(&mut self, con_a: usize, con_b: usize) {
        self.recognizers.swap(con_a, con_b);
    }

    pub fn print(&self) {
        let len = self.num_recs();
        for i in 0..4 {
            for j in 0..len {
                let rec = self.recognizers[j].borrow();
                let rec_len = rec.len();
                let matrix = rec.matrix();
                for k in 0..rec_len {
                    print!("|{:1.2}", matrix[k * 4 + i])
                }

                if i == 0  && j < len - 1{
                    let con = self.connectors[j].borrow();
                    let mu = con.mu();
                    let sigma = con.sigma();
                    print!("|--<{:2.2}, {:2.2}>--", mu, sigma);
                }else{
                    print!("|                 ");
                }
            }
            println!("");       
        }
    }

}

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

pub fn from_value(
    org: &Value,
    org_conf: Option<&OrganismConfig>,
    rec_conf: Option<&recognizer::RecognizerConfig>,
    con_conf: Option<&connector::ConnectorConfig>,
) -> Result<Organism, OrganismError> {

    let nodes = org.as_array().unwrap();
    let num_nodes = nodes.len();
    let mut recognizers: Vec<cell::RefCell<recognizer::Recognizer>> = Vec::new();
    let mut connectors: Vec<cell::RefCell<connector::Connector>> = Vec::new();

    for i in 0..num_nodes {
        match nodes[i].as_object().unwrap()["objectType"]
            .as_str()
            .unwrap()
        {
            "pssm" => recognizers.push(cell::RefCell::new(recognizer::from_value(&nodes[i], rec_conf)?)),
            "connector" => connectors.push(cell::RefCell::new(connector::from_value(&nodes[i], con_conf)?)),
            "shape" => break,
            _ => break,
        }
    }

    Ok(organism(recognizers, connectors, None, org_conf.cloned()))
}

pub fn organism(
    recognizers: Vec<cell::RefCell<recognizer::Recognizer>>,
    connectors: Vec<cell::RefCell<connector::Connector>>,
    id: Option<usize>,
    config: Option<OrganismConfig>,
) -> Organism {

    Organism {
        recognizers,
        connectors,
        id,
        config,
    }
}

pub fn from_json(
    org_file: &str,
    org_num: usize,
    conf_file: Option<&str>,
) -> Result<Organism, OrganismError> {

    let org_file = fs::File::open(org_file)?;
    let org_reader = io::BufReader::new(org_file);
    let org_value: Value = serde_json::from_reader(org_reader)?;

    match conf_file.is_some() {
        true => {
            let conf_file = fs::File::open(conf_file.ok_or_else(||
                OrganismError::IOError(io::Error::new(io::ErrorKind::NotFound, "oh no"))
            )?)?;
            let conf_reader = io::BufReader::new(conf_file);
            let conf_value: Value = serde_json::from_reader(conf_reader)?;
            let org_conf: OrganismConfig = serde_json::from_value(conf_value["organism"].clone())?;
            let rec_conf: recognizer::RecognizerConfig =
                serde_json::from_value(conf_value["recognizer"].clone())?;
            let con_conf: connector::ConnectorConfig =
                serde_json::from_value(conf_value["connector"].clone())?;
            Ok(from_value(
                &org_value[org_num],
                Some(&org_conf),
                Some(&rec_conf),
                Some(&con_conf),
            )?)
        }
        false => Ok(from_value(&org_value[org_num], None, None, None)?),
    }
}

pub fn from_json_list(
    org_file: &str,
    conf_file: Option<&str>,
) -> Result<Vec<Organism>, OrganismError> {

    let org_file = fs::File::open(org_file)?;
    let org_reader = io::BufReader::new(org_file);
    let org_value: Value = serde_json::from_reader(org_reader)?;
    let num_orgs: usize = org_value
        .as_array()
        .ok_or_else(|| {
            OrganismError::ParseJSONError(serde::de::Error::invalid_type(
                serde::de::Unexpected::Option,
                &"hi",
            ))
        })?
        .len();
    let mut orgs: Vec<Organism> = Vec::with_capacity(num_orgs);
    match conf_file.is_some() {
        true => {
            let conf_file = fs::File::open(conf_file.ok_or_else(|| {
                OrganismError::IOError(io::Error::new(io::ErrorKind::NotFound, "oh no"))
            })?)?;
            let conf_reader = io::BufReader::new(conf_file);
            let conf_value: Value = serde_json::from_reader(conf_reader)?;
            let org_conf: OrganismConfig = serde_json::from_value(conf_value["organism"].clone())?;
            let rec_conf: recognizer::RecognizerConfig =
                serde_json::from_value(conf_value["recognizer"].clone())?;
            let con_conf: connector::ConnectorConfig =
                serde_json::from_value(conf_value["connector"].clone())?;
            for i in 0..num_orgs {
                orgs.push(from_value(
                    &org_value[i],
                    Some(&org_conf),
                    Some(&rec_conf),
                    Some(&con_conf),
                )?);
            }
        }
        false => {
            for i in 0..num_orgs {
                orgs.push(from_value(&org_value[i], None, None, None)?);
            }
        }
    }
    Ok(orgs)
}

