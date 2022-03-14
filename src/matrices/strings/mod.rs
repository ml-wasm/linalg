mod iteration;
mod new;
mod utils;

use ndarray::{s, Array, Array2};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use super::macros::{basic::basic, interop::interop};
use crate::vectors::strings::StringsVector;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Clone)]
pub struct StringsMatrix {
    #[wasm_bindgen(skip)]
    pub data: Array2<String>,
}

interop!(StringsMatrix, StringsVector, String);
basic!(StringsMatrix, StringsVector, String);
