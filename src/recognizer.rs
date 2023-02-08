use serde::{Deserialize, Serialize};
use serde_json::Value;

const BASES: [&str; 4] = ["a", "c", "g", "t"];

#[derive(thiserror::Error, Debug)]
pub enum RecognizerError {
    #[error("failed to load recognizer")]
    LoadRecognizerError,

    #[error("failed to parse recognizer from JSON value")]
    ParseJSONError(#[from] serde_json::Error),
}

#[derive(Default, Copy, Clone, Debug)]
pub enum ShapeFeat {
    #[default]
    MGW,
    ProT,
    HelT,
    Roll,
    None,
}

#[derive(Default, Copy, Clone, Debug)]
pub enum RecognizerFeat {
    #[default]
    Sequence,
    Shape(ShapeFeat),
    None,
}

#[derive(Default, Debug, Clone)]
pub struct Recognizer {
    feat: RecognizerFeat,
    len: usize,
    matrix: Vec<f64>,
    mu: f64,
    sigma: f64,
    null: Vec<(f64, f64, f64)>,
    alt: Vec<(f64, f64, f64)>,
    config: Option<RecognizerConfig>,
}

impl Recognizer {
    pub fn feat(&self) -> RecognizerFeat {
        self.feat
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn matrix(&self) -> &Vec<f64> {
        self.matrix.as_ref()
    }

    pub fn matrix_mut(&mut self) -> &mut Vec<f64> {
        self.matrix.as_mut()
    }

    pub fn mu(&self) -> f64 {
        self.mu
    }

    pub fn sigma(&self) -> f64 {
        self.sigma
    }

    pub fn null(&self) -> &Vec<(f64, f64, f64)> {
        self.null.as_ref()
    }

    pub fn alt(&self) -> &Vec<(f64, f64, f64)> {
        self.alt.as_ref()
    }

    pub fn config(&self) -> &RecognizerConfig {
        self.config.as_ref().expect("recognizer does not have a config")
    }

    pub fn set_feat(&mut self, feat: RecognizerFeat) {
        self.feat = feat;
    }

    pub fn set_len(&mut self, len: usize) {
        self.len = len;
    }

    pub fn set_at(&mut self, val: f64, index: (usize, usize)) {
        self.matrix[index.0 * self.len + index.1] = val;
    }

    pub fn set_mu(&mut self, mu: f64) {
        self.mu = mu;
    }

    pub fn set_sigma(&mut self, sigma: f64) {
        self.sigma = sigma
    }

    pub fn print(&self) {
        let len = self.len();
        for i in 0..4 {
            for j in 0..len {
                print!("|{}", &self.matrix[j * 4 + i])
            }
            println!("");
        }
    }

    pub fn flip_row(&mut self, row: usize) {
        let len = self.len();
        for i in 0..len / 2 {
            self.matrix.swap((i * 4) + row, (len - i) * 4 - (4 - row));
        }
    }

    pub fn flip_col(&mut self, col: usize) {
        for i in 0..2 {
            self.matrix.swap(col * 4 + i, (col + 1) * 4 - 1 - i);
        }
    }

    pub fn swap_cols(&mut self, col_a: usize, col_b: usize) {
        for i in 0..4 {
            self.matrix.swap(col_a * 4 + i, col_b * 4 + i)
        }
    }

    pub fn swap_rows(&mut self, row_a: usize, row_b: usize) {
        let len = self.len();
        for i in 0..len {
            self.matrix.swap(row_a + i * 4, row_b + i * 4)
        }
    }

    pub fn shift_left(&mut self) {
        let len = self.len();
        for i in 0..len - 1 {
            self.swap_cols(i, i + 1);
        }
    }

    pub fn shift_right(&mut self) {
        let len = self.len();
        for i in len - 1..0 {
            self.swap_cols(i, i - 1);
        }
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

pub fn from_value(
    rec: &Value,
    conf: Option<&RecognizerConfig>,
) -> Result<Recognizer, RecognizerError> {
    match rec.as_object().unwrap()["objectType"]
        .as_str()
        .ok_or_else(|| {
            RecognizerError::ParseJSONError(serde::de::Error::invalid_type(
                serde::de::Unexpected::Option,
                &"hi",
            ))
        })? {
        "pssm" => pssm_from_value(&rec["pwm"], conf),
        "shape" => shape_from_value(&rec, conf),
        _ => Err(RecognizerError::LoadRecognizerError),
    }
}

pub fn pssm_from_value(
    rec: &Value,
    conf: Option<&RecognizerConfig>,
) -> Result<Recognizer, RecognizerError> {
    //let rec = rec.as_object()?["pwm"].as_array()?;
    let rec = rec.as_array().ok_or_else(|| {
        RecognizerError::ParseJSONError(serde::de::Error::invalid_type(
            serde::de::Unexpected::Option,
            &"hi",
        ))
    })?;
    let len = rec.len();
    let mut matrix: Vec<f64> = vec![0.00; len * 4];
    for i in 0..len {
        let col = &rec[i].as_object().ok_or_else(|| {
            RecognizerError::ParseJSONError(serde::de::Error::invalid_type(
                serde::de::Unexpected::Option,
                &"hi",
            ))
        })?;
        for j in 0..BASES.len() {
            matrix[i * 4 + j] = col[BASES[j]].as_f64().ok_or_else(|| {
                RecognizerError::ParseJSONError(serde::de::Error::invalid_type(
                    serde::de::Unexpected::Option,
                    &"hi",
                ))
            })?;
        }
    }
    Ok(pssm(
        RecognizerFeat::Sequence,
        len,
        Some(matrix),
        conf.cloned(),
    ))
}

pub fn shape_from_value(
    rec: &Value,
    conf: Option<&RecognizerConfig>,
) -> Result<Recognizer, RecognizerError> {
    let feat = match rec["feature"].as_str().ok_or_else(|| {
        RecognizerError::ParseJSONError(serde::de::Error::invalid_type(
            serde::de::Unexpected::Option,
            &"hi",
        ))
    })? {
        "MGW" => RecognizerFeat::Shape(ShapeFeat::MGW),
        "ProT" => RecognizerFeat::Shape(ShapeFeat::ProT),
        "HelT" => RecognizerFeat::Shape(ShapeFeat::HelT),
        "Roll" => RecognizerFeat::Shape(ShapeFeat::Roll),
        _ => RecognizerFeat::Shape(ShapeFeat::None),
    };

    let mu = rec["mu"].as_f64().ok_or_else(|| {
        RecognizerError::ParseJSONError(serde::de::Error::invalid_type(
            serde::de::Unexpected::Option,
            &"hi",
        ))
    })?;
    let sigma = rec["sigma"].as_f64().ok_or_else(|| {
        RecognizerError::ParseJSONError(serde::de::Error::invalid_type(
            serde::de::Unexpected::Option,
            &"hi",
        ))
    })?;
    let len = rec["length"].as_u64().ok_or_else(|| {
        RecognizerError::ParseJSONError(serde::de::Error::invalid_type(
            serde::de::Unexpected::Option,
            &"hi",
        ))
    })? as usize;

    Ok(shape(feat, len, Some(mu), Some(sigma), conf.cloned()))
}

pub fn shape(
    feat: RecognizerFeat,
    len: usize,
    mu: Option<f64>,
    sigma: Option<f64>,
    config: Option<RecognizerConfig>,
) -> Recognizer {
    Recognizer {
        feat,
        len,
        matrix: Vec::new(),
        mu: if mu.is_some() {
            mu.unwrap()
        } else {
            f64::default()
        },
        sigma: if sigma.is_some() {
            sigma.unwrap()
        } else {
            f64::default()
        },
        null: Vec::new(),
        alt: Vec::new(),
        config,
    }
}

pub fn pssm(
    feat: RecognizerFeat,
    len: usize,
    matrix: Option<Vec<f64>>,
    config: Option<RecognizerConfig>,
) -> Recognizer {
    Recognizer {
        feat,
        len,
        matrix: if matrix.is_some() {
            matrix.unwrap()
        } else {
            Vec::new()
        },
        mu: f64::default(),
        sigma: f64::default(),
        null: Vec::new(),
        alt: Vec::new(),
        config,
    }
}
