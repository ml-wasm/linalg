use crate::vectors::integers::IntegersVector;

use super::IntegersMatrix;
use js_sys;
use ndarray::Axis;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl IntegersMatrix {
    pub fn map(&self, js_func: js_sys::Function) -> Self {
        Self {
            data: self.data.map(|x| {
                js_func
                    .call1(&JsValue::NULL, &JsValue::from(x.clone()))
                    .unwrap()
                    .as_f64()
                    .unwrap() as i32
            }),
        }
    }

    pub fn transform(&mut self, js_func: js_sys::Function) {
        self.data.map_inplace(|x| {
            *x = js_func
                .call1(&JsValue::NULL, &JsValue::from(x.clone()))
                .unwrap()
                .as_f64()
                .unwrap() as i32
        });
    }

    #[wasm_bindgen(js_name = forEach)]
    pub fn for_each(&self, js_func: js_sys::Function) {
        self.data.for_each(|x| {
            js_func
                .call1(&JsValue::NULL, &JsValue::from(x.clone()))
                .unwrap();
        });
    }

    #[wasm_bindgen(js_name = mapRows)]
    pub fn map_rows(&self, js_func: js_sys::Function) {
        self.data.map_axis(Axis(1), |x| {
            js_func
                .call1(&JsValue::NULL, &JsValue::from(
                    IntegersVector {
                        data: x.to_owned()
                    }
                ))
                .unwrap()
        });
    }

    #[wasm_bindgen(js_name = mapCols)]
    pub fn map_cols(&self, js_func: js_sys::Function) {
        self.data.map_axis(Axis(0), |x| {
            js_func
                .call1(&JsValue::NULL, &JsValue::from(
                    IntegersVector {
                        data: x.to_owned()
                    }
                ))
                .unwrap()
        });
    }
}

