extern crate kdtree;

use kdtree::ErrorKind;
use kdtree::KdTree;
use kdtree::distance::squared_euclidean;

static POINT_A: ([f64; 2], usize) = ([0f64, 0f64], 0);
static POINT_B: ([f64; 2], usize) = ([1f64, 1f64], 1);
static POINT_C: ([f64; 2], usize) = ([2f64, 2f64], 2);
static POINT_D: ([f64; 2], usize) = ([3f64, 3f64], 3);

#[test]
fn it_works() {
    let dimensions = 2;
    let capacity_per_node = 2;
    let mut kdtree = KdTree::with_capacity(dimensions, capacity_per_node);

    kdtree.add(&POINT_A.0, POINT_A.1).unwrap();
    kdtree.add(&POINT_B.0, POINT_B.1).unwrap();
    kdtree.add(&POINT_C.0, POINT_C.1).unwrap();
    kdtree.add(&POINT_D.0, POINT_D.1).unwrap();

    assert_eq!(kdtree.size(), 4);
    assert_eq!(kdtree.nearest(&POINT_A.0, 0, &squared_euclidean).unwrap(), vec![]);
    assert_eq!(
        kdtree.nearest(&POINT_A.0, 1, &squared_euclidean).unwrap(),
        vec![(0f64, &0)]
    );
    assert_eq!(
        kdtree.nearest(&POINT_A.0, 2, &squared_euclidean).unwrap(),
        vec![(0f64, &0), (2f64, &1)]
    );
    assert_eq!(
        kdtree.nearest(&POINT_A.0, 3, &squared_euclidean).unwrap(),
        vec![(0f64, &0), (2f64, &1), (8f64, &2)]
    );
    assert_eq!(
        kdtree.nearest(&POINT_A.0, 4, &squared_euclidean).unwrap(),
        vec![(0f64, &0), (2f64, &1), (8f64, &2), (18f64, &3)]
    );
    assert_eq!(
        kdtree.nearest(&POINT_A.0, 5, &squared_euclidean).unwrap(),
        vec![(0f64, &0), (2f64, &1), (8f64, &2), (18f64, &3)]
    );
    assert_eq!(
        kdtree.nearest(&POINT_B.0, 4, &squared_euclidean).unwrap(),
        vec![(0f64, &1), (2f64, &0), (2f64, &2), (8f64, &3)]
    );

    assert_eq!(
        kdtree.within(&POINT_A.0, 0.0, &squared_euclidean).unwrap(),
        vec![(0.0, &0)]
    );
    assert_eq!(
        kdtree.within(&POINT_B.0, 1.0, &squared_euclidean).unwrap(),
        vec![(0.0, &1)]
    );
    assert_eq!(
        kdtree.within(&POINT_B.0, 2.0, &squared_euclidean).unwrap(),
        vec![(0.0, &1), (2.0, &2), (2.0, &0)]
    );

    let unsorted1 = kdtree.within_unsorted(&POINT_A.0, 0.0, &squared_euclidean).unwrap();
    let ans1 = [(0.0, &0)];
    assert_eq!(unsorted1.len(), ans1.len());
    assert_eq!(
        kdtree.within_count(&POINT_A.0, 0.0, &squared_euclidean).unwrap(),
        ans1.len()
    );
    for item in unsorted1 {
        assert!(ans1.contains(&item));
    }

    let unsorted2 = kdtree.within_unsorted(&POINT_B.0, 1.0, &squared_euclidean).unwrap();
    let ans2 = [(0.0, &1)];
    assert_eq!(unsorted2.len(), ans2.len());
    assert_eq!(
        kdtree.within_count(&POINT_B.0, 1.0, &squared_euclidean).unwrap(),
        ans2.len()
    );
    for item in unsorted2 {
        assert!(ans2.contains(&item));
    }

    let unsorted3 = kdtree.within_unsorted(&POINT_B.0, 2.0, &squared_euclidean).unwrap();
    let ans3 = [(0.0, &1), (2.0, &2), (2.0, &0)];
    assert_eq!(unsorted3.len(), ans3.len());
    assert_eq!(
        kdtree.within_count(&POINT_B.0, 2.0, &squared_euclidean).unwrap(),
        ans3.len()
    );
    for item in unsorted3 {
        assert!(ans3.contains(&item));
    }

    assert_eq!(
        kdtree
            .iter_nearest(&POINT_A.0, &squared_euclidean)
            .unwrap()
            .collect::<Vec<_>>(),
        vec![(0f64, &0), (2f64, &1), (8f64, &2), (18f64, &3)]
    );

    let iter = kdtree
        .iter_nearest_mut(&POINT_A.0, &squared_euclidean)
        .unwrap()
        .next()
        .unwrap();
    *iter.1 = 10;

    assert_eq!(
        kdtree
            .iter_nearest(&POINT_A.0, &squared_euclidean)
            .unwrap()
            .collect::<Vec<_>>(),
        vec![(0f64, &10), (2f64, &1), (8f64, &2), (18f64, &3)]
    );
}

