extern crate organism;
use organism::OrganismConfig;
use serde_json::{Result, Value};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufReader, error::Error};

fn main() {
    let file = File::open("config.json").unwrap();
    let reader = BufReader::new(file);
    let conf: Value = serde_json::from_reader(reader).unwrap();
    let org_conf: OrganismConfig = serde_json::from_value(conf["organism"].to_owned()).unwrap();
    println!("{}", org_conf.mutate_probability_node_mutation as f64);
    
}
