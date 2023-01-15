pub mod recognizer;
pub mod connector;
use serde_json::{Result, Value};
use serde::{Deserialize, Serialize};
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
            rec.print();        
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
    /*Organism {
        recognizers,
        connectors,
        config: Default::default(),
    }*/
    Organism::default()
}

pub fn import_org(org: Value, config: (Value, Value, Value)) -> Organism  {
    let num_nodes = org.as_array().unwrap().len();
    let curr_node: usize = 0;
    

    /*
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let v: Value = serde_json::from_reader(reader).unwrap();
    let num_orgs = v.as_array().unwrap().len();

    let mut scores: Vec<f64> = vec![0.0; max_rec_size * 4];
    let mut orgs: Vec<Organism> = Vec::with_capacity(num_orgs);
    let mut c: usize = 0;
    let mut p: usize = 0;
    let mut b: usize = 0;
    let mut o: usize = 0;

    while o < num_orgs {
        let num_nodes = v[o].as_array().unwrap().len();
        let mut recs: Vec<recognizer::Recognizer> = Vec::with_capacity(max_recs);
        let mut conns: Vec<connector::Connector> = Vec::with_capacity(max_recs - 1);
        while c < num_nodes {
            let s = &v[o][c].as_object().unwrap(); 
            match s["objectType"].as_str().unwrap() {
                
                "pssm" => {
                    let s = &s["pwm"].as_array().unwrap();
                    while p < s.len() {
                        let s = &s[p].as_object().unwrap();
                        for base in ["a", "c", "g", "t"] {
                            scores[p * s.len() + b] = s[base].as_f64().unwrap();
                            b += 1
                        }
                        b = 0;
                        p += 1
                    }
                    recs.push(recognizer::build_rec(Vec::to_owned(&scores), 'p', s.len(), max_rec_size));
                },

                "connector" => {
                    conns.push(connector::build_conn(s["mu"].as_f64().unwrap(), s["sigma"].as_f64().unwrap()));
                }
                _ => std::process::exit(5),
            
            }
            p = 0;
            c += 1;
        }
    //build_org(recs, conns, max_recs, max_rec_size).print();
    orgs.push(build_org(recs, conns, max_recs, max_rec_size));
    c = 0;
    o += 1;
    }
    orgs
    */
    Organism::default()
}
