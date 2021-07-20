use super::IntegersVector;
use js_sys;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl IntegersVector {
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
}
