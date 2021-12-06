use std::marker::PhantomData;

use super::{
    array::ScreepsArray,
    cast::{ScreepsFromJsValue, ScreepsToJsKey, ScreepsToJsValue},
};
use js_sys::{Object, Reflect};
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

pub struct ScreepsMap<K, T> {
    map: JsValue,
    pub phantom_key: PhantomData<K>,
    pub phantom_value: PhantomData<T>,
}

impl<K, T> ScreepsMap<K, T> {
    pub fn get(&self, key: K) -> T
    where
        K: ScreepsToJsKey,
        T: ScreepsFromJsValue,
    {
        T::from_js_value(Reflect::get(&self.map, &key.to_js_key()).unwrap())
    }

    pub fn values_ref(&self) -> ScreepsArray<T> {
        ScreepsArray {
            array: Object::values(Object::unchecked_from_js_ref(&self.map)),
            phantom_type: PhantomData::default(),
        }
    }
}

impl<K, T> wasm_bindgen::convert::IntoWasmAbi for ScreepsMap<K, T>
where
    T: ScreepsFromJsValue + ScreepsToJsValue,
    K: ScreepsToJsKey,
{
    type Abi = <js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;

    #[inline]
    fn into_abi(self) -> Self::Abi {
        self.map.into_abi()
    }
}

impl<K, T> wasm_bindgen::convert::FromWasmAbi for ScreepsMap<K, T>
where
    T: ScreepsFromJsValue + ScreepsToJsValue,
    K: ScreepsToJsKey,
{
    type Abi = <JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;

    #[inline]
    unsafe fn from_abi(js: Self::Abi) -> ScreepsMap<K, T> {
        ScreepsMap {
            map: JsValue::from_abi(js),
            phantom_key: PhantomData::default(),
            phantom_value: PhantomData::default(),
        }
    }
}

impl<K, T> wasm_bindgen::describe::WasmDescribe for ScreepsMap<K, T>
where
    T: ScreepsFromJsValue + ScreepsToJsValue,
    K: ScreepsToJsKey,
{
    fn describe() {
        Object::describe()
    }
}
