use num_traits::Float;

pub fn distance_to_space<F, T>(p1: &[T], min_bounds: &[T], max_bounds: &[T], distance: &F) -> T
where
    F: Fn(&[T], &[T]) -> T,
    T: Float,
{
    let mut p2 = vec![T::nan(); p1.len()];
    for i in 0..p1.len() {
        if p1[i] > max_bounds[i] {
            p2[i] = max_bounds[i];
        } else if p1[i] < min_bounds[i] {
            p2[i] = min_bounds[i];
        } else {
            p2[i] = p1[i];
        }
    }
    distance(p1, &p2[..])
}

#[cfg(test)]
mod tests {
    use super::distance_to_space;
    use crate::distance::squared_euclidean;
    use std::f64::{INFINITY, NEG_INFINITY};

    #[test]
    fn test_normal_distance_to_space() {
        let dis = distance_to_space(&[0.0, 0.0], &[1.0, 1.0], &[2.0, 2.0], &squared_euclidean);
        assert_eq!(dis, 2.0);
    }

    #[test]
    fn test_distance_outside_inf() {
        let dis = distance_to_space(
            &[0.0, 0.0],
            &[1.0, 1.0],
            &[INFINITY, INFINITY],
            &squared_euclidean,
        );
        assert_eq!(dis, 2.0);
    }

    #[test]
    fn test_distance_inside_inf() {
        let dis = distance_to_space(
            &[2.0, 2.0],
            &[NEG_INFINITY, NEG_INFINITY],
            &[INFINITY, INFINITY],
            &squared_euclidean,
        );
        assert_eq!(dis, 0.0);
    }

    #[test]
    fn test_distance_inside_normal() {
        let dis = distance_to_space(&[2.0, 2.0], &[0.0, 0.0], &[3.0, 3.0], &squared_euclidean);
        assert_eq!(dis, 0.0);
    }

    #[test]
    fn distance_to_half_space() {
        let dis = distance_to_space(
            &[-2.0, 0.0],
            &[0.0, NEG_INFINITY],
            &[INFINITY, INFINITY],
            &squared_euclidean,
        );
        assert_eq!(dis, 4.0);
    }
}
