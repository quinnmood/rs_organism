pub mod recognizer;
pub mod connector;
use serde_json::Value;
use serde::{Deserialize, Serialize};
use std::result;
use std::{fs::File, io::BufReader, error::Error};

#[derive(Clone)]
pub struct Organism {
    pub recognizers: Vec<recognizer::Recognizer>,
    pub connectors: Vec<connector::Connector>,
    pub config: OrganismConfig,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "organism")]
#[serde(rename_all = "UPPERCASE")]
pub struct OrganismConfig {
    pub cumulative_fit_method: Option<String>,
    pub energy_threshold_method: Option<String>,
    pub energy_threshold_param: Option<usize>,
    pub insertion_method: Option<String>,
    pub deletion_method: Option<String>,
    pub mutate_probability_node_mutation: Option<f64>,
    pub mutate_probability_delete_recognizer: Option<f64>,
    pub mutate_probability_insert_recognizer: Option<f64>,
    pub mutate_probability_substitute_pssm: Option<f64>,
    pub min_nodes: Option<usize>,
    pub max_nodes: Option<usize>,
    pub precompute: Option<bool>,
}

impl Organism {
    pub fn print(self){
        for rec in self.recognizers {
            rec.clone().print();        
        }
    }
}

impl Default for Organism {
    fn default() -> Organism {
        Organism { 
            recognizers: Default::default(), 
            connectors: Default::default(), 
            config: Default::default() 
        } 
    }
}

impl Default for OrganismConfig {
    fn default() -> OrganismConfig {
        OrganismConfig { 
            cumulative_fit_method: Default::default(), 
            energy_threshold_method: Default::default(),
            energy_threshold_param: Default::default(), 
            insertion_method: Default::default(),
            deletion_method: Default::default(),
            mutate_probability_node_mutation: Default::default(), 
            mutate_probability_delete_recognizer: Default::default(), 
            mutate_probability_insert_recognizer: Default::default(),
            mutate_probability_substitute_pssm: Default::default(),
            min_nodes: Default::default(), 
            max_nodes: Default::default(), 
            precompute: Default::default() 
        } 
    }
}

pub fn build_org(recognizers: Vec<recognizer::Recognizer>, connectors: Vec<connector::Connector>) -> Organism {
    Organism {
        recognizers,
        connectors,
        config: Default::default(),
    }
}

pub fn import_org_from_value(org: Value, config: (Value, Value, Value)) -> result::Result<Organism, Box<dyn Error>>  {
    let num_nodes = org.as_array().expect("Organism is not an array of nodes").len();

    let mut curr_node_num: usize = 0;
    let mut curr_col: usize = 0;
    let mut curr_base: usize = 0;
    let org_conf: OrganismConfig = serde_json::from_value(config.0).unwrap();
    let rec_conf: recognizer::PssmConfig = serde_json::from_value(config.1).unwrap();
    let con_conf: connector::ConnectorConfig = serde_json::from_value(config.2).unwrap();
    let mut scores: Vec<f64> = vec![0.0; rec_conf.max_columns.unwrap() * 4];
    let mut recs: Vec<recognizer::Recognizer> = Vec::with_capacity(org_conf.max_nodes.unwrap() / 2 + 1);
    let mut conns: Vec<connector::Connector> = Vec::with_capacity(org_conf.max_nodes.unwrap() / 2);


    while curr_node_num < num_nodes {
        let curr_node = &org.as_array()
                            .unwrap()
                            [curr_node_num]
                            .as_object()
                            .unwrap();

        match curr_node["objectType"]
                        .as_str()
                        .unwrap(){
            "pssm" => {
                    
                    let r = &curr_node["pwm"]
                                      .as_array()
                                      .unwrap();

                    while curr_col < r.len() {
                        let r = &r[curr_col]
                                  .as_object()
                                  .unwrap();

                        for base in ["a", "c", "g", "t"] {
                            scores[curr_col * 4 + curr_base] = r[base].as_f64().unwrap();
                            curr_base += 1;
                        }

                        curr_base = 0;
                        curr_col += 1;
                    }

                    curr_col = 0;
                    recs.push(recognizer::build_rec(scores.to_owned(), 'p', r.len(), Some(rec_conf.to_owned()))); 
                },

            "connector" => {
                    conns.push(connector::build_conn(curr_node["mu"].as_f64().unwrap(), curr_node["sigma"].as_f64().unwrap(), Some(con_conf.to_owned())));
                },

            _ => println!("hi"),
        }
        curr_node_num += 1;
    }
     
    let org: Organism = build_org(recs, conns);
    Ok(org)
}

pub fn import_org_from_json(org_file: &str, conf_file: &str, org_num: usize) -> result::Result<Organism, Box<dyn Error>>{
    let conf_file = File::open(conf_file).unwrap();
    let conf_reader = BufReader::new(conf_file);
    let conf_value: Value = serde_json::from_reader(conf_reader).unwrap();
    let org_file = File::open(org_file).unwrap();
    let org_reader = BufReader::new(org_file);
    let org_value: Value = serde_json::from_reader(org_reader).unwrap(); 
    Ok(import_org_from_value(org_value[org_num].to_owned(), (conf_value["organism"].to_owned(), conf_value["pssm"].to_owned(), conf_value["connector"].to_owned())).unwrap())
}
