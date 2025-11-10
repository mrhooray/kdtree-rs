mod __util__;

use __util__::{POINT_A, POINT_B, POINT_C, basic_tree};
use kdtree::distance::squared_euclidean;
use kdtree::{ErrorKind, KdTree};

fn assert_ordered_usize(results: Vec<(f64, &usize)>, expected: &[(f64, usize)]) {
    assert_eq!(results.into_iter().map(|(d, v)| (d, *v)).collect::<Vec<_>>(), expected);
}

#[test]
fn nearest_queries_match_expected() {
    let tree = basic_tree();
    assert_eq!(tree.size(), 4);

    assert_ordered_usize(tree.nearest(&POINT_A.0, 0, &squared_euclidean).unwrap(), &[]);
    assert_ordered_usize(tree.nearest(&POINT_A.0, 1, &squared_euclidean).unwrap(), &[(0f64, 0)]);
    assert_ordered_usize(
        tree.nearest(&POINT_A.0, 2, &squared_euclidean).unwrap(),
        &[(0f64, 0), (2f64, 1)],
    );
    assert_ordered_usize(
        tree.nearest(&POINT_A.0, 3, &squared_euclidean).unwrap(),
        &[(0f64, 0), (2f64, 1), (8f64, 2)],
    );
    assert_ordered_usize(
        tree.nearest(&POINT_A.0, 4, &squared_euclidean).unwrap(),
        &[(0f64, 0), (2f64, 1), (8f64, 2), (18f64, 3)],
    );
    assert_ordered_usize(
        tree.nearest(&POINT_A.0, 5, &squared_euclidean).unwrap(),
        &[(0f64, 0), (2f64, 1), (8f64, 2), (18f64, 3)],
    );
    assert_ordered_usize(
        tree.nearest(&POINT_B.0, 4, &squared_euclidean).unwrap(),
        &[(0f64, 1), (2f64, 0), (2f64, 2), (8f64, 3)],
    );
}

#[test]
fn iter_nearest_matches_batch() {
    let tree = basic_tree();
    let collected: Vec<_> = tree.iter_nearest(&POINT_A.0, &squared_euclidean).unwrap().collect();
    assert_ordered_usize(collected, &[(0f64, 0), (2f64, 1), (8f64, 2), (18f64, 3)]);
}

#[test]
fn iter_nearest_mut_applies_updates() {
    let mut tree = basic_tree();
    let (dist, value) = tree
        .iter_nearest_mut(&POINT_A.0, &squared_euclidean)
        .unwrap()
        .next()
        .unwrap();
    assert_eq!(dist, 0f64);
    *value = 10;

    let collected: Vec<_> = tree.iter_nearest(&POINT_A.0, &squared_euclidean).unwrap().collect();
    assert_ordered_usize(collected, &[(0f64, 10), (2f64, 1), (8f64, 2), (18f64, 3)]);
}

#[test]
fn nearest_within_radius_matches_unbounded_when_unlimited() {
    let tree = basic_tree();
    let unrestricted = tree.nearest(&POINT_A.0, 4, &squared_euclidean).unwrap();
    let bounded = tree
        .nearest_within_radius(&POINT_A.0, 4, None, &squared_euclidean)
        .unwrap();
    assert_eq!(unrestricted, bounded);
}

#[test]
fn nearest_within_radius_respects_limit() {
    let tree = basic_tree();
    let filtered = tree
        .nearest_within_radius(&POINT_A.0, 4, Some(1.5), &squared_euclidean)
        .unwrap();
    assert_eq!(filtered, vec![(0f64, &0)]);
}

#[test]
fn nearest_within_radius_returns_empty_for_zero_k() {
    let tree = basic_tree();
    let filtered = tree
        .nearest_within_radius(&POINT_A.0, 0, Some(1.5), &squared_euclidean)
        .unwrap();
    assert!(filtered.is_empty());
}

#[test]
fn iter_nearest_within_radius_collects_results() {
    let tree = basic_tree();
    let collected: Vec<_> = tree
        .iter_nearest_within_radius(&POINT_B.0, Some(3.0), &squared_euclidean)
        .unwrap()
        .collect();
    assert_ordered_usize(collected, &[(0f64, 1), (2f64, 0), (2f64, 2)]);
}

#[test]
fn iter_nearest_within_radius_mut_changes_only_within_radius() {
    let mut tree = basic_tree();
    {
        let mut iter = tree
            .iter_nearest_within_radius_mut(&POINT_A.0, Some(0.5), &squared_euclidean)
            .unwrap();
        let (dist, value) = iter.next().unwrap();
        assert_eq!(dist, 0f64);
        *value = 42;
        assert!(iter.next().is_none());
    }
    let collected = tree.nearest(&POINT_A.0, 2, &squared_euclidean).unwrap();
    assert_eq!(collected[0], (0f64, &42));
    assert_eq!(collected[1], (2f64, &1));
}

