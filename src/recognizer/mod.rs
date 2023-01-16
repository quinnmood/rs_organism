use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct Recognizer {
    pub rec_scores: Vec<f64>,
    pub rec_type: char,
    pub rec_size: usize,
    pub config: PssmConfig,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "pssm")]
#[serde(rename_all = "UPPERCASE")]
pub struct PssmConfig {
    pub mutate_probability_random_col: Option<f64>,
    pub mutate_probability_mutate_col: Option<f64>,
    pub mutate_probability_flip_col: Option<f64>,
    pub mutate_probability_flip_row: Option<f64>,
    pub mutate_probability_shift_left: Option<f64>,
    pub mutate_probability_shift_right: Option<f64>,
    pub mutate_probability_increase_pwm: Option<f64>,
    pub mutate_probability_decrease_pwm: Option<f64>,
    pub min_columns: Option<usize>,
    pub max_columns: Option<usize>,
    pub upper_print_probability: Option<f64>,
    pub pseudo_count: Option<f64>,
    pub scan_reverse_complement: Option<bool>,
}

impl Default for PssmConfig {
    fn default() -> PssmConfig {
        PssmConfig {
            mutate_probability_random_col: Default::default(),
            mutate_probability_mutate_col: Default::default(),
            mutate_probability_flip_col: Default::default(),
            mutate_probability_flip_row: Default::default(),
            mutate_probability_shift_left: Default::default(),
            mutate_probability_shift_right: Default::default(),
            mutate_probability_increase_pwm: Default::default(),
            mutate_probability_decrease_pwm: Default::default(),
            min_columns: Default::default(),
            max_columns: Default::default(),
            upper_print_probability: Default::default(),
            pseudo_count: Default::default(),
            scan_reverse_complement: Default::default(),
        } 
    }
}

impl Recognizer {
    pub fn print(&self) { 
        println!("type: {}, size: {}", self.rec_type, self.rec_size);
        let mut i: usize = 0;
        let mut c: usize = 0;
        while i < 4 {
            while c < self.rec_size {
                print!("|{:01.2} : {:02.}", self.rec_scores[c * 4 + i], c * 4 + i);
                c += 1;
            }
            print!("|");
            println!();
            c = 0;
            i += 1;
        }
    }

    pub fn print_config(&self) {
        println!("{}", self.config.mutate_probability_random_col.unwrap());
        println!("{}", self.config.mutate_probability_mutate_col.unwrap());
        println!("{}", self.config.mutate_probability_flip_col.unwrap());
        println!("{}", self.config.mutate_probability_flip_row.unwrap());
        println!("{}", self.config.mutate_probability_shift_left.unwrap());
        println!("{}", self.config.mutate_probability_shift_right.unwrap());
        println!("{}", self.config.mutate_probability_increase_pwm.unwrap());
        println!("{}", self.config.mutate_probability_decrease_pwm.unwrap());
        println!("{}", self.config.min_columns.unwrap());
        println!("{}", self.config.max_columns.unwrap());
        println!("{}", self.config.upper_print_probability.unwrap());
        println!("{}", self.config.pseudo_count.unwrap());
        println!("{}", self.config.scan_reverse_complement.unwrap());
    }

    pub fn set_type(&mut self, rec_type: char){
        self.rec_type = rec_type;
    }

    pub fn set_size(&mut self, rec_size: usize){
        self.rec_size = rec_size; 
    }

    pub fn flip_row(&mut self, row: usize){
        for i in 0..self.rec_size/2 {
            self.rec_scores.swap((i * 4) + row, (self.rec_size - i) * 4 - (4 - row));
            //println!("swapping index {} and {}",(i * 4) + row,  (self.rec_size * 4) - (4 - row) - (i * 4));
        }
    }

    pub fn flip_col(&mut self, col: usize){
        for i in 0..2 {
            self.rec_scores.swap(col * 4 + i, (col + 1) * 4 - 1 - i);
            //println!("swapping index {} and {}", col * 4 + i,  (col + 1) * 4 - 1 - i);
        }
    }

    pub fn swap_cols(&mut self, col_a: usize, col_b: usize){
        for i in 0..4 {
            self.rec_scores.swap(col_a * 4 + i, col_b * 4 + i)
        }
    }

    pub fn shift_left(&mut self){
        for i in 0..self.rec_size - 1 {
            self.swap_cols(i, i + 1);
        }
    }
    
    pub fn shift_right(&mut self){
        for i in self.rec_size - 1..0 {
            self.swap_cols(i, i - 1);
        }
    }
}

pub fn build_rec(rec_scores: Vec<f64>, rec_type: char, rec_size: usize, config: Option<PssmConfig>) -> Recognizer {
    Recognizer {
        rec_scores,
        rec_type,
        rec_size,
        config: if config.is_some() {config.expect("Failed to set recognizer config")} else {Default::default()},
    }
}


