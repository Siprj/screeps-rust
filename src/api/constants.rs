/// Representation of effects. Not sure what they are...
/// TODO: Find out what these values can be.
pub struct EffectOrPowerId(u32);

impl wasm_bindgen::convert::IntoWasmAbi for EffectOrPowerId {
    type Abi = u32;

    #[inline]
    fn into_abi(self) -> Self::Abi {
        match self {
            EffectOrPowerId(v) => v.into_abi()
        }
    }
}

impl wasm_bindgen::convert::FromWasmAbi for EffectOrPowerId {
    type Abi = u32;

    #[inline]
    unsafe fn from_abi(js: u32) -> Self {
        EffectOrPowerId(js)
    }
}

impl wasm_bindgen::describe::WasmDescribe for EffectOrPowerId {
    fn describe() {
        wasm_bindgen::describe::inform(wasm_bindgen::describe::U32)
    }
}
