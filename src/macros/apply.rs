macro_rules! zero {
	($stuc:ty {
        $(
            $func:ident  $doc:literal
        ),*
    }) => {
        paste::paste! {
            #[wasm_bindgen]
            impl $stuc {
                $(
                    #[wasm_bindgen(js_name = $func:camel:uncapitalize)]
                    #[doc=$doc]
                    pub fn $func(mut self) -> Self {
                        self.data.map_inplace(|x| *x = x.$func());
                        self
                    }

                    #[wasm_bindgen(js_name = $func:camel:uncapitalize _)]
                    #[doc=$doc]
                    pub fn [<$func _par>](mut self) -> Self {
                        self.data.par_map_inplace(|x| *x = x.$func());
                        self
                    }
                )*
            }
        }
	};
	($stuc:ty {
        $(
            $func:ident = $alias:ident $doc:literal
        ),*
    }) => {
        paste::paste! {
            #[wasm_bindgen]
            impl $stuc {
                $(
                    #[wasm_bindgen(js_name = $alias:camel:uncapitalize)]
                    #[doc=$doc]
                    pub fn $alias(mut self) -> Self {
                        self.data.map_inplace(|x| *x = x.$func());
                        self
                    }

                    #[wasm_bindgen(js_name = $alias:camel:uncapitalize _)]
                    #[doc=$doc]
                    pub fn [<$alias _par>](mut self) -> Self {
                        self.data.par_map_inplace(|x| *x = x.$func());
                        self
                    }
                )*
            }
        }
	};
}

pub(crate) use zero;

macro_rules! one {
	($stuc:ty, $arg:ty {
        $(
            $func:ident $doc:literal
        ),*
    }) => {
        paste::paste! {
            #[wasm_bindgen]
            impl $stuc {
                $(
                    #[wasm_bindgen(js_name = $func:camel:uncapitalize)]
                    #[doc=$doc]
                    pub fn $func(mut self, a: $arg) -> Self {
                        self.data.map_inplace(|x| *x = x.$func(a));
                        self
                    }

                    #[wasm_bindgen(js_name = $func:camel:uncapitalize _)]
                    #[doc=$doc]
                    pub fn [<$func _par>](mut self, a: $arg) -> Self {
                        self.data.par_map_inplace(|x| *x = x.$func(a));
                        self
                    }
                )*
            }
        }
	};
	($stuc:ty, $arg:ty {
        $(
            $func:ident = $alias:ident $doc:literal
        ),*
    }) => {
        paste::paste! {
            #[wasm_bindgen]
            impl $stuc {
                $(
                    #[wasm_bindgen(js_name = $alias:camel:uncapitalize)]
                    #[doc=$doc]
                    pub fn $alias(mut self, a: $arg) -> Self {
                        self.data.map_inplace(|x| *x = x.$func(a));
                        self
                    }

                    #[wasm_bindgen(js_name = $alias:camel:uncapitalize _)]
                    #[doc=$doc]
                    pub fn [<$alias _par>](mut self, a: $arg) -> Self {
                        self.data.par_map_inplace(|x| *x = x.$func(a));
                        self
                    }
                )*
            }
        }
	};
}

pub(crate) use one;

macro_rules! two {
	($stuc:ty, $arg1:ty, $arg2:ty {
        $(
            $func:ident $doc:literal
        ),*
    }) => {
        paste::paste! {
            #[wasm_bindgen]
            impl $stuc {
                $(
                    #[wasm_bindgen(js_name = $func:camel:uncapitalize)]
                    #[doc=$doc]
                    pub fn $func(mut self, a: $arg1, b: $arg2) -> Self {
                        self.data.map_inplace(|x| *x = x.$func(a, b));
                        self
                    }

                    #[wasm_bindgen(js_name = $func:camel:uncapitalize _)]
                    #[doc=$doc]
                    pub fn [<$func _par>](mut self, a: $arg1, b: $arg2) -> Self {
                        self.data.par_map_inplace(|x| *x = x.$func(a, b));
                        self
                    }
                )*
            }
        }
	};
	($stuc:ty, $arg1:ty, $arg2:ty {
        $(
            $func:ident = $alias:ident $doc:literal
        ),*
    }) => {
        paste::paste! {
            #[wasm_bindgen]
            impl $stuc {
                $(
                    #[wasm_bindgen(js_name = $alias:camel:uncapitalize)]
                    #[doc=$doc]
                    pub fn $alias(mut self, a: $arg1, b: $arg2) -> Self {
                        self.data.map_inplace(|x| *x = x.$func(a, b));
                        self
                    }

                    #[wasm_bindgen(js_name = $alias:camel:uncapitalize _)]
                    #[doc=$doc]
                    pub fn [<$alias _par>](mut self, a: $arg1, b: $arg2) -> Self {
                        self.data.par_map_inplace(|x| *x = x.$func(a, b));
                        self
                    }
                )*
            }
        }
	};
}

pub(crate) use two;
