//! # kdtree
//! K-dimensional tree for Rust(bucket point-region implementation)
//!
//! ## Usage
//! ```
//! use kdtree::KdTree;
//! use kdtree::ErrorKind;
//! use kdtree::distance::square_euclidean;
//!
//! let a: ([f64; 2], usize) = ([0f64, 0f64], 0);
//! let b: ([f64; 2], usize) = ([1f64, 1f64], 1);
//! let c: ([f64; 2], usize) = ([2f64, 2f64], 2);
//! let d: ([f64; 2], usize) = ([3f64, 3f64], 3);
//!
//! let dimensions = 2;
//! let mut kd_tree = KdTree::new(dimensions);
//!
//! kd_tree.add(&a.0, &a.1).unwrap();
//! kd_tree.add(&b.0, &b.1).unwrap();
//! kd_tree.add(&c.0, &c.1).unwrap();
//! kd_tree.add(&d.0, &d.1).unwrap();
//!
//! assert_eq!(kd_tree.size(), 4);
//! assert_eq!(
//!     kd_tree.nearest(&a.0, 0, &square_euclidean).unwrap(),
//!     vec![]
//!     );
//! assert_eq!(
//!     kd_tree.nearest(&a.0, 1, &square_euclidean).unwrap(),
//!     vec![(0f64, &0)]
//!     );
//! assert_eq!(
//!     kd_tree.nearest(&a.0, 2, &square_euclidean).unwrap(),
//!     vec![(0f64, &0), (2f64, &1)]
//!     );
//! assert_eq!(
//!     kd_tree.nearest(&a.0, 3, &square_euclidean).unwrap(),
//!     vec![(0f64, &0), (2f64, &1), (8f64, &2)]
//!     );
//! assert_eq!(
//!     kd_tree.nearest(&a.0, 4, &square_euclidean).unwrap(),
//!     vec![(0f64, &0), (2f64, &1), (8f64, &2), (18f64, &3)]
//!     );
//! assert_eq!(
//!     kd_tree.nearest(&a.0, 5, &square_euclidean).unwrap(),
//!     vec![(0f64, &0), (2f64, &1), (8f64, &2), (18f64, &3)]
//!     );
//! assert_eq!(
//!     kd_tree.nearest(&b.0, 4, &square_euclidean).unwrap(),
//!     vec![(0f64, &1), (2f64, &0), (2f64, &2), (8f64, &3)]
//!     );
//! ```

#![feature(box_raw)]
#[allow(raw_pointer_derive)]
pub mod kd_tree;
pub mod distance;
mod heap_element;
mod util;
pub use kd_tree::KdTree;
pub use kd_tree::ErrorKind;
