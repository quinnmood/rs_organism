pub mod connector;
pub mod placement;
pub mod recognizer;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{fs::File, io::BufReader};

#[derive(Clone, Debug, Default)]
pub struct Organism {
    pub recognizers: Vec<recognizer::Recognizer>,
    pub connectors: Vec<connector::Connector>,
    pub id: Option<usize>,
    pub config: Option<OrganismConfig>,
}

impl Organism {
    pub fn print(&self) {
        for rec in self.recognizers.clone() {
            rec.print();
        }
    }

    pub fn mutate(&self) {}

    pub fn place(&self, sequence: String) -> placement::Placement {
        let placemen: placement::Placement = Default::default();
        todo!()
    }
}

#[derive(Deserialize, Serialize, Defualt, Debug, Clone)]
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
    pub fn cumulative_fit_method(&self) -> String {
        self.clone().cumulative_fit_method
    }
    pub fn energy_threshold_method(&self) -> String {
        self.clone().energy_threshold_method
    }
    pub fn energy_threshold_param(&self) -> usize {
        self.clone().energy_threshold_param
    }
    pub fn insertion_method(&self) -> String {
        self.clone().insertion_method
    }
    pub fn deletion_method(&self) -> String {
        self.clone().deletion_method
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

pub fn build_org(
    recognizers: Vec<recognizer::Recognizer>,
    connectors: Vec<connector::Connector>,
    id: Option<u8>,
    config: Option<OrganismConfig>,
) -> Organism {
    Organism {
        recognizers,
        connectors,
        id: if id.is_some() {
            id.unwrap()
        } else {
            Default::default()
        },
        config: if config.is_some() {
            config.unwrap()
        } else {
            Default::default()
        },
    }
}

pub fn import_org_from_value(org: Value, config: Option<(Value, Value, Value)>) -> Organism {
    let num_nodes = org
        .as_array()
        .expect("Organism is not an array of nodes")
        .len();

    let mut curr_node_num: usize = 0;
    let mut curr_col: usize = 0;
    let mut curr_base: usize = 0;
    let with_config: bool = config.is_some();

    let org_conf: OrganismConfig = if with_config {
        serde_json::from_value(config.clone().unwrap().0).unwrap()
    } else {
        Default::default()
    };

    let rec_conf: recognizer::PssmConfig = if with_config {
        serde_json::from_value(config.clone().unwrap().1).unwrap()
    } else {
        Default::default()
    };

    let con_conf: connector::ConnectorConfig = if with_config {
        serde_json::from_value(config.clone().unwrap().2).unwrap()
    } else {
        Default::default()
    };

    let mut scores: Vec<[f64; 4]> = if with_config {
        vec![0.0; rec_conf.max_columns()]
    } else {
        Vec::new()
    };

    let mut recs: Vec<recognizer::Recognizer> = if with_config {
        Vec::with_capacity(org_conf.max_nodes() / 2 + 1)
    } else {
        Vec::new()
    };

    let mut conns: Vec<connector::Connector> = if with_config {
        Vec::with_capacity(org_conf.max_nodes() / 2)
    } else {
        Vec::new()
    };

    while curr_node_num < num_nodes {
        let curr_node = &org.as_array().unwrap()[curr_node_num].as_object().unwrap();

        match curr_node["objectType"].as_str().unwrap() {
            "pssm" => {
                let r = &curr_node["pwm"].as_array().unwrap();

                while curr_col < r.len() {
                    let r = &r[curr_col].as_object().unwrap();

                    for base in ["a", "c", "g", "t"] {
                        if with_config {
                            scores[curr_col][curr_base] = r[base].as_f64().unwrap();
                        } else {
                            scores.push(r[base].as_f64().unwrap());
                        }
                        curr_base += 1;
                    }

                    curr_base = 0;
                    curr_col += 1;
                }

                curr_col = 0;
                if with_config {
                    recs.push(recognizer::pssm(
                        scores.to_owned(),
                        recognzier::RecognizerType::Sequence,
                        r.len(),
                        Some(rec_conf.to_owned()),
                    ));
                } else {
                    recs.push(recognizer::pssm(
                        scores.to_owned(),
                        recognzier::RecognizerType::Sequence,
                        r.len(),
                        None,
                    ));
                }
            }

            "connector" => {
                if with_config {
                    conns.push(connector::build_conn(
                        curr_node["mu"].as_f64().unwrap(),
                        curr_node["sigma"].as_f64().unwrap(),
                        Some(con_conf.to_owned()),
                    ));
                } else {
                    conns.push(connector::build_conn(
                        curr_node["mu"].as_f64().unwrap(),
                        curr_node["sigma"].as_f64().unwrap(),
                        None,
                    ));
                }
            }

            _ => println!("hi"),
        }
        if !with_config {
            scores.clear();
        }
        curr_node_num += 1;
    }

    build_org(recs, conns, None, Some(org_conf))
}

pub fn import_org_from_json(org_file: &str, org_num: usize, conf_file: Option<&str>) -> Organism {
    let conf = if conf_file.is_some() {
        let conf_file = File::open(conf_file.unwrap()).unwrap();
        let conf_reader = BufReader::new(conf_file);
        let conf_value: Value = serde_json::from_reader(conf_reader).unwrap();
        Some((
            conf_value["organism"].to_owned(),
            conf_value["pssm"].to_owned(),
            conf_value["connector"].to_owned(),
        ))
    } else {
        None
    };
    let org_file = File::open(org_file).unwrap();
    let org_reader = BufReader::new(org_file);
    let org_value: Value = serde_json::from_reader(org_reader).unwrap();
    import_org_from_value(org_value[org_num].to_owned(), conf)
}
