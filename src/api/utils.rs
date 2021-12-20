use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = utils)]
extern "C" {
    #[wasm_bindgen]
    pub fn set_key_value(obj: &JsValue, key: &JsValue, val: &JsValue);

    #[wasm_bindgen]
    pub fn get_key_value(obj: &JsValue, key: &JsValue) -> JsValue;
}
