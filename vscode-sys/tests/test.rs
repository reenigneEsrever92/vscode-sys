use wasm_bindgen_test::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use js_sys::{Object, Promise};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(str: &str);
}

#[wasm_bindgen_test] 
async fn test_stuff () {
    log("something");
}
