use std;

pub fn distance_to_space<F> (p1: &[f64], min_bounds: &[f64], max_bounds: &[f64], distance: &F) -> f64
where F: Fn(&[f64], &[f64]) -> f64 {
    let mut p2 = vec![std::f64::NAN; p1.len()];
    for i in 0..p1.len() {
        if p1[i] > max_bounds[i] {
            p2[i] = max_bounds[i];
        } else  if p1[i] < min_bounds[i] {
            p2[i] = min_bounds[i];
        } else {
            p2[i] = p1[i];
        }
    }
    distance(p1, &p2[..])
}
