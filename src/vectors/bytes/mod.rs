pub mod iteration;
pub mod new;
pub mod serde;
pub mod stats;
pub mod utils;

// use crate::macros::apply;
use ndarray::{arr1, s, Array1, Axis};
use wasm_bindgen::prelude::*;

use crate::{
    one_dimensional_basic_methods, one_dimensional_interop_methods, one_dimensional_math_methods,
    vectors_sampling_methods,
};

#[wasm_bindgen]
#[derive(Clone)]
pub struct BytesVector {
    #[wasm_bindgen(skip)]
    pub data: Array1<u8>,
}

one_dimensional_interop_methods!(BytesVector, u8);
one_dimensional_basic_methods!(BytesVector, u8);
one_dimensional_math_methods!(BytesVector, u8);
vectors_sampling_methods!(BytesVector);
