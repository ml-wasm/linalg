use crate::vectors::floats::FloatsVector;

use super::FloatsMatrix;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl FloatsMatrix {
    #[wasm_bindgen(js_name = toVector)]
    pub fn to_vector(&self) -> FloatsVector {
        let data = self.data.clone().into_raw_vec();
        FloatsVector::new(data)
    }
}
