//! # kdtree
//!
//! K-dimensional tree for Rust (bucket point-region implementation)
//!
//! ## Usage
//!
//! ```
//! use kdtree::KdTree;
//! use kdtree::ErrorKind;
//! use kdtree::distance::squared_euclidean;
//!
//! let a: ([f64; 2], usize) = ([0f64, 0f64], 0);
//! let b: ([f64; 2], usize) = ([1f64, 1f64], 1);
//! let c: ([f64; 2], usize) = ([2f64, 2f64], 2);
//! let d: ([f64; 2], usize) = ([3f64, 3f64], 3);
//!
//! let dimensions = 2;
//! let mut kdtree = KdTree::new(dimensions);
//!
//! kdtree.add(&a.0, a.1).unwrap();
//! kdtree.add(&b.0, b.1).unwrap();
//! kdtree.add(&c.0, c.1).unwrap();
//! kdtree.add(&d.0, d.1).unwrap();
//!
//! assert_eq!(kdtree.size(), 4);
//! assert_eq!(
//!     kdtree.nearest(&a.0, 0, &squared_euclidean).unwrap(),
//!     vec![]
//! );
//! assert_eq!(
//!     kdtree.nearest(&a.0, 1, &squared_euclidean).unwrap(),
//!     vec![(0f64, &0)]
//! );
//! assert_eq!(
//!     kdtree.nearest(&a.0, 2, &squared_euclidean).unwrap(),
//!     vec![(0f64, &0), (2f64, &1)]
//! );
//! assert_eq!(
//!     kdtree.nearest(&a.0, 3, &squared_euclidean).unwrap(),
//!     vec![(0f64, &0), (2f64, &1), (8f64, &2)]
//! );
//! assert_eq!(
//!     kdtree.nearest(&a.0, 4, &squared_euclidean).unwrap(),
//!     vec![(0f64, &0), (2f64, &1), (8f64, &2), (18f64, &3)]
//! );
//! assert_eq!(
//!     kdtree.nearest(&a.0, 5, &squared_euclidean).unwrap(),
//!     vec![(0f64, &0), (2f64, &1), (8f64, &2), (18f64, &3)]
//! );
//! assert_eq!(
//!     kdtree.nearest(&b.0, 4, &squared_euclidean).unwrap(),
//!     vec![(0f64, &1), (2f64, &0), (2f64, &2), (8f64, &3)]
//! );
//! ```

extern crate num_traits;

#[cfg(feature = "serialize")]
#[cfg_attr(feature = "serialize", macro_use)]
extern crate serde_derive;

pub mod distance;
mod heap_element;
pub mod kdtree;
mod util;
pub use crate::kdtree::ErrorKind;
pub use crate::kdtree::KdTree;
