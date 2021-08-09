#[macro_export]
macro_rules! vectors_sampling_methods {
    ($stuc:ty) => {
        #[wasm_bindgen]
        impl $stuc {
            /// Sample n elements from the given matrix
            pub fn sample(&self, n: usize, replacement: bool) -> Self {
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
        }
    };
}
