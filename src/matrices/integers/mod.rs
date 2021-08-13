mod iteration;
mod new;
mod stats;
mod utils;

use ndarray::{s, Array, Array2, Axis};
use serde::{Deserialize, Serialize};
use std::ops::*;
use wasm_bindgen::prelude::*;

use crate::{
    apply_functions, apply_functions_with_arg, apply_functions_with_arg_with_alias,
    matrices_sampling_methods, two_dimensional_basic_methods, two_dimensional_interop_methods,
    two_dimensional_math_methods, vectors::integers::IntegersVector,
};

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct IntegersMatrix {
    #[wasm_bindgen(skip)]
    pub data: Array2<i32>,
}

two_dimensional_interop_methods!(IntegersMatrix, IntegersVector, i32);
two_dimensional_basic_methods!(IntegersMatrix, IntegersVector, i32);
two_dimensional_math_methods!(IntegersMatrix, IntegersVector, i32);
matrices_sampling_methods!(IntegersMatrix);

apply_functions!(IntegersMatrix {
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

apply_functions_with_arg!(IntegersMatrix, u32 {
    pow
    "Raise each element to an integer",

    saturating_pow = saturatingPow
    "Raise each element to an integer, saturating at numeric bounds instead of overflowing"
});

apply_functions_with_arg_with_alias!(IntegersMatrix, i32 {
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
