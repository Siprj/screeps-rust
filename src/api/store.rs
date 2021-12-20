use crate::api::{constants::ResourceType, map::ScreepsMap};
use wasm_bindgen::prelude::*;

use super::structure::spawn::Spawn;

#[wasm_bindgen]
extern "C" {
    pub type Store;

    #[wasm_bindgen(method, js_name = getCapacity)]
    pub fn capacity(this: &Store) -> u32;

    #[wasm_bindgen(method, js_name = getCapacity)]
    pub fn capacity_by_resource(this: &Store, resource_type: ResourceType) -> u32;

    #[wasm_bindgen(method, js_name = getFreeCapacity)]
    pub fn free_space(this: &Store) -> ScreepsMap<String, Spawn>;

    #[wasm_bindgen(method, js_name = getFreeCapacity)]
    pub fn free_space_by_resource(this: &Store, resource_type: ResourceType) -> u32;

    #[wasm_bindgen(method, js_name = getUsedCapacity)]
    pub fn stored(this: &Store) -> u32;

    #[wasm_bindgen(method, js_name = getUsedCapacity)]
    pub fn stored_by_resource(this: &Store, resource_type: ResourceType) -> u32;
}

// impl wasm_bindgen::convert::IntoWasmAbi for Store{
//     type Abi = <Object as wasm_bindgen::convert::IntoWasmAbi>::Abi;
//
//     #[inline]
//     fn into_abi(self) -> Self::Abi {
//         self.unchecked_ref()
//     }
// }
//
// impl wasm_bindgen::convert::FromWasmAbi for Store {
//     type Abi = <Object as wasm_bindgen::convert::FromWasmAbi>::Abi;
//
//     #[inline]
//     unsafe fn from_abi(val: Self::Abi) -> Self {
//         val.unchecked_into()
//     }
// }
//
// impl wasm_bindgen::describe::WasmDescribe for Store {
//     fn describe() {
//         Object::describe()
//     }
// }
