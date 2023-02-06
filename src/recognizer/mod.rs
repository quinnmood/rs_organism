use serde::{Deserialize, Serialize};
#[derive(Default, Copy, Clone, Debug)]
pub enum ShapeFeat {
    #[default]
    MGW,
    ProT,
    HelT,
    Roll,
}

#[derive(Default, Copy, Clone, Debug)]
pub enum RecognizerFeat {
    #[default]
    Sequence,
    Shape(ShapeFeat),
}

#[derive(Clone)]
pub struct Recognizer {
    feat: RecognizerFeat,
    len: usize,
    matrix: Option<Vec<[f64; 4]>>,
    mu: Option<f64>,
    sigma: Option<f64>,
    null: Option<Vec<(f64, f64, f64)>>,
    alt: Option<Vec<(f64, f64, f64)>>,
    config: Option<PssmConfig>,
}

impl Recognizer {
    pub fn flip_row(&mut self, row: usize) {
        /*
        for i in 0..self.rec_size/2 {
            self.rec_scores.swap((i * 4) + row, (self.rec_size - i) * 4 - (4 - row));
        }
        */
    }

    pub fn flip_col(&mut self, col: usize) {
        /*
        for i in 0..2 {
            self.rec_scores.swap(col * 4 + i, (col + 1) * 4 - 1 - i);
        }
        */
    }

    pub fn swap_cols(&mut self, col_a: usize, col_b: usize) {
        for i in 0..4 {
            self.rec_scores.swap(col_a * 4 + i, col_b * 4 + i)
        }
    }

    pub fn shift_left(&mut self) {
        for i in 0..self.rec_size - 1 {
            self.swap_cols(i, i + 1);
        }
    }

    pub fn shift_right(&mut self) {
        for i in self.rec_size - 1..0 {
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

pub fn pssm(
    feat: RecognizerFeat,
    len: usize,
    matrix: Vec<[f64; 4]>,
    config: Option<RecognizerConfig>,
) -> Recognizer {
    Recognizer {
        feat,
        len,
        matrix,
        mu: None,
        sigma: None,
        null: None,
        alt: None,
        config: if config.is_some() {
            config.expect("Failed to set recognizer config")
        } else {
            Default::default()
        },
    }
}
