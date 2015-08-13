pub fn squared_euclidean(a: &[f64], b: &[f64]) -> f64 {
    let mut dist = 0f64;
    for i in 0..a.len() {
        dist += (a[i] - b[i]) * (a[i] - b[i]);
    }
    return dist;
}
