#[macro_export]
macro_rules! two_dimensional_basic_methods {
    ($stuc:ident, $stucsm:ident, $typ:ty) => {
        #[wasm_bindgen]
        impl $stuc {
            /// Get the number of columns in the array
            pub fn ncols(&self) -> usize {
                self.data.ncols()
            }

            /// Get the number of rows in the array
            pub fn nrows(&self) -> usize {
                self.data.nrows()
            }

            /// Get the shape of array as a javascript
            #[wasm_bindgen(js_name = shape)]
            pub fn shape_to_js(&self) -> JsValue {
                JsValue::from_serde(self.data.shape()).unwrap()
            }

            /// Get the value at the given index
            pub fn get(&self, index: js_sys::Array) -> $typ {
                let index = (
                    index.get(0).as_f64().unwrap() as usize,
                    index.get(1).as_f64().unwrap() as usize,
                );
                self.data[index].clone()
            }

            /// Get the row at the specified index
            #[wasm_bindgen(js_name = getR)]
            pub fn get_r(&self, index: usize) -> $stucsm {
                $stucsm {
                    data: self.data.row(index).into_owned(),
                }
            }

            /// Get the column at the specified index
            #[wasm_bindgen(js_name = getC)]
            pub fn get_c(&self, index: usize) -> $stucsm {
                $stucsm {
                    data: self.data.column(index).into_owned(),
                }
            }

            /// Set the value at the specified index
            pub fn set(&mut self, index: js_sys::Array, value: $typ) {
                let index = (
                    index.get(0).as_f64().unwrap() as usize,
                    index.get(1).as_f64().unwrap() as usize,
                );
                self.data[index] = value;
            }

            /// Set the row at the specified index
            #[wasm_bindgen(js_name = setR)]
            pub fn set_r(&mut self, index: usize, array: &$stucsm) {
                self.data
                    .row_mut(index)
                    .iter_mut()
                    .zip(array.data.iter())
                    .for_each(|(to, from)| {
                        *to = from.clone();
                    });
            }

            /// Set the column at the specified index
            #[wasm_bindgen(js_name = setC)]
            pub fn set_c(&mut self, index: usize, array: &$stucsm) {
                self.data
                    .column_mut(index)
                    .iter_mut()
                    .zip(array.data.iter())
                    .for_each(|(to, from)| {
                        *to = from.clone();
                    });
            }

            /// Swap the values at the specified indices
            pub fn swap(&mut self, index1: js_sys::Array, index2: js_sys::Array) {
                let index1 = (
                    index1.get(0).as_f64().unwrap() as usize,
                    index1.get(1).as_f64().unwrap() as usize,
                );
                let index2 = (
                    index2.get(0).as_f64().unwrap() as usize,
                    index2.get(1).as_f64().unwrap() as usize,
                );

                self.data.swap(index1, index2);
            }

            /// Swap the rows at the specified indices
            #[wasm_bindgen(js_name = swapR)]
            pub fn swap_r(&mut self, index1: usize, index2: usize) {
                let row1 = self.get_r(index1);
                let row2 = self.get_r(index2);

                self.set_r(index1, &row1);
                self.set_r(index2, &row2);
            }

            /// Swap the columns at the specified indices
            #[wasm_bindgen(js_name = swapC)]
            pub fn swap_c(&mut self, index1: usize, index2: usize) {
                let col1 = self.get_c(index1);
                let col2 = self.get_c(index2);

                self.set_c(index1, &col1);
                self.set_c(index2, &col2);
            }

            /// Append a new row to the array
            #[wasm_bindgen(js_name = appendR)]
            pub fn append_r(&mut self, row: &$stucsm) {
                let new_array_vec = self
                    .data
                    .iter()
                    .chain(row.data.iter())
                    .map(|x| x.clone())
                    .collect::<Vec<$typ>>();
                self.data =
                    Array::from_shape_vec((self.nrows() + 1, self.ncols()), new_array_vec).unwrap();
            }

            /// Return the result of appending a new row to the array
            #[wasm_bindgen(js_name = appendedR)]
            pub fn appended_r(&self, row: &$stucsm) -> $stuc {
                let new_array_vec = self
                    .data
                    .iter()
                    .chain(row.data.iter())
                    .map(|x| x.clone())
                    .collect::<Vec<$typ>>();

                $stuc {
                    data: Array::from_shape_vec((self.nrows() + 1, self.ncols()), new_array_vec)
                        .unwrap(),
                }
            }

            /// Append a new column to the array
            #[wasm_bindgen(js_name = appendC)]
            pub fn append_c(&mut self, col: &$stucsm) {
                let new_array_vec = self
                    .data
                    .t()
                    .iter()
                    .chain(col.data.iter())
                    .map(|x| x.clone())
                    .collect::<Vec<$typ>>();

                self.data = Array::from_shape_vec((self.ncols() + 1, self.nrows()), new_array_vec)
                    .unwrap()
                    .t()
                    .into_owned();
            }

            /// Return the reuslt of appending a new column to the array
            #[wasm_bindgen(js_name = appendedC)]
            pub fn appended_c(&mut self, col: &$stucsm) -> $stuc {
                let new_array_vec = self
                    .data
                    .t()
                    .iter()
                    .chain(col.data.iter())
                    .map(|x| x.clone())
                    .collect::<Vec<$typ>>();

                $stuc {
                    data: Array::from_shape_vec((self.ncols() + 1, self.nrows()), new_array_vec)
                        .unwrap()
                        .t()
                        .into_owned(),
                }
            }

            /// Extend the array by appending rows from another array
            #[wasm_bindgen(js_name = extendR)]
            pub fn extend_r(&mut self, other: &$stuc) {
                let new_array_vec = self
                    .data
                    .iter()
                    .chain(other.data.iter())
                    .map(|x| x.clone())
                    .collect::<Vec<$typ>>();
                self.data = Array::from_shape_vec(
                    (self.nrows() + other.nrows(), self.ncols()),
                    new_array_vec,
                )
                .unwrap();
            }

            /// Return the result of extending the array by appending rows for another
            /// array
            #[wasm_bindgen(js_name = extendedR)]
            pub fn extended_r(&mut self, other: &$stuc) -> $stuc {
                let new_array_vec = self
                    .data
                    .iter()
                    .chain(other.data.iter())
                    .map(|x| x.clone())
                    .collect::<Vec<$typ>>();

                $stuc {
                    data: Array::from_shape_vec(
                        (self.nrows() + other.nrows(), self.ncols()),
                        new_array_vec,
                    )
                    .unwrap(),
                }
            }

            /// Extend the array by appending columns from another array
            #[wasm_bindgen(js_name = extendC)]
            pub fn extend_c(&mut self, other: &$stuc) {
                let new_array_vec = self
                    .data
                    .t()
                    .iter()
                    .chain(other.data.t().iter())
                    .map(|x| x.clone())
                    .collect::<Vec<$typ>>();

                self.data = Array::from_shape_vec(
                    (self.ncols() + other.ncols(), self.nrows()),
                    new_array_vec,
                )
                .unwrap()
                .t()
                .into_owned();
            }

            /// Return the result of extending the array by appending columns from
            /// another array
            #[wasm_bindgen(js_name = extendedC)]
            pub fn extended_c(&self, other: &$stuc) -> $stuc {
                let new_array_vec = self
                    .data
                    .t()
                    .iter()
                    .chain(other.data.t().iter())
                    .map(|x| x.clone())
                    .collect::<Vec<$typ>>();

                $stuc {
                    data: Array::from_shape_vec(
                        (self.ncols() + other.ncols(), self.nrows()),
                        new_array_vec,
                    )
                    .unwrap()
                    .t()
                    .into_owned(),
                }
            }

            /// Insert a row at the specified index
            #[wasm_bindgen(js_name = insertR)]
            pub fn insert_r(&mut self, index: usize, row: $stucsm) {
                let (first, second) = self
                    .data
                    .multi_slice_mut((s![..index, ..], s![index.., ..]));

                let new_array_vec = first
                    .iter()
                    .chain(row.data.iter())
                    .chain(second.iter())
                    .map(|x| x.clone())
                    .collect::<Vec<$typ>>();

                self.data =
                    Array::from_shape_vec((self.nrows() + 1, self.ncols()), new_array_vec).unwrap();
            }

            /// Insert a row at the specified index
            #[wasm_bindgen(js_name = insertedR)]
            pub fn inserted_r(&mut self, index: usize, row: $stucsm) -> $stuc {
                let (first, second) = self
                    .data
                    .multi_slice_mut((s![..index, ..], s![index.., ..]));

                let new_array_vec = first
                    .iter()
                    .chain(row.data.iter())
                    .chain(second.iter())
                    .map(|x| x.clone())
                    .collect::<Vec<$typ>>();

                $stuc {
                    data: Array::from_shape_vec((self.nrows() + 1, self.ncols()), new_array_vec)
                        .unwrap(),
                }
            }

            /// Insert a row at the specified index
            #[wasm_bindgen(js_name = insertC)]
            pub fn insert_c(&mut self, index: usize, row: $stucsm) {
                let (first, second) = self
                    .data
                    .multi_slice_mut((s![.., ..index], s![.., index..]));

                let new_array_vec = first
                    .t()
                    .iter()
                    .chain(row.data.iter())
                    .chain(second.t().iter())
                    .map(|x| x.clone())
                    .collect::<Vec<$typ>>();

                self.data = Array::from_shape_vec((self.ncols() + 1, self.nrows()), new_array_vec)
                    .unwrap()
                    .t()
                    .into_owned();
            }

            /// Insert a row at the specified index
            #[wasm_bindgen(js_name = insertedC)]
            pub fn inserted_c(&mut self, index: usize, row: $stucsm) -> $stuc {
                let (first, second) = self
                    .data
                    .multi_slice_mut((s![.., ..index], s![.., index..]));

                let new_array_vec = first
                    .t()
                    .iter()
                    .chain(row.data.iter())
                    .chain(second.t().iter())
                    .map(|x| x.clone())
                    .collect::<Vec<$typ>>();

                $stuc {
                    data: Array::from_shape_vec((self.ncols() + 1, self.nrows()), new_array_vec)
                        .unwrap()
                        .t()
                        .into_owned(),
                }
            }

            /// Remove the row at the specified index and return it
            #[wasm_bindgen(js_name = spliceR)]
            pub fn splice_r(&mut self, index: usize) -> $stucsm {
                let row = self.get_r(index);
                let (first, second) = self
                    .data
                    .multi_slice_mut((s![..index, ..], s![(index + 1).., ..]));
                let new_array_vec = first
                    .iter()
                    .chain(second.iter())
                    .map(|x| x.clone())
                    .collect::<Vec<$typ>>();

                self.data =
                    Array::from_shape_vec((self.nrows() - 1, self.ncols()), new_array_vec).unwrap();

                row
            }

            /// Remove the row at the specified index and return the modified array and
            /// the removed row
            #[wasm_bindgen(js_name = splicedR)]
            pub fn spliced_r(&mut self, index: usize) -> js_sys::Array {
                let row = self.get_r(index);
                let (first, second) = self
                    .data
                    .multi_slice_mut((s![..index, ..], s![(index + 1).., ..]));
                let new_array_vec = first
                    .iter()
                    .chain(second.iter())
                    .map(|x| x.clone())
                    .collect::<Vec<$typ>>();

                let spliced = $stuc {
                    data: Array::from_shape_vec((self.nrows() - 1, self.ncols()), new_array_vec)
                        .unwrap(),
                };

                js_sys::Array::of2(&JsValue::from(spliced), &JsValue::from(row))
            }

            /// Remove the column at the specified index and return it
            #[wasm_bindgen(js_name = spliceC)]
            pub fn splice_c(&mut self, index: usize) -> $stucsm {
                let col = self.get_c(index);
                let (first, second) = self
                    .data
                    .multi_slice_mut((s![.., ..index], s![.., (index + 1)..]));
                let new_array_vec = first
                    .t()
                    .iter()
                    .chain(second.t().iter())
                    .map(|x| x.clone())
                    .collect::<Vec<$typ>>();

                self.data = Array::from_shape_vec((self.nrows(), self.ncols() - 1), new_array_vec)
                    .unwrap()
                    .t()
                    .to_owned();

                col
            }

            /// Remove the column at the specified index and return the modified array
            /// and the removed column
            #[wasm_bindgen(js_name = splicedC)]
            pub fn spliced_c(&mut self, index: usize) -> js_sys::Array {
                let col = self.get_c(index);
                let (first, second) = self
                    .data
                    .multi_slice_mut((s![.., ..index], s![.., (index + 1)..]));
                let new_array_vec = first
                    .t()
                    .iter()
                    .chain(second.t().iter())
                    .map(|x| x.clone())
                    .collect::<Vec<$typ>>();

                let spliced = $stuc {
                    data: Array::from_shape_vec((self.nrows(), self.ncols() - 1), new_array_vec)
                        .unwrap()
                        .t()
                        .into_owned(),
                };

                js_sys::Array::of2(&JsValue::from(spliced), &JsValue::from(col))
            }
        }
    };
}
