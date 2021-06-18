pub mod stats;

use ndarray::{s, Array, Array2, Axis};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::{
    one_dimensional::floats::Floats1d, two_dimensional_basic_methods,
    two_dimensional_interop_methods, two_dimensional_math_methods,
};

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Floats2d {
    #[wasm_bindgen(skip)]
    pub data: Array2<f64>,
}

two_dimensional_interop_methods!(Floats2d, Floats1d, f64);
two_dimensional_basic_methods!(Floats2d, Floats1d, f64);
two_dimensional_math_methods!(Floats2d, Floats1d, f64);
