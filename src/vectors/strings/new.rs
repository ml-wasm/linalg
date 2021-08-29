use super::StringsVector;
use ndarray::Array1;
use ndarray_csv::Array2Reader;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl StringsVector {
 
    /// Create new Vector from csv
    #[wasm_bindgen(js_name = newFromCSV)]
    pub async fn new_from_csv(file : web_sys::File) -> StringsVector {
        let jsdata = wasm_bindgen_futures::JsFuture::from(file.text())
            .await
            .unwrap_throw();

        let data = jsdata.as_string().unwrap();

        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(data.as_bytes());

        let data_res = reader.deserialize_array2_dynamic();
        
        match data_res {
            Ok(data) => {
                return StringsVector::new(data.into_raw_vec());
            },
            Err(err) => {
                panic!("{}", err);
            }
        }
    }
    
    /// Create a new Strings1d of the length calling the specified function
    /// without any arguments
    #[wasm_bindgen(js_name = "newWithSimpleFunc")]
    pub fn new_with_simple_func(len: usize, js_func: js_sys::Function) -> StringsVector {
        StringsVector {
            data: Array1::from_shape_simple_fn([len], move || {
                js_func.call0(&JsValue::NULL).unwrap().as_string().unwrap()
            }),
        }
    }

    /// Create a new Strings1d of the length calling the specified function
    /// with the index as the argument
    #[wasm_bindgen(js_name = "newWithFunc")]
    pub fn new_with_func(len: usize, js_func: js_sys::Function) -> StringsVector {
        StringsVector {
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
