use wasm_bindgen::prelude::*;

macro_rules! constants {
	($($name:ident: $type:ty = $value:expr);*$(;)?) => {
		$(
			#[wasm_bindgen]
			#[allow(non_snake_case)]
			pub fn $name() -> $type {
				$value
        	}
		)*
	};
}

constants!(
    INT_MIN: i32 = i32::MIN;
    INT_MAX: i32 = i32::MAX;
);

constants!(
    BYTE_MIN: u8 = u8::MIN;
    BYTE_MAX: u8 = u8::MAX;
);

constants!(
    FLOAT_EPSILON: f64 = f64::EPSILON;
    FLOAT_NAN: f64 = f64::NAN;
    FLOAT_INFINITY: f64 = f64::INFINITY;
    FLOAT_NEG_INFINITY: f64 = f64::NEG_INFINITY;
    FLOAT_MAX: f64 = f64::MAX;
    FLOAT_MIN: f64 = f64::MIN;
    FLOAT_MIN_POSITIVE: f64 = f64::MIN_POSITIVE;
    FLOAT_E: f64 = std::f64::consts::E;
    FLOAT_PI: f64 = std::f64::consts::PI;
    FLOAT_TAU: f64 = std::f64::consts::TAU;
);
