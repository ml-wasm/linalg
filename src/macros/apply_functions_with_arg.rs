#[macro_export]
macro_rules! apply_functions_with_arg {
	($stuc:ty, $argtyp:ty {
        $(
            $func:ident $(= $jsname:ident)? $doc:literal
        ),*
    }) => {
		#[wasm_bindgen]
        impl $stuc {
            $(
                $(#[wasm_bindgen(js_name = $jsname)])?
                #[doc=$doc]
                pub fn $func(&mut self, arg: $argtyp) {
                    self.data.par_map_inplace(|x| *x = x.$func(arg))
                }
            )*
        }
	};
}

#[macro_export]
macro_rules! apply_functions_with_arg_with_alias {
	($stuc:ty, $argtyp:ty {
        $(
            $func:ident | $rustfunc:ident $(= $jsname:ident)? $doc:literal
        ),*
    }) => {
		#[wasm_bindgen]
        impl $stuc {
            $(
                $(#[wasm_bindgen(js_name = $jsname)])?
                #[doc=$doc]
                pub fn $rustfunc(&mut self, arg: $argtyp) {
                    self.data.par_map_inplace(|x| *x = x.$func(arg))
                }
            )*
        }
	};
}
