use std::any::Any;

use wasm_bindgen::prelude::*;
use js_sys::Object;

use crate::functions::*;

pub type Function = Closure<dyn FnMut()>;

#[wasm_bindgen(getter_with_clone)]
pub struct Command {
    pub title: String,
    pub command: String
}

pub struct VSCode;

impl VSCode {
    pub async fn show_information_message_0(message: &str) -> Result<JsValue, JsValue> {
        let fut = wasm_bindgen_futures::JsFuture::from(show_information_message_0(message));
        let result = fut.await?;

        return Ok(result);
    }

    pub async fn show_information_message_1(message: &str, btn_1: &str) -> Result<JsValue, JsValue> {
        return wasm_bindgen_futures::JsFuture::from(show_information_message_1(message, btn_1)).await;
    }
}

// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(extends = Object, typescript_type = "Thenable<any>")]
//     #[derive(Clone, Debug)]
//     pub type Disposable;

//     #[wasm_bindgen(method, js_namespace= ["vscode", "Disposable"], js_name = "isDisposable")]
//     pub fn is_disposable(obj: &JsValue);
// }

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object, typescript_type = "Thenable<any>")]
    #[derive(Clone, Debug)]
    pub type Thenable;

    #[wasm_bindgen(method, js_name = "then")]
    pub fn then(this: &Thenable, onFulfilled: &Closure<dyn FnMut(JsValue)>, onRejected: &Closure<dyn FnMut(JsValue)>) -> Thenable;

}
