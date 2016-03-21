pub fn squared_euclidean(a: &[f64], b: &[f64]) -> f64 {
    debug_assert!(a.len() == b.len());
    a.iter().zip(b.iter())
            .map(|(x, y)| (x - y) * (x - y))
            .fold(0f64, ::std::ops::Add::add)
}
