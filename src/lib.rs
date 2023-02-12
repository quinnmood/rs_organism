mod aux;
mod config;
mod connector;
mod error;
mod placement;
mod recognizer;
use crate::config::{ConnectorConfig, OrganismConfig, RecognizerConfig};
use crate::connector::Connector;
use crate::error::OrganismError;
use crate::placement::Placement;
use crate::recognizer::Recognizer;
use rand::random;
use serde_json::Value;
use std::{cell, f64, fs, io, mem};

#[derive(Clone, Debug, Default)]
pub struct Organism {
    recs: Vec<cell::RefCell<Recognizer>>,
    cons: Vec<cell::RefCell<Connector>>,
    id: Option<usize>,
    config: Option<OrganismConfig>,
}

impl Organism {
    pub fn recs(&self) -> &Vec<cell::RefCell<Recognizer>> {
        &self.recs
    }

    pub fn cons(&self) -> &Vec<cell::RefCell<Connector>> {
        &self.cons
    }

    pub fn id(&self) -> usize {
        self.id.expect("organism does not have an id")
    }

    pub fn config(&self) -> &OrganismConfig {
        self.config
            .as_ref()
            .expect("organism does not have a config")
    }

    pub fn len(&self) -> usize {
        self.recs.len() + self.cons.len()
    }

    pub fn num_recs(&self) -> usize {
        self.recs.len()
    }

    pub fn num_cons(&self) -> usize {
        self.cons.len()
    }

    pub fn len_recs(&self) -> usize {
        let mut sum: usize = 0;
        for i in 0..self.recs.len() {
            sum += &self.recs[i].borrow().len()
        }
        sum
    }

    pub fn rec_at(&self, rec_idx: usize) -> &cell::RefCell<recognizer::Recognizer> {
        &self.recs[rec_idx]
    }

    pub fn rec_at_mut(&mut self, rec_idx: usize) -> &mut cell::RefCell<recognizer::Recognizer> {
        &mut self.recs[rec_idx]
    }

    pub fn con_at(&self, con_idx: usize) -> &cell::RefCell<connector::Connector> {
        &self.cons[con_idx]
    }

    pub fn con_at_mut(&mut self, con_idx: usize) -> &mut cell::RefCell<connector::Connector> {
        &mut self.cons[con_idx]
    }

    pub fn swap_rec(&mut self, rec_a: usize, rec_b: usize) {
        self.recs.swap(rec_a, rec_b);
    }

    pub fn swap_con(&mut self, con_a: usize, con_b: usize) {
        self.recs.swap(con_a, con_b);
    }

    pub fn remove(&mut self, rec_idx: usize) {
        let num_recs = self.num_recs();
        let num_cons = self.num_cons();
        if num_recs < 2 {
            return;
        }

        let con_idx;
        let adj_idx;
        match rec_idx {
            0 => {
                con_idx = 0;
                adj_idx = 0;
            }

            _ if rec_idx == num_cons => {
                con_idx = num_recs - 2;
                adj_idx = con_idx - 1
            }

            _ => {
                con_idx = if random() { rec_idx - 1 } else { rec_idx };
                adj_idx = con_idx
            }
        }

        let binding = self.recs.remove(rec_idx);
        let deleted_rec = binding.borrow();
        let binding = self.cons.remove(con_idx);
        let deleted_con = binding.borrow();
        if self.config.is_none() {
            return;
        }

        let method = self.config();
        if method.deletion_method() == "intelligent" && num_cons > 1 {
            self.cons[adj_idx]
                .borrow_mut()
                .add_mu(deleted_rec.len() as f64 + deleted_con.mu());

            self.cons[adj_idx]
                .borrow_mut()
                .add_sigma(deleted_con.sigma());
            return;
        }
    }

    pub fn insert(&mut self, rec_idx: usize) {}

    pub fn print(&self) {
        let len = self.num_recs();
        for i in 0..4 {
            for j in 0..len {
                let rec = self.recs[j].borrow();
                let rec_len = rec.len();
                let matrix = rec.matrix();
                for k in 0..rec_len {
                    print!("|{:1.2}", matrix[k * 4 + i])
                }

                if i == 0 && j < len - 1 {
                    let con = self.cons[j].borrow();
                    let mu = con.mu();
                    let sigma = con.sigma();
                    print!("|--<{:2.2}, {:2.2}>--", mu, sigma);
                } else {
                    print!("|                 ");
                }
            }
            println!("");
        }
    }

