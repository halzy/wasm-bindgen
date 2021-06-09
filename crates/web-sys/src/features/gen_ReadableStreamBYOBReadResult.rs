#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ReadableStreamBYOBReadResult)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ReadableStreamBYOBReadResult` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamBYOBReadResult`*"]
    pub type ReadableStreamBYOBReadResult;
    # [wasm_bindgen (structural , method , getter , js_class = "ReadableStreamBYOBReadResult" , js_name = value)]
    #[doc = "Getter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBReader/read)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamBYOBReadResult`*"]
    pub fn value(this: &ReadableStreamBYOBReadResult) -> JsValue;
    # [wasm_bindgen (structural , method , getter , js_class = "ReadableStreamBYOBReadResult" , js_name = done)]
    #[doc = "Getter for the `done` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBReader/read)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamBYOBReadResult`*"]
    pub fn done(this: &ReadableStreamBYOBReadResult) -> bool;
}
