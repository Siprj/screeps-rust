use super::creep::Creep;
use super::structure::spawn::Spawn;
use crate::api::map::ScreepsMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type Game;
    #[wasm_bindgen(static_method_of = Game, getter = creeps)]
    fn creeps() -> ScreepsMap<String, Creep>;
    #[wasm_bindgen(static_method_of = Game, getter = spawns)]
    fn spawns() -> ScreepsMap<String, Spawn>;
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
