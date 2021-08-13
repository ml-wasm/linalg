pub mod iteration;
pub mod new;
pub mod stats;
pub mod utils;

use ndarray::{s, Array, Array2, Axis};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::{
    apply_functions, apply_functions_with_alias, apply_functions_with_arg,
    apply_functions_with_two_args, matrices_sampling_methods, two_dimensional_basic_methods,
    two_dimensional_interop_methods, two_dimensional_math_methods, vectors::floats::FloatsVector,
};

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct FloatsMatrix {
    #[wasm_bindgen(skip)]
    pub data: Array2<f64>,
}

two_dimensional_interop_methods!(FloatsMatrix, FloatsVector, f64);
two_dimensional_basic_methods!(FloatsMatrix, FloatsVector, f64);
two_dimensional_math_methods!(FloatsMatrix, FloatsVector, f64);
matrices_sampling_methods!(FloatsMatrix);

apply_functions!(FloatsMatrix {
    floor
    "Computes the largest integer less than equal to the number",

    ceil
    "Computes the smallest integer greater than equal to the number",

    round
    "Computes the closest integer to the number",

    trunc
    "Computes only the integer part of the number",

    fract
    "Computes only the fraction part of the number",

    abs
    "Computes the absolute value of the number",

    signum
    "Changes the element 1.0 if the number is positive, +0.0 or INFINITY, \
     -1 if negative, -0.0 or NEG_INFINITY and NAN if NAN",

    sqrt
    "Computes the square root of the element",

    cbrt
    "Computes the cube root of the element",

    exp
    "Computes `e^element`",

    exp2
    "Computes `2^element`",

    ln
    "Computes the natural logarithm of the element",

    log2
    "Computes the base 2 logarithm of the element",

    log10
    "Computes the base 10 logarithm of the element",

    sin
    "Computes the sine of the element (in radians)",

    cos
    "Computes the cosine of the element (in radians)",

    tan
    "Computes the tangent of the element (in radians)",

    asin
    "Computes the arcsine of the element, NaN is outside [-1, 1]",

    acos
    "Computes the arccosine of the element, NaN is outside [-1, 1]",

    atan
    "Computes the arctangent of the element",

    sinh
    "Computes the hyperbolic sine of the element (in radians)",

    cosh
    "Computes the hyperbolic cosine of the element (in radians)",

    tanh
    "Computes the hyperbolic tangent of the element (in radians)",

    asinh
    "Computes the inverse hyperbolic sine of the element, NaN is outside [-1, 1]",

    acosh
    "Computes the inverse hyperbolic cosine of the element, NaN is outside [-1, 1]",

    atanh
    "Computes the inverse hyperbolic tangent of the element"
});

apply_functions_with_arg!(FloatsMatrix, f64 {
    div_euclid = divEuclid
    "Calculates the quotient of euclidean division on each element",

    rem_euclid = remEuclid
    "Calculates the remainder of euclideam division on each element",

    powf
    "Raises each element to an integer power",

    log
    "Computes the base `a` logarithm of the element",

    hypot
    "Calculate the hypotenuse of the right angled triangle with the element as one \
     side and `a` as the other",

    atan2
    "Computes the four quadrant arctangent of the element"
});

apply_functions_with_arg!(FloatsMatrix, i32 {
    powi
    "Raises each element to an integer"
});

apply_functions_with_alias!(FloatsMatrix {
    exp_m1 | exp_minus_1 = expMinus1
    "Computes `e^x - 1`",

    ln_1p | ln_plus_1 = lnPlus1
    "Computes `ln(n + 1)`"
});

apply_functions_with_two_args!(FloatsMatrix, f64, f64 {
    mul_add = mulAdd
    "Compute `x*a + b` for each element `x`"
});
