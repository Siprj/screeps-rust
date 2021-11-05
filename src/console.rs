use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(msg: JsValue);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn kwa2(msg: String);
}

