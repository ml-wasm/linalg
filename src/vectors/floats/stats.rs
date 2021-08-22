use super::FloatsVector;
use ndarray_stats::{QuantileExt, SummaryStatisticsExt};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl FloatsVector {
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

    /// Get weighted mean of elements
    pub fn weighted_mean(&self, weights: &FloatsVector) -> f64 {
        self.data.weighted_mean(&weights.data).unwrap()
    }

    /// Get weighted sum
    pub fn weighted_sum(&self, weights: &FloatsVector) -> f64 {
        self.data.weighted_sum(&weights.data).unwrap()
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

    /// Get weighted variance
    pub fn weighted_var(&self, weights: &FloatsVector, ddof: f64) -> f64 {
        self.data.weighted_var(&weights.data, ddof).unwrap()
    }

    /// Get the mean of all the elements in the array
    pub fn std(&self, ddof: f64) -> f64 {
        self.data.std(ddof)
    }

    /// Get weighted standard deviation
    pub fn weighted_std(&self, weights: &FloatsVector, ddof: f64) -> f64 {
        self.data.weighted_std(&weights.data, ddof).unwrap()
    }
}
