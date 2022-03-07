// use crate::matrices::integers::IntegersMatrix;

use super::BytesVector;

// use ndarray::Array2;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl BytesVector {
    // #[wasm_bindgen(js_name = toMatrix)]
    // pub fn to_mat(&self, num_rows: usize, num_cols: usize) -> IntegersMatrix {
    //     IntegersMatrix {
    //         data: Array2::from_shape_vec((num_rows, num_cols), self.data.to_vec()).unwrap()
    //     }
    // }

    #[wasm_bindgen(js_name = toTypedArray)]
    pub fn to_typed_array(&self) -> js_sys::Uint8Array {
        let vec = self.data.to_vec();
        js_sys::Uint8Array::from(vec.as_slice())
    }
}
