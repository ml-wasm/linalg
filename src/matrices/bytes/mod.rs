mod iteration;
mod new;
mod stats;
mod utils;

use ndarray::{s, Array, Array2, Axis};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// use crate::macros::apply;
use crate::{
    matrices_sampling_methods, two_dimensional_basic_methods, two_dimensional_interop_methods,
    two_dimensional_math_methods, vectors::bytes::BytesVector,
};

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Clone)]
pub struct BytesMatrix {
    #[wasm_bindgen(skip)]
    pub data: Array2<u8>,
}

two_dimensional_interop_methods!(BytesMatrix, BytesVector, u8);
two_dimensional_basic_methods!(BytesMatrix, BytesVector, u8);
two_dimensional_math_methods!(BytesMatrix, BytesVector, u8);
matrices_sampling_methods!(BytesMatrix);
