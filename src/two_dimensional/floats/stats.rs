use super::Floats2d;
use ndarray::{parallel::prelude::*, Axis};
use ndarray_stats::QuantileExt;
use wasm_bindgen::prelude::*;

use crate::one_dimensional::floats::Floats1d;

#[wasm_bindgen]
impl Floats2d {
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
    pub fn max_c(&self) -> Floats1d {
        let mut vec = Vec::new();
        self.data
            .axis_iter(Axis(0))
            .into_par_iter()
            .map(|x| *x.max().unwrap())
            .collect_into_vec(&mut vec);

        Floats1d {
            data: ndarray::Array1::from_vec(vec),
        }
    }

    /// Get the maximum element of each row in the matrix
    #[wasm_bindgen(js_name = maxR)]
    pub fn max_r(&self) -> Floats1d {
        let mut vec = Vec::new();
        self.data
            .axis_iter(Axis(1))
            .into_par_iter()
            .map(|x| *x.max().unwrap())
            .collect_into_vec(&mut vec);

        Floats1d {
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
    pub fn min_c(&self) -> Floats1d {
        let mut vec = Vec::new();
        self.data
            .axis_iter(Axis(0))
            .into_par_iter()
            .map(|x| *x.min().unwrap())
            .collect_into_vec(&mut vec);

        Floats1d {
            data: ndarray::Array1::from_vec(vec),
        }
    }

    /// Get the minimum element of each row in the matrix
    #[wasm_bindgen(js_name = minR)]
    pub fn min_r(&self) -> Floats1d {
        let mut vec = Vec::new();
        self.data
            .axis_iter(Axis(1))
            .into_par_iter()
            .map(|x| *x.min().unwrap())
            .collect_into_vec(&mut vec);

        Floats1d {
            data: ndarray::Array1::from_vec(vec),
        }
    }

    /// Get the var of all the elements in the array
    #[wasm_bindgen]
    pub fn var(&self, dof: f64) -> f64 {
        self.data.var(dof)
    }

    /// Get the var of each row in the matrix
    #[wasm_bindgen(js_name = varC)]
    pub fn var_c(&self, dof: f64) -> Floats1d {
        Floats1d {
            data: self.data.var_axis(Axis(0), dof),
        }
    }

    /// Get the var of each column in the matrix
    #[wasm_bindgen(js_name = varR)]
    pub fn var_r(&self, dof: f64) -> Floats1d {
        Floats1d {
            data: self.data.var_axis(Axis(1), dof),
        }
    }

    /// Get the mean of all the elements in the array
    #[wasm_bindgen(js_name = std)]
    pub fn std(&self, dof: f64) -> f64 {
        self.data.std(dof)
    }

    /// Get the standard deviation of each row in the matrix
    #[wasm_bindgen(js_name = stdC)]
    pub fn std_c(&self, dof: f64) -> Floats1d {
        Floats1d {
            data: self.data.std_axis(Axis(0), dof),
        }
    }

    /// Get the standard deviation of each column in the matrix
    #[wasm_bindgen(js_name = stdR)]
    pub fn std_r(&self, dof: f64) -> Floats1d {
        Floats1d {
            data: self.data.std_axis(Axis(1), dof),
        }
    }
}
