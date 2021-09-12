use js_sys::Array;
use wasm_bindgen::prelude::*;
use vscode_sys::functions::*;

#[wasm_bindgen]
pub fn test_show_information_message() {

    let closure = Closure::wrap(Box::new(|val: JsValue| {
        showInformationMessage0("message from future");
    }) as Box<dyn FnMut(JsValue)>);

    showInformationMessage1("Hello from Wasm", &Array::of1(&JsValue::from_str("test")))
        .then(&closure);
}

#[wasm_bindgen]
pub fn register_command() {
    let closure = Closure::wrap(Box::new(|| {
        showInformationMessage0("message from command");
    }) as Box<dyn FnMut()>);

    registerCommand("vs-code-test-sys.testcmd", &closure);

    loop {
        
    }
}