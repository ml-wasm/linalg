mod iteration;
mod new;
mod stats;
mod utils;

use ndarray::{s, Array, Array2, Axis};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use super::macros::{basic::basic, interop::interop, math::math, sampling::sampling};
use crate::macros::apply;
use crate::vectors::integers::IntegersVector;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Clone)]
pub struct IntegersMatrix {
    #[wasm_bindgen(skip)]
    pub data: Array2<i32>,
}

interop!(IntegersMatrix, IntegersVector, i32);
basic!(IntegersMatrix, IntegersVector, i32);
math!(IntegersMatrix, IntegersVector, i32);
sampling!(IntegersMatrix);

use std::ops::*;

apply::zero!(IntegersMatrix {
    abs
    "Computes the absolute value of each element",

    saturating_abs
    "Computes the absolute value of each element, saturating at numeric bounds instead of overflowing",

    neg
    "Negates each element",

    saturating_neg
    "Negates each element, saturating at numeric bounds instead of overflowing",

    signum
    "Changes the number to 0 if 0, -1 if negative, 1 if positive"
});

apply::one!(IntegersMatrix, u32 {
    pow
    "Raise each element to an integer",

    saturating_pow
    "Raise each element to an integer, saturating at numeric bounds instead of overflowing"
});

apply::one!(IntegersMatrix, i32 {
    div_euclid
    "Perform euclidean division on each element by an integer",

    rem
    "Compute modular remainder for each element with an integer",

    rem_euclid
    "Compute euclidean remainder for each element with an integer",

    saturating_add
    "Add an integer to each element, saturating at numeric bounds instead of overflowing",

    saturating_sub
    "Subtract an integer from each element, saturating at numeric bounds instead of overflowing",

    saturating_mul
    "Multiply an integer to each element, saturating at numeric bounds instead of overflowing"
});

apply::one!(IntegersMatrix, i32 {
    add = add_constant
    "Add an integer to each element",

    sub = sub_constant
    "Subtract an integer from each element",

    mul = mul_constant
    "Multiply an integer to each element",

    div = div_constant
    "Divide each element by an integer"
});
