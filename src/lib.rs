pub mod one_dimensional;
pub mod two_dimensional;
pub mod utils;

use wasm_bindgen::prelude::*;

pub use wasm_bindgen_rayon::init_thread_pool;

// Run when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn start() {
    utils::set_panic_hook();
}
