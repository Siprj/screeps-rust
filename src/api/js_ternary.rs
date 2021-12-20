use wasm_bindgen::prelude::*;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
enum JsTernary<V> {
    Undefined,
    Null,
    Value { value: V }
}

impl<T> wasm_bindgen::convert::IntoWasmAbi for JsTernary<T>
where
    T: ScreepsFromJsValue + ScreepsToJsValue,
{
    type Abi = <js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;

    #[inline]
    fn into_abi(self) -> Self::Abi {
        match self {
            JsTernary::Undefined => JsValue::UNDEFINED.into_abi(),
            JsTernary::Null => JsValue::Null.into_abi(),
            JsTernary::Value { value } => value.into(),
        }
    }
}

impl<T> wasm_bindgen::convert::FromWasmAbi for JsTernary<T>
where
    T: ScreepsFromJsValue + ScreepsToJsValue,
{
    type Abi = <js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;

    #[inline]
    unsafe fn from_abi(js: Self::Abi) -> ScreepsArray<T> {
        ScreepsArray {
            array: Array::from_abi(js),
            phantom_type: PhantomData::default(),
        }
    }
}

impl<T> wasm_bindgen::describe::WasmDescribe for JsTernary<T>
where
    T: ScreepsFromJsValue + ScreepsToJsValue,
{
    fn describe() {
        Array::describe()
    }
}

