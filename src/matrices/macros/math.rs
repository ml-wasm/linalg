#[macro_export]
macro_rules! two_dimensional_math_methods {
    ($stuc:ident, $stucsm:ident, $typ:ty) => {
        #[wasm_bindgen]
        impl $stuc {
            /// Transpose the matrix
            pub fn transpose(&mut self) {
                self.data.swap_axes(0, 1);
            }

            /// Return the transpose of the matrix
            pub fn transposed(&self) -> $stuc {
                $stuc {
                    data: self.data.t().into_owned(),
                }
            }

            /// Perform element-wise addition on two matrices and return the
            /// result
            pub fn add(&self, other: &$stuc) -> $stuc {
                $stuc {
                    data: &self.data + &other.data,
                }
            }

            /// Perform element-wise subtraction on two matrices and return the
            /// result
            pub fn sub(&self, other: &$stuc) -> $stuc {
                $stuc {
                    data: &self.data - &other.data,
                }
            }

            /// Perform element-wise multiplication on two matrices and return
            /// the result
            pub fn mul(&self, other: &$stuc) -> $stuc {
                $stuc {
                    data: &self.data * &other.data,
                }
            }

            /// Perform element-wise divison on two matrices and return the
            /// result
            pub fn div(&self, other: &$stuc) -> $stuc {
                $stuc {
                    data: &self.data / &other.data,
                }
            }

            /// Return the dot product of the two matrices
            pub fn dot(&self, other: &$stuc) -> $stuc {
                $stuc {
                    data: self.data.dot(&other.data),
                }
            }

            /// Efficiently performs self += alpha * rhs
            #[wasm_bindgen(js_name = scaledAdd)]
            pub fn scaled_add(&mut self, alpha: $typ, other: &$stuc) {
                self.data.scaled_add(alpha, &other.data);
            }

            /// Returns the sum of all the elements in the matrix
            pub fn sum(&self) -> $typ {
                self.data.sum()
            }

            /// Returns the sums of each row in the matrix
            #[wasm_bindgen(js_name = sumC)]
            pub fn sum_c(&self) -> $stucsm {
                $stucsm {
                    data: self.data.sum_axis(Axis(0)),
                }
            }

            /// Returns the sums of each column in the matrix
            #[wasm_bindgen(js_name = sumR)]
            pub fn sum_r(&self) -> $stucsm {
                $stucsm {
                    data: self.data.sum_axis(Axis(1)),
                }
            }

            /// Returns the product of all the elements in the array
            pub fn product(&self) -> $typ {
                self.data.product()
            }

            /// Returns the mean of all the elements in the array
            pub fn mean(&self) -> $typ {
                self.data.mean().unwrap()
            }

            /// Returns the means of each row in the matrix
            #[wasm_bindgen(js_name = meanC)]
            pub fn mean_c(&self) -> $stucsm {
                $stucsm {
                    data: self.data.mean_axis(Axis(0)).unwrap(),
                }
            }

            /// Returns the means of each row in the matrix
            #[wasm_bindgen(js_name = meanR)]
            pub fn mean_r(&self) -> $stucsm {
                $stucsm {
                    data: self.data.mean_axis(Axis(1)).unwrap(),
                }
            }
        }
    };
}
