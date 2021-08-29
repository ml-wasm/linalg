use super::FloatsVector;
use ndarray::Array1;
use ndarray_csv::Array2Reader;
use ndarray_rand::{rand_distr, RandomExt};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl FloatsVector {
    /// Create a new FloatsVector of the given length filled with zeros
    #[wasm_bindgen(js_name = "newWithZeros")]
    pub fn new_with_zeros(len: usize) -> FloatsVector {
        FloatsVector {
            data: Array1::zeros([len]),
        }
    }

    /// Create a new FloatsVector of the given length filled with ones
    #[wasm_bindgen(js_name = "newWithOnes")]
    pub fn new_with_ones(len: usize) -> FloatsVector {
        FloatsVector {
            data: Array1::ones([len]),
        }
    }

    /// Create a new FloatsVector of the given length filled with the specified
    /// element
    #[wasm_bindgen(js_name = "newWithElement")]
    pub fn new_with_elem(len: usize, element: f64) -> FloatsVector {
        FloatsVector {
            data: Array1::from_elem([len], element),
        }
    }

    /// Create new Vector from csv
    #[wasm_bindgen(js_name = newFromCSV)]
    pub async fn new_from_csv(file : web_sys::File) -> FloatsVector {
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
                return FloatsVector::new(data.into_raw_vec());
            },
            Err(err) => {
                panic!("{}", err);
            }
        }
    }

    /// Create a new FloatsVector of the length calling the specified function
    /// without any arguments
    #[wasm_bindgen(js_name = "newWithSimpleFunc")]
    pub fn new_with_simple_func(len: usize, js_func: js_sys::Function) -> FloatsVector {
        FloatsVector {
            data: Array1::from_shape_simple_fn([len], move || {
                js_func.call0(&JsValue::NULL).unwrap().as_f64().unwrap()
            }),
        }
    }

    /// Create a new FloatsVector of the length calling the specified function
    /// with the index as the argument
    #[wasm_bindgen(js_name = "newWithFunc")]
    pub fn new_with_func(len: usize, js_func: js_sys::Function) -> FloatsVector {
        FloatsVector {
            data: Array1::from_shape_fn([len], move |idx| {
                js_func
                    .call1(&JsValue::NULL, &JsValue::from(idx as u32))
                    .unwrap()
                    .as_f64()
                    .unwrap()
            }),
        }
    }

    /// Create a new FloatsVector with n evenly spaced elements from start to end
    #[wasm_bindgen(js_name = "newWithLinspace")]
    pub fn new_with_linspace(start: f64, end: f64, n: usize) -> FloatsVector {
        FloatsVector {
            data: Array1::linspace(start, end, n),
        }
    }

    /// Create a new FloatsVector with elements from start to end incrementing by
    /// step
    #[wasm_bindgen(js_name = "newWithRange")]
    pub fn new_with_range(start: f64, end: f64, step: f64) -> FloatsVector {
        FloatsVector {
            data: Array1::range(start, end, step),
        }
    }

    /// Create a new FloatsVector with n logarithmically spaced elements from
    /// base^start to base^end
    #[wasm_bindgen(js_name = "newWithLogspace")]
    pub fn new_with_logspace(base: f64, start: f64, end: f64, n: usize) -> FloatsVector {
        FloatsVector {
            data: Array1::logspace(base, start, end, n),
        }
    }

    /// Create a new FloatsVector with n geometrically spaced elements from start to
    /// end
    #[wasm_bindgen(js_name = "newWithGeomspace")]
    pub fn new_with_geomspace(start: f64, end: f64, n: usize) -> FloatsVector {
        FloatsVector {
            data: Array1::geomspace(start, end, n).unwrap(),
        }
    }

    /// Create a new FloatsVector of specified length randomly in the range (0, 1]
    #[wasm_bindgen(js_name = "newWithRandom")]
    pub fn new_with_random(len: usize) -> FloatsVector {
        FloatsVector {
            data: Array1::random(len, rand_distr::Standard),
        }
    }

    /// Create a new FloatsVector of specified length randomly using Beta
    /// distribution with the given alpha and beta parameters
    #[wasm_bindgen(js_name = "newWithRandomBeta")]
    pub fn new_with_random_beta(len: usize, alpha: f64, beta: f64) -> FloatsVector {
        FloatsVector {
            data: Array1::random(len, rand_distr::Beta::new(alpha, beta).unwrap()),
        }
    }

    /// Create a new FloatsVector of specified length randomly using Cauchy
    /// distribution with the given median and scale factor
    #[wasm_bindgen(js_name = "newWithRandomCauchy")]
    pub fn new_with_random_cauchy(len: usize, median: f64, scale: f64) -> FloatsVector {
        FloatsVector {
            data: Array1::random(len, rand_distr::Cauchy::new(median, scale).unwrap()),
        }
    }

    /// Create a new FloatsVector of specified length randomly using ChiSquared
    /// distribution with k degrees of freedom
    #[wasm_bindgen(js_name = "newWithRandomChiSquared")]
    pub fn new_with_random_chi_squared(len: usize, k: f64) -> FloatsVector {
        FloatsVector {
            data: Array1::random(len, rand_distr::ChiSquared::new(k).unwrap()),
        }
    }

    /// Create a new FloatsVector of specified length randomly using exponential
    /// distribution with the given shape parameter lambda
    #[wasm_bindgen(js_name = "newWithRandomExp")]
    pub fn new_with_random_exp(len: usize, lambda: f64) -> FloatsVector {
        FloatsVector {
            data: Array1::random(len, rand_distr::Exp::new(lambda).unwrap()),
        }
    }

    /// Create a new FloatsVector of specified length randomly using exponential
    /// distribution with the given shape parameter lambda = 1
    #[wasm_bindgen(js_name = "newWithRandomExp1")]
    pub fn new_with_random_exp1(len: usize) -> FloatsVector {
        FloatsVector {
            data: Array1::random(len, rand_distr::Exp1),
        }
    }

    /// Create a new FloatsVector of specified length randomly using Fischer
    /// distribution with m and n as parameters
    #[wasm_bindgen(js_name = "newWithRandomFisher")]
    pub fn new_with_random_fisher(len: usize, m: f64, n: f64) -> FloatsVector {
        FloatsVector {
            data: Array1::random(len, rand_distr::FisherF::new(m, n).unwrap()),
        }
    }

    /// Create a new FloatsVector of specified length randomly using Gamma
    /// distribution with the given shape and scale
    #[wasm_bindgen(js_name = "newWithRandomGamma")]
    pub fn new_with_random_gamma(len: usize, shape: f64, scale: f64) -> FloatsVector {
        FloatsVector {
            data: Array1::random(len, rand_distr::Gamma::new(shape, scale).unwrap()),
        }
    }

    /// Create a new FloatsVector of specified length randomly using InverseGaussian
    /// distribution with the given mean and scale
    #[wasm_bindgen(js_name = "newWithRandomInverseGaussian")]
    pub fn new_with_random_inverse_gaussian(len: usize, mean: f64, shape: f64) -> FloatsVector {
        FloatsVector {
            data: Array1::random(len, rand_distr::InverseGaussian::new(mean, shape).unwrap()),
        }
    }

    /// Create a new FloatsVector of specified length randomly using InverseGaussian
    /// distribution with mu - mean of the underlying distribution, sigma -
    /// standard deviation of the underlying normal distribution
    #[wasm_bindgen(js_name = "newWithRandomLogNormal")]
    pub fn new_with_random_log_normal(len: usize, mu: f64, sigma: f64) -> FloatsVector {
        FloatsVector {
            data: Array1::random(len, rand_distr::LogNormal::new(mu, sigma).unwrap()),
        }
    }

    /// Create a new FloatsVector of specified length randomly using Normal
    /// distribution with mu - mean of the distribution, sigma - standard
    /// deviation of the normal distribution
    #[wasm_bindgen(js_name = "newWithRandomNormal")]
    pub fn new_with_random_normal(len: usize, mu: f64, sigma: f64) -> FloatsVector {
        FloatsVector {
            data: Array1::random(len, rand_distr::Normal::new(mu, sigma).unwrap()),
        }
    }

    /// Create a new FloatsVector of specified length randomly using Normal
    /// distribution with mu - mean of the distribution = 0.0, sigma - standard
    /// deviation of the normal distribution = 1.0
    #[wasm_bindgen(js_name = "newWithRandomStandardNormal")]
    pub fn new_with_random_standard_normal(len: usize) -> FloatsVector {
        FloatsVector {
            data: Array1::random(len, rand_distr::StandardNormal),
        }
    }

    /// Create a new FloatsVector of specified length randomly using Normal-Inverse
    /// Gaussian distribution with alpha and beta parameters
    #[wasm_bindgen(js_name = "newWithRandomNormalInverseGaussian")]
    pub fn new_with_random_normal_inverse_gaussian(
        len: usize,
        alpha: f64,
        beta: f64,
    ) -> FloatsVector {
        FloatsVector {
            data: Array1::random(
                len,
                rand_distr::NormalInverseGaussian::new(alpha, beta).unwrap(),
            ),
        }
    }

    /// Create a new FloatsVector of specified length randomly uniformaly distributed
    /// in the open interval (0, 1)
    #[wasm_bindgen(js_name = "newWithRandomOpen01")]
    pub fn new_with_random_open_01(len: usize) -> FloatsVector {
        FloatsVector {
            data: Array1::random(len, rand_distr::OpenClosed01),
        }
    }

    /// Create a new FloatsVector of specified length randomly uniformaly distributed
    /// in the half open interval (0, 1]
    #[wasm_bindgen(js_name = "newWithRandomOpenClosed01")]
    pub fn new_with_random_open_closed_01(len: usize) -> FloatsVector {
        FloatsVector {
            data: Array1::random(len, rand_distr::OpenClosed01),
        }
    }

    /// Create a new FloatsVector of specified length randomly using Pareto
    /// distribution with the given scale and shape
    #[wasm_bindgen(js_name = "newWithRandomPareto")]
    pub fn new_with_random_pareto(len: usize, scale: f64, shape: f64) -> FloatsVector {
        FloatsVector {
            data: Array1::random(len, rand_distr::Pareto::new(scale, shape).unwrap()),
        }
    }

    /// Create a new FloatsVector of specified length randomly using PERT
    /// distribution with the given min, max and mode and shape == 4.0
    #[wasm_bindgen(js_name = "newWithRandomPERT")]
    pub fn new_with_random_pert(len: usize, min: f64, max: f64, mode: f64) -> FloatsVector {
        FloatsVector {
            data: Array1::random(len, rand_distr::Pert::new(min, max, mode).unwrap()),
        }
    }

    /// Create a new FloatsVector of specified length randomly using PERT
    /// distribution with the given min, max, mode and shape
    #[wasm_bindgen(js_name = "newWithRandomPERTwithShape")]
    pub fn new_with_random_pert_with_shape(
        len: usize,
        min: f64,
        max: f64,
        mode: f64,
        shape: f64,
    ) -> FloatsVector {
        FloatsVector {
            data: Array1::random(
                len,
                rand_distr::Pert::new_with_shape(min, max, mode, shape).unwrap(),
            ),
        }
    }

    /// Create a new FloatsVector of specified length randomly using Poisson
    /// distribution with the shape parameter lambda
    #[wasm_bindgen(js_name = "newWithRandomPoisson")]
    pub fn new_with_random_poisson(len: usize, lambda: f64) -> FloatsVector {
        FloatsVector {
            data: Array1::random(len, rand_distr::Poisson::new(lambda).unwrap()),
        }
    }

    /// Create a new FloatsVector of specified length randomly using Student t
    /// distribution with n degrees of freedom
    #[wasm_bindgen(js_name = "newWithRandomStudentT")]
    pub fn new_with_random_student_t(len: usize, n: f64) -> FloatsVector {
        FloatsVector {
            data: Array1::random(len, rand_distr::StudentT::new(n).unwrap()),
        }
    }

    /// Create a new FloatsVector of specified length randomly using Triangular
    /// distribution with the given min, max and mode
    #[wasm_bindgen(js_name = "newWithRandomTriangular")]
    pub fn new_with_random_triangular(len: usize, min: f64, max: f64, mode: f64) -> FloatsVector {
        FloatsVector {
            data: Array1::random(len, rand_distr::Triangular::new(min, max, mode).unwrap()),
        }
    }

    /// Create a new FloatsVector of specified length randomly using Uniform
    /// distribution in the range start to end
    #[wasm_bindgen(js_name = "newWithRandomUniform")]
    pub fn new_with_random_uniform(len: usize, start: f64, end: f64) -> FloatsVector {
        FloatsVector {
            data: Array1::random(len, rand_distr::Uniform::new(start, end)),
        }
    }

    /// Create a new FloatsVector of specified length randomly using Weibull
    /// distribution in the given scale and shape
    #[wasm_bindgen(js_name = "newWithRandomWeibull")]
    pub fn new_with_random_weibull(len: usize, scale: f64, shape: f64) -> FloatsVector {
        FloatsVector {
            data: Array1::random(len, rand_distr::Weibull::new(scale, shape).unwrap()),
        }
    }
}
