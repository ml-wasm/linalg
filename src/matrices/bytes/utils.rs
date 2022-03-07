use crate::vectors::bytes::BytesVector;

use super::BytesMatrix;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl BytesMatrix {
    #[wasm_bindgen(js_name = toVector)]
    pub fn to_vector(&self) -> BytesVector {
        let data = self.data.clone().into_raw_vec();
        BytesVector::new(data)
    }
}
