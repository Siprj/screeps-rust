use wasm_bindgen::{JsCast, prelude::*};
use crate::api::cast::{ScreepsFromJsValue, ScreepsToJsValue};
use crate::api::creep::BodyPartType;
use crate::api::{array::ScreepsArray, room_position::RoomPosition};
use crate::api::effect::Effect;
use super::super::room::Room;
use super::super::store::Store;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = Spawn)]
    pub type Spawn;

    #[wasm_bindgen(method, getter = pos)]
    pub fn pos(this: &Spawn) -> RoomPosition;

    #[wasm_bindgen(method, getter = effects)]
    pub fn effects(this: &Spawn) -> ScreepsArray<Effect>;

    #[wasm_bindgen(method, getter = room)]
    pub fn room(this: &Spawn) -> Room;

    #[wasm_bindgen(method, getter = hits)]
    pub fn hitpoints(this: &Spawn) -> u32;

    #[wasm_bindgen(method, getter = hitsMax)]
    pub fn hitpoints_maxiumu(this: &Spawn) -> u32;

    #[wasm_bindgen(method, getter = id)]
    pub fn id(this: &Spawn) -> String;

    #[wasm_bindgen(method, getter = name)]
    pub fn name(this: &Spawn) -> String;

    #[wasm_bindgen(method, getter = my)]
    pub fn is_my(this: &Spawn) -> bool;

    #[wasm_bindgen(method, getter = store)]
    pub fn store(this: &Spawn) -> Store;

    #[wasm_bindgen(method, js_name = spawnCreep)]
    pub fn spawn_creep(this: &Spawn, body: ScreepsArray<BodyPartType>, name: &str) -> Store;

    // TODO:
    //  * structureType
    //  * destroy
    //  * isActive
    //  * notifyWhenAttacked
    //  * owner
    //  * memory
    //  * spawning
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
