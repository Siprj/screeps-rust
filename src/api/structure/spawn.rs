use super::super::room::Room;
use super::super::store::Store;
use crate::api::cast::{ScreepsFromJsValue, ScreepsToJsValue};
use crate::api::creep::BodyPartType;
use crate::api::effect::Effect;
use crate::api::object_id::ObjectId;
use crate::api::return_code::ReturnCode;
use crate::api::{array::ScreepsArray, room_position::RoomPosition};
use js_sys::JsString;
use wasm_bindgen::{prelude::*, JsCast};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = StructureSpawn)]
    pub type Spawning;

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

    // TODO:
    //  * setDirections
    //  * directions
}


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = Spawn)]
    pub type Spawn;

    #[wasm_bindgen(method, getter = pos)]
    pub fn position(this: &Spawn) -> RoomPosition;

    #[wasm_bindgen(method, getter = effects)]
    pub fn effects(this: &Spawn) -> ScreepsArray<Effect>;

    #[wasm_bindgen(method, getter = room)]
    pub fn room(this: &Spawn) -> Room;

    #[wasm_bindgen(method, getter = hits)]
    pub fn hitpoints(this: &Spawn) -> u32;

    #[wasm_bindgen(method, getter = hitsMax)]
    pub fn hitpoints_maxiumu(this: &Spawn) -> u32;

    #[wasm_bindgen(method, getter = id)]
    pub fn object_id(this: &Spawn) -> ObjectId<Spawn>;

    #[wasm_bindgen(method, getter = name)]
    pub fn name(this: &Spawn) -> String;

    #[wasm_bindgen(method, getter = my)]
    pub fn is_my(this: &Spawn) -> bool;

    #[wasm_bindgen(method, getter = store)]
    pub fn store(this: &Spawn) -> Store;

    #[wasm_bindgen(method, js_name = spawnCreep)]
    pub fn spawn_creep(this: &Spawn, body: ScreepsArray<BodyPartType>, name: &str) -> Store;

    #[wasm_bindgen(method, js_name = spawning)]
    pub fn spawning(this: &Spawn) -> Option<Spawning>;

    // TODO:
    //  * structureType
    //  * destroy
    //  * isActive
    //  * notifyWhenAttacked
    //  * owner
    //  * memory
    //  * store
    //  * recycleCreep
    //  * renewCreep
}

impl ScreepsFromJsValue for Spawn {
    fn from_js_value(val: JsValue) -> Self {
        val.unchecked_into()
    }
}

impl ScreepsToJsValue for Spawn {
    fn to_js_value(self) -> JsValue {
        self.unchecked_into()
    }
}
