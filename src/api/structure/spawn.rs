use wasm_bindgen::prelude::*;
use crate::api::{array::ScreepsArray, room_position::RoomPosition};
use crate::api::effect::Effect;
use super::super::room::Room;

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

    // TODO:
}
