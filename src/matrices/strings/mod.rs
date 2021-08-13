mod iteration;
mod utils;

use ndarray::{s, Array, Array2};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::{
    two_dimensional_basic_methods, two_dimensional_interop_methods, vectors::strings::StringsVector,
};

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct StringsMatrix {
    #[wasm_bindgen(skip)]
    pub data: Array2<String>,
}

two_dimensional_interop_methods!(StringsMatrix, StringsVector, String);
two_dimensional_basic_methods!(StringsMatrix, StringsVector, String);
