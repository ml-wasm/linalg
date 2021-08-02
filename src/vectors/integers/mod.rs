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
pub struct IntegersVector {
    #[wasm_bindgen(skip)]
    pub data: Array1<i32>,
}

one_dimensional_interop_methods!(IntegersVector, i32);
one_dimensional_basic_methods!(IntegersVector, i32);
one_dimensional_math_methods!(IntegersVector, i32);
