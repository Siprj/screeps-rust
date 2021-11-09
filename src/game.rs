use wasm_bindgen::prelude::*;
use std::marker::PhantomData;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use js_sys::{Array, JsString, Object};

pub struct ScreepHashMap<K, V> {
    pub object: Object,
    pub phantom_key: PhantomData<K>,
    pub phantom_value: PhantomData<V>,
}

// pub struct ScreepValue<V> {
//     pub object: Object,
//     pub phantom_value: PhantomData<V>,
// }

// ScreepObjectIterator<K, V> {
// }

//impl std::iter::Iterator for ScreepObject<K, V> {
//
//}

// #[wasm_bindgen]
// extern "C" {
//     pub type HarvesterMemory;
//
//     #[wasm_bindgen(method, setter)]
//     pub fn set_state(this: &HarvesterMemory, state: JsValue);
//
//     #[wasm_bindgen(method, getter)]
//     pub fn get_state(this: &HarvesterMemory) -> JsValue;
// }

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub type Room;

    #[wasm_bindgen(method)]
    pub fn find(this: &Room, type_: u32) -> Array;

    #[wasm_bindgen(method, getter = controller)]
    pub fn controller(this: &Room) -> Option<Controller>;
}

pub const FIND_SOURCES_ACTIVE: u32 = 104;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub type Source;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub type Controller;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub type RoomPosition;

    #[wasm_bindgen(method, js_name = isNearTo)]
    pub fn is_near_to(this: &RoomPosition, obj: &JsValue) -> bool;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub type Creep;

    #[wasm_bindgen(method, getter)]
    pub fn memory(this: &Creep) -> JsValue;

    #[wasm_bindgen(method, getter = store)]
    pub fn store(this: &Creep) -> Store;

    #[wasm_bindgen(method, getter = room)]
    pub fn room(this: &Creep) -> Room;

    #[wasm_bindgen(method)]
    pub fn harvest(this: &Creep, harvestable: &JsValue) -> i32;

    #[wasm_bindgen(method, getter = pos)]
    pub fn pos(this: &Creep) -> RoomPosition;

    #[wasm_bindgen(method, js_name = moveTo)]
    pub fn move_to(this: &Creep, target: &JsValue);

    #[wasm_bindgen(method, js_name = upgradeController)]
    pub fn upgrade_controller(this: &Creep, target: &Controller) -> ReturnCode;
}

pub mod mem {
    use js_sys::JsString;
    use wasm_bindgen::JsValue;

    pub fn get_string(target: &JsValue, key: &str) -> Option<JsString> {
        js_sys::Reflect::get(target, &JsValue::from_str(key)).map(|v| v.into()).ok()
    }
    pub fn get_bool(target: &JsValue, key: &str) -> Option<bool> {
        js_sys::Reflect::get(target, &JsValue::from_str(key)).ok().map(|v| v.as_bool()).flatten()
    }
    pub fn get_u32(target: &JsValue, key: &str) -> Option<u32> {
        js_sys::Reflect::get(target, &JsValue::from_str(key)).ok().map(|v| v.as_f64().map(|vv| vv as u32)).flatten()
    }
    pub fn get_u32_with_u32(target: &JsValue, key: u32) -> Option<u32> {
        js_sys::Reflect::get_u32(target, key).ok().map(|v| v.as_f64().map(|vv| vv as u32)).flatten()
    }

    pub fn set_string(tarset: &JsValue, key: &str, value: Option<JsString>) {
        match value {
            Some(v) => {
                js_sys::Reflect::set(tarset, &JsValue::from_str(key), &v);
            },
            None => {
                js_sys::Reflect::set(tarset, &JsValue::from_str(key), &JsValue::NULL);
            },
        }
    }
    pub fn set_bool(tarset: &JsValue, key: &str, value: Option<bool>) {
        match value {
            Some(v) => {
                js_sys::Reflect::set(tarset, &JsValue::from_str(key), &JsValue::from_bool(v));
            },
            None => {
                js_sys::Reflect::set(tarset, &JsValue::from_str(key), &JsValue::NULL);
            },
        }
    }
    pub fn set_u32(tarset: &JsValue, key: &str, value: Option<u32>) {
        match value {
            Some(v) => {
                js_sys::Reflect::set(tarset, &JsValue::from_str(key), &JsValue::from_f64(v as f64));
            },
            None => {
                js_sys::Reflect::set(tarset, &JsValue::from_str(key), &JsValue::NULL);
            },
        }
    }
    pub fn set_u32_with_u32(tarset: &JsValue, key: u32, value: Option<u32>) {
        match value {
            Some(v) => {
                js_sys::Reflect::set_u32(tarset, key, &JsValue::from_f64(v as f64));
            },
            None => {
                js_sys::Reflect::set_u32(tarset, key, &JsValue::NULL);
            },
        }
    }
}


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub type Structure;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive)]
#[repr(i8)]
pub enum ReturnCode {
    Ok = 0,
    NotOwner = -1,
    NoPath = -2,
    NameExists = -3,
    Busy = -4,
    NotFound = -5,
    NotEnough = -6,
    InvalidTarget = -7,
    Full = -8,
    NotInRange = -9,
    InvalidArgs = -10,
    Tired = -11,
    NoBodypart = -12,
    RclNotEnough = -14,
    GclNotEnough = -15,
}

