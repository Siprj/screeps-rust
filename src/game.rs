use wasm_bindgen::prelude::*;
use std::marker::PhantomData;
use js_sys::{JsString, Object};

struct ScreepObject<K, V> {
    object: Object,
    phantom_key: PhantomData<K>,
    phantom_value: PhantomData<V>,
}

impl ScreepObject<K, V> {
    fn map(self, f: Fn<&V>) {
        let f_wraped = |object: JsValue| {
            ScreepObject {
                object,
                phantom_key: PhantomData::default(),
                phantom_value: PhantomData::default(),
            }
        };
        obj_map(self.object, f_wraped);
    }
}

#[wasm_bindgen(module = "utils")]
extern "C" {
    fn obj_map(obj: &JsValue, f: Fn<&JsValue>);
}

// ScreepObjectIterator<K, V> {
// }

//impl std::iter::Iterator for ScreepObject<K, V> {
//
//}

struct Spawn;

#[wasm_bindgen]
extern "C" {
    pub type Game;
    #[wasm_bindgen(static_method_of = Game, getter = creeps)]
    fn creeps() -> Object;
    #[wasm_bindgen(static_method_of = Game, getter = spawns)]
    fn spawns() -> Object;
}

pub fn spawns() -> ScreepObject<JsString, Spawn> {
    ScreepObject {
        object: Game::spawns(),
        phantom_key: PhantomData::default(),
        phantom_value: PhantomData::default(),
    }
}

pub fn creeps() -> Object {
    Game::creeps()
}
