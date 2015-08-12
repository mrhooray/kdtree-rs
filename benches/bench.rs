#![feature(test)]
extern crate kdtree;
extern crate test;
extern crate rand;

use test::Bencher;
use kdtree::KdTree;
use kdtree::distance::square_euclidean;

fn rand_f64() -> f64 {
    rand::random::<f64>()
}

#[bench]
fn bench_add(b: &mut Bencher) {
    let len = 1000u32;
    let point = ([rand_f64(), rand_f64(), rand_f64()], rand_f64());
    let mut points = vec![];
    let mut kd_node = KdTree::<f64>::new_with_capacity(3, 16);
    for _ in 0..len {
        points.push(([rand_f64(), rand_f64(), rand_f64()], rand_f64()));
    }
    let points = &points[..];
    for i in 0..points.len() {
        kd_node.add(&points[i].0, &points[i].1).unwrap();
    }
    b.iter(|| kd_node.add(&point.0, &point.1).unwrap());
}

#[bench]
fn bench_nearest(b: &mut Bencher) {
    let len = 1000u32;
    let point = ([rand_f64(), rand_f64(), rand_f64()], rand_f64());
    let mut points = vec![];
    let mut kd_node = KdTree::<f64>::new_with_capacity(3, 16);
    for _ in 0..len {
        points.push(([rand_f64(), rand_f64(), rand_f64()], rand_f64()));
    }
    let points = &points[..];
    for i in 0..points.len() {
        kd_node.add(&points[i].0, &points[i].1).unwrap();
    }
    b.iter(|| kd_node.nearest(&point.0, 8, &square_euclidean).unwrap());
}
