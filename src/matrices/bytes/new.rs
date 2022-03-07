use super::BytesMatrix;
use ndarray::Array2;
use ndarray_csv::Array2Reader;
use ndarray_rand::{rand_distr, RandomExt};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl BytesMatrix {
    /// Create new matrix of given size with give element
    #[wasm_bindgen(js_name = newWithElement)]
    pub fn new_with_elem(num_rows: usize, num_cols: usize, element: u8) -> BytesMatrix {
        BytesMatrix {
            data: Array2::from_elem([num_rows, num_cols], element),
        }
    }

    /// Create new matrix of given size initialize to 0
    #[wasm_bindgen(js_name = newWithZeroes)]
    pub fn new_with_zeros(num_rows: usize, num_cols: usize) -> BytesMatrix {
        BytesMatrix {
            data: Array2::zeros([num_rows, num_cols]),
        }
    }

    /// Create new matrix of give size initialized to 1
    #[wasm_bindgen(js_name = newWithOnes)]
    pub fn new_with_ones(num_rows: usize, num_cols: usize) -> BytesMatrix {
        BytesMatrix {
            data: Array2::ones([num_rows, num_cols]),
        }
    }

    /// Create new BytesMatrix from csv file
    #[wasm_bindgen(js_name = newFromCSV)]
    pub async fn new_from_csv(file: web_sys::File) -> BytesMatrix {
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
                //data.reshape((num_rows, num_cols));
                return BytesMatrix { data };
            }
            Err(err) => {
                panic!("{}", err);
            }
        }
    }

    /// Create new matrix from typed array with given shape
    #[wasm_bindgen(js_name = "newFromTypedArray")]
    pub fn new_from_typed_array(
        typed_array: &js_sys::Uint8Array,
        num_rows: usize,
        num_cols: usize,
    ) -> BytesMatrix {
        let mut raw_data = vec![0; typed_array.length() as usize];
        typed_array.copy_to(raw_data.as_mut_slice());

        BytesMatrix {
            data: Array2::from_shape_vec((num_rows, num_cols), raw_data).unwrap(),
        }
    }

    /// Create a new BytesMatrix of specified length randomly in the range [BYTE_MIN, BYTE_MAX]
    #[wasm_bindgen(js_name = "newWithRandom")]
    pub fn new_with_random(size: js_sys::Array) -> BytesMatrix {
        let row = size.get(0).as_f64().unwrap() as usize;
        let col = size.get(1).as_f64().unwrap() as usize;
        BytesMatrix {
            data: Array2::random((row, col), rand_distr::Standard),
        }
    }

    /// Create new identity matrix of size n
    #[wasm_bindgen(js_name = newEye)]
    pub fn new_eye(size: usize) -> BytesMatrix {
        BytesMatrix {
            data: Array2::eye(size),
        }
    }
}
