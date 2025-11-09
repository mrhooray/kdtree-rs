extern crate criterion;
extern crate kdtree;

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use kdtree::KdTree;
use kdtree::distance::squared_euclidean;
use std::collections::BTreeSet;

fn deterministic_points(len: usize) -> (Vec<([f64; 3], f64)>, ([f64; 3], f64)) {
    fn next(state: &mut u64) -> f64 {
        *state = state.wrapping_mul(636_413_622_384_679_3005).wrapping_add(1);
        ((*state >> 11) as f64) / ((1u64 << 53) as f64)
    }

    let mut state = 0x1a2b_3c4d_5e6f_7788;
    let mut points = Vec::with_capacity(len);
    for _ in 0..len {
        let coords = [next(&mut state), next(&mut state), next(&mut state)];
        let value = next(&mut state);
        points.push((coords, value));
    }
    let point = ([next(&mut state), next(&mut state), next(&mut state)], next(&mut state));
    (points, point)
}

fn bench_add_to_kdtree_with_1k_3d_points(c: &mut Criterion) {
    let len = 1000usize;
    let (points, point) = deterministic_points(len);
    let mut kdtree = KdTree::with_capacity(3, 16);
    for point in points.iter() {
        kdtree.add(&point.0, point.1).unwrap();
    }
    c.bench_function("bench_add_to_kdtree_with_1k_3d_points", |b| {
        b.iter(|| kdtree.add(&point.0, point.1).unwrap());
    });
}

fn bench_nearest_from_kdtree_with_1k_3d_points(c: &mut Criterion) {
    let len = 1000usize;
    let (points, point) = deterministic_points(len);
    let mut kdtree = KdTree::with_capacity(3, 16);
    for point in points.iter() {
        kdtree.add(&point.0, point.1).unwrap();
    }
    c.bench_function("bench_nearest_from_kdtree_with_1k_3d_points", |b| {
        b.iter(|| kdtree.nearest(&point.0, 8, &squared_euclidean).unwrap());
    });
}

fn bench_nearest_within_radius_from_kdtree_with_1k_3d_points(c: &mut Criterion) {
    let len = 1000usize;
    let (points, point) = deterministic_points(len);
    let mut kdtree = KdTree::with_capacity(3, 16);
    for point in points.iter() {
        kdtree.add(&point.0, point.1).unwrap();
    }
    c.bench_function("bench_nearest_within_radius_from_kdtree_with_1k_3d_points", |b| {
        b.iter(|| {
            kdtree
                .nearest_within_radius(&point.0, 8, Some(0.02), &squared_euclidean)
                .unwrap()
        });
    });
}

fn bench_nearest_within_radius_comparisons(c: &mut Criterion) {
    let len = 1000usize;
    let (points, point) = deterministic_points(len);
    let mut kdtree = KdTree::with_capacity(3, 16);
    for entry in points.iter() {
        kdtree.add(&entry.0, entry.1).unwrap();
    }
    let mut group = c.benchmark_group("nearest_within_radius_comparisons_1k_3d_points");
    let scenarios: &[(usize, f64)] = &[(4, 0.02), (8, 0.02), (8, 0.2)];
    for k in scenarios.iter().map(|(k, _)| *k).collect::<BTreeSet<_>>() {
        group.bench_with_input(BenchmarkId::new("nearest", format!("k{}", k)), &k, |b, &k| {
            b.iter(|| kdtree.nearest(&point.0, k, &squared_euclidean).unwrap());
        });
    }
    for &(k, radius) in scenarios {
        group.bench_with_input(
            BenchmarkId::new("nearest_within_radius", format!("k{}_r_{:.2}", k, radius)),
            &(k, radius),
            |b, &(k, radius)| {
                b.iter(|| {
                    kdtree
                        .nearest_within_radius(&point.0, k, Some(radius), &squared_euclidean)
                        .unwrap()
                });
            },
        );
    }
    group.finish();
}

fn bench_within_2k_data_01_radius(c: &mut Criterion) {
    let len = 2000usize;
    let (points, point) = deterministic_points(len);
    let mut kdtree = KdTree::with_capacity(3, 16);
    for point in points.iter() {
        kdtree.add(&point.0, point.1).unwrap();
    }
    c.bench_function("bench_within_2k_data_01_radius", |b| {
        b.iter(|| kdtree.within(&point.0, 0.1, &squared_euclidean).unwrap());
    });
}

fn bench_within_2k_data_02_radius(c: &mut Criterion) {
    let len = 2000usize;
    let (points, point) = deterministic_points(len);
    let mut kdtree = KdTree::with_capacity(3, 16);
    for point in points.iter() {
        kdtree.add(&point.0, point.1).unwrap();
    }
    c.bench_function("bench_within_2k_data_02_radius", |b| {
        b.iter(|| kdtree.within(&point.0, 0.2, &squared_euclidean).unwrap());
    });
}

fn bench_within_unsorted_2k_data_01_radius(c: &mut Criterion) {
    let len = 2000usize;
    let (points, point) = deterministic_points(len);
    let mut kdtree = KdTree::with_capacity(3, 16);
    for point in points.iter() {
        kdtree.add(&point.0, point.1).unwrap();
    }
    c.bench_function("bench_within_unsorted_2k_data_01_radius", |b| {
        b.iter(|| kdtree.within_unsorted(&point.0, 0.1, &squared_euclidean).unwrap());
    });
}

fn bench_within_unsorted_2k_data_02_radius(c: &mut Criterion) {
    let len = 2000usize;
    let (points, point) = deterministic_points(len);
    let mut kdtree = KdTree::with_capacity(3, 16);
    for point in points.iter() {
        kdtree.add(&point.0, point.1).unwrap();
    }
    c.bench_function("bench_within_unsorted_2k_data_02_radius", |b| {
        b.iter(|| kdtree.within_unsorted(&point.0, 0.2, &squared_euclidean).unwrap());
    });
}

fn bench_within_count_2k_data_01_radius(c: &mut Criterion) {
    let len = 2000usize;
    let (points, point) = deterministic_points(len);
    let mut kdtree = KdTree::with_capacity(3, 16);
    for point in points.iter() {
        kdtree.add(&point.0, point.1).unwrap();
    }
    c.bench_function("bench_within_count_2k_data_01_radius", |b| {
        b.iter(|| kdtree.within_count(&point.0, 0.1, &squared_euclidean).unwrap());
    });
}

fn bench_within_count_2k_data_02_radius(c: &mut Criterion) {
    let len = 2000usize;
    let (points, point) = deterministic_points(len);
    let mut kdtree = KdTree::with_capacity(3, 16);
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
    bench_nearest_within_radius_from_kdtree_with_1k_3d_points,
    bench_nearest_within_radius_comparisons,
    bench_within_2k_data_01_radius,
    bench_within_2k_data_02_radius,
    bench_within_unsorted_2k_data_01_radius,
    bench_within_unsorted_2k_data_02_radius,
    bench_within_count_2k_data_01_radius,
    bench_within_count_2k_data_02_radius,
);

criterion_main!(benches);
