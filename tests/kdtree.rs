extern crate kdtree;

use kdtree::KdTree;
use kdtree::ErrorKind;
use kdtree::distance::squared_euclidean;

static POINT_A: ([f64; 2], usize) = ([0f64, 0f64], 0);
static POINT_B: ([f64; 2], usize) = ([1f64, 1f64], 1);
static POINT_C: ([f64; 2], usize) = ([2f64, 2f64], 2);
static POINT_D: ([f64; 2], usize) = ([3f64, 3f64], 3);

#[test]
fn it_works() {
    let dimensions = 2;
    let capacity_per_node = 2;
    let mut kdtree = KdTree::new_with_capacity(dimensions, capacity_per_node);

    kdtree.add(&POINT_A.0, POINT_A.1).unwrap();
    kdtree.add(&POINT_B.0, POINT_B.1).unwrap();
    kdtree.add(&POINT_C.0, POINT_C.1).unwrap();
    kdtree.add(&POINT_D.0, POINT_D.1).unwrap();

    assert_eq!(kdtree.size(), 4);
    assert_eq!(
        kdtree.nearest(&POINT_A.0, 0, &squared_euclidean).unwrap(),
        vec![]
    );
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
    let mut kdtree = KdTree::new_with_capacity(2, 0);
    assert_eq!(kdtree.add(&POINT_A.0, POINT_A.1), Err(ErrorKind::ZeroCapacity));
    assert_eq!(kdtree.nearest(&POINT_A.0, 1, &squared_euclidean).unwrap(), vec![]);
}

#[test]
fn handles_wrong_dimension() {
    let point = ([0f64], 0f64);
    let mut kdtree = KdTree::new_with_capacity(2, 1);
    assert_eq!(kdtree.add(&point.0, point.1), Err(ErrorKind::WrongDimension));
    assert_eq!(kdtree.nearest(&point.0, 1, &squared_euclidean), Err(ErrorKind::WrongDimension));
}

#[test]
fn handles_non_finite_coordinate() {
    let point_a = ([std::f64::NAN, std::f64::NAN], 0f64);
    let point_b = ([std::f64::INFINITY, std::f64::INFINITY], 0f64);
    let mut kdtree = KdTree::new_with_capacity(2, 1);
    assert_eq!(kdtree.add(&point_a.0, point_a.1), Err(ErrorKind::NonFiniteCoordinate));
    assert_eq!(kdtree.add(&point_b.0, point_b.1), Err(ErrorKind::NonFiniteCoordinate));
    assert_eq!(kdtree.nearest(&point_a.0, 1, &squared_euclidean), Err(ErrorKind::NonFiniteCoordinate));
    assert_eq!(kdtree.nearest(&point_b.0, 1, &squared_euclidean), Err(ErrorKind::NonFiniteCoordinate));
}

#[test]
fn handles_singularity() {
    let mut kdtree = KdTree::new_with_capacity(2, 1);
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
