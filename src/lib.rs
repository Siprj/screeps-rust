extern crate wee_alloc;
mod console;
mod game;

use game::*;
use js_sys::{Array, Object};
use wasm_bindgen::{JsCast, JsValue};


// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[no_mangle]
pub fn main() {
    let creep_count = Object::values(&creeps().object).length();

    console::log(&format!("creep count: {}", creep_count));

    for v in Object::values(&spawns().object).iter() {
        let spawn: Spawn = v.unchecked_into();
        if spawn.spawning().is_some() {
            console::log(&format!("spawin [{}] is busy", spawn.id()));
        } else {
            let stored_energy = spawn.store().get_used_capacity(&resource_energy());
            console::log(&format!("energy: {}", stored_energy));
            if stored_energy == 300 && creep_count < 5 {
                let body = Array::new();
                body.push(&JsValue::from_str("move"));
                body.push(&JsValue::from_str("move"));
                body.push(&JsValue::from_str("carry"));
                body.push(&JsValue::from_str("work"));


                spawn.spawn_creep(&body, &format!("harvester-{}", Game::time()), None);
            }
        }
    }

    for v in Object::values(&creeps().object).iter() {
        let creep: Creep = v.unchecked_into();

        let creep_memory = creep.memory();
        match mem::get_bool(&creep_memory, "harvesting") {
            Some(true) => {
                if creep.store().get_free_capacity(&resource_energy()) == 0 {
                    mem::set_bool(&creep_memory, "harvesting", Some(false));
                }
            },
            _ => {
                if creep.store().get_used_capacity(&resource_energy()) == 0 {
                    mem::set_bool(&creep_memory, "harvesting", Some(true));
                }
            }
        }

        if mem::get_bool(&creep_memory, "harvesting").unwrap() {
            let source = &creep
                .room()
                .find(FIND_SOURCES_ACTIVE).to_vec()[0];
            if creep.pos().is_near_to(source) {
                creep.harvest(source);
            } else {
                creep.move_to(source);
            }
        } else {
            if let Some(c) = creep
                .room()
                .controller()
            {
                let r = creep.upgrade_controller(&c);
                if r == ReturnCode::NotInRange {
                    creep.move_to(&c);
                } else if r != ReturnCode::Ok {
                    console::log(&format!("couldn't upgrade: {:?}", r));
                }
            } else {
                console::log("creep room has no controller!");
            }
        }
    }
}
