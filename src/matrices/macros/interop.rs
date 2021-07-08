#[macro_export]
macro_rules! two_dimensional_interop_methods {
    ($stuc:ident, $stucsm:ident, $typ:ty) => {
        impl $stuc {
            /// Create a new Floats1d
            pub fn new(array: Vec<Vec<$typ>>) -> $stuc {
                let (x, y) = (array.len(), array[0].len());
                $stuc {
                    data: Array2::from_shape_vec((x, y), array.into_iter().flatten().collect())
                        .unwrap(),
                }
            }
        }

        #[wasm_bindgen]
        impl $stuc {
            /// Create a new Floats2d from javascript
            #[wasm_bindgen(constructor)]
            pub fn new_with_js(js_array: JsValue) -> $stuc {
                let vector: Vec<Vec<$typ>> = js_array.into_serde().unwrap();

                $stuc::new(vector)
            }

            /// Gives the value contained in the ndarray as a javascript array
            #[wasm_bindgen(getter, js_name = data)]
            pub fn data_to_js(&self) -> JsValue {
                let mut repr: Vec<Vec<$typ>> = Vec::new();

                for row_idx in 0..self.data.nrows() {
                    repr.push(self.data.row(row_idx).to_vec());
                }

                JsValue::from_serde(&repr).unwrap()
            }

            /// Get the string representation of the underlying ndarray
            #[wasm_bindgen(js_name = toString)]
            pub fn to_string(&self) -> String {
                format!("{:#?}", self.data)
            }

            /// Clone the object
            #[wasm_bindgen(js_name = clone)]
            pub fn clone_for_wasm(&self) -> $stuc {
                $stuc {
                    data: self.data.clone(),
                }
            }
        }
    };
}
