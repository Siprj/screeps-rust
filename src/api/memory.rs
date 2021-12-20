use wasm_bindgen::JsValue;
use super::utils::*;

pub struct Memory {
    value: JsValue
}

impl Memory {
    pub fn get_string(self: &Memory, key: &str) -> Option<String> {
        get_key_value(&self.value, &JsValue::from_str(key)).as_string()
    }

    pub fn get_bool(self: &Memory, key: &str) -> Option<bool> {
        get_key_value(&self.value, &JsValue::from_str(key)).as_bool()
    }

    pub fn get_u32(self: &Memory, key: &str) -> Option<u32> {
        get_key_value(&self.value, &JsValue::from_str(key)).as_f64().map(|v| v as u32)
    }

    pub fn get_u32_with_u32(self: &Memory, key: u32) -> Option<u32> {
        get_key_value(&self.value, &JsValue::from_f64(key as f64)).as_f64().map(|v| v as u32)
    }

    pub fn set_string(self: &Memory, key: &str, value: Option<String>) {
        match value {
            Some(v) => {
                set_key_value(&self.value, &JsValue::from_str(&key), &JsValue::from_str(&v));
            }
            None => {
                set_key_value(&self.value, &JsValue::from_str(&key), &JsValue::UNDEFINED);
            }
        }
    }

    pub fn set_bool(self: &Memory, key: &str, value: Option<bool>) {
        match value {
            Some(v) => {
                set_key_value(&self.value, &JsValue::from_str(key), &JsValue::from_bool(v));
            }
            None => {
                set_key_value(&self.value, &JsValue::from_str(key), &JsValue::UNDEFINED);
            }
        }
    }

    pub fn set_u32(self: &Memory, key: &str, value: Option<u32>) {
        match value {
            Some(v) => {
                set_key_value(&self.value, &JsValue::from_str(key), &JsValue::from_f64(v as f64));
            }
            None => {
                set_key_value(&self.value, &JsValue::from_str(key), &JsValue::UNDEFINED);
            }
        }
    }

    pub fn set_u32_with_u32(self: &Memory, key: u32, value: Option<u32>) {
        match value {
            Some(v) => {
                set_key_value(&self.value, &JsValue::from_f64(key as f64), &JsValue::from_f64(v as f64));
            }
            None => {
                set_key_value(&self.value, &JsValue::from_f64(key as f64), &JsValue::UNDEFINED);
            }
        }
    }
}

impl wasm_bindgen::convert::IntoWasmAbi for Memory {
    type Abi = <JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;

    #[inline]
    fn into_abi(self) -> Self::Abi {
        self.value.into_abi()
    }
}

impl wasm_bindgen::convert::FromWasmAbi for Memory {
    type Abi = <JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;

    #[inline]
    unsafe fn from_abi(js_str: Self::Abi) -> Self {
        Memory {
            value: JsValue::from_abi(js_str)
        }
    }
}

impl wasm_bindgen::describe::WasmDescribe for Memory {
    fn describe() {
        JsValue::describe()
    }
}
