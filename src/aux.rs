pub fn maxf_idx(vec: &Vec<f64>) -> usize {
    let mut c_idx: usize = 0;
    for i in 0..vec.len() {
        if vec[i] > vec[c_idx] {
            c_idx = i
        }
    }
    c_idx
}
