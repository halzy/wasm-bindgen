#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = WritableStream , typescript_type = "WritableStream")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WritableStream` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStream`*"]
    pub type WritableStream;
    # [wasm_bindgen (structural , method , getter , js_class = "WritableStream" , js_name = locked)]
    #[doc = "Getter for the `locked` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStream/locked)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStream`*"]
    pub fn locked(this: &WritableStream) -> bool;
    # [wasm_bindgen (structural , method , js_class = "WritableStream" , js_name = abort)]
    #[doc = "The `abort()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStream/abort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStream`*"]
    pub fn abort(this: &WritableStream) -> ::js_sys::Promise;
    # [wasm_bindgen (structural , method , js_class = "WritableStream" , js_name = abort)]
    #[doc = "The `abort()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStream/abort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStream`*"]
    pub fn abort_with_reason(this: &WritableStream, reason: &str) -> ::js_sys::Promise;
    # [wasm_bindgen (structural , method , js_class = "WritableStream" , js_name = close)]
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStream`*"]
    pub fn close(this: &WritableStream) -> ::js_sys::Promise;
    #[cfg(feature = "WritableStreamDefaultWriter")]
    # [wasm_bindgen (structural , method , js_class = "WritableStream" , js_name = getWriter)]
    #[doc = "The `getWriter()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStream/getWriter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStream`, `WritableStreamDefaultWriter`*"]
    pub fn get_writer(this: &WritableStream) -> WritableStreamDefaultWriter;
}
