use std::any::Any;

use js_sys::Array;
use wasm_bindgen::{convert::FromWasmAbi, prelude::*};

use crate::types::{Function, Thenable};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = tests)]
    pub fn createTestController(id: &str, label: &str);
    #[wasm_bindgen(js_namespace = ["vscode", "window"], js_name="showInformationMessage")]
    pub fn showInformationMessage0(message: &str) -> Thenable;
    #[wasm_bindgen(js_namespace = ["vscode", "window"], js_name="showInformationMessage")]
    pub fn showInformationMessage1(message: &str, items: &Array) -> Thenable;
    #[wasm_bindgen(js_namespace = ["vscode", "commands"])]
    pub fn registerCommand(command: &str, callback: &Closure<dyn FnMut()>);
}
