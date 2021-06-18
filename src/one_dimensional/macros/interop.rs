#[macro_export]
macro_rules! one_dimensional_interop_methods {
    ($stuc:ident, $typ:ty) => {
        impl $stuc {
            /// Create a new Strings1d
            pub fn new(array: Vec<$typ>) -> Self {
                Self { data: arr1(&array) }
            }
        }

        #[wasm_bindgen]
        impl $stuc {
            /// Create a new $stuc from javascript
            #[wasm_bindgen(constructor)]
            pub fn new_with_js(value: JsValue) -> Self {
                serde_wasm_bindgen::from_value(value).unwrap()
            }

            /// Gives the JSON representation of the array
            #[wasm_bindgen(js_name = toJSON)]
            pub fn to_json(&self) -> JsValue {
                serde_wasm_bindgen::to_value(self).unwrap()
            }

            /// Gives the value contained in the ndarray as a javascript array
            #[wasm_bindgen(getter, js_name = data)]
            pub fn data_to_js(&self) -> JsValue {
                self.to_json()
            }

            /// Get the string representation of the underlying ndarray
            #[wasm_bindgen(js_name = toString)]
            pub fn to_string(&self) -> String {
                format!("{:#?}", self.data)
            }

            /// Clone the object
            #[wasm_bindgen(js_name = clone)]
            pub fn clone_for_wasm(&self) -> Self {
                Self {
                    data: self.data.clone(),
                }
            }
        }
    };
}