impl wasm_bindgen::convert::IntoWasmAbi for ReturnCode {
    type Abi = i32;

    #[inline]
    fn into_abi(self) -> Self::Abi {
        (self as i32).into_abi()
    }
}

impl wasm_bindgen::convert::FromWasmAbi for ReturnCode {
    type Abi = i32;

    #[inline]
    unsafe fn from_abi(js: i32) -> Self {
        Self::from_i32(js).unwrap()
    }
}

impl wasm_bindgen::describe::WasmDescribe for ReturnCode {
    fn describe() {
        wasm_bindgen::describe::inform(wasm_bindgen::describe::I32)
    }
}

// impl TryFrom<JsValue> for ReturnCode {
//     type Error = String;
//
//     fn try_from(value: JsValue) -> Result<Self, Self::Error> {
//         value.as_f64().and_then(|f| Self::from_i32(f as i32)).ok_or_else(|| "expected number for return code".to_owned())
//     }
// }


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = StructureSpawn)]
    pub type Spawning;

    #[wasm_bindgen(method, getter)]
    pub fn directions(this: &Spawning) -> Array;

    #[wasm_bindgen(method, getter)]
    pub fn name(this: &Spawning) -> JsString;

    #[wasm_bindgen(method, getter = needTime)]
    pub fn need_time(this: &Spawning) -> u32;

    #[wasm_bindgen(method, getter = remainingTime)]
    pub fn remaining_time(this: &Spawning) -> u32;

    #[wasm_bindgen(method, getter)]
    pub fn spawn(this: &Spawning) -> Spawn;

    #[wasm_bindgen(method)]
    pub fn cancel(this: &Spawning) -> ReturnCode;

    #[wasm_bindgen(method, js_name = setDirections)]
    pub fn set_directions(this: &Spawning, directions: &Array) -> ReturnCode;
}

#[wasm_bindgen(module = utils)]
extern "C" {
    #[wasm_bindgen]
    pub fn resource_energy() -> JsValue;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub type Store;

    // FIXME: Should be list of resources not just one resource
    #[wasm_bindgen(method, js_name = getCapacity)]
    pub fn get_capacity(this: &Store, resource: &JsValue) -> u32;

    // FIXME: Should be list of resources not just one resource
    #[wasm_bindgen(method, js_name = getFreeCapacity)]
    pub fn get_free_capacity(this: &Store, resource: &JsValue) -> u32;

    // FIXME: Should be list of resources not just one resource
    #[wasm_bindgen(method, js_name = getUsedCapacity)]
    pub fn get_used_capacity(this: &Store, resource: &JsValue) -> u32;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub type Spawn;

    #[wasm_bindgen(method, getter = id)]
    pub fn id(this: &Spawn) -> JsString;

    #[wasm_bindgen(method, getter = structureType)]
    pub fn structure_type(this: &Spawn) -> JsString;

    #[wasm_bindgen(method, getter = my)]
    pub fn my(this: &Spawn) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn spawning(this: &Spawn) -> Option<Spawning>;

    #[wasm_bindgen(method, js_name = spawnCreep)]
    pub fn spawn_creep(this: &Spawn, body: &Array, name: &str, options: Option<&Object>) -> ReturnCode;

    #[wasm_bindgen(method, getter = store)]
    pub fn store(this: &Spawn) -> Store;
}

#[wasm_bindgen]
extern "C" {
    pub type Game;
    #[wasm_bindgen(static_method_of = Game, getter = creeps)]
    fn creeps() -> Object;
    #[wasm_bindgen(static_method_of = Game, getter = spawns)]
    fn spawns() -> Object;
    #[wasm_bindgen(static_method_of = Game, getter = time)]
    pub fn time() -> u32;
}

pub fn spawns() -> ScreepHashMap<JsString, Spawn> {
    ScreepHashMap {
        object: Game::spawns(),
        phantom_key: PhantomData::default(),
        phantom_value: PhantomData::default(),
    }
}

pub fn creeps() -> ScreepHashMap<JsString, Spawn> {
    ScreepHashMap {
        object: Game::creeps(),
        phantom_key: PhantomData::default(),
        phantom_value: PhantomData::default(),
    }

}
