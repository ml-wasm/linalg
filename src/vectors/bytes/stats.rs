use super::BytesVector;
use ndarray_stats::QuantileExt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl BytesVector {
    /// Get minimun element
    pub fn min(&self) -> u8 {
        *self.data.min().unwrap()
    }

    /// Get maximun element
    pub fn max(&self) -> u8 {
        *self.data.max().unwrap()
    }

    /// Get the mean of all the elements in the array
    pub fn mean(&self) -> u8 {
        self.data.mean().unwrap()
    }
}
