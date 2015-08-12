# kdtree [![Build Status](https://travis-ci.org/mrhooray/kdtree-rs.svg?branch=master)](https://travis-ci.org/mrhooray/kdtree-rs)
> K-dimensional tree for Rust(bucket point-region implementation)

* [Crate](https://crates.io/crates/kdtree)
* [Documentation](http://mrhooray.github.io/kdtree-rs/kdtree/index.html)
* [Usage](#usage)
* [Benchmark](#benchmark)
* [License](#license)

##Usage
Add `kdtree` to `Cargo.toml`
```toml
[dependencies]
kdtree = "~0.1.0"
```
or
```toml
[dependencies.kdtree]
git = "https://github.com/mrhooray/kdtree-rs"
```

```rust
use kdtree::KdTree;
use kdtree::ErrorKind;
use kdtree::distance::square_euclidean;

let a: ([f64; 2], usize) = ([0f64, 0f64], 0);
let b: ([f64; 2], usize) = ([1f64, 1f64], 1);
let c: ([f64; 2], usize) = ([2f64, 2f64], 2);
let d: ([f64; 2], usize) = ([3f64, 3f64], 3);

let dimensions = 2;
let mut kd_tree = KdTree::new(dimensions);

kd_tree.add(&a.0, &a.1).unwrap();
kd_tree.add(&b.0, &b.1).unwrap();
kd_tree.add(&c.0, &c.1).unwrap();
kd_tree.add(&d.0, &d.1).unwrap();

assert_eq!(kd_tree.size(), 4);
assert_eq!(
    kd_tree.nearest(&a.0, 0, &square_euclidean).unwrap(),
    vec![]
    );
assert_eq!(
    kd_tree.nearest(&a.0, 1, &square_euclidean).unwrap(),
    vec![(0f64, &0)]
    );
assert_eq!(
    kd_tree.nearest(&a.0, 2, &square_euclidean).unwrap(),
    vec![(0f64, &0), (2f64, &1)]
    );
assert_eq!(
    kd_tree.nearest(&a.0, 3, &square_euclidean).unwrap(),
    vec![(0f64, &0), (2f64, &1), (8f64, &2)]
    );
assert_eq!(
    kd_tree.nearest(&a.0, 4, &square_euclidean).unwrap(),
    vec![(0f64, &0), (2f64, &1), (8f64, &2), (18f64, &3)]
    );
assert_eq!(
    kd_tree.nearest(&a.0, 5, &square_euclidean).unwrap(),
    vec![(0f64, &0), (2f64, &1), (8f64, &2), (18f64, &3)]
    );
assert_eq!(
    kd_tree.nearest(&b.0, 4, &square_euclidean).unwrap(),
    vec![(0f64, &1), (2f64, &0), (2f64, &2), (8f64, &3)]
    );
```

##Benchmark
`cargo bench` with 2.3 GHz Intel Core i7:
```
cargo bench
     Running target/release/bench-a26a346635ebfc8f

running 2 tests
test bench_add_at_1k_3d_points     ... bench:         116 ns/iter (+/- 24)
test bench_nearest_at_1k_3d_points ... bench:       2,661 ns/iter (+/- 1,769)

test result: ok. 0 passed; 0 failed; 0 ignored; 2 measured
```

##License
MIT
