pub mod new;
pub mod serde;
pub mod stats;

use ndarray::{arr1, s, Array1, Axis};
use wasm_bindgen::prelude::*;

use crate::{
    one_dimensional_basic_methods, one_dimensional_interop_methods, one_dimensional_math_methods,
};

#[wasm_bindgen]
pub struct Integers1d {
    #[wasm_bindgen(skip)]
    pub data: Array1<i32>,
}

one_dimensional_interop_methods!(Integers1d, i32);
one_dimensional_basic_methods!(Integers1d, i32);
one_dimensional_math_methods!(Integers1d, i32);