    pub fn place(&self, seq: &[char], precomp: Option<&[f64]>) -> Placement {
        let num_recs: usize = self.num_recs();
        let min_len: usize = self.len_recs();
        let seq_len: usize = seq.len();
        let eff_len: usize = seq_len - min_len + num_recs;
        let n_align: usize = seq_len - min_len + 1;

        let mut f_offset: usize = 0;
        let mut r_offset: usize = seq_len - min_len;

        let mut t_row: Vec<f64> = vec![f64::NEG_INFINITY; n_align];
        let mut c_row: Vec<f64> = vec![0.0; n_align];

        let mut rs_matrix: Vec<Vec<f64>> = vec![vec![0.0; n_align]; num_recs];
        let mut gs_matrix: Vec<Vec<f64>> = vec![vec![0.0; n_align]; num_recs - 1];
        let mut tr_matrix: Vec<Vec<usize>> = vec![vec![0; n_align]; num_recs - 1];
        let mut rec_lengths: Vec<usize> = vec![0; num_recs];

        for i in 0..num_recs {
            let curr_rec = self.recs[i].borrow();
            rec_lengths[i] = curr_rec.len();
            r_offset += curr_rec.len();
            curr_rec.calculate_row(&seq[f_offset..r_offset], &mut rs_matrix[i]);
            if i > 0 {
                let curr_con = self.cons[i - 1].borrow();
                for j in 0..n_align {
                    for k in 0..(j + 1) {
                        let gap = j - k;
                        let g_score = curr_con.score(gap, seq_len, eff_len, num_recs);
                        if t_row[j] < c_row[k] + g_score + rs_matrix[i][j] {
                            t_row[j] = c_row[k] + g_score + rs_matrix[i][j];
                            tr_matrix[i - 1][j] = gap;
                            gs_matrix[i - 1][j] = g_score;
                        }
                    }
                }
                c_row = mem::replace(&mut t_row, vec![f64::NEG_INFINITY; n_align]);
            } else {
                c_row.copy_from_slice(&rs_matrix[i])
            }
            f_offset += curr_rec.len();
        }
        Placement::from_matrix(
            &seq,
            &rs_matrix,
            &gs_matrix,
            &tr_matrix,
            &rec_lengths,
            &c_row,
            min_len,
        )
    }

    pub fn check(&mut self, seq_len: usize) -> Result<(), OrganismError> {
        if self.len_recs() > seq_len {
            return Err(OrganismError::ExceedSeqError);
        }

        for i in 0..self.num_cons() {
            let mut con = self.con_at_mut(i).borrow_mut();
            if seq_len > con.len() {
                con.compute_until(seq_len);
            }
        }

        Ok(())
    }
}

pub fn from_value(
    org: &Value,
    org_conf: Option<&OrganismConfig>,
    rec_conf: Option<&RecognizerConfig>,
    con_conf: Option<&ConnectorConfig>,
) -> Result<Organism, OrganismError> {
    let nodes = org.as_array().unwrap();
    let num_nodes = nodes.len();
    let mut recs: Vec<cell::RefCell<Recognizer>> = Vec::new();
    let mut cons: Vec<cell::RefCell<Connector>> = Vec::new();

    for i in 0..num_nodes {
        match nodes[i].as_object().unwrap()["objectType"]
            .as_str()
            .unwrap()
        {
            "pssm" => recs.push(cell::RefCell::new(recognizer::from_value(
                &nodes[i], rec_conf,
            )?)),
            "connector" => cons.push(cell::RefCell::new(connector::from_value(
                &nodes[i], con_conf,
            )?)),
            "shape" => break,
            _ => break,
        }
    }

    Ok(organism(recs, cons, None, org_conf.cloned()))
}

pub fn organism(
    recs: Vec<cell::RefCell<recognizer::Recognizer>>,
    cons: Vec<cell::RefCell<connector::Connector>>,
    id: Option<usize>,
    config: Option<OrganismConfig>,
) -> Organism {
    Organism {
        recs,
        cons,
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
            let conf_file = fs::File::open(conf_file.ok_or_else(|| {
                OrganismError::IOError(io::Error::new(io::ErrorKind::NotFound, "oh no"))
            })?)?;
            let conf_reader = io::BufReader::new(conf_file);
            let conf_value: Value = serde_json::from_reader(conf_reader)?;
            let org_conf: OrganismConfig = serde_json::from_value(conf_value["organism"].clone())?;
            let rec_conf: RecognizerConfig =
                serde_json::from_value(conf_value["recognizer"].clone())?;
            let con_conf: ConnectorConfig =
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
            let rec_conf: RecognizerConfig =
                serde_json::from_value(conf_value["recognizer"].clone())?;
            let con_conf: ConnectorConfig =
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
