use super::FloatsMatrix;
use ndarray::Array2;
use ndarray_rand::{rand_distr, RandomExt};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl FloatsMatrix {
    /// Create a new FloatsMatrix of specified length randomly in the range (0, 1]
    #[wasm_bindgen(js_name = "newWithRandom")]
    pub fn new_with_random(row: usize, col: usize) -> FloatsMatrix {
        FloatsMatrix {
            data: Array2::random((row, col), rand_distr::Standard),
        }
    }

    /// Create a new FloatsMatrix of specified length randomly using Beta
    /// distribution with the given alpha and beta parameters
    #[wasm_bindgen(js_name = "newWithRandomBeta")]
    pub fn new_with_random_beta(row: usize, col: usize, alpha: f64, beta: f64) -> FloatsMatrix {
        FloatsMatrix {
            data: Array2::random((row, col), rand_distr::Beta::new(alpha, beta).unwrap()),
        }
    }

    /// Create a new FloatsMatrix of specified length randomly using Cauchy
    /// distribution with the given median and scale factor
    #[wasm_bindgen(js_name = "newWithRandomCauchy")]
    pub fn new_with_random_cauchy(row: usize, col: usize, median: f64, scale: f64) -> FloatsMatrix {
        FloatsMatrix {
            data: Array2::random((row, col), rand_distr::Cauchy::new(median, scale).unwrap()),
        }
    }

    /// Create a new FloatsMatrix of specified length randomly using ChiSquared
    /// distribution with k degrees of freedom
    #[wasm_bindgen(js_name = "newWithRandomChiSquared")]
    pub fn new_with_random_chi_squared(row: usize, col: usize, k: f64) -> FloatsMatrix {
        FloatsMatrix {
            data: Array2::random((row, col), rand_distr::ChiSquared::new(k).unwrap()),
        }
    }

    /// Create a new FloatsMatrix of specified length randomly using exponential
    /// distribution with the given shape parameter lambda
    #[wasm_bindgen(js_name = "newWithRandomExp")]
    pub fn new_with_random_exp(row: usize, col: usize, lambda: f64) -> FloatsMatrix {
        FloatsMatrix {
            data: Array2::random((row, col), rand_distr::Exp::new(lambda).unwrap()),
        }
    }

    /// Create a new FloatsMatrix of specified length randomly using exponential
    /// distribution with the given shape parameter lambda = 1
    #[wasm_bindgen(js_name = "newWithRandomExp1")]
    pub fn new_with_random_exp1(row: usize, col: usize) -> FloatsMatrix {
        FloatsMatrix {
            data: Array2::random((row, col), rand_distr::Exp1),
        }
    }

    /// Create a new FloatsMatrix of specified length randomly using Fischer
    /// distribution with m and n as parameters
    #[wasm_bindgen(js_name = "newWithRandomFisher")]
    pub fn new_with_random_fisher(row: usize, col: usize, m: f64, n: f64) -> FloatsMatrix {
        FloatsMatrix {
            data: Array2::random((row, col), rand_distr::FisherF::new(m, n).unwrap()),
        }
    }

    /// Create a new FloatsMatrix of specified length randomly using Gamma
    /// distribution with the given shape and scale
    #[wasm_bindgen(js_name = "newWithRandomGamma")]
    pub fn new_with_random_gamma(row: usize, col: usize, shape: f64, scale: f64) -> FloatsMatrix {
        FloatsMatrix {
            data: Array2::random((row, col), rand_distr::Gamma::new(shape, scale).unwrap()),
        }
    }

    /// Create a new FloatsMatrix of specified length randomly using InverseGaussian
    /// distribution with the given mean and scale
    #[wasm_bindgen(js_name = "newWithRandomInverseGaussian")]
    pub fn new_with_random_inverse_gaussian(
        row: usize,
        col: usize,
        mean: f64,
        shape: f64,
    ) -> FloatsMatrix {
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
    pub fn new_with_random_log_normal(row: usize, col: usize, mu: f64, sigma: f64) -> FloatsMatrix {
        FloatsMatrix {
            data: Array2::random((row, col), rand_distr::LogNormal::new(mu, sigma).unwrap()),
        }
    }

    /// Create a new FloatsMatrix of specified length randomly using Normal
    /// distribution with mu - mean of the distribution, sigma - standard
    /// deviation of the normal distribution
    #[wasm_bindgen(js_name = "newWithRandomNormal")]
    pub fn new_with_random_normal(row: usize, col: usize, mu: f64, sigma: f64) -> FloatsMatrix {
        FloatsMatrix {
            data: Array2::random((row, col), rand_distr::Normal::new(mu, sigma).unwrap()),
        }
    }

    /// Create a new FloatsMatrix of specified length randomly using Normal
    /// distribution with mu - mean of the distribution = 0.0, sigma - standard
    /// deviation of the normal distribution = 1.0
    #[wasm_bindgen(js_name = "newWithRandomStandardNormal")]
    pub fn new_with_random_standard_normal(row: usize, col: usize) -> FloatsMatrix {
        FloatsMatrix {
            data: Array2::random((row, col), rand_distr::StandardNormal),
        }
    }

    /// Create a new FloatsMatrix of specified length randomly using Normal-Inverse
    /// Gaussian distribution with alpha and beta parameters
    #[wasm_bindgen(js_name = "newWithRandomNormalInverseGaussian")]
    pub fn new_with_random_normal_inverse_gaussian(
        row: usize,
        col: usize,
        alpha: f64,
        beta: f64,
    ) -> FloatsMatrix {
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
    pub fn new_with_random_open_01(row: usize, col: usize) -> FloatsMatrix {
        FloatsMatrix {
            data: Array2::random((row, col), rand_distr::OpenClosed01),
        }
    }

    /// Create a new FloatsMatrix of specified length randomly uniformaly distributed
    /// in the half open interval (0, 1]
    #[wasm_bindgen(js_name = "newWithRandomOpenClosed01")]
    pub fn new_with_random_open_closed_01(row: usize, col: usize) -> FloatsMatrix {
        FloatsMatrix {
            data: Array2::random((row, col), rand_distr::OpenClosed01),
        }
    }

    /// Create a new FloatsMatrix of specified length randomly using Pareto
    /// distribution with the given scale and shape
    #[wasm_bindgen(js_name = "newWithRandomPareto")]
    pub fn new_with_random_pareto(row: usize, col: usize, scale: f64, shape: f64) -> FloatsMatrix {
        FloatsMatrix {
            data: Array2::random((row, col), rand_distr::Pareto::new(scale, shape).unwrap()),
        }
    }

    /// Create a new FloatsMatrix of specified length randomly using PERT
    /// distribution with the given min, max and mode and shape == 4.0
    #[wasm_bindgen(js_name = "newWithRandomPERT")]
    pub fn new_with_random_pert(
        row: usize,
        col: usize,
        min: f64,
        max: f64,
        mode: f64,
    ) -> FloatsMatrix {
        FloatsMatrix {
            data: Array2::random((row, col), rand_distr::Pert::new(min, max, mode).unwrap()),
        }
    }

    /// Create a new FloatsMatrix of specified length randomly using PERT
    /// distribution with the given min, max, mode and shape
    #[wasm_bindgen(js_name = "newWithRandomPERTwithShape")]
    pub fn new_with_random_pert_with_shape(
        row: usize,
        col: usize,
        min: f64,
        max: f64,
        mode: f64,
        shape: f64,
    ) -> FloatsMatrix {
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
    pub fn new_with_random_poisson(row: usize, col: usize, lambda: f64) -> FloatsMatrix {
        FloatsMatrix {
            data: Array2::random((row, col), rand_distr::Poisson::new(lambda).unwrap()),
        }
    }

    /// Create a new FloatsMatrix of specified length randomly using Student t
    /// distribution with n degrees of freedom
    #[wasm_bindgen(js_name = "newWithRandomStudentT")]
    pub fn new_with_random_student_t(row: usize, col: usize, n: f64) -> FloatsMatrix {
        FloatsMatrix {
            data: Array2::random((row, col), rand_distr::StudentT::new(n).unwrap()),
        }
    }

    /// Create a new FloatsMatrix of specified length randomly using Triangular
    /// distribution with the given min, max and mode
    #[wasm_bindgen(js_name = "newWithRandomTriangular")]
    pub fn new_with_random_triangular(
        row: usize,
        col: usize,
        min: f64,
        max: f64,
        mode: f64,
    ) -> FloatsMatrix {
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
    pub fn new_with_random_uniform(row: usize, col: usize, start: f64, end: f64) -> FloatsMatrix {
        FloatsMatrix {
            data: Array2::random((row, col), rand_distr::Uniform::new(start, end)),
        }
    }

    /// Create a new FloatsMatrix of specified length randomly using Weibull
    /// distribution in the given scale and shape
    #[wasm_bindgen(js_name = "newWithRandomWeibull")]
    pub fn new_with_random_weibull(row: usize, col: usize, scale: f64, shape: f64) -> FloatsMatrix {
        FloatsMatrix {
            data: Array2::random((row, col), rand_distr::Weibull::new(scale, shape).unwrap()),
        }
    }
}