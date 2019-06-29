# EMD
[![Build Status](https://travis-ci.org/gchers/rust-emd.svg?branch=master)](https://travis-ci.org/gchers/rust-emd)

A simple Rust library for computing the Earth Mover's Distance (or
"Wasserstein distance" or "Kantorovich–Rubinstein distance").

This is a wrapper of Gary Doran's [pyemd](https://github.com/garydoranjr/pyemd).

## Basic usage

Include the following in `Cargo.toml`:

```
emd = "0.1.1"
```

Then:

```rust
extern crate emd;
#[macro_use(array)]
extern crate ndarray;

use emd::*;
use ndarray::*;

let x = array![0., 1.];
let y = array![5., 3.];
assert_eq!(distance(&x.view(), &y.view()), 3.5);
```

Check out the [docs](https://docs.rs/crate/emd) for more.

## Maintainers
[gchers](https://github.com/gchers), [ehentgen](https://github.com/ehentgen)
