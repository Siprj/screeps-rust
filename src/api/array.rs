use std::{marker::PhantomData, ops::Range};

use super::cast::{ScreepsFromJsValue, ScreepsToJsValue};
use js_sys::Array;

pub struct ScreepsArray<T>
{
    pub array: Array,
    pub phantom_type: PhantomData<T>,
}

pub struct ScreepsArrayIterator<T>
{
    array: Array,
    range: Range<u32>,
    phantom_type: PhantomData<T>,
}

impl<T> std::iter::Iterator for ScreepsArrayIterator<T>
where
    T: ScreepsFromJsValue + ScreepsToJsValue,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let index = self.range.next()?;
        Some(T::from_js_value(self.array.get(index)))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.range.size_hint()
    }
}

impl<T> std::iter::ExactSizeIterator for ScreepsArrayIterator<T>
where
    T: ScreepsFromJsValue + ScreepsToJsValue,
{
    fn len(&self) -> usize {
        self.array.length() as usize
    }

    fn is_empty(&self) -> bool {
        self.array.iter().is_empty()
    }
}

impl<T> wasm_bindgen::convert::IntoWasmAbi for ScreepsArray<T>
where
    T: ScreepsFromJsValue + ScreepsToJsValue,
{
    type Abi = <js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;

    #[inline]
    fn into_abi(self) -> Self::Abi {
        self.array.into_abi()
    }
}

impl<T> wasm_bindgen::convert::FromWasmAbi for ScreepsArray<T>
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

impl<T> wasm_bindgen::describe::WasmDescribe for ScreepsArray<T>
where
    T: ScreepsFromJsValue + ScreepsToJsValue,
{
    fn describe() {
        Array::describe()
    }
}
