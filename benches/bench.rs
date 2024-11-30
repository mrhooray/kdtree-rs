extern crate criterion;
extern crate kdtree;
extern crate rand;

use criterion::{criterion_group, criterion_main, Criterion};
use kdtree::distance::squared_euclidean;
use kdtree::KdTree;

fn rand_data() -> ([f64; 3], f64) {
    rand::random()
}

fn bench_add_to_kdtree_with_1k_3d_points(c: &mut Criterion) {
    let len = 1000usize;
    let point = rand_data();
    let mut points = vec![];
    let mut kdtree = KdTree::with_capacity(3, 16);
    for _ in 0..len {
        points.push(rand_data());
    }
    for point in points.iter() {
        kdtree.add(&point.0, point.1).unwrap();
    }
    c.bench_function("bench_add_to_kdtree_with_1k_3d_points", |b| {
        b.iter(|| kdtree.add(&point.0, point.1).unwrap());
    });
}

fn bench_nearest_from_kdtree_with_1k_3d_points(c: &mut Criterion) {
    let len = 1000usize;
    let point = rand_data();
    let mut points = vec![];
    let mut kdtree = KdTree::with_capacity(3, 16);
    for _ in 0..len {
        points.push(rand_data());
    }
    for point in points.iter() {
        kdtree.add(&point.0, point.1).unwrap();
    }
    c.bench_function("bench_nearest_from_kdtree_with_1k_3d_points", |b| {
        b.iter(|| kdtree.nearest(&point.0, 8, &squared_euclidean).unwrap());
    });
}

fn bench_within_2k_data_01_radius(c: &mut Criterion) {
    let len = 2000usize;
    let point = rand_data();
    let mut points = vec![];
    let mut kdtree = KdTree::with_capacity(3, 16);
    for _ in 0..len {
        points.push(rand_data());
    }
    for point in points.iter() {
        kdtree.add(&point.0, point.1).unwrap();
    }
    c.bench_function("bench_within_2k_data_01_radius", |b| {
        b.iter(|| kdtree.within(&point.0, 0.1, &squared_euclidean).unwrap());
    });
}

fn bench_within_2k_data_02_radius(c: &mut Criterion) {
    let len = 2000usize;
    let point = rand_data();
    let mut points = vec![];
    let mut kdtree = KdTree::with_capacity(3, 16);
    for _ in 0..len {
        points.push(rand_data());
    }
    for point in points.iter() {
        kdtree.add(&point.0, point.1).unwrap();
    }
    c.bench_function("bench_within_2k_data_02_radius", |b| {
        b.iter(|| kdtree.within(&point.0, 0.2, &squared_euclidean).unwrap());
    });
}

fn bench_within_unsorted_2k_data_01_radius(c: &mut Criterion) {
    let len = 2000usize;
    let point = rand_data();
    let mut points = vec![];
    let mut kdtree = KdTree::with_capacity(3, 16);
    for _ in 0..len {
        points.push(rand_data());
    }
    for point in points.iter() {
        kdtree.add(&point.0, point.1).unwrap();
    }
    c.bench_function("bench_within_unsorted_2k_data_01_radius", |b| {
        b.iter(|| kdtree.within_unsorted(&point.0, 0.1, &squared_euclidean).unwrap());
    });
}

fn bench_within_unsorted_2k_data_02_radius(c: &mut Criterion) {
    let len = 2000usize;
    let point = rand_data();
    let mut points = vec![];
    let mut kdtree = KdTree::with_capacity(3, 16);
    for _ in 0..len {
        points.push(rand_data());
    }
    for point in points.iter() {
        kdtree.add(&point.0, point.1).unwrap();
    }
    c.bench_function("bench_within_unsorted_2k_data_02_radius", |b| {
        b.iter(|| kdtree.within_unsorted(&point.0, 0.2, &squared_euclidean).unwrap());
    });
}

fn bench_within_count_2k_data_01_radius(c: &mut Criterion) {
    let len = 2000usize;
    let point = rand_data();
    let mut points = vec![];
    let mut kdtree = KdTree::with_capacity(3, 16);
    for _ in 0..len {
        points.push(rand_data());
    }
    for point in points.iter() {
        kdtree.add(&point.0, point.1).unwrap();
    }
    c.bench_function("bench_within_count_2k_data_01_radius", |b| {
        b.iter(|| kdtree.within_count(&point.0, 0.1, &squared_euclidean).unwrap());
    });
}

fn bench_within_count_2k_data_02_radius(c: &mut Criterion) {
    let len = 2000usize;
    let point = rand_data();
    let mut points = vec![];
    let mut kdtree = KdTree::with_capacity(3, 16);
    for _ in 0..len {
        points.push(rand_data());
    }
    for point in points.iter() {
        kdtree.add(&point.0, point.1).unwrap();
    }
    c.bench_function("bench_within_count_2k_data_02_radius", |b| {
        b.iter(|| kdtree.within_count(&point.0, 0.2, &squared_euclidean).unwrap());
    });
}

criterion_group!(
    benches,
    bench_add_to_kdtree_with_1k_3d_points,
    bench_nearest_from_kdtree_with_1k_3d_points,
    bench_within_2k_data_01_radius,
    bench_within_2k_data_02_radius,
    bench_within_unsorted_2k_data_01_radius,
    bench_within_unsorted_2k_data_02_radius,
    bench_within_count_2k_data_01_radius,
    bench_within_count_2k_data_02_radius,
);

criterion_main!(benches);
