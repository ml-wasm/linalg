pub mod iteration;
pub mod stats;

use ndarray::{s, Array, Array2, Axis};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::{
    two_dimensional_basic_methods, two_dimensional_interop_methods, two_dimensional_math_methods,
    vectors::floats::FloatsVector,
};

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct FloatsMatrix {
    #[wasm_bindgen(skip)]
    pub data: Array2<f64>,
}

two_dimensional_interop_methods!(FloatsMatrix, FloatsVector, f64);
two_dimensional_basic_methods!(FloatsMatrix, FloatsVector, f64);
two_dimensional_math_methods!(FloatsMatrix, FloatsVector, f64);
