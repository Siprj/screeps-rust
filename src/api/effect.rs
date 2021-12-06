use wasm_bindgen::{prelude::*, JsCast};

use crate::api::constants::EffectOrPowerId;

use super::cast::{ScreepsFromJsValue, ScreepsToJsValue};

#[wasm_bindgen]
extern "C" {

    /// Object returned by [RoomObject.effects]](https://docs.screeps.com/api/#RoomObject.effects)
    pub type Effect;
    #[wasm_bindgen(method, getter = efect)]

    /// Effect ID of the applied effect. Can be either natural effect ID or Power ID.
    pub fn effect(this: &Effect) -> EffectOrPowerId;

    /// Power level of the applied effect. Absent if the effect is not a Power effect.
    #[wasm_bindgen(method, getter = level)]
    pub fn level(this: &Effect) -> Option<u32>;

    /// How many ticks will the effect last.
    #[wasm_bindgen(method, getter = ticksRemaining)]
    pub fn ticksRemaining(this: &Effect) -> u32;
}

impl ScreepsFromJsValue for Effect {
    fn from_js_value(val: JsValue) -> Self {
        val.unchecked_into()
    }
}

impl ScreepsToJsValue for Effect {
    fn to_js_value(self) -> JsValue {
        self.unchecked_into()
    }
}
