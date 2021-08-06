use super::IntegersMatrix;
use ndarray::Array2;
use ndarray_rand::{rand_distr, RandomExt};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl IntegersMatrix {
    /// Create a new IntegersMatrix of specified length randomly in the range [INT_MIN, INT_MAX]
    #[wasm_bindgen(js_name = "newWithRandom")]
    pub fn new_with_random(row: usize, col: usize) -> IntegersMatrix {
        IntegersMatrix {
            data: Array2::random((row, col), rand_distr::Standard),
        }
    }

    //    /// Create a new IntegersMatrix of specified length randomly using Binomial
    //    /// distribution with n - number of trials and p - probability of success
    //    #[wasm_bindgen(js_name = "newWithRandomBinomial")]
    //    pub fn new_with_random_binomial(row: usize, col: usize, n: u64, p: u64) -> IntegersMatrix {
    //        IntegersMatrix {
    //            data: Array2::random((row, col), rand_distr::Binomial::new(n, p).unwrap()),
    //        }
    //    }
    //
    //    /// Create a new IntegersMatrix of specified length randomly using Geometric
    //    /// distribution with shape paremeter p
    //    #[wasm_bindgen(js_name = "newWithRandomGeometric")]
    //    pub fn new_with_random_geometric(row: usize, col: usize, p: f64) -> IntegersMatrix {
    //        IntegersMatrix {
    //            data: Array2::random((row, col), rand_distr::Geometric::new(p).unwrap()),
    //        }
    //    }
    //
    //    /// Create a new IntegersMatrix of specified length randomly using Geometric
    //    /// distribution with shape paremeter p = 0.5
    //    #[wasm_bindgen(js_name = "newWithRandomStandardGeometric")]
    //    pub fn new_with_random_standard_geometric(row: usize, col: usize) -> IntegersMatrix {
    //        IntegersMatrix {
    //            data: Array2::random((row, col), rand_distr::StandardGeometric),
    //        }
    //    }
    //
    //    /// Create a new IntegersMatrix of specified length randomly using Hypergeometric
    //    /// distribution with n - population size, k - population with feature and
    //    /// s - sample size
    //    #[wasm_bindgen(js_name = "newWithRandomHypergeometric")]
    //    pub fn new_with_random_hypergeometric(row: usize, col: usize, n: u64, k: u64, s: u64) -> IntegersMatrix {
    //        IntegersMatrix {
    //            data: Array2::random((row, col), rand_distr::Hypergeometric::new(n, k, s).unwrap()),
    //        }
    //    }
}
