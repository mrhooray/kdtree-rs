#![feature(box_raw)]
#[allow(raw_pointer_derive)]
pub mod kd_tree;
pub mod distance;
mod heap_element;
mod util;
pub use kd_tree::KdTree;
pub use kd_tree::ErrorKind;
