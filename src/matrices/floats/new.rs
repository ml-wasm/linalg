use super::FloatsMatrix;
use ndarray::Array2;
use ndarray_csv::Array2Reader;
use ndarray_rand::{rand_distr, RandomExt};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl FloatsMatrix {
    /// Create new matrix of given size with give element
    #[wasm_bindgen(js_name = newWithElement)]
    pub fn new_with_elem(num_rows: usize, num_cols: usize, element: f64) -> FloatsMatrix {
        FloatsMatrix {
            data: Array2::from_elem([num_rows, num_cols], element)
        }
    }

    /// Create new matrix of given size initialize to 0
    #[wasm_bindgen(js_name = newWithZeroes)]
    pub fn new_with_zeros(num_rows: usize, num_cols: usize) -> FloatsMatrix {
        FloatsMatrix {
            data: Array2::zeros([num_rows, num_cols])
        }
    }

    /// Create new matrix of give size initialized to 1
    #[wasm_bindgen(js_name = newWithOnes)]
    pub fn new_with_ones(num_rows: usize, num_cols: usize) -> FloatsMatrix {
        FloatsMatrix {
            data: Array2::ones([num_rows, num_cols])
        }
    }
    
    /// Create new identity matrix of size n
    #[wasm_bindgen(js_name = newEye)]
    pub fn new_eye(size: usize) -> FloatsMatrix {
        FloatsMatrix {
            data: Array2::eye(size)
        }
    }
    
    /// Create new FloatsMatrix from csv file 
    #[wasm_bindgen(js_name = newFromCSV)]
    pub async fn new_from_csv(file : web_sys::File) -> FloatsMatrix {
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
                return FloatsMatrix {
                    data
                };
            },
            Err(err) => {
                panic!("{}", err);
            }
        }
    }

    /// Create a new FloatsMatrix of specified length randomly in the range (0, 1]
    #[wasm_bindgen(js_name = "newWithRandom")]
    pub fn new_with_random(size: js_sys::Array) -> FloatsMatrix {
        let row = size.get(0).as_f64().unwrap() as usize;
        let col = size.get(1).as_f64().unwrap() as usize;
        FloatsMatrix {
            data: Array2::random((row, col), rand_distr::Standard),
        }
    }

    /// Create a new FloatsMatrix of specified length randomly using Beta
    /// distribution with the given alpha and beta parameters
    #[wasm_bindgen(js_name = "newWithRandomBeta")]
    pub fn new_with_random_beta(size: js_sys::Array, alpha: f64, beta: f64) -> FloatsMatrix {
        let row = size.get(0).as_f64().unwrap() as usize;
        let col = size.get(1).as_f64().unwrap() as usize;
        FloatsMatrix {
            data: Array2::random((row, col), rand_distr::Beta::new(alpha, beta).unwrap()),
        }
    }

    /// Create a new FloatsMatrix of specified length randomly using Cauchy
    /// distribution with the given median and scale factor
    #[wasm_bindgen(js_name = "newWithRandomCauchy")]
    pub fn new_with_random_cauchy(size: js_sys::Array, median: f64, scale: f64) -> FloatsMatrix {
        let row = size.get(0).as_f64().unwrap() as usize;
        let col = size.get(1).as_f64().unwrap() as usize;
        FloatsMatrix {
            data: Array2::random((row, col), rand_distr::Cauchy::new(median, scale).unwrap()),
        }
    }

    /// Create a new FloatsMatrix of specified length randomly using ChiSquared
    /// distribution with k degrees of freedom
    #[wasm_bindgen(js_name = "newWithRandomChiSquared")]
    pub fn new_with_random_chi_squared(size: js_sys::Array, k: f64) -> FloatsMatrix {
        let row = size.get(0).as_f64().unwrap() as usize;
        let col = size.get(1).as_f64().unwrap() as usize;
        FloatsMatrix {
            data: Array2::random((row, col), rand_distr::ChiSquared::new(k).unwrap()),
        }
    }

    /// Create a new FloatsMatrix of specified length randomly using exponential
    /// distribution with the given shape parameter lambda
    #[wasm_bindgen(js_name = "newWithRandomExp")]
    pub fn new_with_random_exp(size: js_sys::Array, lambda: f64) -> FloatsMatrix {
        let row = size.get(0).as_f64().unwrap() as usize;
        let col = size.get(1).as_f64().unwrap() as usize;
        FloatsMatrix {
            data: Array2::random((row, col), rand_distr::Exp::new(lambda).unwrap()),
        }
    }

    /// Create a new FloatsMatrix of specified length randomly using exponential
    /// distribution with the given shape parameter lambda = 1
    #[wasm_bindgen(js_name = "newWithRandomExp1")]
    pub fn new_with_random_exp1(size: js_sys::Array) -> FloatsMatrix {
        let row = size.get(0).as_f64().unwrap() as usize;
        let col = size.get(1).as_f64().unwrap() as usize;
        FloatsMatrix {
            data: Array2::random((row, col), rand_distr::Exp1),
        }
    }

    /// Create a new FloatsMatrix of specified length randomly using Fischer
    /// distribution with m and n as parameters
    #[wasm_bindgen(js_name = "newWithRandomFisher")]
    pub fn new_with_random_fisher(size: js_sys::Array, m: f64, n: f64) -> FloatsMatrix {
        let row = size.get(0).as_f64().unwrap() as usize;
        let col = size.get(1).as_f64().unwrap() as usize;
        FloatsMatrix {
            data: Array2::random((row, col), rand_distr::FisherF::new(m, n).unwrap()),
        }
    }

    /// Create a new FloatsMatrix of specified length randomly using Gamma
    /// distribution with the given shape and scale
    #[wasm_bindgen(js_name = "newWithRandomGamma")]
    pub fn new_with_random_gamma(size: js_sys::Array, shape: f64, scale: f64) -> FloatsMatrix {
        let row = size.get(0).as_f64().unwrap() as usize;
        let col = size.get(1).as_f64().unwrap() as usize;
        FloatsMatrix {
            data: Array2::random((row, col), rand_distr::Gamma::new(shape, scale).unwrap()),
        }
    }

    /// Create a new FloatsMatrix of specified length randomly using InverseGaussian
    /// distribution with the given mean and scale
    #[wasm_bindgen(js_name = "newWithRandomInverseGaussian")]
    pub fn new_with_random_inverse_gaussian(
        size: js_sys::Array,
        mean: f64,
        shape: f64,
    ) -> FloatsMatrix {
        let row = size.get(0).as_f64().unwrap() as usize;
        let col = size.get(1).as_f64().unwrap() as usize;
        FloatsMatrix {
            data: Array2::random(
                (row, col),
                rand_distr::InverseGaussian::new(mean, shape).unwrap(),
            ),
        }
    }

    /// Create a new FloatsMatrix of specified length randomly using InverseGaussian
    /// distribution with mu - mean of the underlying distribution, sigma -
    /// standard deviation of the underlying normal distribution
    #[wasm_bindgen(js_name = "newWithRandomLogNormal")]
    pub fn new_with_random_log_normal(size: js_sys::Array, mu: f64, sigma: f64) -> FloatsMatrix {
        let row = size.get(0).as_f64().unwrap() as usize;
        let col = size.get(1).as_f64().unwrap() as usize;
        FloatsMatrix {
            data: Array2::random((row, col), rand_distr::LogNormal::new(mu, sigma).unwrap()),
        }
    }

    /// Create a new FloatsMatrix of specified length randomly using Normal
    /// distribution with mu - mean of the distribution, sigma - standard
    /// deviation of the normal distribution
    #[wasm_bindgen(js_name = "newWithRandomNormal")]
    pub fn new_with_random_normal(size: js_sys::Array, mu: f64, sigma: f64) -> FloatsMatrix {
        let row = size.get(0).as_f64().unwrap() as usize;
        let col = size.get(1).as_f64().unwrap() as usize;
        FloatsMatrix {
            data: Array2::random((row, col), rand_distr::Normal::new(mu, sigma).unwrap()),
        }
    }

    /// Create a new FloatsMatrix of specified length randomly using Normal
    /// distribution with mu - mean of the distribution = 0.0, sigma - standard
    /// deviation of the normal distribution = 1.0
    #[wasm_bindgen(js_name = "newWithRandomStandardNormal")]
    pub fn new_with_random_standard_normal(size: js_sys::Array) -> FloatsMatrix {
        let row = size.get(0).as_f64().unwrap() as usize;
        let col = size.get(1).as_f64().unwrap() as usize;
        FloatsMatrix {
            data: Array2::random((row, col), rand_distr::StandardNormal),
        }
    }

    /// Create a new FloatsMatrix of specified length randomly using Normal-Inverse
    /// Gaussian distribution with alpha and beta parameters
    #[wasm_bindgen(js_name = "newWithRandomNormalInverseGaussian")]
    pub fn new_with_random_normal_inverse_gaussian(
        size: js_sys::Array,
        alpha: f64,
        beta: f64,
    ) -> FloatsMatrix {
        let row = size.get(0).as_f64().unwrap() as usize;
        let col = size.get(1).as_f64().unwrap() as usize;
        FloatsMatrix {
            data: Array2::random(
                (row, col),
                rand_distr::NormalInverseGaussian::new(alpha, beta).unwrap(),
            ),
        }
    }

    /// Create a new FloatsMatrix of specified length randomly uniformaly distributed
    /// in the open interval (0, 1)
    #[wasm_bindgen(js_name = "newWithRandomOpen01")]
    pub fn new_with_random_open_01(size: js_sys::Array) -> FloatsMatrix {
        let row = size.get(0).as_f64().unwrap() as usize;
        let col = size.get(1).as_f64().unwrap() as usize;
        FloatsMatrix {
            data: Array2::random((row, col), rand_distr::OpenClosed01),
        }
    }

    /// Create a new FloatsMatrix of specified length randomly uniformaly distributed
    /// in the half open interval (0, 1]
    #[wasm_bindgen(js_name = "newWithRandomOpenClosed01")]
    pub fn new_with_random_open_closed_01(size: js_sys::Array) -> FloatsMatrix {
        let row = size.get(0).as_f64().unwrap() as usize;
        let col = size.get(1).as_f64().unwrap() as usize;
        FloatsMatrix {
            data: Array2::random((row, col), rand_distr::OpenClosed01),
        }
    }

    /// Create a new FloatsMatrix of specified length randomly using Pareto
    /// distribution with the given scale and shape
    #[wasm_bindgen(js_name = "newWithRandomPareto")]
    pub fn new_with_random_pareto(size: js_sys::Array, scale: f64, shape: f64) -> FloatsMatrix {
        let row = size.get(0).as_f64().unwrap() as usize;
        let col = size.get(1).as_f64().unwrap() as usize;
        FloatsMatrix {
            data: Array2::random((row, col), rand_distr::Pareto::new(scale, shape).unwrap()),
        }
    }

    /// Create a new FloatsMatrix of specified length randomly using PERT
    /// distribution with the given min, max and mode and shape == 4.0
    #[wasm_bindgen(js_name = "newWithRandomPERT")]
    pub fn new_with_random_pert(
        size: js_sys::Array,
        min: f64,
        max: f64,
        mode: f64,
    ) -> FloatsMatrix {
        let row = size.get(0).as_f64().unwrap() as usize;
        let col = size.get(1).as_f64().unwrap() as usize;
        FloatsMatrix {
            data: Array2::random((row, col), rand_distr::Pert::new(min, max, mode).unwrap()),
        }
    }

    /// Create a new FloatsMatrix of specified length randomly using PERT
    /// distribution with the given min, max, mode and shape
    #[wasm_bindgen(js_name = "newWithRandomPERTwithShape")]
    pub fn new_with_random_pert_with_shape(
        size: js_sys::Array,
        min: f64,
        max: f64,
        mode: f64,
        shape: f64,
    ) -> FloatsMatrix {
        let row = size.get(0).as_f64().unwrap() as usize;
        let col = size.get(1).as_f64().unwrap() as usize;
        FloatsMatrix {
            data: Array2::random(
                (row, col),
                rand_distr::Pert::new_with_shape(min, max, mode, shape).unwrap(),
            ),
        }
    }

    /// Create a new FloatsMatrix of specified length randomly using Poisson
    /// distribution with the shape parameter lambda
    #[wasm_bindgen(js_name = "newWithRandomPoisson")]
    pub fn new_with_random_poisson(size: js_sys::Array, lambda: f64) -> FloatsMatrix {
        let row = size.get(0).as_f64().unwrap() as usize;
        let col = size.get(1).as_f64().unwrap() as usize;
        FloatsMatrix {
            data: Array2::random((row, col), rand_distr::Poisson::new(lambda).unwrap()),
        }
    }

    /// Create a new FloatsMatrix of specified length randomly using Student t
    /// distribution with n degrees of freedom
    #[wasm_bindgen(js_name = "newWithRandomStudentT")]
    pub fn new_with_random_student_t(size: js_sys::Array, n: f64) -> FloatsMatrix {
        let row = size.get(0).as_f64().unwrap() as usize;
        let col = size.get(1).as_f64().unwrap() as usize;
        FloatsMatrix {
            data: Array2::random((row, col), rand_distr::StudentT::new(n).unwrap()),
        }
    }

    /// Create a new FloatsMatrix of specified length randomly using Triangular
    /// distribution with the given min, max and mode
    #[wasm_bindgen(js_name = "newWithRandomTriangular")]
    pub fn new_with_random_triangular(
        size: js_sys::Array,
        min: f64,
        max: f64,
        mode: f64,
    ) -> FloatsMatrix {
        let row = size.get(0).as_f64().unwrap() as usize;
        let col = size.get(1).as_f64().unwrap() as usize;
        FloatsMatrix {
            data: Array2::random(
                (row, col),
                rand_distr::Triangular::new(min, max, mode).unwrap(),
            ),
        }
    }

    /// Create a new FloatsMatrix of specified length randomly using Uniform
    /// distribution in the range start to end
    #[wasm_bindgen(js_name = "newWithRandomUniform")]
    pub fn new_with_random_uniform(size: js_sys::Array, start: f64, end: f64) -> FloatsMatrix {
        let row = size.get(0).as_f64().unwrap() as usize;
        let col = size.get(1).as_f64().unwrap() as usize;
        FloatsMatrix {
            data: Array2::random((row, col), rand_distr::Uniform::new(start, end)),
        }
    }

    /// Create a new FloatsMatrix of specified length randomly using Weibull
    /// distribution in the given scale and shape
    #[wasm_bindgen(js_name = "newWithRandomWeibull")]
    pub fn new_with_random_weibull(size: js_sys::Array, scale: f64, shape: f64) -> FloatsMatrix {
        let row = size.get(0).as_f64().unwrap() as usize;
        let col = size.get(1).as_f64().unwrap() as usize;
        FloatsMatrix {
            data: Array2::random((row, col), rand_distr::Weibull::new(scale, shape).unwrap()),
        }
    }
}
