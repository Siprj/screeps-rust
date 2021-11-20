use wasm_bindgen::{JsCast, prelude::*};
use crate::api::array::ScreepsArray;

use super::{cast::{ScreepsFromJsValue, ScreepsToJsValue}, room_position::RoomPosition};
use super::room::Room;
use super::effect::Effect;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = Creep)]
    pub type Creep;

    #[wasm_bindgen(method, getter = pos)]
    pub fn pos(this: &Creep) -> RoomPosition;

    #[wasm_bindgen(method, getter = effects)]
    pub fn effects(this: &Creep) -> ScreepsArray<Effect>;

    #[wasm_bindgen(method, getter = room)]
    pub fn room(this: &Creep) -> Room;

    // TODO:
    //  * effects
    //  * room
    //  * body
    //  * fatigue
    //  * hits
    //  * hitsMax
    //  * id
    //  * memory
    //  * my
    //  * name
    //  * owner
    //  * saying
    //  * spawning
    //  * store
    //  * ticksToLive
    //  * attack
    //  * attackController
    //  * build
    //  * cancelOrder
    //  * claimController
    //  * dismantel
    //  * drop
    //  * generateSeafeMode
    //  * getActiveBodyparts
    //  * harvest
    //  * heal
    //  * move
    //  * homveByPath
    //  * moveTo
    //  * notifyWhenAttacked
    //  * pickup
    //  * pull
    //  * rangedAttack
    //  * rangedHeal
    //  * rangedMassAttack
    //  * repair
    //  * reserveController
    //  * say
    //  * signController
    //  * suicide
    //  * transfer
    //  * upgradeController
    //  * withdraw
}

impl ScreepsFromJsValue for Creep {
    fn from_js_value(val: JsValue) -> Self {
        val.unchecked_into()
    }
}

impl ScreepsToJsValue for Creep {
    fn to_js_value(&self) -> JsValue {
        self.into()
    }
}
