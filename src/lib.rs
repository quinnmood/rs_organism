pub mod recognizer;
pub mod connector;
use serde_json::Value;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufReaderError};

#[derive(Clone)]
pub struct Organism {
    pub recognizers: Vec<recognizer::Recognizer>,
    pub connectors: Vec<connector::Connector>,
    pub config: OrganismConfig,
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

pub fn import_org_from_value(org: Value, config: Option<(Value, Value, Value)>) -> Organism  {
    let num_nodes = org.as_array().expect("Organism is not an array of nodes").len();

    let mut curr_node_num: usize = 0;
    let mut curr_col: usize = 0;
    let mut curr_base: usize = 0;
    let with_config: bool = config.is_some();

    let org_conf: OrganismConfig = 
        if with_config {
            serde_json::from_value(config.clone().unwrap().0).unwrap()
        }else{ 
            Default::default()
        };

    let rec_conf: recognizer::PssmConfig = 
        if with_config {
            serde_json::from_value(config.clone().unwrap().1).unwrap()
        }else{
            Default::default()
        };

    let con_conf: connector::ConnectorConfig = 
        if with_config {
            serde_json::from_value(config.clone().unwrap().2).unwrap()
        }else{
            Default::default()
        };

    let mut scores: Vec<f64> = 
        if with_config {
            vec![0.0; rec_conf.max_columns.unwrap() * 4]
        }else{
            Vec::new()
        };

    let mut recs: Vec<recognizer::Recognizer> = 
        if with_config {
            Vec::with_capacity(org_conf.max_nodes.unwrap() / 2 + 1)
        }else{
            Vec::new()
        };

    let mut conns: Vec<connector::Connector> = 
        if with_config {
            Vec::with_capacity(org_conf.max_nodes.unwrap() / 2)
        }else{
            Vec::new()
        };


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
                            if with_config{
                                scores[curr_col * 4 + curr_base] = r[base].as_f64().unwrap();
                            }else{
                                scores.push(r[base].as_f64().unwrap());
                            }
                            curr_base += 1;
                        }

                        curr_base = 0;
                        curr_col += 1;
                    }

                    curr_col = 0;
                    if with_config {
                        recs.push(recognizer::build_rec(scores.to_owned(), 'p', r.len(), Some(rec_conf.to_owned()))); 
                    }else{
                        recs.push(recognizer::build_rec(scores.to_owned(), 'p', r.len(), None)); 
                    }
                },

            "connector" => {
                    if with_config {
                        conns.push(connector::build_conn(curr_node["mu"].as_f64().unwrap(), curr_node["sigma"].as_f64().unwrap(), Some(con_conf.to_owned())));
                    }else{
                        conns.push(connector::build_conn(curr_node["mu"].as_f64().unwrap(), curr_node["sigma"].as_f64().unwrap(), None));
                    }
                },

            _ => println!("hi"),
        }
        if !with_config{
            scores.clear();
        }
        curr_node_num += 1;
    }
     
    build_org(recs, conns)
}

pub fn import_org_from_json(org_file: &str, org_num: usize, conf_file: Option<&str>) -> Organism{
     
    let conf = if conf_file.is_some() {
        let conf_file = File::open(conf_file.unwrap()).unwrap();
        let conf_reader = BufReader::new(conf_file);
        let conf_value: Value = serde_json::from_reader(conf_reader).unwrap();
        Some((conf_value["organism"].to_owned(), conf_value["pssm"].to_owned(), conf_value["connector"].to_owned()))
    } else {
        None
    };
    let org_file = File::open(org_file).unwrap();
    let org_reader = BufReader::new(org_file);
    let org_value: Value = serde_json::from_reader(org_reader).unwrap(); 
    import_org_from_value(org_value[org_num].to_owned(), conf)
}
