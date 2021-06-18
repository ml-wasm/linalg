use super::Strings1d;
use ndarray::Array1;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl Strings1d {
    /// Create a new Strings1d of the length calling the specified function
    /// without any arguments
    #[wasm_bindgen(js_name = "newWithSimpleFunc")]
    pub fn new_with_simple_func(len: usize, js_func: js_sys::Function) -> Strings1d {
        Strings1d {
            data: Array1::from_shape_simple_fn([len], move || {
                js_func.call0(&JsValue::NULL).unwrap().as_string().unwrap()
            }),
        }
    }

    /// Create a new Strings1d of the length calling the specified function
    /// with the index as the argument
    #[wasm_bindgen(js_name = "newWithFunc")]
    pub fn new_with_func(len: usize, js_func: js_sys::Function) -> Strings1d {
        Strings1d {
            data: Array1::from_shape_fn([len], move |idx| {
                js_func
                    .call1(&JsValue::NULL, &JsValue::from(idx as u32))
                    .unwrap()
                    .as_string()
                    .unwrap()
            }),
        }
    }
}
