//! Earth Mover's Distance (EMD)
//!
//! This is a wrapper around `pyemd`[1] for computing the EMD (or Wasserstein)
//! metric.
//! By default, it uses Euclidean distance as a cost function, but this
//! can be changed by using the generic distance function `distance_generic()`.
//!
//!
//! # Examples
//!
//! It allows computing the distance between two vectors (e.g., histograms):
//!
//! ```
//! extern crate emd;
//! #[macro_use(array)]
//! extern crate ndarray;
//!
//! # fn main() {
//! use emd::*;
//! use ndarray::*;
//!
//! let x = array![0., 1.];
//! let y = array![5., 3.];
//!
//! assert_eq!(distance(&x.view(), &y.view()), 3.5);
//! # }
//! ```
//!
//! Or between matrices (e.g., multivariate histograms). Note that in
//! this case the two matrices must have the same number of columns.
//!
//! ```
//! extern crate emd;
//! #[macro_use(array)]
//! extern crate ndarray;
//!
//! # fn main() {
//! use emd::*;
//! use ndarray::*;
//!
//! let x = array![[4., 3.], [3., 6.], [2., 3.]];
//! let y = array![[2., 3.], [1., 3.]];
//!
//! assert_eq!(distance_multi(&x.view(), &y.view()), 2.035183758487996);
//! # }
//! ```
//!
//! Alternatively, one can compute EMD for matrices (and, by extension,
//! vectors) with a chosen distance as cost function by using
//! `distance_generic()`.
//!
//! # References
//!
//! [1] https://github.com/garydoranjr/pyemd
extern crate ndarray;

use ndarray::*;
use std::os::raw::c_double;
use std::ptr::null;

#[link(name = "emd")]
extern "C" {
    fn emd(
        n_x: usize,
        weight_x: *const c_double,
        n_y: usize,
        weight_y: *const c_double,
        cost: *const *const c_double,
        flows: *const *const c_double,
    ) -> c_double;
}

/// Returns the Euclidean distance between two vectors of f64 values.
pub fn euclidean_distance(v1: &ArrayView1<f64>, v2: &ArrayView1<f64>) -> f64 {
    v1.iter()
        .zip(v2.iter())
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f64>()
        .sqrt()
}

/// Computes the EMD between two vectors.
///
/// Uses the Euclidean distance between points as a cost function.
///
/// # Arguments
///
/// * `x` - First vector
/// * `y` - Second vector
pub fn distance(x: &ArrayView1<f64>, y: &ArrayView1<f64>) -> f64 {
    distance_multi(
        &x.into_shape((x.len(), 1)).unwrap(),
        &y.into_shape((y.len(), 1)).unwrap(),
    )
}

/// Computes the EMD between two matrices.
///
/// Uses the Euclidean distance between rows (vectors) in a matrix as a
/// cost function.
/// Each matrix may represent for example a multivariate histogram.
///
/// # Arguments
///
/// * `X` - First matrix
/// * `Y` - Second matrix
#[allow(non_snake_case)]
pub fn distance_multi(X: &ArrayView2<f64>, Y: &ArrayView2<f64>) -> f64 {
    distance_generic(X, Y, euclidean_distance)
}

/// Computes the EMD with a desired cost function (i.e., distance).
///
/// Uses the Euclidean distance between rows (vectors) in a matrix as a
/// cost function.
/// Each matrix may represent for example a multivariate histogram.
///
///# Arguments
///
/// * `X` - First matrix
/// * `Y` - Second matrix
/// * `distance` - Distance metric
#[allow(non_snake_case)]
pub fn distance_generic<D>(X: &ArrayView2<f64>, Y: &ArrayView2<f64>, distance: D) -> f64
where
    D: Fn(&ArrayView1<f64>, &ArrayView1<f64>) -> f64,
{
    assert_eq!(X.cols(), Y.cols());

    // Uniform weights
    let weight_x = vec![1. / (X.rows() as c_double); X.rows()];
    let weight_y = vec![1. / (Y.rows() as c_double); Y.rows()];

    // Pairwise distance matrix
    let mut cost = Vec::with_capacity(X.rows());
    for x in X.outer_iter() {
        let mut cost_i = Vec::with_capacity(Y.rows());
        for y in Y.outer_iter() {
            cost_i.push(distance(&x, &y) as c_double);
        }
        cost.push(Box::into_raw(cost_i.into_boxed_slice()) as *const c_double);
    }

    // Call emd()
    let d = unsafe {
        emd(
            X.rows(),
            weight_x.as_ptr(),
            Y.rows(),
            weight_y.as_ptr(),
            cost.as_ptr(),
            null(),
        )
    };

    for ptr in cost.iter() {
        unsafe {
            let reconstructed_slice =
                std::slice::from_raw_parts_mut(*ptr as *mut f64, Y.rows()) as *mut [f64];
            drop(Box::from_raw(reconstructed_slice));
        }
    }
    d as f64
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_distance() {
        let x = array![4., 3.];
        let y = array![3., 4.];
        assert_eq!(distance(&x.view(), &y.view()), 0.);

        let x = array![4., 3.];
        let y = array![3., 5.];
        assert_eq!(distance(&x.view(), &y.view()), 0.5);

        let x = array![4., 3.];
        let y = array![3., 5., 3., 2.];
        assert_eq!(distance(&x.view(), &y.view()), 0.75);
    }
}
