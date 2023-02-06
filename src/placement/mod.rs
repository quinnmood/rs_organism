pub struct Placement {
    id: u8,
    dna_sequence: String,
    energy: f64,
    recognizer_scores: Vec<f64>,
    connector_scores: Vec<f64>,
    recognizer_positions: Vec<f64>,
    connector_positions: Vec<f64>,
}

impl Default for Placement {
    fn default() -> Placement {
        Placement {
            id: Default::default(),
            dna_sequence: Default::default(),
            energy: Default::default(),
            recognizer_scores: Default::default(),
            connector_scores: Default::default(),
            recognizer_positions: Default::default(),
            connector_positions: Default::default(),
        }
    }
}
