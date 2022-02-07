macro_rules! zero {
	($stuc:ty {
        $(
            $func:ident $(= $jsname:ident)? $doc:literal
        ),*
    }) => {
		#[wasm_bindgen]
        impl $stuc {
            $(
                $(#[wasm_bindgen(js_name = $jsname)])?
                #[doc=$doc]
                pub fn $func(&mut self) {
                    self.data.par_map_inplace(|x| *x = x.$func())
                }
            )*
        }
	};
}

macro_rules! zero_aliased {
	($stuc:ty {
        $(
            $func:ident - $rustname:ident $(= $jsname:ident)? $doc:literal
        ),*
    }) => {
		#[wasm_bindgen]
        impl $stuc {
            $(
                $(#[wasm_bindgen(js_name = $jsname)])?
                #[doc=$doc]
                pub fn $rustname(&mut self) {
                    self.data.par_map_inplace(|x| *x = x.$func())
                }
            )*
        }
	};
}

pub(crate) use zero;
pub(crate) use zero_aliased;

macro_rules! one {
	($stuc:ty, $arg:ty {
        $(
            $func:ident $(= $jsname:ident)? $doc:literal
        ),*
    }) => {
		#[wasm_bindgen]
        impl $stuc {
            $(
                $(#[wasm_bindgen(js_name = $jsname)])?
                #[doc=$doc]
                pub fn $func(&mut self, a: $arg) {
                    self.data.par_map_inplace(|x| *x = x.$func(a))
                }
            )*
        }
	};
}

macro_rules! one_aliased {
	($stuc:ty, $arg:ty {
        $(
            $func:ident - $rustname:ident $(= $jsname:ident)? $doc:literal
        ),*
    }) => {
		#[wasm_bindgen]
        impl $stuc {
            $(
                $(#[wasm_bindgen(js_name = $jsname)])?
                #[doc=$doc]
                pub fn $rustname(&mut self, a: $arg) {
                    self.data.par_map_inplace(|x| *x = x.$func(a))
                }
            )*
        }
	};
}

pub(crate) use one;
pub(crate) use one_aliased;

macro_rules! two {
	($stuc:ty, $arg1:ty, $arg2:ty {
        $(
            $func:ident $(= $jsname:ident)? $doc:literal
        ),*
    }) => {
		#[wasm_bindgen]
        impl $stuc {
            $(
                $(#[wasm_bindgen(js_name = $jsname)])?
                #[doc=$doc]
                pub fn $func(&mut self, a: $arg1, b: $arg2) {
                    self.data.par_map_inplace(|x| *x = x.$func(a, b))
                }
            )*
        }
	};
}

macro_rules! two_aliased {
	($stuc:ty, $arg1:ty, $arg2:ty {
        $(
            $func:ident - $rustname:ident $(= $jsname:ident)? $doc:literal
        ),*
    }) => {
		#[wasm_bindgen]
        impl $stuc {
            $(
                $(#[wasm_bindgen(js_name = $jsname)])?
                #[doc=$doc]
                pub fn $rustname(&mut self, a: $arg1, b: $arg2:ty) {
                    self.data.par_map_inplace(|x| *x = x.$func(a, b))
                }
            )*
        }
	};
}

pub(crate) use two;
pub(crate) use two_aliased;
