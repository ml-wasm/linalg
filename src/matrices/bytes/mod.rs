mod iteration;
mod new;
mod stats;
mod utils;

use ndarray::{s, Array, Array2, Axis};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use super::macros::{basic::basic, interop::interop, math::math, sampling::sampling};
use crate::macros::apply;
use crate::vectors::bytes::BytesVector;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Clone)]
pub struct BytesMatrix {
    #[wasm_bindgen(skip)]
    pub data: Array2<u8>,
}

interop!(BytesMatrix, BytesVector, u8);
basic!(BytesMatrix, BytesVector, u8);
math!(BytesMatrix, BytesVector, u8);
sampling!(BytesMatrix);
