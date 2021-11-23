use js_sys::JsString;
use wasm_bindgen::JsValue;

pub trait ScreepsFromJsValue {
    fn from_js_value(val: JsValue) -> Self;
}

impl ScreepsFromJsValue for JsValue {
    fn from_js_value(val: JsValue) -> Self {
        val
    }
}

pub trait ScreepsToJsValue {
    fn to_js_value(&self) -> &JsValue;
}

impl ScreepsToJsValue for JsValue {
    fn to_js_value(&self) -> &JsValue {
        self.into()
    }
}

pub trait ScreepsToJsKey {
    fn to_js_key(&self) -> JsValue;
}

impl ScreepsToJsKey for JsValue {
    fn to_js_key(&self) -> JsValue {
        self.into()
    }
}

impl ScreepsToJsKey for JsString {
    fn to_js_key(&self) -> JsValue {
        self.into()
    }
}

impl ScreepsToJsKey for String {
    fn to_js_key(&self) -> JsValue {
        self.into()
    }
}

impl ScreepsToJsKey for &str {
    fn to_js_key(&self) -> JsValue {
        (*self).into()
    }
}
