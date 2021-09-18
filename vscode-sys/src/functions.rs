use std::any::Any;

use js_sys::{Array, Promise};
use wasm_bindgen::{convert::{FromWasmAbi, IntoWasmAbi}, prelude::*};

use crate::types::{Command, Function, Thenable};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = tests)]
    pub fn createTestController(id: &str, label: &str);
    #[wasm_bindgen(js_namespace = ["vscode", "window"], js_name="showInformationMessage")]
    pub fn show_information_message_0(message: &str) -> Promise;
    #[wasm_bindgen(js_namespace = ["vscode", "window"], js_name="showInformationMessage")]
    pub fn show_information_message_1(message: &str, button_1: &str) -> Promise;
    #[wasm_bindgen(js_namespace = ["vscode", "window"], js_name="showInformationMessage")]
    pub fn show_information_message_2(message: &str, button_1: &str, button_2: &str) -> Promise;
    #[wasm_bindgen(js_namespace = ["vscode", "window"], js_name="showInformationMessage")]
    pub fn show_information_message_3(message: &str, button_1: &str, button_2: &str, button_3: &str) -> Promise;
    #[wasm_bindgen(js_namespace = ["vscode", "window"], js_name="showInformationMessage")]
    pub fn show_information_message_4(message: &str, button_1: &str, button_2: &str, button_3: &str, button_4: &str) -> Promise;
    #[wasm_bindgen(js_namespace = ["vscode", "window"], js_name="showInformationMessage")]
    pub fn show_information_message_5(message: &str, button_1: &str, button_2: &str, button_3: &str, button_4: &str, button_5: &str) -> Promise;
    #[wasm_bindgen(js_namespace = ["vscode", "commands"], js_name = "registerCommand")]
    pub fn register_command(command: String, callback: &Closure<dyn FnMut()>);
}
