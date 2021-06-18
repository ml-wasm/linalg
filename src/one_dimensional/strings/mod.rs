pub mod new;
pub mod serde;

use ndarray::{arr1, s, Array1, Axis};
use wasm_bindgen::prelude::*;

use crate::{one_dimensional_basic_methods, one_dimensional_interop_methods};

#[wasm_bindgen]
pub struct Strings1d {
    #[wasm_bindgen(skip)]
    pub data: Array1<String>,
}

one_dimensional_interop_methods!(Strings1d, String);
one_dimensional_basic_methods!(Strings1d, String);