#[test]
fn it_works_with_vec() {
    let dimensions = 2;
    let capacity_per_node = 2;
    let mut kdtree = KdTree::with_capacity(dimensions, capacity_per_node);

    kdtree.add(vec![0.0; 2], 0).unwrap();
    kdtree.add(vec![1.0; 2], 1).unwrap();
    kdtree.add(vec![2.0; 2], 2).unwrap();
    kdtree.add(vec![3.0; 2], 3).unwrap();

    assert_eq!(kdtree.size(), 4);
    assert_eq!(kdtree.nearest(&POINT_A.0, 0, &squared_euclidean).unwrap(), vec![]);
    assert_eq!(
        kdtree.nearest(&POINT_A.0, 1, &squared_euclidean).unwrap(),
        vec![(0f64, &0)]
    );
    assert_eq!(
        kdtree.nearest(&POINT_A.0, 2, &squared_euclidean).unwrap(),
        vec![(0f64, &0), (2f64, &1)]
    );
    assert_eq!(
        kdtree.nearest(&POINT_A.0, 3, &squared_euclidean).unwrap(),
        vec![(0f64, &0), (2f64, &1), (8f64, &2)]
    );
    assert_eq!(
        kdtree.nearest(&POINT_A.0, 4, &squared_euclidean).unwrap(),
        vec![(0f64, &0), (2f64, &1), (8f64, &2), (18f64, &3)]
    );
    assert_eq!(
        kdtree.nearest(&POINT_A.0, 5, &squared_euclidean).unwrap(),
        vec![(0f64, &0), (2f64, &1), (8f64, &2), (18f64, &3)]
    );
    assert_eq!(
        kdtree.nearest(&POINT_B.0, 4, &squared_euclidean).unwrap(),
        vec![(0f64, &1), (2f64, &0), (2f64, &2), (8f64, &3)]
    );
}

#[test]
fn handles_zero_capacity() {
    let mut kdtree = KdTree::with_capacity(2, 0);

    assert_eq!(kdtree.add(&POINT_A.0, POINT_A.1), Err(ErrorKind::ZeroCapacity));
    assert_eq!(kdtree.nearest(&POINT_A.0, 1, &squared_euclidean).unwrap(), vec![]);
}

#[test]
fn handles_wrong_dimension() {
    let point = ([0f64], 0f64);
    let mut kdtree = KdTree::with_capacity(2, 1);

    assert_eq!(kdtree.add(&point.0, point.1), Err(ErrorKind::WrongDimension));
    assert_eq!(
        kdtree.nearest(&point.0, 1, &squared_euclidean),
        Err(ErrorKind::WrongDimension)
    );
}

#[test]
fn handles_non_finite_coordinate() {
    let point_a = ([f64::NAN, f64::NAN], 0f64);
    let point_b = ([f64::INFINITY, f64::INFINITY], 0f64);
    let mut kdtree = KdTree::with_capacity(2, 1);

    assert_eq!(kdtree.add(&point_a.0, point_a.1), Err(ErrorKind::NonFiniteCoordinate));
    assert_eq!(kdtree.add(&point_b.0, point_b.1), Err(ErrorKind::NonFiniteCoordinate));
    assert_eq!(
        kdtree.nearest(&point_a.0, 1, &squared_euclidean),
        Err(ErrorKind::NonFiniteCoordinate)
    );
    assert_eq!(
        kdtree.nearest(&point_b.0, 1, &squared_euclidean),
        Err(ErrorKind::NonFiniteCoordinate)
    );
}

