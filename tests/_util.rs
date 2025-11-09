use kdtree::KdTree;

pub mod fixtures {
    use super::KdTree;

    pub const POINT_A: ([f64; 2], usize) = ([0f64, 0f64], 0);
    pub const POINT_B: ([f64; 2], usize) = ([1f64, 1f64], 1);
    pub const POINT_C: ([f64; 2], usize) = ([2f64, 2f64], 2);
    pub const POINT_D: ([f64; 2], usize) = ([3f64, 3f64], 3);

    pub fn basic_tree() -> KdTree<f64, usize, [f64; 2]> {
        let mut tree = KdTree::with_capacity(2, 2);
        for &(coords, value) in &[POINT_A, POINT_B, POINT_C, POINT_D] {
            tree.add(coords, value).unwrap();
        }
        tree
    }
}

pub mod assertions {
    #[allow(dead_code)]
    pub fn assert_ordered_usize(results: Vec<(f64, &usize)>, expected: &[(f64, usize)]) {
        assert_eq!(normalize(results), expected);
    }

    #[allow(dead_code)] // some test crates (e.g., nearest) do not use this helper yet
    pub fn assert_unordered_usize(results: Vec<(f64, &usize)>, expected: &[(f64, usize)]) {
        let mut actual = normalize(results);
        let mut expected_vec = expected.to_vec();
        actual.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap().then_with(|| a.1.cmp(&b.1)));
        expected_vec.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap().then_with(|| a.1.cmp(&b.1)));
        assert_eq!(actual, expected_vec);
    }

    fn normalize(results: Vec<(f64, &usize)>) -> Vec<(f64, usize)> {
        results.into_iter().map(|(d, value)| (d, *value)).collect()
    }
}
