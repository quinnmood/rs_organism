use crate::config::ConnectorConfig;
use crate::error::ConnectorError;
use num_integer::binomial;
use serde_json::Value;
use statrs::distribution::{Continuous, ContinuousCDF, Normal};

#[derive(Debug, Clone)]
pub struct Connector {
    mu: f64,
    sigma: f64,
    alt: Normal,
    pdf: Vec<f64>,
    cdf: Vec<f64>,
    config: Option<ConnectorConfig>,
}

impl Connector {
    pub fn mu(&self) -> f64 {
        self.mu
    }

    pub fn sigma(&self) -> f64 {
        self.sigma
    }

    pub fn alt(&self) -> &Normal {
        &self.alt
    }

    pub fn alt_mut(&mut self) -> &mut Normal {
        &mut self.alt
    }

    pub fn config(&self) -> &ConnectorConfig {
        self.config
            .as_ref()
            .expect("connector does not have a config")
    }

    pub fn config_mut(&mut self) -> &mut ConnectorConfig {
        self.config
            .as_mut()
            .expect("connector does not have a config")
    }

    pub fn len(&self) -> usize {
        self.pdf.len()
    }

    pub fn set_mu(&mut self, mu: f64) {
        self.mu = mu;
    }

    pub fn add_mu(&mut self, mu: f64) {
        self.mu += mu;
    }

    pub fn set_sigma(&mut self, sigma: f64) {
        self.sigma = sigma;
    }

    pub fn add_sigma(&mut self, sigma: f64) {
        self.sigma = (self.sigma * self.sigma + sigma * sigma).sqrt()
    }

    pub fn precompute(&mut self) {
        let len = self.config().max_seq_len();
        self.pdf.clear();
        self.cdf.clear();
        for i in 0..len {
            self.pdf.push(self.alt().pdf(i as f64));
            self.cdf.push(self.alt().cdf(i as f64));
        }
    }

    pub fn precompute_from_size(&mut self, len: usize) {
        self.pdf.clear();
        self.cdf.clear();
        for i in 0..len {
            self.pdf.push(self.alt().pdf(i as f64));
            self.cdf.push(self.alt().cdf(i as f64));
        }
    }

    pub fn compute_until(&mut self, stop: usize) {
        for i in self.len()..stop {
            self.pdf.push(self.alt().pdf(i as f64));
            self.cdf.push(self.alt().cdf(i as f64));
        }
    }

    pub fn score(&self, gap: usize, seq_len: usize, eff_len: usize, num_recs: usize) -> f64 {
        let num: f64 = self.pdf[gap];

        let auc: f64 = self.cdf[seq_len - 1] - self.cdf[0];

        let num: f64 = if auc > 1E-10 {
            num / auc
        } else {
            num / 0.000001
        };

        let den: f64 = binomial((eff_len - (gap + 1)) as u64, (num_recs - 1) as u64) as f64
            / binomial(eff_len as u64, num_recs as u64) as f64;

        num.log2() - den.log2()
    }
}

pub fn from_value(
    con: &Value,
    config: Option<&ConnectorConfig>,
) -> Result<Connector, ConnectorError> {
    let con = con.as_object().ok_or_else(|| {
        ConnectorError::ParseJSONError(serde::de::Error::invalid_type(
            serde::de::Unexpected::Option,
            &"hi",
        ))
    })?;
    let mu = con["mu"].as_f64().ok_or_else(|| {
        ConnectorError::ParseJSONError(serde::de::Error::invalid_type(
            serde::de::Unexpected::Option,
            &"hi",
        ))
    })?;
    let sigma = con["sigma"].as_f64().ok_or_else(|| {
        ConnectorError::ParseJSONError(serde::de::Error::invalid_type(
            serde::de::Unexpected::Option,
            &"hi",
        ))
    })?;
    Ok(connector(mu, sigma, config.cloned()))
}

pub fn connector(mu: f64, sigma: f64, config: Option<ConnectorConfig>) -> Connector {
    let is_conf: bool = config.is_some();

    if is_conf {
        let conf = config.clone().unwrap();
        let max_seq_len = conf.max_seq_len();
        let mut new: Connector = Connector {
            mu,
            sigma,
            alt: Normal::new(mu, sigma).expect("invalid mu or sigma for connector"),
            pdf: Vec::with_capacity(max_seq_len * 10),
            cdf: Vec::with_capacity(max_seq_len * 10),
            config: config,
        };
        new.precompute();
        return new;
    }
    Connector {
        mu,
        sigma,
        alt: Normal::new(mu, sigma).expect("invalid mu or sigma for connector"),
        pdf: Vec::new(),
        cdf: Vec::new(),
        config: None,
    }
}
