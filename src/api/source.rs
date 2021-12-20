use crate::api::array::ScreepsArray;
use wasm_bindgen::{JsCast, prelude::*};
use super::cast::ScreepsFromJsValue;
use super::effect::Effect;
use super::room::Room;
use super::object_id::ObjectId;
use super::room_position::RoomPosition;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = Source)]
    pub type Source;

    #[wasm_bindgen(method, getter = pos)]
    pub fn position(this: &Source) -> RoomPosition;

    #[wasm_bindgen(method, getter = effects)]
    pub fn effects(this: &Source) -> ScreepsArray<Effect>;

    #[wasm_bindgen(method, getter = room)]
    pub fn room(this: &Source) -> Room;

    #[wasm_bindgen(method, getter = energy)]
    pub fn energy(this: &Source) -> i32;

    #[wasm_bindgen(method, getter = energyCapacity)]
    pub fn energy_capacity(this: &Source) -> i32;

    #[wasm_bindgen(method, getter = id)]
    pub fn object_id(this: &Source) -> ObjectId<Source>;

    #[wasm_bindgen(method, getter = ticksToRegeneration)]
    pub fn ticks_to_regeneration(this: &Source) -> ObjectId<Source>;
}

impl ScreepsFromJsValue for Source {
    fn from_js_value(val: JsValue) -> Self {
        val.unchecked_into()
    }
}
