use std::panic;

// Copied from package: https://github.com/rustwasm/console_error_panic_hook

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

use super::console::log;

#[wasm_bindgen]
extern {
    type Error;

    #[wasm_bindgen(constructor)]
    fn new() -> Error;

    #[wasm_bindgen(structural, method, getter)]
    fn stack(error: &Error) -> String;
}

fn hook(info: &panic::PanicInfo) {
    let mut msg = info.to_string();
    msg.push_str("\n\nStack:\n\n");
    let e = Error::new();
    let stack = e.stack();
    msg.push_str(&stack);

    msg.push_str("\n\n");

    log(&msg);
}

#[inline]
pub fn set_hook_once() {
    use std::sync::Once;
    static SET_HOOK: Once = Once::new();
    SET_HOOK.call_once(|| {
        panic::set_hook(Box::new(hook));
    });
}

#[inline]
pub fn set_hook() {
    use std::sync::Once;
    static SET_HOOK: Once = Once::new();
    SET_HOOK.call_once(|| {
        panic::set_hook(Box::new(hook));
    });
}
