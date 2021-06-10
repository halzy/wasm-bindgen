#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ReadableStreamDefaultReadResult)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ReadableStreamDefaultReadResult` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamDefaultReadResult`*"]
    pub type ReadableStreamDefaultReadResult;
}
impl ReadableStreamDefaultReadResult {
    #[doc = "Construct a new `ReadableStreamDefaultReadResult`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamDefaultReadResult`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `done` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamDefaultReadResult`*"]
    pub fn done(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("done"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamDefaultReadResult`*"]
    pub fn value(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("value"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
