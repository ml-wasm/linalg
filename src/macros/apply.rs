macro_rules! zero {
	($stuc:ty {
        $(
            $func:ident  $doc:literal
        ),*
        $(,)?
    }) => {
        paste::paste! {
            #[wasm_bindgen]
            impl $stuc {
                $(
                    #[wasm_bindgen(js_name = $func:camel:uncapitalize)]
                    #[doc = $doc]
                    pub fn $func(&mut self) {
                        self.data.mapv_inplace(|x| x.$func());
                    }

                    #[wasm_bindgen(js_name = $func:camel:uncapitalize _)]
                    #[doc = $doc]
                    pub fn [<$func _consume>](mut self) -> Self {
                        self.data.mapv_inplace(|x| x.$func());
                        self
                    }

                    #[wasm_bindgen(js_name = $func:camel:uncapitalize Par)]
                    #[doc = $doc]
                    pub fn [<$func _par>](&mut self) {
                        self.data.par_mapv_inplace(|x| x.$func());
                    }

                    #[wasm_bindgen(js_name = $func:camel:uncapitalize Par _)]
                    #[doc = $doc]
                    pub fn [<$func _par _consume>](mut self) -> Self {
                        self.data.par_mapv_inplace(|x| x.$func());
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
        $(,)?
    }) => {
        paste::paste! {
            #[wasm_bindgen]
            impl $stuc {
                $(
                    #[wasm_bindgen(js_name = $alias:camel:uncapitalize)]
                    #[doc = $doc]
                    pub fn $alias(&mut self) {
                        self.data.mapv_inplace(|x| x.$func())
                    }

                    #[wasm_bindgen(js_name = $alias:camel:uncapitalize _)]
                    #[doc = $doc]
                    pub fn [<$alias _consume>](mut self) -> Self {
                        self.data.mapv_inplace(|x| x.$func());
                        self
                    }

                    #[wasm_bindgen(js_name = $alias:camel:uncapitalize Par)]
                    #[doc = $doc]
                    pub fn [<$alias _par>](&mut self) {
                        self.data.par_mapv_inplace(|x| x.$func())
                    }

                    #[wasm_bindgen(js_name = $alias:camel:uncapitalize Par _)]
                    #[doc = $doc]
                    pub fn [<$alias _par _consume>](mut self) -> Self {
                        self.data.par_mapv_inplace(|x| x.$func());
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
        $(,)?
    }) => {
        paste::paste! {
            #[wasm_bindgen]
            impl $stuc {
                $(
                    #[wasm_bindgen(js_name = $func:camel:uncapitalize)]
                    #[doc = $doc]
                    pub fn $func(&mut self, a: $arg) {
                        self.data.mapv_inplace(|x| x.$func(a));
                    }

                    #[wasm_bindgen(js_name = $func:camel:uncapitalize _)]
                    #[doc = $doc]
                    pub fn [<$func _consume>](mut self, a: $arg) -> Self {
                        self.data.mapv_inplace(|x| x.$func(a));
                        self
                    }

                    #[wasm_bindgen(js_name = $func:camel:uncapitalize Par)]
                    #[doc = $doc]
                    pub fn [<$func _par>](&mut self, a: $arg) {
                        self.data.par_mapv_inplace(|x| x.$func(a));
                    }

                    #[wasm_bindgen(js_name = $func:camel:uncapitalize Par _)]
                    #[doc = $doc]
                    pub fn [<$func _par _consume>](mut self, a: $arg) -> Self {
                        self.data.par_mapv_inplace(|x| x.$func(a));
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
        $(,)?
    }) => {
        paste::paste! {
            #[wasm_bindgen]
            impl $stuc {
                $(
                    #[wasm_bindgen(js_name = $alias:camel:uncapitalize)]
                    #[doc = $doc]
                    pub fn $alias(&mut self, a: $arg) {
                        self.data.mapv_inplace(|x| x.$func(a));
                    }

                    #[wasm_bindgen(js_name = $alias:camel:uncapitalize _)]
                    #[doc = $doc]
                    pub fn [<$alias _consume>](mut self, a: $arg) -> Self {
                        self.data.mapv_inplace(|x| x.$func(a));
                        self
                    }

                    #[wasm_bindgen(js_name = $alias:camel:uncapitalize Par)]
                    #[doc = $doc]
                    pub fn [<$alias _par>](&mut self, a: $arg) {
                        self.data.par_mapv_inplace(|x| x.$func(a));
                    }

                    #[wasm_bindgen(js_name = $alias:camel:uncapitalize Par _)]
                    #[doc = $doc]
                    pub fn [<$alias _par _consume>](mut self, a: $arg) -> Self {
                        self.data.par_mapv_inplace(|x| x.$func(a));
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
        $(,)?
    }) => {
        paste::paste! {
            #[wasm_bindgen]
            impl $stuc {
                $(
                    #[wasm_bindgen(js_name = $func:camel:uncapitalize)]
                    #[doc = $doc]
                    pub fn $func(&mut self, a: $arg1, b: $arg2) {
                        self.data.mapv_inplace(|x| x.$func(a, b));
                    }

                    #[wasm_bindgen(js_name = $func:camel:uncapitalize _)]
                    #[doc = $doc]
                    pub fn [<$func _consume>](mut self, a: $arg1, b: $arg2) -> Self {
                        self.data.mapv_inplace(|x| x.$func(a, b));
                        self
                    }

                    #[wasm_bindgen(js_name = $func:camel:uncapitalize Par)]
                    #[doc = $doc]
                    pub fn [<$func _par>](&mut self, a: $arg1, b: $arg2) {
                        self.data.par_mapv_inplace(|x| x.$func(a, b));
                    }

                    #[wasm_bindgen(js_name = $func:camel:uncapitalize Par _)]
                    #[doc = $doc]
                    pub fn [<$func _par _consume>](mut self, a: $arg1, b: $arg2) -> Self {
                        self.data.par_mapv_inplace(|x| x.$func(a, b));
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
        $(,)?
    }) => {
        paste::paste! {
            #[wasm_bindgen]
            impl $stuc {
                $(
                    #[wasm_bindgen(js_name = $alias:camel:uncapitalize)]
                    #[doc = $doc]
                    pub fn $alias(&mut self, a: $arg1, b: $arg2) {
                        self.data.mapv_inplace(|x| x.$func(a, b));
                    }

                    #[wasm_bindgen(js_name = $alias:camel:uncapitalize _)]
                    #[doc = $doc]
                    pub fn [<$alias _consume>](mut self, a: $arg1, b: $arg2) -> Self {
                        self.data.mapv_inplace(|x| x.$func(a, b));
                        self
                    }

                    #[wasm_bindgen(js_name = $alias:camel:uncapitalize Par)]
                    #[doc = $doc]
                    pub fn [<$alias _par>](&mut self, a: $arg1, b: $arg2) {
                        self.data.par_mapv_inplace(|x| x.$func(a, b));
                    }

                    #[wasm_bindgen(js_name = $alias:camel:uncapitalize Par _)]
                    #[doc = $doc]
                    pub fn [<$alias _par _consume>](mut self, a: $arg1, b: $arg2) -> Self {
                        self.data.par_mapv_inplace(|x| x.$func(a, b));
                        self
                    }
                )*
            }
        }
	};
}

pub(crate) use two;
