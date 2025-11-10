mod __util__;

use __util__::{POINT_A, POINT_B, basic_tree};
use kdtree::distance::squared_euclidean;
use std::sync::atomic::{AtomicUsize, Ordering};

fn countered_distance<'a>(counter: &'a AtomicUsize) -> impl Fn(&[f64], &[f64]) -> f64 + 'a {
    move |a, b| {
        counter.fetch_add(1, Ordering::SeqCst);
        squared_euclidean(a, b)
    }
}

fn reset(counter: &AtomicUsize) -> usize {
    counter.swap(0, Ordering::SeqCst)
}

#[test]
fn nearest_queries_limit_distance_evaluations() {
    let tree = basic_tree();
    let counter = AtomicUsize::new(0);
    let distance = countered_distance(&counter);

    tree.nearest(&POINT_A.0, 0, &distance).unwrap();
    assert_eq!(reset(&counter), 0);

    tree.nearest(&POINT_A.0, 1, &distance).unwrap();
    assert_eq!(reset(&counter), 2);

    tree.nearest(&POINT_A.0, 2, &distance).unwrap();
    assert_eq!(reset(&counter), 4);

    tree.nearest(&POINT_A.0, 3, &distance).unwrap();
    assert_eq!(reset(&counter), 6);

    tree.nearest(&POINT_A.0, 4, &distance).unwrap();
    assert_eq!(reset(&counter), 6);

    tree.nearest(&POINT_A.0, 5, &distance).unwrap();
    assert_eq!(reset(&counter), 6);

    tree.nearest(&POINT_B.0, 4, &distance).unwrap();
    assert_eq!(reset(&counter), 6);
}

#[test]
fn nearest_within_radius_limits_distance_evaluations() {
    let tree = basic_tree();
    let counter = AtomicUsize::new(0);
    let distance = countered_distance(&counter);

    tree.nearest_within_radius(&POINT_A.0, 4, None, &distance).unwrap();
    assert_eq!(reset(&counter), 6);

    tree.nearest_within_radius(&POINT_A.0, 4, Some(0.0), &distance).unwrap();
    assert_eq!(reset(&counter), 2);

    tree.nearest_within_radius(&POINT_B.0, 4, Some(1.0), &distance).unwrap();
    assert_eq!(reset(&counter), 3);
}

#[test]
fn within_queries_limit_distance_evaluations() {
    let tree = basic_tree();
    let counter = AtomicUsize::new(0);
    let distance = countered_distance(&counter);

    tree.within(&POINT_A.0, 0.0, &distance).unwrap();
    assert_eq!(reset(&counter), 2);

    tree.within(&POINT_B.0, 1.0, &distance).unwrap();
    assert_eq!(reset(&counter), 3);

    tree.within(&POINT_B.0, 2.0, &distance).unwrap();
    assert_eq!(reset(&counter), 6);
}

#[test]
fn iter_nearest_limits_distance_evaluations() {
    let counter = AtomicUsize::new(0);
    let distance = countered_distance(&counter);
    let tree = basic_tree();
    let mut iter = tree.iter_nearest(&POINT_A.0, &distance).unwrap();

    assert_eq!(reset(&counter), 0);
    iter.next().unwrap();
    assert_eq!(reset(&counter), 2);
    iter.next().unwrap();
    assert_eq!(reset(&counter), 2);
    iter.next().unwrap();
    assert_eq!(reset(&counter), 2);
    iter.next().unwrap();
    assert_eq!(reset(&counter), 0);
}
