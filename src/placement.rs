use crate::aux;
use std::collections::VecDeque;
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

impl Node {
    pub fn node_type(&self) -> NodeType {}
}

#[derive(Debug, Default)]
pub struct Placement {
    energy: f64,
    nodes: VecDeque<Node>,
}

pub trait New {
    fn new() -> Placement;
}

pub trait WithCapacity {
    fn with_capacity(len: usize) -> Placement;
}

impl New for Placement {
    fn new() -> Placement {
        Placement {
            energy: 0.0,
            nodes: VecDeque::new(),
        }
    }
}

impl WithCapacity for Placement {
    fn with_capacity(capacity: usize) -> Placement {
        Placement {
            energy: 0.0,
            nodes: VecDeque::with_capacity(capacity * 2 - 1),
        }
    }
}

impl Placement {
    pub fn at(&self, idx: usize) -> &Node {
        &self.nodes[idx]
    }

    pub fn mut_at(&mut self, idx: usize) {
        &mut self.nodes[idx]
    }

    pub fn energy(&self) -> f64 {
        self.energy
    }

    fn push_front(
        &mut self,
        node_type: NodeType,
        start: usize,
        stop: usize,
        energy: f64,
        seq: String,
    ) {
        self.nodes.push_front(Node {
            node_type,
            start,
            stop,
            energy,
            seq: seq.to_string(),
        });
    }

    pub fn from_matrix(
        seq: &[char],
        rs_matrix: &Vec<Vec<f64>>,
        gs_matrix: &Vec<Vec<f64>>,
        tr_matrix: &Vec<Vec<usize>>,
        rec_lengths: &Vec<usize>,
        c_row: &Vec<f64>,
        m_len: usize,
    ) -> Placement {
        /* seq: array of characters representing the dna sequence */
        /* rs_matrix: 2d vector of recognizer scores from sliding recognizers along seq */
        /* gs_matrix: 2d vector of connector scores corresponding to gaps in the traceback matrix */
        /* tr_matrix: 2d vecotr of gap lengths showing traceback of best alignment */
        /* rec_lengths: vector with the len of each recognizer */
        /* c_row: the row of cumulative scores after running the place function */
        /* m_len: sum of the lengths of all the recognizers */

        let m_idx: usize = aux::maxf_idx(&c_row);
        let mut new: Placement = Placement::with_capacity(rec_lengths.len());
        new.energy = c_row[m_idx];
        let mut c_idx = m_idx + m_len - 1;
        let mut m_idx = m_idx;
        let mut i: i64 = tr_matrix.len() as i64;

        while i >= 0 {
            let idx = i as usize;
            let s_idx = c_idx + 1 - rec_lengths[idx];
            new.push_front(
                NodeType::Recognizer,
                s_idx,
                c_idx,
                rs_matrix[idx][m_idx],
                seq[s_idx..c_idx + 1].iter().collect(),
            );

            if i > 0 {
                c_idx = s_idx - tr_matrix[idx - 1][m_idx] - 1;
                new.push_front(
                    NodeType::Connector,
                    c_idx,
                    s_idx,
                    gs_matrix[idx - 1][m_idx],
                    seq[c_idx + 1..s_idx].iter().collect(),
                );
                m_idx -= tr_matrix[idx - 1][m_idx];
            }
            i -= 1;
        }

        new
    }
}
