#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ReadableStreamBYOBReader , typescript_type = "ReadableStreamBYOBReader")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ReadableStreamBYOBReader` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBReader)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamBYOBReader`*"]
    pub type ReadableStreamBYOBReader;
    # [wasm_bindgen (structural , method , js_class = "ReadableStreamBYOBReader" , js_name = read)]
    #[doc = "The `read()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBReader/read)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamBYOBReader`*"]
    pub fn read(this: &ReadableStreamBYOBReader, buffer: &JsValue) -> ::js_sys::Promise;
    # [wasm_bindgen (structural , method , js_class = "ReadableStreamBYOBReader" , js_name = releaseLock)]
    #[doc = "The `releaseLock()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBReader/releaseLock)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamBYOBReader`*"]
    pub fn release_lock(this: &ReadableStreamBYOBReader);
    # [wasm_bindgen (structural , method , getter , js_class = "ReadableStreamBYOBReader" , js_name = closed)]
    #[doc = "Getter for the `closed` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBReader/closed)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamBYOBReader`*"]
    pub fn closed(this: &ReadableStreamBYOBReader) -> ::js_sys::Promise;
    # [wasm_bindgen (structural , method , js_class = "ReadableStreamBYOBReader" , js_name = cancel)]
    #[doc = "The `cancel()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBReader/cancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamBYOBReader`*"]
    pub fn cancel(this: &ReadableStreamBYOBReader) -> ::js_sys::Promise;
    # [wasm_bindgen (structural , method , js_class = "ReadableStreamBYOBReader" , js_name = cancel)]
    #[doc = "The `cancel()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBReader/cancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamBYOBReader`*"]
    pub fn cancel_with_reason(this: &ReadableStreamBYOBReader, reason: &str) -> ::js_sys::Promise;

}
