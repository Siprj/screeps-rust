use std::str::FromStr;

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

impl ToString for BodyPartType {
    fn to_string(&self) -> String {
        match self {
            BodyPartType::Move => "move".to_string(),
            BodyPartType::Work => "work".to_string(),
            BodyPartType::Carry => "carry".to_string(),
            BodyPartType::Attack => "attack".to_string(),
            BodyPartType::RangedAttack => "ranged_attack".to_string(),
            BodyPartType::Tough => "tough".to_string(),
            BodyPartType::Heal => "heal".to_string(),
            BodyPartType::Claim => "claim".to_string()
        }
    }
}

impl FromStr for BodyPartType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
             "move" => Ok(BodyPartType::Move),
             "work" => Ok(BodyPartType::Work),
             "carry" => Ok(BodyPartType::Carry),
             "attack" => Ok(BodyPartType::Attack),
             "rangedAttack" => Ok(BodyPartType::RangedAttack),
             "tough" => Ok(BodyPartType::Tough),
             "heal" => Ok(BodyPartType::Heal),
             "claim" => Ok(BodyPartType::Claim),
             v => Err(format!("unknown creep body part: {}", v))
        }
    }
}

impl wasm_bindgen::convert::IntoWasmAbi for BodyPartType {
    type Abi = <String as wasm_bindgen::convert::IntoWasmAbi>::Abi;

    #[inline]
    fn into_abi(self) -> Self::Abi {
        self.to_string().into_abi()
    }
}

impl wasm_bindgen::convert::FromWasmAbi for BodyPartType {
    type Abi = <String as wasm_bindgen::convert::FromWasmAbi>::Abi;

    #[inline]
    unsafe fn from_abi(js_str: Self::Abi) -> Self {
        FromStr::from_str(String::from_abi(js_str).as_str()).unwrap()
    }
}

impl wasm_bindgen::describe::WasmDescribe for BodyPartType {
    fn describe() {
        String::describe()
    }
}

impl ScreepsToJsValue for BodyPartType {
    fn to_js_value(self) -> JsValue {
        JsValue::from_str(&self.to_string())
    }
}

impl ScreepsFromJsValue for BodyPartType {
    fn from_js_value(val: JsValue) -> Self {
        FromStr::from_str(&val.as_string().unwrap()).unwrap()
    }
}

impl ScreepsToJsValue for BodyPart {
    fn to_js_value(self) -> JsValue {
        self.unchecked_into()
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

    #[wasm_bindgen(method, getter = hits)]
    pub fn hitpoints(this: &Creep) -> u32;

    #[wasm_bindgen(method, getter = hitsMax)]
    pub fn hitpoints_maxiumu(this: &Creep) -> u32;

    /// [Creep.body](]https://docs.screeps.com/api/#Creep.body) return array
    /// of body parts. See: https://docs.screeps.com/api/#Creep for more
    /// details regardin what body part is.
    #[wasm_bindgen(method, getter = body)]
    pub fn body(this: &Creep) -> ScreepsArray<BodyPart>;

    /// [Creep.fatigue](https://docs.screeps.com/api/#Creep.fatigue)
    /// The movement fatigue indicator. If it is greater than zero, the creep
    /// cannot move.
    #[wasm_bindgen(method, getter = fatigue)]
    pub fn fatigue(this: &Creep) -> u32;

    #[wasm_bindgen(method, getter = id)]
    pub fn id(this: &Creep) -> String;

    #[wasm_bindgen(method, getter = my)]
    pub fn is_my(this: &Creep) -> bool;

    #[wasm_bindgen(method, getter = name)]
    pub fn name(this: &Creep) -> String;

    #[wasm_bindgen(method, js_name = moveTo)]
    pub fn move_to_xy(this: &Creep, x: u8, y: u8) -> String;

    #[wasm_bindgen(method, js_name = moveTo)]
    pub fn move_to_pos(this: &Creep, pos: RoomPosition) -> String;

    // TODO:
    //  * memory
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
    fn to_js_value(self) -> JsValue {
        self.unchecked_into()
    }
}
