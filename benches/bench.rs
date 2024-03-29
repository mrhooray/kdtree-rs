#![feature(test)]
extern crate kdtree;
extern crate rand;
extern crate test;

use kdtree::distance::squared_euclidean;
use kdtree::KdTree;
use test::Bencher;

fn rand_data() -> ([f64; 3], f64) {
    rand::random()
}

#[bench]
fn bench_add_to_kdtree_with_1k_3d_points(b: &mut Bencher) {
    let len = 1000usize;
    let point = rand_data();
    let mut points = vec![];
    let mut kdtree = KdTree::with_capacity(3, 16);
    for _ in 0..len {
        points.push(rand_data());
    }
    for i in 0..points.len() {
        kdtree.add(&points[i].0, points[i].1).unwrap();
    }
    b.iter(|| kdtree.add(&point.0, point.1).unwrap());
}

#[bench]
fn bench_nearest_from_kdtree_with_1k_3d_points(b: &mut Bencher) {
    let len = 1000usize;
    let point = rand_data();
    let mut points = vec![];
    let mut kdtree = KdTree::with_capacity(3, 16);
    for _ in 0..len {
        points.push(rand_data());
    }
    for i in 0..points.len() {
        kdtree.add(&points[i].0, points[i].1).unwrap();
    }
    b.iter(|| kdtree.nearest(&point.0, 8, &squared_euclidean).unwrap());
}

#[bench]
fn bench_within_2k_data_01_radius(b: &mut Bencher) {
    let len = 2000usize;
    let point = rand_data();
    let mut points = vec![];
    let mut kdtree = KdTree::with_capacity(3, 16);
    for _ in 0..len {
        points.push(rand_data());
    }
    for i in 0..points.len() {
        kdtree.add(&points[i].0, points[i].1).unwrap();
    }

    b.iter(|| kdtree.within(&point.0, 0.1, &squared_euclidean).unwrap());
}

#[bench]
fn bench_within_2k_data_02_radius(b: &mut Bencher) {
    let len = 2000usize;
    let point = rand_data();
    let mut points = vec![];
    let mut kdtree = KdTree::with_capacity(3, 16);
    for _ in 0..len {
        points.push(rand_data());
    }
    for i in 0..points.len() {
        kdtree.add(&points[i].0, points[i].1).unwrap();
    }

    b.iter(|| kdtree.within(&point.0, 0.2, &squared_euclidean).unwrap());
}

#[bench]
fn bench_within_unsorted_2k_data_01_radius(b: &mut Bencher) {
    let len = 2000usize;
    let point = rand_data();
    let mut points = vec![];
    let mut kdtree = KdTree::with_capacity(3, 16);
    for _ in 0..len {
        points.push(rand_data());
    }
    for i in 0..points.len() {
        kdtree.add(&points[i].0, points[i].1).unwrap();
    }

    b.iter(|| kdtree.within_unsorted(&point.0, 0.1, &squared_euclidean).unwrap());
}

#[bench]
fn bench_within_unsorted_2k_data_02_radius(b: &mut Bencher) {
    let len = 2000usize;
    let point = rand_data();
    let mut points = vec![];
    let mut kdtree = KdTree::with_capacity(3, 16);
    for _ in 0..len {
        points.push(rand_data());
    }
    for i in 0..points.len() {
        kdtree.add(&points[i].0, points[i].1).unwrap();
    }

    b.iter(|| kdtree.within_unsorted(&point.0, 0.2, &squared_euclidean).unwrap());
}

#[bench]
fn bench_within_count_2k_data_01_radius(b: &mut Bencher) {
    let len = 2000usize;
    let point = rand_data();
    let mut points = vec![];
    let mut kdtree = KdTree::with_capacity(3, 16);
    for _ in 0..len {
        points.push(rand_data());
    }
    for i in 0..points.len() {
        kdtree.add(&points[i].0, points[i].1).unwrap();
    }

    b.iter(|| kdtree.within_count(&point.0, 0.1, &squared_euclidean).unwrap());
}

#[bench]
fn bench_within_count_2k_data_02_radius(b: &mut Bencher) {
    let len = 2000usize;
    let point = rand_data();
    let mut points = vec![];
    let mut kdtree = KdTree::with_capacity(3, 16);
    for _ in 0..len {
        points.push(rand_data());
    }
    for i in 0..points.len() {
        kdtree.add(&points[i].0, points[i].1).unwrap();
    }

    b.iter(|| kdtree.within_count(&point.0, 0.2, &squared_euclidean).unwrap());
}
