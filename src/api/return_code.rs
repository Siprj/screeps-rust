use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive)]
#[repr(i8)]
/// Type corresponding to `ERR_*` constants. This type is usually return by
/// Screeps API functions.
///
/// Note: Due to Screeps API constants not being injective we had to merge some of
/// the constants together!!!
///
/// See [Screeps::Constants](https://docs.screeps.com/api/#Constants) for more
/// details.
pub enum ReturnCode {
    Ok = 0,
    NotOwner = -1,
    NoPath = -2,
    NameExists = -3,
    Busy = -4,
    NotFound = -5,
    NotEnough = -6,
    InvalidTarget = -7,
    Full = -8,
    NotInRange = -9,
    InvalidArgs = -10,
    Tired = -11,
    NoBodypart = -12,
    RclNotEnough = -14,
    GclNotEnough = -15,
}

impl wasm_bindgen::convert::IntoWasmAbi for ReturnCode {
    type Abi = i32;

    #[inline]
    fn into_abi(self) -> Self::Abi {
        (self as i32).into_abi()
    }
}

impl wasm_bindgen::convert::FromWasmAbi for ReturnCode {
    type Abi = i32;

    #[inline]
    unsafe fn from_abi(js: i32) -> Self {
        Self::from_i32(js).unwrap()
    }
}

impl wasm_bindgen::describe::WasmDescribe for ReturnCode {
    fn describe() {
        wasm_bindgen::describe::inform(wasm_bindgen::describe::I32)
    }
}
