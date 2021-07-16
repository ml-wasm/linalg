use super::IntegersMatrix;
use ndarray::{parallel::prelude::*, Axis};
use wasm_bindgen::prelude::*;

use crate::vectors::integers::IntegersVector;

#[wasm_bindgen]
impl IntegersMatrix {
    /// Get the maximum element in the array
    pub fn max(&self) -> i32 {
        *self.data.into_par_iter().max().unwrap()
    }

    /// Get the maximum element of each column in the matrix
    #[wasm_bindgen(js_name = maxC)]
    pub fn max_c(&self) -> IntegersVector {
        let mut vec = Vec::new();
        self.data
            .axis_iter(Axis(1))
            .into_par_iter()
            .map(|x| *x.iter().max().unwrap())
            .collect_into_vec(&mut vec);

        IntegersVector {
            data: ndarray::Array1::from_vec(vec),
        }
    }

    /// Get the maximum element of each row in the matrix
    #[wasm_bindgen(js_name = maxR)]
    pub fn max_r(&self) -> IntegersVector {
        let mut vec = Vec::new();
        self.data
            .axis_iter(Axis(0))
            .into_par_iter()
            .map(|x| *x.iter().max().unwrap())
            .collect_into_vec(&mut vec);

        IntegersVector {
            data: ndarray::Array1::from_vec(vec),
        }
    }

    /// Get the minimum element in the array
    pub fn min(&self) -> i32 {
        *self.data.into_par_iter().min().unwrap()
    }

    /// Get the minimum element of each column in the matrix
    #[wasm_bindgen(js_name = minC)]
    pub fn min_c(&self) -> IntegersVector {
        let mut vec = Vec::new();
        self.data
            .axis_iter(Axis(1))
            .into_par_iter()
            .map(|x| *x.iter().min().unwrap())
            .collect_into_vec(&mut vec);

        IntegersVector {
            data: ndarray::Array1::from_vec(vec),
        }
    }

    /// Get the minimum element of each row in the matrix
    #[wasm_bindgen(js_name = minR)]
    pub fn min_r(&self) -> IntegersVector {
        let mut vec = Vec::new();
        self.data
            .axis_iter(Axis(0))
            .into_par_iter()
            .map(|x| *x.iter().min().unwrap())
            .collect_into_vec(&mut vec);

        IntegersVector {
            data: ndarray::Array1::from_vec(vec),
        }
    }
}
