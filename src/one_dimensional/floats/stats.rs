use super::Floats1d;
use ndarray_stats::QuantileExt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl Floats1d {
    /// Get minimun element
    pub fn min(&self) -> f64 {
        *self.data.min().unwrap()
    }

    /// Get maximun element
    pub fn max(&self) -> f64 {
        *self.data.max().unwrap()
    }

    /// Get the mean of all the elements in the array
    pub fn mean(&self) -> f64 {
        self.data.mean().unwrap()
    }

    /// Get median off all elements
    pub fn median(&self) -> f64 {
        let mut d = self.data.to_vec();
        d.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mid = d.len() / 2;
        if d.len() % 2 == 0 {
            (d[mid - 1] + d[mid]) as f64 / 2.0
        } else {
            d[mid] as f64
        }
    }

    /// Get the variance of all the elements in the array
    pub fn var(&self, ddof: f64) -> f64 {
        self.data.var(ddof)
    }

    /// Get the mean of all the elements in the array
    pub fn std(&self, ddof: f64) -> f64 {
        self.data.std(ddof)
    }
}
