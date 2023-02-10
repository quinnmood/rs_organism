use::std::collections::VecDeque;
#[derive(Debug, Default)]
pub struct RecPlacement {
    start: usize,
    stop: usize,
    energy: f64,
    seq: String,
}

#[derive(Debug, Default)]
pub struct ConPlacement {
    start: usize,
    stop: usize,
    energy: f64,
    seq: String,
}

#[derive(Debug, Default)]
enum NodeType {
    #[default]
    Recognizer,
    Connector,
}

#[derive(Debug, Default)]
struct Node {
    node_type: NodeType,
    start: usize,
    stop: usize,
    energy: f64,
    seq: String,
}

#[derive(Debug, Default)]
pub struct Placement {
    id: u8,
    energy: f64,
    nodes: VecDeque<Node>,
    recs: VecDeque<RecPlacement>,
    cons: VecDeque<ConPlacement>,
}

pub trait New  {
    fn new() -> Placement;
}

pub trait WithCapacity {
    fn with_capacity(len: usize) -> Placement;
}

impl New for Placement {
    fn new() -> Placement {
        Placement {
            id: 0,
            energy: 0.0,
            nodes: VecDeque::new(),
            recs: VecDeque::new(),
            cons: VecDeque::new(),
        }
    }
}

impl WithCapacity for Placement {
    fn with_capacity(capacity: usize ) -> Placement {
        Placement {
            id: 0,
            energy: 0.0,
            nodes: VecDeque::with_capacity(capacity * 2 - 1),
            recs: VecDeque::with_capacity(capacity),
            cons: VecDeque::with_capacity(capacity),
        }
    }
}


impl Placement {
    fn add_rec(&mut self, start: usize, stop: usize, energy: f64, seq: String){
        self.recs.push_front(RecPlacement {
            start,
            stop,
            energy,
            seq: seq.to_string(),
        });
    }
    
    fn add_con(&mut self, start: usize, stop: usize, energy: f64, seq: String){
        self.cons.push_front(ConPlacement {
            start,
            stop,
            energy,
            seq: seq.to_string(),
        });
    }

    fn push_front(&mut self, node_type: NodeType, start: usize, stop: usize, energy: f64, seq: String){
        self.nodes.push_front(Node {
            node_type,
            start,
            stop,
            energy,
            seq: seq.to_string(),
        });
    }
 
    pub fn from_matrix(seq: &[char], rs_matrix: &Vec<Vec<f64>>, gs_matrix: &Vec<Vec<f64>>, tr_matrix: &Vec<Vec<usize>>, rec_lengths: &Vec<usize>, m_idx: usize, m_len: usize) -> Placement{
        let mut new: Placement = Placement::with_capacity(rec_lengths.len());
        let mut c_idx = m_idx + m_len - 1;
        let mut m_idx = m_idx;
        let mut i: i64 = tr_matrix.len() as i64;
        while i >= 0 {
            let idx = i as usize;
            let s_idx = c_idx + 1 - rec_lengths[idx];
            new.push_front(NodeType::Recognizer, s_idx, c_idx, rs_matrix[idx][m_idx], seq[s_idx..c_idx + 1].iter().collect());

            if i > 0 {
                c_idx = s_idx - tr_matrix[idx - 1][m_idx] - 1;
                new.push_front(NodeType::Connector, c_idx, s_idx, gs_matrix[idx - 1][m_idx], seq[c_idx + 1..s_idx].iter().collect());
                m_idx -= tr_matrix[idx - 1][m_idx];
            }
            i -= 1;
        }

        
        new
    }
}
