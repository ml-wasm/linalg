#[macro_export]
macro_rules! one_dimensional_math_methods {
    ($stuc:ident, $typ:ty) => {
        #[wasm_bindgen]
        impl $stuc {
            /// Add two arrays and return the result
            pub fn add(&self, other: &$stuc) -> Self {
                Self {
                    data: &self.data + &other.data,
                }
            }

            /// Substract two arrays and return the result
            pub fn sub(&self, other: &$stuc) -> Self {
                Self {
                    data: &self.data - &other.data,
                }
            }

            /// Multiply two Self's and return the result
            pub fn mul(&self, other: &$stuc) -> Self {
                Self {
                    data: &self.data * &other.data,
                }
            }

            /// Divide two Self's and return the result
            pub fn div(&self, other: &$stuc) -> Self {
                Self {
                    data: &self.data / &other.data,
                }
            }

            /// Efficiently performs self += alpha * rhs
            pub fn scaled_add(&mut self, alpha: $typ, other: &$stuc) {
                self.data.scaled_add(alpha, &other.data);
            }

            /// Get the sum of all the elements in the array
            pub fn sum(&self) -> $typ {
                self.data.sum()
            }

            /// Get the product of all the elements in the array
            pub fn product(&self) -> $typ {
                self.data.product()
            }
        }
    };
}
