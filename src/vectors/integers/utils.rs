use crate::matrices::integers::IntegersMatrix;

use super::IntegersVector;

use ndarray::Array2;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl IntegersVector {
    #[wasm_bindgen(js_name = toMatrix)]
    pub fn to_mat(&self, num_rows: usize, num_cols: usize) -> IntegersMatrix {
        IntegersMatrix {
            data: Array2::from_shape_vec((num_rows, num_cols), self.data.to_vec()).unwrap()
        }
    }

    #[wasm_bindgen(js_name = toTypedArray)]
    pub fn to_typed_array(&self) -> js_sys::Int32Array{
        let vec = self.data.to_vec();
        js_sys::Int32Array::from(vec.as_slice())
    }
}
