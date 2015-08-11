extern crate kdtree;

use kdtree::KdNode;
use kdtree::ErrorKind;
use kdtree::distance::square_euclidean;

static POINT_A: ([f64; 2], f64) = ([0f64, 0f64], 0f64);
static POINT_B: ([f64; 2], f64) = ([1f64, 1f64], 1f64);
static POINT_C: ([f64; 2], f64) = ([2f64, 2f64], 2f64);
static POINT_D: ([f64; 2], f64) = ([3f64, 3f64], 3f64);

#[test]
fn it_works() {
    let mut kd_node = KdNode::new_with_capacity(2, 2);
    kd_node.add(&POINT_A.0, &POINT_A.1).unwrap();
    kd_node.add(&POINT_B.0, &POINT_B.1).unwrap();
    kd_node.add(&POINT_C.0, &POINT_C.1).unwrap();
    kd_node.add(&POINT_D.0, &POINT_D.1).unwrap();
    assert_eq!(kd_node.len(), 4);
    assert_eq!(kd_node.nearest(&POINT_A.0, 0, &square_euclidean).unwrap(), vec![]);
    assert_eq!(kd_node.nearest(&POINT_A.0, 1, &square_euclidean).unwrap(), vec![(0f64, &0f64)]);
    assert_eq!(kd_node.nearest(&POINT_A.0, 2, &square_euclidean).unwrap(), vec![(0f64, &0f64), (2f64, &1f64)]);
    assert_eq!(kd_node.nearest(&POINT_A.0, 3, &square_euclidean).unwrap(), vec![(0f64, &0f64), (2f64, &1f64), (8f64, &2f64)]);
    assert_eq!(kd_node.nearest(&POINT_A.0, 4, &square_euclidean).unwrap(), vec![(0f64, &0f64), (2f64, &1f64), (8f64, &2f64), (18f64, &3f64)]);
    assert_eq!(kd_node.nearest(&POINT_A.0, 5, &square_euclidean).unwrap(), vec![(0f64, &0f64), (2f64, &1f64), (8f64, &2f64), (18f64, &3f64)]);
    assert_eq!(kd_node.nearest(&POINT_B.0, 4, &square_euclidean).unwrap(), vec![(0f64, &1f64), (2f64, &0f64), (2f64, &2f64), (8f64, &3f64)]);
}

#[test]
fn handles_zero_capacity() {
    let mut kd_node = KdNode::new_with_capacity(2, 0);
    assert_eq!(kd_node.add(&POINT_A.0, &POINT_A.1), Err(ErrorKind::ZeroCapacity));
    assert_eq!(kd_node.nearest(&POINT_A.0, 1, &square_euclidean).unwrap(), vec![]);
}

#[test]
fn handles_wrong_dimension() {
    let point = ([0f64], 0f64);
    let mut kd_node = KdNode::new_with_capacity(2, 1);
    assert_eq!(kd_node.add(&point.0, &point.1), Err(ErrorKind::WrongDimension));
    assert_eq!(kd_node.nearest(&point.0, 1, &square_euclidean), Err(ErrorKind::WrongDimension));
}

#[test]
fn handles_non_finite_coordinate() {
    let point_a = ([std::f64::NAN, std::f64::NAN], 0f64);
    let point_b = ([std::f64::INFINITY, std::f64::INFINITY], 0f64);
    let mut kd_node = KdNode::new_with_capacity(2, 1);
    assert_eq!(kd_node.add(&point_a.0, &point_a.1), Err(ErrorKind::NonFiniteCoordinate));
    assert_eq!(kd_node.add(&point_b.0, &point_b.1), Err(ErrorKind::NonFiniteCoordinate));
    assert_eq!(kd_node.nearest(&point_a.0, 1, &square_euclidean), Err(ErrorKind::NonFiniteCoordinate));
    assert_eq!(kd_node.nearest(&point_b.0, 1, &square_euclidean), Err(ErrorKind::NonFiniteCoordinate));
}

#[test]
fn handles_singularity() {
    let mut kd_node = KdNode::new_with_capacity(2, 1);
    kd_node.add(&POINT_A.0, &POINT_A.1).unwrap();
    kd_node.add(&POINT_A.0, &POINT_A.1).unwrap();
    kd_node.add(&POINT_A.0, &POINT_A.1).unwrap();
    kd_node.add(&POINT_B.0, &POINT_B.1).unwrap();
    kd_node.add(&POINT_B.0, &POINT_B.1).unwrap();
    kd_node.add(&POINT_B.0, &POINT_B.1).unwrap();
    kd_node.add(&POINT_C.0, &POINT_C.1).unwrap();
    kd_node.add(&POINT_C.0, &POINT_C.1).unwrap();
    kd_node.add(&POINT_C.0, &POINT_C.1).unwrap();
    assert_eq!(kd_node.len(), 9);
}
