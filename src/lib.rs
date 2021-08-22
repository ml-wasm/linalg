pub mod constants;
pub mod macros;
pub mod matrices;
pub mod utils;
pub mod vectors;

pub use wasm_bindgen_rayon::init_thread_pool;

// Run when the wasm module is instantiated
#[cfg(feature = "start")]
#[wasm_bindgen(start)]
pub fn start() {
    utils::set_panic_hook();
}

pub enum Vector {
    Integers(vectors::integers::IntegersVector),
    Floats(vectors::floats::FloatsVector),
    Strings(vectors::strings::StringsVector),
}

pub enum Matrix {
    Integers(matrices::integers::IntegersMatrix),
    Floats(matrices::floats::FloatsMatrix),
    Strings(matrices::strings::StringsMatrix),
}

pub enum Linalg {
    Vector(Vector),
    Matrix(Matrix),
}
