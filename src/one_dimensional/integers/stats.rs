use super::Integers1d;
use ndarray_stats::QuantileExt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl Integers1d {
    /// Get minimun element
    pub fn min(&self) -> i32 {
        *self.data.min().unwrap()
    }

    /// Get maximun element
    pub fn max(&self) -> i32 {
        *self.data.max().unwrap()
    }

    /// Get the mean of all the elements in the array
    pub fn mean(&self) -> i32 {
        self.data.mean().unwrap()
    }

    /// Get median of all elements
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

    /// Get variance of all elements in array
    pub fn var(&self, ddof: f64) -> f64 {
        let mean = self.mean();
        let sqr_diff = self.data.iter().map(|x| (x - mean).pow(2)).sum::<i32>() as f64;

        return sqr_diff / (self.len() as f64 - ddof);
    }

    /// Get standard_deviation of all elements in array
    pub fn std(&self, ddof: f64) -> f64 {
        self.var(ddof).sqrt()
    }
}