#[test]
fn handles_overflowed_distances() {
    let point_a = ([1e200, 1e200], 1f64);
    let point_b = ([1e200 + 1.0, 1e200 + 1.0], 2f64);
    let query_point = ([0.0, 0.0], 0f64);

    let mut kdtree = KdTree::with_capacity(2, 2);
    kdtree.add(&point_a.0, point_a.1).unwrap();
    kdtree.add(&point_b.0, point_b.1).unwrap();

    let results = kdtree.nearest(&query_point.0, 2, &squared_euclidean).unwrap();
    assert_eq!(results.len(), 2);

    let values: Vec<_> = results.iter().map(|&(_, &v)| v).collect();
    assert!(values.contains(&1f64));
    assert!(values.contains(&2f64));
}

#[test]
fn handles_singularity() {
    let mut kdtree = KdTree::with_capacity(2, 1);
    kdtree.add(&POINT_A.0, POINT_A.1).unwrap();
    kdtree.add(&POINT_A.0, POINT_A.1).unwrap();
    kdtree.add(&POINT_A.0, POINT_A.1).unwrap();
    kdtree.add(&POINT_B.0, POINT_B.1).unwrap();
    kdtree.add(&POINT_B.0, POINT_B.1).unwrap();
    kdtree.add(&POINT_B.0, POINT_B.1).unwrap();
    kdtree.add(&POINT_C.0, POINT_C.1).unwrap();
    kdtree.add(&POINT_C.0, POINT_C.1).unwrap();
    kdtree.add(&POINT_C.0, POINT_C.1).unwrap();
    assert_eq!(kdtree.size(), 9);
}

#[test]
fn handles_pending_order() {
    let item1 = ([0f64], 1);
    let item2 = ([100f64], 2);
    let item3 = ([45f64], 3);
    let item4 = ([55f64], 4);

    // Build a kd tree
    let dimensions = 1;
    let capacity_per_node = 2;
    let mut kdtree = KdTree::with_capacity(dimensions, capacity_per_node);

    kdtree.add(&item1.0, item1.1).unwrap();
    kdtree.add(&item2.0, item2.1).unwrap();
    kdtree.add(&item3.0, item3.1).unwrap();
    kdtree.add(&item4.0, item4.1).unwrap();
    assert_eq!(
        kdtree.nearest(&[51f64], 2, &squared_euclidean).unwrap(),
        vec![(16.0, &4), (36.0, &3)]
    );
    assert_eq!(
        kdtree.nearest(&[51f64], 4, &squared_euclidean).unwrap(),
        vec![(16.0, &4), (36.0, &3), (2401.0, &2), (2601.0, &1)]
    );
    assert_eq!(
        kdtree.nearest(&[49f64], 2, &squared_euclidean).unwrap(),
        vec![(16.0, &3), (36.0, &4)]
    );
    assert_eq!(
        kdtree.nearest(&[49f64], 4, &squared_euclidean).unwrap(),
        vec![(16.0, &3), (36.0, &4), (2401.0, &1), (2601.0, &2)]
    );

    assert_eq!(
        kdtree
            .iter_nearest(&[49f64], &squared_euclidean)
            .unwrap()
            .collect::<Vec<_>>(),
        vec![(16.0, &3), (36.0, &4), (2401.0, &1), (2601.0, &2)]
    );
    assert_eq!(
        kdtree
            .iter_nearest(&[51f64], &squared_euclidean)
            .unwrap()
            .collect::<Vec<_>>(),
        vec![(16.0, &4), (36.0, &3), (2401.0, &2), (2601.0, &1)]
    );

    assert_eq!(kdtree.within(&[50f64], 1.0, &squared_euclidean).unwrap(), vec![]);
    assert_eq!(
        kdtree.within(&[50f64], 25.0, &squared_euclidean).unwrap(),
        vec![(25.0, &3), (25.0, &4)]
    );
    assert_eq!(
        kdtree.within(&[50f64], 30.0, &squared_euclidean).unwrap(),
        vec![(25.0, &3), (25.0, &4)]
    );
    assert_eq!(
        kdtree.within(&[55f64], 5.0, &squared_euclidean).unwrap(),
        vec![(0.0, &4)]
    );
    assert_eq!(
        kdtree.within(&[56f64], 5.0, &squared_euclidean).unwrap(),
        vec![(1.0, &4)]
    );
}

