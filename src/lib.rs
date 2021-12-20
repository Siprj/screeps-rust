#![feature(exact_size_is_empty)]
extern crate wee_alloc;
pub mod api;
mod console;

use api::return_code::*;
use api::panic_hook;
use api::game::Game;

use crate::api::array::ScreepsArray;
use crate::api::constants::ResourceType;
use crate::api::creep::BodyPartType;
use crate::api::find::FindSources;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[no_mangle]
pub fn main() {
    panic_hook::set_hook();
    let creep_iter = Game::creeps().values_ref().iter();
    let creep_count = creep_iter.len();

    console::log(&format!("creep count: {}", creep_count));

    for spawn in Game::spawns().values_ref().iter() {
        console::log(&format!("bla"));
        let s = spawn.spawning();
        console::log(&format!("bla44444444"));
        if s.is_some() {
            console::log(&format!("spawin is busy"));
            console::log(&format!("spawin [{:?}] is busy", spawn.object_id()));
        } else {
            console::log(&format!("bla33333333"));
            let stored_energy = spawn.store().stored();
            console::log(&format!("energy: {}", stored_energy));
            if stored_energy == 300 && creep_count < 5 {
                let body = ScreepsArray::new();
                body.push(BodyPartType::Move);
                body.push(BodyPartType::Move);
                body.push(BodyPartType::Carry);
                body.push(BodyPartType::Work);

                spawn.spawn_creep(body, &format!("harvester-{}", Game::time()));
            }
        }
    }
    console::log(&format!("bla22222222"));

    for creep in creep_iter {
        let creep_memory = &creep.memory();
        match creep_memory.get_bool("harvesting") {
            Some(true) => {
                if creep.store().stored_by_resource(ResourceType::Energy) == 0 {
                    creep_memory.set_bool("harvesting", Some(false));
                }
            }
            _ => {
                if creep.store().stored_by_resource(ResourceType::Energy) == 0 {
                    creep_memory.set_bool("harvesting", Some(false));
                }
            }
        }

        if creep_memory.get_bool("harvesting").unwrap() {
            let source = creep.room().find(FindSources::SourcesActive).get(0);
            if creep.position().is_near_to(source.position()) {
                creep.harvest_energy(&source);
            } else {
                creep.move_to_position(&source.position());
            }
        } else {
            if let Some(c) = creep.room().controller() {
                let r = creep.upgrade_controller(&c);
                if r == ReturnCode::NotInRange {
                    creep.move_to_position(&c.position());
                } else if r != ReturnCode::Ok {
                    console::log(&format!("couldn't upgrade: {:?}", r));
                }
            } else {
                console::log("creep room has no controller!");
            }
        }
    }
}
