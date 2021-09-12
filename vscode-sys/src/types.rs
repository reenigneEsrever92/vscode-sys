use wasm_bindgen::prelude::*;
use js_sys::Object;

pub type Function = Closure<dyn FnMut()>;

#[wasm_bindgen]
pub struct Command {
    title: &'static str,
    command: &'static str
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object, typescript_type = "Thenable<any>")]
    #[derive(Clone, Debug)]
    pub type Thenable;

    #[wasm_bindgen(method, js_name = "then")]
    pub fn then(this: &Thenable, cb: &Closure<dyn FnMut(JsValue)>) -> Thenable;

}