#[test]
fn nearest_supports_vec_points() {
    let dimensions = 2;
    let capacity_per_node = 2;
    let mut tree = KdTree::with_capacity(dimensions, capacity_per_node);

    tree.add(vec![0.0; 2], 0).unwrap();
    tree.add(vec![1.0; 2], 1).unwrap();
    tree.add(vec![2.0; 2], 2).unwrap();
    tree.add(vec![3.0; 2], 3).unwrap();

    assert_ordered_usize(tree.nearest(&POINT_A.0, 0, &squared_euclidean).unwrap(), &[]);
    assert_ordered_usize(tree.nearest(&POINT_A.0, 1, &squared_euclidean).unwrap(), &[(0f64, 0)]);
    assert_ordered_usize(
        tree.nearest(&POINT_A.0, 2, &squared_euclidean).unwrap(),
        &[(0f64, 0), (2f64, 1)],
    );
    assert_ordered_usize(
        tree.nearest(&POINT_A.0, 3, &squared_euclidean).unwrap(),
        &[(0f64, 0), (2f64, 1), (8f64, 2)],
    );
    assert_ordered_usize(
        tree.nearest(&POINT_A.0, 4, &squared_euclidean).unwrap(),
        &[(0f64, 0), (2f64, 1), (8f64, 2), (18f64, 3)],
    );
    assert_ordered_usize(
        tree.nearest(&POINT_A.0, 5, &squared_euclidean).unwrap(),
        &[(0f64, 0), (2f64, 1), (8f64, 2), (18f64, 3)],
    );
    assert_ordered_usize(
        tree.nearest(&POINT_B.0, 4, &squared_euclidean).unwrap(),
        &[(0f64, 1), (2f64, 0), (2f64, 2), (8f64, 3)],
    );
}

#[test]
fn handles_zero_capacity() {
    let mut tree = KdTree::with_capacity(2, 0);
    assert_eq!(tree.add(&POINT_A.0, POINT_A.1), Err(ErrorKind::ZeroCapacity));
    assert_ordered_usize(tree.nearest(&POINT_A.0, 1, &squared_euclidean).unwrap(), &[]);
}

#[test]
fn handles_wrong_dimension() {
    let point = ([0f64], 0f64);
    let mut tree = KdTree::with_capacity(2, 1);

    assert_eq!(tree.add(&point.0, point.1), Err(ErrorKind::WrongDimension));
    assert_eq!(
        tree.nearest(&point.0, 1, &squared_euclidean),
        Err(ErrorKind::WrongDimension)
    );
}

#[test]
fn handles_non_finite_coordinate() {
    let point_a = ([f64::NAN, f64::NAN], 0f64);
    let point_b = ([f64::INFINITY, f64::INFINITY], 0f64);
    let mut tree = KdTree::with_capacity(2, 1);

    assert_eq!(tree.add(&point_a.0, point_a.1), Err(ErrorKind::NonFiniteCoordinate));
    assert_eq!(tree.add(&point_b.0, point_b.1), Err(ErrorKind::NonFiniteCoordinate));
    assert_eq!(
        tree.nearest(&point_a.0, 1, &squared_euclidean),
        Err(ErrorKind::NonFiniteCoordinate)
    );
    assert_eq!(
        tree.nearest(&point_b.0, 1, &squared_euclidean),
        Err(ErrorKind::NonFiniteCoordinate)
    );
}

#[test]
fn handles_singularity() {
    let mut tree = KdTree::with_capacity(2, 1);
    tree.add(&POINT_A.0, POINT_A.1).unwrap();
    tree.add(&POINT_A.0, POINT_A.1).unwrap();
    tree.add(&POINT_A.0, POINT_A.1).unwrap();
    tree.add(&POINT_B.0, POINT_B.1).unwrap();
    tree.add(&POINT_B.0, POINT_B.1).unwrap();
    tree.add(&POINT_B.0, POINT_B.1).unwrap();
    tree.add(&POINT_C.0, POINT_C.1).unwrap();
    tree.add(&POINT_C.0, POINT_C.1).unwrap();
    tree.add(&POINT_C.0, POINT_C.1).unwrap();
    assert_eq!(tree.size(), 9);
}

#[test]
fn handles_pending_order() {
    let item1 = ([0f64], 1);
    let item2 = ([100f64], 2);
    let item3 = ([45f64], 3);
    let item4 = ([55f64], 4);

    let mut tree = KdTree::with_capacity(1, 2);
    tree.add(&item1.0, item1.1).unwrap();
    tree.add(&item2.0, item2.1).unwrap();
    tree.add(&item3.0, item3.1).unwrap();
    tree.add(&item4.0, item4.1).unwrap();

    assert_ordered_usize(
        tree.nearest(&[50f64], 2, &squared_euclidean).unwrap(),
        &[(25f64, 3), (25f64, 4)],
    );
}
