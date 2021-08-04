#[macro_export]
macro_rules! apply_functions_with_two_args {
	($stuc:ty, $argtyp1:ty, $argtyp2:ty {
        $(
            $func:ident $(= $jsname:ident)? $doc:literal
        ),*
    }) => {
		#[wasm_bindgen]
        impl $stuc {
            $(
                $(#[wasm_bindgen(js_name = $jsname)])?
                #[doc=$doc]
                pub fn $func(&mut self, a: $argtyp1, b: $argtyp2) {
                    self.data.par_map_inplace(|x| *x = x.$func(a, b))
                }
            )*
        }
	};
}

#[macro_export]
macro_rules! apply_functions_with_two_args_with_alias {
	($stuc:ty, $argtyp1:ty, $argtyp2:ty {
        $(
            $func:ident | $rustfunc:ident $(= $jsname:ident)? $doc:literal
        ),*
    }) => {
		#[wasm_bindgen]
        impl $stuc {
            $(
                $(#[wasm_bindgen(js_name = $jsname)])?
                #[doc=$doc]
                pub fn $rustfunc(&mut self, a: $argtyp1, b: $argtyp2) {
                    self.data.par_map_inplace(|x| *x = x.$func(a, b))
                }
            )*
        }
	};
}
