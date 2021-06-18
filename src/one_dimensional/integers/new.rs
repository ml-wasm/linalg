use super::Integers1d;
use ndarray::Array1;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl Integers1d {
    /// Create a new Integers1d of the given length filled with zeros
    #[wasm_bindgen(js_name = "newWithZeros")]
    pub fn new_with_zeros(len: usize) -> Integers1d {
        Integers1d {
            data: Array1::zeros([len]),
        }
    }

    /// Create a new Integers1d of the given length filled with ones
    #[wasm_bindgen(js_name = "newWithOnes")]
    pub fn new_with_ones(len: usize) -> Integers1d {
        Integers1d {
            data: Array1::ones([len]),
        }
    }

    /// Create a new Integers1d of the given length filled with the specified
    /// element
    #[wasm_bindgen(js_name = "newWithElement")]
    pub fn new_with_elem(len: usize, element: i32) -> Integers1d {
        Integers1d {
            data: Array1::from_elem([len], element),
        }
    }

    /// Create a new Integers1d of the length calling the specified function
    /// without any arguments
    #[wasm_bindgen(js_name = "newWithSimpleFunc")]
    pub fn new_with_simple_func(len: usize, js_func: js_sys::Function) -> Integers1d {
        Integers1d {
            data: Array1::from_shape_simple_fn([len], move || {
                js_func.call0(&JsValue::NULL).unwrap().as_f64().unwrap() as i32
            }),
        }
    }

    /// Create a new Integers1d of the length calling the specified function
    /// with the index as the argument
    #[wasm_bindgen(js_name = "newWithFunc")]
    pub fn new_with_func(len: usize, js_func: js_sys::Function) -> Integers1d {
        Integers1d {
            data: Array1::from_shape_fn([len], move |idx| {
                js_func
                    .call1(&JsValue::NULL, &JsValue::from(idx as u32))
                    .unwrap()
                    .as_f64()
                    .unwrap() as i32
            }),
        }
    }
}
