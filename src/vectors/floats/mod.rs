pub mod iteration;
pub mod new;
pub mod serde;
pub mod stats;

use ndarray::{arr1, s, Array1, Axis};
use wasm_bindgen::prelude::*;

use crate::{
    one_dimensional_basic_methods, one_dimensional_interop_methods, one_dimensional_math_methods,
};

#[wasm_bindgen]
pub struct FloatsVector {
    #[wasm_bindgen(skip)]
    pub data: Array1<f64>,
}

one_dimensional_interop_methods!(FloatsVector, f64);
one_dimensional_basic_methods!(FloatsVector, f64);
one_dimensional_math_methods!(FloatsVector, f64);
