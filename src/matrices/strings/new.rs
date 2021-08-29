use super::StringsMatrix;
use ndarray_csv::Array2Reader;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl StringsMatrix {
    /// Create new StringsMatrix from csv file 
    #[wasm_bindgen(js_name = newFromCSV)]
    pub async fn new_from_csv(file : web_sys::File) -> StringsMatrix {
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
                return StringsMatrix {
                    data
                };
            },
            Err(err) => {
                panic!("{}", err);
            }
        }
    }
}
