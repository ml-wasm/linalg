use ndarray::{s, Array, Array2};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::{
    one_dimensional::strings::Strings1d, two_dimensional_basic_methods,
    two_dimensional_interop_methods,
};

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Strings2d {
    #[wasm_bindgen(skip)]
    pub data: Array2<String>,
}

two_dimensional_interop_methods!(Strings2d, Strings1d, String);
two_dimensional_basic_methods!(Strings2d, Strings1d, String);
