mod __util__;

use __util__::{POINT_A, POINT_B, basic_tree};
use kdtree::KdTree;
use kdtree::distance::squared_euclidean;

fn assert_unordered_usize(results: Vec<(f64, &usize)>, expected: &[(f64, usize)]) {
    let mut actual = results.into_iter().map(|(d, v)| (d, *v)).collect::<Vec<_>>();
    let mut expected_vec = expected.to_vec();
    actual.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap().then_with(|| a.1.cmp(&b.1)));
    expected_vec.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap().then_with(|| a.1.cmp(&b.1)));
    assert_eq!(actual, expected_vec);
}

#[test]
fn within_queries_match_expected() {
    let tree = basic_tree();

    assert_unordered_usize(tree.within(&POINT_A.0, 0.0, &squared_euclidean).unwrap(), &[(0.0, 0)]);
    assert_unordered_usize(tree.within(&POINT_B.0, 1.0, &squared_euclidean).unwrap(), &[(0.0, 1)]);
    assert_unordered_usize(
        tree.within(&POINT_B.0, 2.0, &squared_euclidean).unwrap(),
        &[(0.0, 1), (2.0, 2), (2.0, 0)],
    );

    assert_eq!(tree.within_count(&POINT_A.0, 0.0, &squared_euclidean).unwrap(), 1);
    assert_eq!(tree.within_count(&POINT_B.0, 1.0, &squared_euclidean).unwrap(), 1);
    assert_eq!(tree.within_count(&POINT_B.0, 2.0, &squared_euclidean).unwrap(), 3);
}

#[test]
fn within_handles_pending_order_tree() {
    let item1 = ([0f64], 1);
    let item2 = ([100f64], 2);
    let item3 = ([45f64], 3);
    let item4 = ([55f64], 4);

    let mut tree = KdTree::with_capacity(1, 2);
    tree.add(&item1.0, item1.1).unwrap();
    tree.add(&item2.0, item2.1).unwrap();
    tree.add(&item3.0, item3.1).unwrap();
    tree.add(&item4.0, item4.1).unwrap();

    assert_unordered_usize(
        tree.within(&[50f64], 25.0, &squared_euclidean).unwrap(),
        &[(25.0, 3), (25.0, 4)],
    );
    assert_unordered_usize(
        tree.within(&[50f64], 30.0, &squared_euclidean).unwrap(),
        &[(25.0, 3), (25.0, 4)],
    );
    assert_unordered_usize(tree.within(&[55f64], 5.0, &squared_euclidean).unwrap(), &[(0.0, 4)]);
    assert_unordered_usize(tree.within(&[56f64], 5.0, &squared_euclidean).unwrap(), &[(1.0, 4)]);
}
