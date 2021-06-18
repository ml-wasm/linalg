use super::Floats1d;
use ndarray::Array1;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl Floats1d {
    /// Create a new Floats1d of the given length filled with zeros
    #[wasm_bindgen(js_name = "newWithZeros")]
    pub fn new_with_zeros(len: usize) -> Floats1d {
        Floats1d {
            data: Array1::zeros([len]),
        }
    }

    /// Create a new Floats1d of the given length filled with ones
    #[wasm_bindgen(js_name = "newWithOnes")]
    pub fn new_with_ones(len: usize) -> Floats1d {
        Floats1d {
            data: Array1::ones([len]),
        }
    }

    /// Create a new Floats1d of the given length filled with the specified
    /// element
    #[wasm_bindgen(js_name = "newWithElement")]
    pub fn new_with_elem(len: usize, element: f64) -> Floats1d {
        Floats1d {
            data: Array1::from_elem([len], element),
        }
    }

    /// Create a new Floats1d of the length calling the specified function
    /// without any arguments
    #[wasm_bindgen(js_name = "newWithSimpleFunc")]
    pub fn new_with_simple_func(len: usize, js_func: js_sys::Function) -> Floats1d {
        Floats1d {
            data: Array1::from_shape_simple_fn([len], move || {
                js_func.call0(&JsValue::NULL).unwrap().as_f64().unwrap()
            }),
        }
    }

    /// Create a new Floats1d of the length calling the specified function
    /// with the index as the argument
    #[wasm_bindgen(js_name = "newWithFunc")]
    pub fn new_with_func(len: usize, js_func: js_sys::Function) -> Floats1d {
        Floats1d {
            data: Array1::from_shape_fn([len], move |idx| {
                js_func
                    .call1(&JsValue::NULL, &JsValue::from(idx as u32))
                    .unwrap()
                    .as_f64()
                    .unwrap()
            }),
        }
    }

    /// Create a new Floats1d with n evenly spaced elements from start to end
    #[wasm_bindgen(js_name = "newWithLinspace")]
    pub fn new_with_linspace(start: f64, end: f64, n: usize) -> Floats1d {
        Floats1d {
            data: Array1::linspace(start, end, n),
        }
    }

    /// Create a new Floats1d with elements from start to end incrementing by
    /// step
    #[wasm_bindgen(js_name = "newWithRange")]
    pub fn new_with_range(start: f64, end: f64, step: f64) -> Floats1d {
        Floats1d {
            data: Array1::range(start, end, step),
        }
    }

    /// Create a new Floats1d with n logarithmically spaced elements from
    /// base^start to base^end
    #[wasm_bindgen(js_name = "newWithLogspace")]
    pub fn new_with_logspace(base: f64, start: f64, end: f64, n: usize) -> Floats1d {
        Floats1d {
            data: Array1::logspace(base, start, end, n),
        }
    }

    /// Create a new Floats1d with n geometrically spaced elements from start to
    /// end
    #[wasm_bindgen(js_name = "newWithGeomspace")]
    pub fn new_with_geomspace(start: f64, end: f64, n: usize) -> Floats1d {
        Floats1d {
            data: Array1::geomspace(start, end, n).unwrap(),
        }
    }
}
