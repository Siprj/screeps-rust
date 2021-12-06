use js_sys::{JsString, Object};
use wasm_bindgen::{prelude::*, JsCast};

use super::cast::{ScreepsFromJsValue, ScreepsToJsValue};
use super::room_position::RoomPosition;

use super::cost_matrix::CostMatrix;

struct RoomName {
    name: JsString,
}

impl wasm_bindgen::convert::IntoWasmAbi for RoomName {
    type Abi = <JsString as wasm_bindgen::convert::IntoWasmAbi>::Abi;

    #[inline]
    fn into_abi(self) -> Self::Abi {
        self.name.into_abi()
    }
}

impl wasm_bindgen::convert::FromWasmAbi for RoomName {
    type Abi = <JsString as wasm_bindgen::convert::FromWasmAbi>::Abi;

    #[inline]
    unsafe fn from_abi(js_str: Self::Abi) -> Self {
        RoomName {
            name: JsString::from_abi(js_str),
        }
    }
}

impl wasm_bindgen::describe::WasmDescribe for RoomName {
    fn describe() {
        JsString::describe()
    }
}

#[wasm_bindgen]
extern "C" {
    pub type FindOptions;

    #[wasm_bindgen(method, setter = ignoreCreeps)]
    fn ignoreCreeps(this: &FindOptions, v: bool);

    #[wasm_bindgen(method, setter = ignoreDestructibleStructures)]
    fn ignoreDestructibleStructures(this: &FindOptions, v: bool);

    #[wasm_bindgen(method, setter = ignoreRoads)]
    fn ignoreRoads(this: &FindOptions, v: bool);

    #[wasm_bindgen(method, setter = costCallback)]
    fn costCallback(this: &FindOptions, v: &Closure<dyn FnMut(RoomName, CostMatrix) -> JsValue>);

    #[wasm_bindgen(method, setter = serialize)]
    fn serialize(this: &FindOptions, v: bool);

    #[wasm_bindgen(method, setter = range)]
    fn range(this: &FindOptions, v: bool);

    #[wasm_bindgen(method, setter = maxOps)]
    fn maxOps(this: &FindOptions, v: u32);

    #[wasm_bindgen(method, setter = heuristicWeight)]
    fn heuristicWeight(this: &FindOptions, v: f32);

    #[wasm_bindgen(method, setter = maxRooms)]
    fn maxRooms(this: &FindOptions, v: u8);

    #[wasm_bindgen(method, setter = plainCost)]
    fn plainCost(this: &FindOptions, v: u32);

    #[wasm_bindgen(method, setter = swampCost)]
    fn swampCost(this: &FindOptions, v: u32);
}

impl FindOptions {
    #[inline]
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}

#[wasm_bindgen]
extern "C" {

    /// An object representing the room in which your units and structures are
    /// in. It can be used to look around, find paths, etc. Every RoomObject
    /// in the room contains its linked Room instance in the room property
    ///
    /// [Room]](https://docs.screeps.com/api/#Room)
    pub type Room;

    #[wasm_bindgen(method, getter = energyAvailable)]
    fn energy_avalilable(this: &Room) -> u32;

    #[wasm_bindgen(method, getter = energyCapacityAvailabl)]
    fn energy_capacity(this: &Room) -> u32;

    #[wasm_bindgen(method, js_name = findPath)]
    fn find_path(this: &Room, from: &RoomPosition, to: &RoomPosition) -> u32;

    #[wasm_bindgen(method, js_name = findPath)]
    fn find_path_opt(this: &Room, from: &RoomPosition, to: &RoomPosition, options: FindOptions) -> u32;

    // TODO:
    //  * controller
    //  * memory
    //  * name
    //  * storage
    //  * terminal
    //  * visual
    //  * serializePath
    //  * createConstructionSite
    //  * createFlag
    //  * find
    //  * findExitTo
    //  * getEventLog
    //  * getPositionAt
    //  * getTerrain
    //  * lookAt
    //  * lookAtArea
    //  * lookForAt
    //  * lookForAtArea
}

impl ScreepsFromJsValue for Room {
    fn from_js_value(val: JsValue) -> Self {
        val.unchecked_into()
    }
}

impl ScreepsToJsValue for Room {
    fn to_js_value(self) -> JsValue {
        self.unchecked_into()
    }
}
