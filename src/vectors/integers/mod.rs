pub mod iteration;
pub mod new;
pub mod serde;
pub mod stats;
pub mod utils;

use ndarray::{arr1, s, Array1, Axis};
use std::ops::*;
use wasm_bindgen::prelude::*;

use crate::{
    apply_functions, apply_functions_with_arg, apply_functions_with_arg_with_alias,
    one_dimensional_basic_methods, one_dimensional_interop_methods, one_dimensional_math_methods,
    vectors_sampling_methods,
};

#[wasm_bindgen]
pub struct IntegersVector {
    #[wasm_bindgen(skip)]
    pub data: Array1<i32>,
}

one_dimensional_interop_methods!(IntegersVector, i32);
one_dimensional_basic_methods!(IntegersVector, i32);
one_dimensional_math_methods!(IntegersVector, i32);
vectors_sampling_methods!(IntegersVector);

apply_functions!(IntegersVector {
    abs
    "Computes the absolute value of each element",

    saturating_abs = saturatingAbs
    "Computes the absolute value of each element, saturating at numeric bounds instead of overflowing",

    neg
    "Negates each element",

    saturating_neg = saturatingNeg
    "Negates each element, saturating at numeric bounds instead of overflowing",

    signum
    "Changes the number to 0 if 0, -1 if negative, 1 if positive"
});

apply_functions_with_arg!(IntegersVector, u32 {
    pow
    "Raise each element to an integer",

    saturating_pow = saturatingPow
    "Raise each element to an integer, saturating at numeric bounds instead of overflowing"
});

apply_functions_with_arg_with_alias!(IntegersVector, i32 {
    add | add_constant = addConstant
    "Add an integer to each element",

    sub | sub_constant = subConstant
    "Subtract an integer from each element",

    mul | mul_constant = mulConstant
    "Multiply an integer to each element",

    div | div_constant = divConstant
    "Divide each element by an integer",

    div_euclid | div_euclid_constant = divEuclidConstant
    "Perform euclidean division on each element by an integer",

    rem | rem_constant = remConstant
    "Compute modular remainder for each element with an integer",

    rem_euclid | rem_euclid_constant = remEuclidConstant
    "Compute euclidean remainder for each element with an integer",

    saturating_add | saturating_add_constant = saturatingAddConstant
    "Add an integer to each element, saturating at numeric bounds instead of overflowing",

    saturating_sub | saturating_sub_constant = saturatingSubConstant
    "Subtract an integer from each element, saturating at numeric bounds instead of overflowing",

    saturating_mul | saturating_mul_constant = saturatingMulConstant
    "Multiply an integer to each element, saturating at numeric bounds instead of overflowing"
});
