#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    pub type WritableStreamDefaultWriter;

    # [wasm_bindgen (structural , method , getter , js_class = "WritableStreamDefaultWriter" , js_name = closed)]
    #[doc = "Getter for the `closed` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultWriter/closed)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStreamDefaultWriter`*"]
    pub fn closed(this: &WritableStreamDefaultWriter) -> ::js_sys::Promise;
    # [wasm_bindgen (structural , method , getter , js_class = "WritableStreamDefaultWriter" , js_name = desiredSize)]
    #[doc = "Getter for the `desiredSize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultWriter/desiredSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStreamDefaultWriter`*"]
    pub fn desired_size(this: &WritableStreamDefaultWriter) -> f64;
    # [wasm_bindgen (structural , method , getter , js_class = "WritableStreamDefaultWriter" , js_name = ready)]
    #[doc = "Getter for the `ready` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultWriter/ready)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStreamDefaultWriter`*"]
    pub fn ready(this: &WritableStreamDefaultWriter) -> ::js_sys::Promise;
    # [wasm_bindgen (structural , method , js_class = "WritableStreamDefaultWriter" , js_name = abort)]
    #[doc = "The `abort()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultWriter/abort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStreamDefaultWriter`*"]
    pub fn abort(this: &WritableStreamDefaultWriter) -> ::js_sys::Promise;
    # [wasm_bindgen (structural , method , js_class = "WritableStreamDefaultWriter" , js_name = abort)]
    #[doc = "The `abort()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultWriter/abort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStreamDefaultWriter`*"]
    pub fn abort_with_reason(this: &WritableStreamDefaultWriter, reason: &str)
        -> ::js_sys::Promise;
    # [wasm_bindgen (structural , method , js_class = "WritableStreamDefaultWriter" , js_name = close)]
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultWriter/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStreamDefaultWriter`*"]
    pub fn close(this: &WritableStreamDefaultWriter) -> ::js_sys::Promise;
    # [wasm_bindgen (structural , method , js_class = "WritableStreamDefaultWriter" , js_name = releaseLock)]
    #[doc = "The `releaseLock()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultWriter/releaseLock)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStreamDefaultWriter`*"]
    pub fn release_lock(this: &WritableStreamDefaultWriter);
    # [wasm_bindgen (structural , method , js_class = "WritableStreamDefaultWriter" , js_name = write)]
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultWriter/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStreamDefaultWriter`*"]
    pub fn write(this: &WritableStreamDefaultWriter, chunk: &JsValue) -> ::js_sys::Promise;
}
