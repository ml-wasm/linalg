use crate::vectors::integers::IntegersVector;

use super::IntegersMatrix;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl IntegersMatrix {
    #[wasm_bindgen(js_name = toVector)]
    pub fn to_vector(&self) -> IntegersVector {
        let data = self.data.clone().into_raw_vec();
        IntegersVector::new(data)
    }
}
