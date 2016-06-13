#![feature(test)]
extern crate kdtree;
extern crate test;
extern crate rand;

use test::Bencher;
use kdtree::KdTree;
use kdtree::distance::squared_euclidean;

fn rand_f64() -> f64 {
    rand::random::<f64>()
}

#[bench]
fn bench_add_to_kdtree_with_1k_3d_points(b: &mut Bencher) {
    let len = 1000usize;
    let point = ([rand_f64(), rand_f64(), rand_f64()], rand_f64());
    let mut points = vec![];
    let mut kdtree = KdTree::new_with_capacity(3, 16);
    for _ in 0..len {
        points.push(([rand_f64(), rand_f64(), rand_f64()], rand_f64()));
    }
    for i in 0..points.len() {
        kdtree.add(&points[i].0, points[i].1).unwrap();
    }
    b.iter(|| kdtree.add(&point.0, point.1).unwrap());
}

#[bench]
fn bench_nearest_from_kdtree_with_1k_3d_points(b: &mut Bencher) {
    let len = 1000usize;
    let point = ([rand_f64(), rand_f64(), rand_f64()], rand_f64());
    let mut points = vec![];
    let mut kdtree = KdTree::new_with_capacity(3, 16);
    for _ in 0..len {
        points.push(([rand_f64(), rand_f64(), rand_f64()], rand_f64()));
    }
    for i in 0..points.len() {
        kdtree.add(&points[i].0, points[i].1).unwrap();
    }
    b.iter(|| kdtree.nearest(&point.0, 8, &squared_euclidean).unwrap());
}
