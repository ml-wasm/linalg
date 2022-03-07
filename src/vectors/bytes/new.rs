use super::BytesVector;
use ndarray::Array1;
use ndarray_csv::Array2Reader;
use ndarray_rand::{rand_distr, RandomExt};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl BytesVector {
    /// Create a new Integers1d of the given length filled with zeros
    #[wasm_bindgen(js_name = "newWithZeros")]
    pub fn new_with_zeros(len: usize) -> BytesVector {
        BytesVector {
            data: Array1::zeros([len]),
        }
    }

    /// Create a new Integers1d of the given length filled with ones
    #[wasm_bindgen(js_name = "newWithOnes")]
    pub fn new_with_ones(len: usize) -> BytesVector {
        BytesVector {
            data: Array1::ones([len]),
        }
    }

    /// Create a new Integers1d of the given length filled with the specified
    /// element
    #[wasm_bindgen(js_name = "newWithElement")]
    pub fn new_with_elem(len: usize, element: u8) -> BytesVector {
        BytesVector {
            data: Array1::from_elem([len], element),
        }
    }

    /// Create new Vector from csv
    #[wasm_bindgen(js_name = newFromCSV)]
    pub async fn new_from_csv(file: web_sys::File) -> BytesVector {
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
                return BytesVector::new(data.into_raw_vec());
            }
            Err(err) => {
                panic!("{}", err);
            }
        }
    }

    /// Create new Integer vector from typed array
    #[wasm_bindgen(js_name = "newFromTypedArray")]
    pub fn new_from_typed_array(typed_array: &js_sys::Uint8Array) -> BytesVector {
        let mut raw_data = vec![0; typed_array.length() as usize];
        typed_array.copy_to(raw_data.as_mut_slice());

        BytesVector {
            data: Array1::from_vec(raw_data),
        }
    }

    /// Create a new Integers1d of the length calling the specified function
    /// without any arguments
    #[wasm_bindgen(js_name = "newWithSimpleFunc")]
    pub fn new_with_simple_func(len: usize, js_func: js_sys::Function) -> BytesVector {
        BytesVector {
            data: Array1::from_shape_simple_fn([len], move || {
                js_func.call0(&JsValue::NULL).unwrap().as_f64().unwrap() as u8
            }),
        }
    }

    /// Create a new Integers1d of the length calling the specified function
    /// with the index as the argument
    #[wasm_bindgen(js_name = "newWithFunc")]
    pub fn new_with_func(len: usize, js_func: js_sys::Function) -> BytesVector {
        BytesVector {
            data: Array1::from_shape_fn([len], move |idx| {
                js_func
                    .call1(&JsValue::NULL, &JsValue::from(idx as u32))
                    .unwrap()
                    .as_f64()
                    .unwrap() as u8
            }),
        }
    }

    /// Create a new IntegersVector of specified length randomly in the range [INT_MIN, INT_MAX]
    #[wasm_bindgen(js_name = "newWithRandom")]
    pub fn new_with_random(len: usize) -> BytesVector {
        BytesVector {
            data: Array1::random(len, rand_distr::Standard),
        }
    }
}
