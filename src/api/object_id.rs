use std::{fmt::{Debug, Formatter, Result}, marker::PhantomData};

use js_sys::JsString;

pub struct ObjectId<T> {
    object_id: JsString,
    phantom: PhantomData<T>,
}

impl<T> wasm_bindgen::convert::IntoWasmAbi for ObjectId<T> {
    type Abi = <JsString as wasm_bindgen::convert::IntoWasmAbi>::Abi;

    #[inline]
    fn into_abi(self) -> Self::Abi {
        self.object_id.into_abi()
    }
}

impl<T> wasm_bindgen::convert::FromWasmAbi for ObjectId<T> {
    type Abi = <JsString as wasm_bindgen::convert::FromWasmAbi>::Abi;

    #[inline]
    unsafe fn from_abi(value_id: Self::Abi) -> Self {
        ObjectId { object_id: JsString::from_abi(value_id), phantom: PhantomData::default() }
    }
}

impl<T> wasm_bindgen::describe::WasmDescribe for ObjectId<T> {
    fn describe() {
        JsString::describe()
    }
}

impl<T> Debug for ObjectId<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("ObjectId")
         .field("object_id", &self.object_id)
         .finish()
    }
}
