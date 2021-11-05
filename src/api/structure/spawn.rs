use wasm_bindgen::prelude::*;
use js_sys::{ Map, Object };


struct Spawns;

#[wasm_bindgen]
extern "C" {
    pub type Spawn;
    #[wasm_bindgen(static_method_of = Game, getter = creeps)]
    pub fn creeps() -> Map;
}
