use super::FloatsMatrix;
use ndarray::{Array1, ArrayView1, Axis, parallel::prelude::*};
use ndarray_stats::{QuantileExt, SummaryStatisticsExt};
use wasm_bindgen::prelude::*;

use crate::vectors::floats::FloatsVector;

#[wasm_bindgen]
impl FloatsMatrix {
    /// Get the maximum element in the array
    pub fn max(&self) -> f64 {
        *self
            .data
            .into_par_iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()
    }

    /// Get the maximum element of each column in the matrix
    #[wasm_bindgen(js_name = maxC)]
    pub fn max_c(&self) -> FloatsVector {
        let mut vec = Vec::new();
        self.data
            .axis_iter(Axis(1))
            .into_par_iter()
            .map(|x| *x.max().unwrap())
            .collect_into_vec(&mut vec);

        FloatsVector {
            data: ndarray::Array1::from_vec(vec),
        }
    }

    /// Get the maximum element of each row in the matrix
    #[wasm_bindgen(js_name = maxR)]
    pub fn max_r(&self) -> FloatsVector {
        let mut vec = Vec::new();
        self.data
            .axis_iter(Axis(0))
            .into_par_iter()
            .map(|x| *x.max().unwrap())
            .collect_into_vec(&mut vec);

        FloatsVector {
            data: ndarray::Array1::from_vec(vec),
        }
    }

    /// Get the minimum element in the array
    pub fn min(&self) -> f64 {
        *self
            .data
            .into_par_iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()
    }

    /// Get the minimum element of each column in the matrix
    #[wasm_bindgen(js_name = minC)]
    pub fn min_c(&self) -> FloatsVector {
        let mut vec = Vec::new();
        self.data
            .axis_iter(Axis(1))
            .into_par_iter()
            .map(|x| *x.min().unwrap())
            .collect_into_vec(&mut vec);

        FloatsVector {
            data: ndarray::Array1::from_vec(vec),
        }
    }

    /// Get the minimum element of each row in the matrix
    #[wasm_bindgen(js_name = minR)]
    pub fn min_r(&self) -> FloatsVector {
        let mut vec = Vec::new();
        self.data
            .axis_iter(Axis(0))
            .into_par_iter()
            .map(|x| *x.min().unwrap())
            .collect_into_vec(&mut vec);

        FloatsVector {
            data: ndarray::Array1::from_vec(vec),
        }
    }

    #[wasm_bindgen(js_name = weightedMean)]
    pub fn weighted_mean(&self, weights: &FloatsMatrix) -> f64 {
        self.data.weighted_mean(&weights.data).unwrap()
    }

    #[wasm_bindgen(js_name = weightedMeanR)]
    pub fn weighted_mean_r(&self, weights: &FloatsVector) -> FloatsVector {
        let mut vec : Vec<f64> = Vec::new();
        self.data
            .axis_iter(Axis(0))
            .into_par_iter()
            .map(|x| x.weighted_mean(&ArrayView1::from(&weights.data)).unwrap())
            .collect_into_vec(&mut vec);

        FloatsVector {
            data: Array1::from_vec(vec)
        }
    }

    #[wasm_bindgen(js_name = weightedMeanC)]
    pub fn weighted_mean_c(&self, weights: &FloatsVector) -> FloatsVector {
        let mut vec : Vec<f64> = Vec::new();
        self.data
            .axis_iter(Axis(1))
            .into_par_iter()
            .map(|x| x.weighted_mean(&ArrayView1::from(&weights.data)).unwrap())
            .collect_into_vec(&mut vec);

        FloatsVector {
            data: Array1::from_vec(vec)
        }
    }

    /// Get the var of all the elements in the array
    #[wasm_bindgen]
    pub fn var(&self, dof: f64) -> f64 {
        self.data.var(dof)
    }

    /// Get the var of each row in the matrix
    #[wasm_bindgen(js_name = varC)]
    pub fn var_c(&self, dof: f64) -> FloatsVector {
        FloatsVector {
            data: self.data.var_axis(Axis(0), dof),
        }
    }

