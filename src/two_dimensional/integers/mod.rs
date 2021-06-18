use ndarray::{s, Array, Array2, Axis};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::{
    one_dimensional::integers::Integers1d, two_dimensional_basic_methods,
    two_dimensional_interop_methods, two_dimensional_math_methods,
};

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Integers2d {
    #[wasm_bindgen(skip)]
    pub data: Array2<i32>,
}

two_dimensional_interop_methods!(Integers2d, Integers1d, i32);
two_dimensional_basic_methods!(Integers2d, Integers1d, i32);
two_dimensional_math_methods!(Integers2d, Integers1d, i32);
