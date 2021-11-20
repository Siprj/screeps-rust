use crate::api::map::ScreepsMap;
use js_sys::Object;
use wasm_bindgen::prelude::*;
use super::creep::Creep;

#[wasm_bindgen]
extern "C" {
    pub type Game;
    #[wasm_bindgen(static_method_of = Game, getter = creeps)]
    fn creeps() -> ScreepsMap<String, Creep>;
    #[wasm_bindgen(static_method_of = Game, getter = spawns)]
    fn spawns() -> Object;
    #[wasm_bindgen(static_method_of = Game, getter = time)]
    pub fn time() -> u32;

    // TODO:
    //   * constructionSites
    //   * cpi
    //   * flags
    //   * gcl
    //   * gpl
    //   * map
    //   * market
    //   * powerCreeps
    //   * resources
    //   * rooms
    //   * spawns
    //   * shard
    //   * structures
    //   * cpu.getHeapStatistics
    //   * cpu.getUsed
    //   * cpu.halt
    //   * cpu.setShardLimits
    //   * cpu.unlock
    //   * cpu.generatePixel
    //   * getObjectById
    //   * notify
}
