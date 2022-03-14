macro_rules! sampling {
    ($stuc:ty) => {
        #[wasm_bindgen]
        impl $stuc {
            /// Sample n rows from the given matrix
            #[wasm_bindgen(js_name = sampleR)]
            pub fn sample_r(&self, n: usize, replacement: bool) -> Self {
                let strategy = if replacement {
                    ndarray_rand::SamplingStrategy::WithReplacement
                } else {
                    ndarray_rand::SamplingStrategy::WithReplacement
                };

                Self {
                    data: ndarray_rand::RandomExt::sample_axis(
                        &self.data,
                        ndarray::Axis(0),
                        n,
                        strategy,
                    ),
                }
            }

            /// Sample n columns from the given matrix
            #[wasm_bindgen(js_name = sampleC)]
            pub fn sample_c(&self, n: usize, replacement: bool) -> Self {
                let strategy = if replacement {
                    ndarray_rand::SamplingStrategy::WithReplacement
                } else {
                    ndarray_rand::SamplingStrategy::WithReplacement
                };

                Self {
                    data: ndarray_rand::RandomExt::sample_axis(
                        &self.data,
                        ndarray::Axis(1),
                        n,
                        strategy,
                    ),
                }
            }
        }
    };
}

pub(crate) use sampling;