    /// Get the var of each column in the matrix
    #[wasm_bindgen(js_name = varR)]
    pub fn var_r(&self, dof: f64) -> FloatsVector {
        FloatsVector {
            data: self.data.var_axis(Axis(1), dof),
        }
    }

    /// Get weighted variance of all elements in matrix
    #[wasm_bindgen(js_name = weightedVar)]
    pub fn weighted_var(&self, weights: &FloatsMatrix, dof: f64) -> f64 {
        self.data.weighted_var(&weights.data, dof).unwrap()
    }

    /// Get weighted variance of all columns
    #[wasm_bindgen(js_name = weightedVarC)]
    pub fn weighted_var_c(&self, weights: &FloatsVector, dof: f64) -> FloatsVector {
        let mut vec : Vec<f64> = Vec::new();
        self.data
            .axis_iter(Axis(0))
            .into_par_iter()
            .map(|x| x.weighted_var(&ArrayView1::from(&weights.data), dof).unwrap())
            .collect_into_vec(&mut vec);

        FloatsVector {
            data: Array1::from_vec(vec)
        }
    }
 
    /// Get weighted variance of all rows
    #[wasm_bindgen(js_name = weightedVarR)]
    pub fn weighted_var_r(&self, weights: &FloatsVector, dof: f64) -> FloatsVector {
        let mut vec : Vec<f64> = Vec::new();
        self.data
            .axis_iter(Axis(1))
            .into_par_iter()
            .map(|x| x.weighted_var(&ArrayView1::from(&weights.data), dof).unwrap())
            .collect_into_vec(&mut vec);

        FloatsVector {
            data: Array1::from_vec(vec)
        }
    }

    /// Get the mean of all the elements in the array
    #[wasm_bindgen(js_name = std)]
    pub fn std(&self, dof: f64) -> f64 {
        self.data.std(dof)
    }

    /// Get the standard deviation of each row in the matrix
    #[wasm_bindgen(js_name = stdC)]
    pub fn std_c(&self, dof: f64) -> FloatsVector {
        FloatsVector {
            data: self.data.std_axis(Axis(0), dof),
        }
    }

    /// Get the standard deviation of each column in the matrix
    #[wasm_bindgen(js_name = stdR)]
    pub fn std_r(&self, dof: f64) -> FloatsVector {
        FloatsVector {
            data: self.data.std_axis(Axis(1), dof),
        }
    }

    /// get weighted std dev of all elements
    #[wasm_bindgen(js_name = weightedStd)]
    pub fn weighted_std(&self, weights: &FloatsMatrix, dof: f64) -> f64 {
        self.data.weighted_std(&weights.data, dof).unwrap()
    }

    /// Get weighted standard deviation of all columns
    #[wasm_bindgen(js_name = weightedStdC)]
    pub fn weighted_std_c(&self, weights: &FloatsVector, dof: f64) -> FloatsVector {
        let mut vec : Vec<f64> = Vec::new();
        self.data
            .axis_iter(Axis(0))
            .into_par_iter()
            .map(|x| x.weighted_std(&ArrayView1::from(&weights.data), dof).unwrap())
            .collect_into_vec(&mut vec);

        FloatsVector {
            data: Array1::from_vec(vec)
        }
    }
 
    /// Get weighted standard deviation of all rows
    #[wasm_bindgen(js_name = weightedStdR)]
    pub fn weighted_std_r(&self, weights: &FloatsVector, dof: f64) -> FloatsVector {
        let mut vec : Vec<f64> = Vec::new();
        self.data
            .axis_iter(Axis(1))
            .into_par_iter()
            .map(|x| x.weighted_std(&ArrayView1::from(&weights.data), dof).unwrap())
            .collect_into_vec(&mut vec);

        FloatsVector {
            data: Array1::from_vec(vec)
        }
    }
}
