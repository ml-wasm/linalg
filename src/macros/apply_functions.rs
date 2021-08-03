#[macro_export]
macro_rules! apply_functions {
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

#[macro_export]
macro_rules! apply_functions_with_alias {
	($stuc:ty {
        $(
            $func:ident | $rustfunc:ident $(= $jsname:ident)? $doc:literal
        ),*
    }) => {
		#[wasm_bindgen]
        impl $stuc {
            $(
                $(#[wasm_bindgen(js_name = $jsname)])?
                #[doc=$doc]
                pub fn $rustfunc(&mut self) {
                    self.data.par_map_inplace(|x| *x = x.$func())
                }
            )*
        }
	};
}
