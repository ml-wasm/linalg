use super::IntegersMatrix;
use ndarray::Array2;
use ndarray_rand::{rand_distr, RandomExt};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl IntegersMatrix {
    /// Create new matrix of given size with give element
    #[wasm_bindgen(js_name = newWithElement)]
    pub fn new_with_elem(num_rows: usize, num_cols: usize, element: i32) -> IntegersMatrix {
        IntegersMatrix {
            data: Array2::from_elem([num_rows, num_cols], element)
        }
    }

    /// Create new matrix of given size initialize to 0
    #[wasm_bindgen(js_name = newWithZeroes)]
    pub fn new_with_zeros(num_rows: usize, num_cols: usize) -> IntegersMatrix {
        IntegersMatrix {
            data: Array2::zeros([num_rows, num_cols])
        }
    }

    /// Create new matrix of give size initialized to 1
    #[wasm_bindgen(js_name = newWithOnes)]
    pub fn new_with_ones(num_rows: usize, num_cols: usize) -> IntegersMatrix {
        IntegersMatrix {
            data: Array2::ones([num_rows, num_cols])
        }
    }
    
    /// Create a new IntegersMatrix of specified length randomly in the range [INT_MIN, INT_MAX]
    #[wasm_bindgen(js_name = "newWithRandom")]
    pub fn new_with_random(size: js_sys::Array) -> IntegersMatrix {
        let row = size.get(0).as_f64().unwrap() as usize;
        let col = size.get(0).as_f64().unwrap() as usize;
        IntegersMatrix {
            data: Array2::random((row, col), rand_distr::Standard),
        }
    }

    /// Create new identity matrix of size n
    #[wasm_bindgen(js_name = newEye)]
    pub fn new_eye(size: usize) -> IntegersMatrix {
        IntegersMatrix {
            data: Array2::eye(size)
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
