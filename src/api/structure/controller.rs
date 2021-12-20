use super::super::room::Room;
use crate::api::effect::Effect;
use crate::api::object_id::ObjectId;
use crate::api::{array::ScreepsArray, room_position::RoomPosition};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub type Controller;

    #[wasm_bindgen(method, getter = pos)]
    pub fn position(this: &Controller) -> RoomPosition;

    #[wasm_bindgen(method, getter = effects)]
    pub fn effects(this: &Controller) -> ScreepsArray<Effect>;

    #[wasm_bindgen(method, getter = room)]
    pub fn room(this: &Controller) -> Room;

    #[wasm_bindgen(method, getter = hits)]
    pub fn hitpoints(this: &Controller) -> u32;

    #[wasm_bindgen(method, getter = hitsMax)]
    pub fn hitpoints_maxiumu(this: &Controller) -> u32;

    #[wasm_bindgen(method, getter = id)]
    pub fn object_id(this: &Controller) -> ObjectId<Controller>;

    #[wasm_bindgen(method, getter = my)]
    pub fn is_my(this: &Controller) -> bool;

    // TODO:
    //  * structureType
    //  * destroy
    //  * isActive
    //  * notifyWhenAttacked
    //  * owner
    //  * isPowerEnabled
    //  * level
    //  * progress
    //  * progressTotal
    //  * reservation
    //  * safeMode
    //  * safeModeAvailable
    //  * safeModeCooldown
    //  * sign
    //  * ticksToDowngrade
    //  * upgradeBlocked
    //  * activateSafeMode
    //  * unclaim
}

