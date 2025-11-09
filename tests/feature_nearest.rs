mod _util;

use _util::assertions::assert_ordered_usize;
use _util::fixtures::{POINT_A, POINT_B, POINT_C, basic_tree};
use kdtree::distance::squared_euclidean;
use kdtree::{ErrorKind, KdTree};

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
        tree.nearest(&[51f64], 2, &squared_euclidean).unwrap(),
        &[(16.0, 4), (36.0, 3)],
    );
    assert_ordered_usize(
        tree.nearest(&[51f64], 4, &squared_euclidean).unwrap(),
        &[(16.0, 4), (36.0, 3), (2401.0, 2), (2601.0, 1)],
    );
    assert_ordered_usize(
        tree.nearest(&[49f64], 2, &squared_euclidean).unwrap(),
        &[(16.0, 3), (36.0, 4)],
    );
    assert_ordered_usize(
        tree.nearest(&[49f64], 4, &squared_euclidean).unwrap(),
        &[(16.0, 3), (36.0, 4), (2401.0, 1), (2601.0, 2)],
    );

    let collected: Vec<_> = tree.iter_nearest(&[49f64], &squared_euclidean).unwrap().collect();
    assert_ordered_usize(collected, &[(16.0, 3), (36.0, 4), (2401.0, 1), (2601.0, 2)]);
    let collected: Vec<_> = tree.iter_nearest(&[51f64], &squared_euclidean).unwrap().collect();
    assert_ordered_usize(collected, &[(16.0, 4), (36.0, 3), (2401.0, 2), (2601.0, 1)]);
}

#[test]
fn handles_drops_correctly() {
    use std::ops::Drop;
    use std::sync::{Arc, Mutex};

    struct Test(Arc<Mutex<i32>>);

    impl PartialEq<Test> for Test {
        fn eq(&self, other: &Test) -> bool {
            *self.0.lock().unwrap() == *other.0.lock().unwrap()
        }
    }

    impl Drop for Test {
        fn drop(&mut self) {
            let mut drop_counter = self.0.lock().unwrap();
            *drop_counter += 1;
        }
    }

    let drop_counter = Arc::new(Mutex::new(0));

    let item1 = ([0f64, 0f64], Test(drop_counter.clone()));
    let item2 = ([1f64, 1f64], Test(drop_counter.clone()));
    let item3 = ([2f64, 2f64], Test(drop_counter.clone()));
    let item4 = ([3f64, 3f64], Test(drop_counter.clone()));

    {
        let mut tree = KdTree::with_capacity(2, 1);
        tree.add(&item1.0, item1.1).unwrap();
        tree.add(&item2.0, item2.1).unwrap();
        tree.add(&item3.0, item3.1).unwrap();
        tree.add(&item4.0, item4.1).unwrap();
        assert_eq!(*drop_counter.lock().unwrap(), 0);
    }

    assert_eq!(*drop_counter.lock().unwrap(), 4);
}

#[test]
fn handles_remove_correctly() {
    let item1 = ([0f64], 1);
    let item2 = ([100f64], 2);
    let item3 = ([45f64], 3);
    let item4 = ([55f64], 4);

    let mut tree = KdTree::with_capacity(1, 2);
    tree.add(&item1.0, item1.1).unwrap();
    tree.add(&item2.0, item2.1).unwrap();
    tree.add(&item3.0, item3.1).unwrap();
    tree.add(&item4.0, item4.1).unwrap();

    let removed = tree.remove(&&item3.0, &item3.1).unwrap();
    assert_eq!(tree.size(), 3);
    assert_eq!(removed, 1);
    assert_ordered_usize(
        tree.nearest(&[51f64], 2, &squared_euclidean).unwrap(),
        &[(16.0, 4), (2401.0, 2)],
    );
}

#[test]
fn handles_remove_multiple_match() {
    let item1 = ([0f64], 1);
    let item2 = ([0f64], 1);
    let item3 = ([100f64], 2);
    let item4 = ([45f64], 3);

    let mut tree = KdTree::with_capacity(1, 2);
    tree.add(&item1.0, item1.1).unwrap();
    tree.add(&item2.0, item2.1).unwrap();
    tree.add(&item3.0, item3.1).unwrap();
    tree.add(&item4.0, item4.1).unwrap();

    assert_eq!(tree.size(), 4);
    let removed = tree.remove(&&[0f64], &1).unwrap();
    assert_eq!(tree.size(), 2);
    assert_eq!(removed, 2);
    assert_ordered_usize(tree.nearest(&[45f64], 1, &squared_euclidean).unwrap(), &[(0.0, 3)]);
}

#[test]
fn handles_remove_no_match() {
    let item1 = ([0f64], 1);
    let item2 = ([100f64], 2);
    let item3 = ([45f64], 3);
    let item4 = ([55f64], 4);

    let mut tree = KdTree::with_capacity(1, 2);
    tree.add(&item1.0, item1.1).unwrap();
    tree.add(&item2.0, item2.1).unwrap();
    tree.add(&item3.0, item3.1).unwrap();
    tree.add(&item4.0, item4.1).unwrap();

    let removed = tree.remove(&&[1f64], &2).unwrap();
    assert_eq!(tree.size(), 4);
    assert_eq!(removed, 0);
    assert_ordered_usize(
        tree.nearest(&[51f64], 2, &squared_euclidean).unwrap(),
        &[(16.0, 4), (36.0, 3)],
    );
}

#[test]
fn handles_remove_overlapping_points() {
    let a = ([0f64, 0f64], 0);
    let b = ([0f64, 0f64], 1);
    let mut tree = KdTree::new(2);

    tree.add(a.0, a.1).unwrap();
    tree.add(b.0, b.1).unwrap();

    let removed = tree.remove(&[0f64, 0f64], &1).unwrap();
    assert_eq!(tree.size(), 1);
    assert_eq!(removed, 1);
    assert_ordered_usize(tree.nearest(&[0f64, 0f64], 1, &squared_euclidean).unwrap(), &[(0.0, 0)]);
}
