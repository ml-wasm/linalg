use crate::vectors::strings::StringsVector;

use super::StringsMatrix;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl StringsMatrix {
    #[wasm_bindgen(js_name = toVector)]
    pub fn to_vector(&self) -> StringsVector {
        let data = self.data.clone().into_raw_vec();
        StringsVector::new(data)
    }
}
