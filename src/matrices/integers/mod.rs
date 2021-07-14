mod stats;

use ndarray::{s, Array, Array2, Axis};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::{
    two_dimensional_basic_methods, two_dimensional_interop_methods, two_dimensional_math_methods,
    vectors::integers::IntegersVector,
};

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct IntegersMatrix {
    #[wasm_bindgen(skip)]
    pub data: Array2<i32>,
}

two_dimensional_interop_methods!(IntegersMatrix, IntegersVector, i32);
two_dimensional_basic_methods!(IntegersMatrix, IntegersVector, i32);
two_dimensional_math_methods!(IntegersMatrix, IntegersVector, i32);
