use wasm_bindgen::prelude::*;
use vscode_sys::{functions::*, types::{Command, VSCode}};
use web_sys::console;
use js_sys::Promise;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace  = console)]
    pub fn log(message: &str);
}

#[wasm_bindgen]
pub async fn test_show_information_message_test() -> Result<JsValue, JsValue> {
    match VSCode::show_information_message_1("Hello from Wasm", "btn1").await {
        Ok(value) => {
            console::debug_1(&JsValue::from(format!("Button {:?} clicked!", value).as_str()));
            Ok(value)
        },
        Err(error) => Err(error),
    }
}

#[wasm_bindgen]
pub fn register_command_test() {
    let closure = Closure::wrap(Box::new(|| {
        show_information_message_0("message from command");
    }) as Box<dyn FnMut()>);

    register_command("vscode-sys-test.testcmd".into(), &closure);
}