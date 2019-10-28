# kdtree [![Build Status](https://travis-ci.org/mrhooray/kdtree-rs.svg?branch=master)](https://travis-ci.org/mrhooray/kdtree-rs)
> K-dimensional tree in Rust for fast geospatial indexing and nearest neighbors lookup

* [Crate](https://crates.io/crates/kdtree)
* [Documentation](https://docs.rs/kdtree)
* [Usage](#usage)
* [Benchmark](#benchmark)
* [License](#license)

## Usage
Add `kdtree` to `Cargo.toml`
```toml
[dependencies]
kdtree = "0.5.1"
```

Add points to kdtree and query nearest n points with distance function
```rust
use kdtree::KdTree;
use kdtree::ErrorKind;
use kdtree::distance::squared_euclidean;

let a: ([f64; 2], usize) = ([0f64, 0f64], 0);
let b: ([f64; 2], usize) = ([1f64, 1f64], 1);
let c: ([f64; 2], usize) = ([2f64, 2f64], 2);
let d: ([f64; 2], usize) = ([3f64, 3f64], 3);

let dimensions = 2;
let mut kdtree = KdTree::new(dimensions);

kdtree.add(&a.0, a.1).unwrap();
kdtree.add(&b.0, b.1).unwrap();
kdtree.add(&c.0, c.1).unwrap();
kdtree.add(&d.0, d.1).unwrap();

assert_eq!(kdtree.size(), 4);
assert_eq!(
    kdtree.nearest(&a.0, 0, &squared_euclidean).unwrap(),
    vec![]
);
assert_eq!(
    kdtree.nearest(&a.0, 1, &squared_euclidean).unwrap(),
    vec![(0f64, &0)]
);
assert_eq!(
    kdtree.nearest(&a.0, 2, &squared_euclidean).unwrap(),
    vec![(0f64, &0), (2f64, &1)]
);
assert_eq!(
    kdtree.nearest(&a.0, 3, &squared_euclidean).unwrap(),
    vec![(0f64, &0), (2f64, &1), (8f64, &2)]
);
assert_eq!(
    kdtree.nearest(&a.0, 4, &squared_euclidean).unwrap(),
    vec![(0f64, &0), (2f64, &1), (8f64, &2), (18f64, &3)]
);
assert_eq!(
    kdtree.nearest(&a.0, 5, &squared_euclidean).unwrap(),
    vec![(0f64, &0), (2f64, &1), (8f64, &2), (18f64, &3)]
);
assert_eq!(
    kdtree.nearest(&b.0, 4, &squared_euclidean).unwrap(),
    vec![(0f64, &1), (2f64, &0), (2f64, &2), (8f64, &3)]
);
```

## Benchmark
`cargo bench` with 2.3 GHz Intel i5-7360U:
```
cargo bench
     Running target/release/deps/bench-9e622e6a4ed9b92a

running 2 tests
test bench_add_to_kdtree_with_1k_3d_points       ... bench:         106 ns/iter (+/- 25)
test bench_nearest_from_kdtree_with_1k_3d_points ... bench:       1,237 ns/iter (+/- 266)

test result: ok. 0 passed; 0 failed; 0 ignored; 2 measured; 0 filtered out
```
Thanks [Eh2406](https://github.com/Eh2406) for various fixes and perf improvements.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
