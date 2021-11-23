use wasm_bindgen::{JsCast, prelude::*};
use crate::api::array::ScreepsArray;

use super::{cast::{ScreepsFromJsValue, ScreepsToJsValue}, room_position::RoomPosition};
use super::room::Room;
use super::effect::Effect;

pub enum BodyPartType {
    Move,
    Work,
    Carry,
    Attack,
    RangedAttack,
    Tough,
    Heal,
    Claim
}

impl wasm_bindgen::convert::IntoWasmAbi for BodyPartType {
    type Abi = <String as wasm_bindgen::convert::IntoWasmAbi>::Abi;

    #[inline]
    fn into_abi(self) -> Self::Abi {
        let v = match self {
            BodyPartType::Move => "move",
            BodyPartType::Work => "work",
            BodyPartType::Carry => "carry",
            BodyPartType::Attack => "attack",
            BodyPartType::RangedAttack => "ranged_attack",
            BodyPartType::Tough => "tough",
            BodyPartType::Heal => "heal",
            BodyPartType::Claim => "claim"
        };
        v.into_abi()
    }
}

impl wasm_bindgen::convert::FromWasmAbi for BodyPartType {
    type Abi = <String as wasm_bindgen::convert::FromWasmAbi>::Abi;

    #[inline]
    unsafe fn from_abi(js_str: Self::Abi) -> Self {
        match String::from_abi(js_str).as_str() {
             "move" => BodyPartType::Move,
             "work" => BodyPartType::Work,
             "carry" => BodyPartType::Carry,
             "attack" => BodyPartType::Attack,
             "rangedAttack" => BodyPartType::RangedAttack,
             "tough" => BodyPartType::Tough,
             "heal" => BodyPartType::Heal,
             "claim" => BodyPartType::Claim,
             v => panic!("unknown creep body part: {}", v)
        }
    }
}

impl wasm_bindgen::describe::WasmDescribe for BodyPartType {
    fn describe() {
        String::describe()
    }
}

impl ScreepsToJsValue for BodyPart {
    fn to_js_value(&self) -> &JsValue {
        self.unchecked_ref()
    }
}

impl ScreepsFromJsValue for BodyPart {
    fn from_js_value(val: JsValue) -> Self {
        val.unchecked_into()
    }
}

#[wasm_bindgen]
extern "C" {
    pub type BodyPart;

    // TODO:
    //  * boost

    #[wasm_bindgen(method, getter = type)]
    pub fn body_type(this: &BodyPart) -> BodyPartType;

    #[wasm_bindgen(method, getter = hits)]
    pub fn hitpoints(this: &BodyPart) -> u32;

}


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

    /// [Creep.body](]https://docs.screeps.com/api/#Creep.body) return array
    /// of body parts. See: https://docs.screeps.com/api/#Creep for more
    /// details regardin what body part is.
    #[wasm_bindgen(method, getter = body)]
    pub fn body(this: &Creep) -> ScreepsArray<BodyPart>;

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
    fn to_js_value(&self) -> &JsValue {
        self.unchecked_ref()
    }
}
