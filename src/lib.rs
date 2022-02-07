pub mod constants;
pub mod macros;
pub mod matrices;
pub mod utils;
pub mod vectors;

use wasm_bindgen::prelude::*;
pub use wasm_bindgen_rayon::init_thread_pool;

// Run when the wasm module is instantiated
#[cfg(feature = "start")]
#[wasm_bindgen(start)]
pub fn start() {
    utils::set_panic_hook();
}
