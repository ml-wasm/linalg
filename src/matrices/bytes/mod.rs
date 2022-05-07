mod iteration;
mod new;
mod stats;
mod utils;

use ndarray::{s, Array, Array2, Axis};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use super::macros::{basic::basic, interop::interop, math::math, sampling::sampling};
use crate::macros::apply;
use crate::vectors::bytes::BytesVector;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Clone)]
pub struct BytesMatrix {
    #[wasm_bindgen(skip)]
    pub data: Array2<u8>,
}

interop!(BytesMatrix, BytesVector, u8);
basic!(BytesMatrix, BytesVector, u8);
math!(BytesMatrix, BytesVector, u8);
sampling!(BytesMatrix);

use std::ops::*;

apply::zero!(BytesMatrix {
    reverse_bits
    "Reverses the order of bits",
});

apply::one!(BytesMatrix, u8 {
    add = add_constant
    "Add a constant to each element",

    sub = sub_constant
    "Subtract a constant from each element",

    mul = mul_constant
    "Multiply a constant to each element",

    div = div_constant
    "Divide each element by a constant",

    saturating_add = saturating_add_constant
    "Add a constant to each element, saturating at numeric bounds instead of overflowing",

    saturating_sub = saturating_sub_constant
    "Subtract a constant from each element, saturating at numeric bounds instead of overflowing",

    saturating_mul = saturating_mul_constant
    "Multiply a constant to each element, saturating at numeric bounds instead of overflowing",

    saturating_div = saturating_div_constant
    "Divide each element by a constant, saturating at numeric bounds instead of overflowing",
});

apply::one!(BytesMatrix, u8 {
    rem_euclid
    "Calculates the remainder of each element",
});

apply::one!(BytesMatrix, u32 {
    rotate_left
    "Rotates the bits to the left by a specified amount",

    rotate_right
    "Rotates the bits to the right by a specified amount",

    pow
    "Raises each element to the power of a given element",
});
