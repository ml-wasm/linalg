pub mod iteration;
pub mod new;
pub mod serde;
pub mod stats;
pub mod utils;

use crate::macros::apply;
use ndarray::{arr1, s, Array1, Axis};
use wasm_bindgen::prelude::*;

use crate::{
    one_dimensional_basic_methods, one_dimensional_interop_methods, one_dimensional_math_methods,
    vectors_sampling_methods,
};

#[wasm_bindgen]
#[derive(Clone)]
pub struct IntegersVector {
    #[wasm_bindgen(skip)]
    pub data: Array1<i32>,
}

one_dimensional_interop_methods!(IntegersVector, i32);
one_dimensional_basic_methods!(IntegersVector, i32);
one_dimensional_math_methods!(IntegersVector, i32);
vectors_sampling_methods!(IntegersVector);

use std::ops::*;

apply::zero!(IntegersVector {
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

apply::one!(IntegersVector, u32 {
    pow
    "Raise each element to an integer",

    saturating_pow
    "Raise each element to an integer, saturating at numeric bounds instead of overflowing"
});

apply::one!(IntegersVector, i32 {
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

apply::one!(IntegersVector, i32 {
    add = add_constant
    "Add an integer to each element",

    sub = sub_constant
    "Subtract an integer from each element",

    mul = mul_constant
    "Multiply an integer to each element",

    div = div_constant
    "Divide each element by an integer"
});
