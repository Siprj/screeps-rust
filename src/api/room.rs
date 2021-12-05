use wasm_bindgen::{JsCast, prelude::*};

use super::cast::{ScreepsFromJsValue, ScreepsToJsValue};

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
    fn find_path(this: &Room) -> u32;

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
    //  * findPath
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
