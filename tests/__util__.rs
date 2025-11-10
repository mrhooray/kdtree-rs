#![allow(dead_code)]

use kdtree::KdTree;

pub const POINT_A: ([f64; 2], usize) = ([0f64, 0f64], 0);
pub const POINT_B: ([f64; 2], usize) = ([1f64, 1f64], 1);
pub const POINT_C: ([f64; 2], usize) = ([2f64, 2f64], 2);
pub const POINT_D: ([f64; 2], usize) = ([3f64, 3f64], 3);
pub const POINTS: [([f64; 2], usize); 4] = [POINT_A, POINT_B, POINT_C, POINT_D];

pub fn basic_tree() -> KdTree<f64, usize, [f64; 2]> {
    let mut tree = KdTree::with_capacity(2, 2);
    for &(coords, value) in POINTS.iter() {
        tree.add(coords, value).unwrap();
    }
    tree
}