#[test]
fn handles_drops_correctly() {
    use std::ops::Drop;
    use std::sync::{Arc, Mutex};

    // Mock up a structure to keep track of Drops
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
        // Build a kd tree
        let dimensions = 2;
        let capacity_per_node = 1;
        let mut kdtree = KdTree::with_capacity(dimensions, capacity_per_node);

        kdtree.add(&item1.0, item1.1).unwrap();
        kdtree.add(&item2.0, item2.1).unwrap();
        kdtree.add(&item3.0, item3.1).unwrap();
        kdtree.add(&item4.0, item4.1).unwrap();

        // Pre-drop check
        assert_eq!(*drop_counter.lock().unwrap(), 0);
    }

    // Post-drop check
    assert_eq!(*drop_counter.lock().unwrap(), 4);
}

#[test]
fn handles_remove_correctly() {
    let item1 = ([0f64], 1);
    let item2 = ([100f64], 2);
    let item3 = ([45f64], 3);
    let item4 = ([55f64], 4);

    // Build a kd tree
    let dimensions = 1;
    let capacity_per_node = 2;
    let mut kdtree = KdTree::with_capacity(dimensions, capacity_per_node);

    kdtree.add(&item1.0, item1.1).unwrap();
    kdtree.add(&item2.0, item2.1).unwrap();
    kdtree.add(&item3.0, item3.1).unwrap();
    kdtree.add(&item4.0, item4.1).unwrap();

    let num_removed = kdtree.remove(&&item3.0, &item3.1).unwrap();
    assert_eq!(kdtree.size(), 3);
    assert_eq!(num_removed, 1);
    assert_eq!(
        kdtree.nearest(&[51f64], 2, &squared_euclidean).unwrap(),
        vec![(16.0, &4), (2401.0, &2)]
    );
}

#[test]
fn handles_remove_multiple_match() {
    let item1 = ([0f64], 1);
    let item2 = ([0f64], 1);
    let item3 = ([100f64], 2);
    let item4 = ([45f64], 3);

    // Build a kd tree
    let dimensions = 1;
    let capacity_per_node = 2;
    let mut kdtree = KdTree::with_capacity(dimensions, capacity_per_node);

    kdtree.add(&item1.0, item1.1).unwrap();
    kdtree.add(&item2.0, item2.1).unwrap();
    kdtree.add(&item3.0, item3.1).unwrap();
    kdtree.add(&item4.0, item4.1).unwrap();

    assert_eq!(kdtree.size(), 4);
    let num_removed = kdtree.remove(&&[0f64], &1).unwrap();
    assert_eq!(kdtree.size(), 2);
    assert_eq!(num_removed, 2);
    assert_eq!(
        kdtree.nearest(&[45f64], 1, &squared_euclidean).unwrap(),
        vec![(0.0, &3)]
    );
}

#[test]
fn handles_remove_no_match() {
    let item1 = ([0f64], 1);
    let item2 = ([100f64], 2);
    let item3 = ([45f64], 3);
    let item4 = ([55f64], 4);

    // Build a kd tree
    let dimensions = 1;
    let capacity_per_node = 2;
    let mut kdtree = KdTree::with_capacity(dimensions, capacity_per_node);

    kdtree.add(&item1.0, item1.1).unwrap();
    kdtree.add(&item2.0, item2.1).unwrap();
    kdtree.add(&item3.0, item3.1).unwrap();
    kdtree.add(&item4.0, item4.1).unwrap();

    let num_removed = kdtree.remove(&&[1f64], &2).unwrap();
    assert_eq!(kdtree.size(), 4);
    assert_eq!(num_removed, 0);
    assert_eq!(
        kdtree.nearest(&[51f64], 2, &squared_euclidean).unwrap(),
        vec![(16.0, &4), (36.0, &3)]
    );
}

#[test]
fn handles_remove_overlapping_points() {
    let a = ([0f64, 0f64], 0);
    let b = ([0f64, 0f64], 1);
    let mut kdtree = KdTree::new(2);

    kdtree.add(a.0, a.1).unwrap();
    kdtree.add(b.0, b.1).unwrap();

    let num_removed = kdtree.remove(&[0f64, 0f64], &1).unwrap();
    assert_eq!(kdtree.size(), 1);
    assert_eq!(num_removed, 1);
    assert_eq!(
        kdtree.nearest(&[0f64, 0f64], 1, &squared_euclidean).unwrap(),
        vec![(0.0, &0)]
    );
}
