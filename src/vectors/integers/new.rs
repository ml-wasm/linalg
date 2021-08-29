use super::IntegersVector;
use ndarray::Array1;
use ndarray_csv::Array2Reader;
use ndarray_rand::{rand_distr, RandomExt};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl IntegersVector {
    /// Create a new Integers1d of the given length filled with zeros
    #[wasm_bindgen(js_name = "newWithZeros")]
    pub fn new_with_zeros(len: usize) -> IntegersVector {
        IntegersVector {
            data: Array1::zeros([len]),
        }
    }

    /// Create a new Integers1d of the given length filled with ones
    #[wasm_bindgen(js_name = "newWithOnes")]
    pub fn new_with_ones(len: usize) -> IntegersVector {
        IntegersVector {
            data: Array1::ones([len]),
        }
    }

    /// Create a new Integers1d of the given length filled with the specified
    /// element
    #[wasm_bindgen(js_name = "newWithElement")]
    pub fn new_with_elem(len: usize, element: i32) -> IntegersVector {
        IntegersVector {
            data: Array1::from_elem([len], element),
        }
    }

    /// Create new Vector from csv
    #[wasm_bindgen(js_name = newFromCSV)]
    pub async fn new_from_csv(file : web_sys::File) -> IntegersVector {
        let jsdata = wasm_bindgen_futures::JsFuture::from(file.text())
            .await
            .unwrap_throw();

        let data = jsdata.as_string().unwrap();

        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(data.as_bytes());

        let data_res = reader.deserialize_array2_dynamic();
        
        match data_res {
            Ok(data) => {
                return IntegersVector::new(data.into_raw_vec());
            },
            Err(err) => {
                panic!("{}", err);
            }
        }
    }

    /// Create a new Integers1d of the length calling the specified function
    /// without any arguments
    #[wasm_bindgen(js_name = "newWithSimpleFunc")]
    pub fn new_with_simple_func(len: usize, js_func: js_sys::Function) -> IntegersVector {
        IntegersVector {
            data: Array1::from_shape_simple_fn([len], move || {
                js_func.call0(&JsValue::NULL).unwrap().as_f64().unwrap() as i32
            }),
        }
    }

    /// Create a new Integers1d of the length calling the specified function
    /// with the index as the argument
    #[wasm_bindgen(js_name = "newWithFunc")]
    pub fn new_with_func(len: usize, js_func: js_sys::Function) -> IntegersVector {
        IntegersVector {
            data: Array1::from_shape_fn([len], move |idx| {
                js_func
                    .call1(&JsValue::NULL, &JsValue::from(idx as u32))
                    .unwrap()
                    .as_f64()
                    .unwrap() as i32
            }),
        }
    }

    /// Create a new IntegersVector of specified length randomly in the range [INT_MIN, INT_MAX]
    #[wasm_bindgen(js_name = "newWithRandom")]
    pub fn new_with_random(len: usize) -> IntegersVector {
        IntegersVector {
            data: Array1::random(len, rand_distr::Standard),
        }
    }

    //    /// Create a new IntegersVector of specified length randomly using Binomial
    //    /// distribution with n - number of trials and p - probability of success
    //    #[wasm_bindgen(js_name = "newWithRandomBinomial")]
    //    pub fn new_with_random_binomial(len: usize, n: u64, p: u64) -> IntegersVector {
    //        IntegersVector {
    //            data: Array1::random(len, rand_distr::Binomial::new(n, p).unwrap()),
    //        }
    //    }
    //
    //    /// Create a new IntegersVector of specified length randomly using Geometric
    //    /// distribution with shape paremeter p
    //    #[wasm_bindgen(js_name = "newWithRandomGeometric")]
    //    pub fn new_with_random_geometric(len: usize, p: f64) -> IntegersVector {
    //        IntegersVector {
    //            data: Array1::random(len, rand_distr::Geometric::new(p).unwrap()),
    //        }
    //    }
    //
    //    /// Create a new IntegersVector of specified length randomly using Geometric
    //    /// distribution with shape paremeter p = 0.5
    //    #[wasm_bindgen(js_name = "newWithRandomStandardGeometric")]
    //    pub fn new_with_random_standard_geometric(len: usize) -> IntegersVector {
    //        IntegersVector {
    //            data: Array1::random(len, rand_distr::StandardGeometric),
    //        }
    //    }
    //
    //    /// Create a new IntegersVector of specified length randomly using Hypergeometric
    //    /// distribution with n - population size, k - population with feature and
    //    /// s - sample size
    //    #[wasm_bindgen(js_name = "newWithRandomHypergeometric")]
    //    pub fn new_with_random_hypergeometric(len: usize, n: u64, k: u64, s: u64) -> IntegersVector {
    //        IntegersVector {
    //            data: Array1::random(len, rand_distr::Hypergeometric::new(n, k, s).unwrap()),
    //        }
    //    }
}
