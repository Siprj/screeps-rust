use wasm_bindgen::prelude::*;

struct SerializedCostMatrix {
    v: JsValue,
}

impl wasm_bindgen::convert::IntoWasmAbi for SerializedCostMatrix {
    type Abi = <JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;

    #[inline]
    fn into_abi(self) -> Self::Abi {
        self.v.into_abi()
    }
}

impl wasm_bindgen::convert::FromWasmAbi for SerializedCostMatrix {
    type Abi = <JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;

    #[inline]
    unsafe fn from_abi(value: Self::Abi) -> Self {
        SerializedCostMatrix {
            v: JsValue::from_abi(value),
        }
    }
}

impl wasm_bindgen::describe::WasmDescribe for SerializedCostMatrix {
    fn describe() {
        JsValue::describe()
    }
}

#[wasm_bindgen]
extern "C" {
    pub type CostMatrix;

    #[wasm_bindgen(constructor)]
    fn new() -> CostMatrix;

    #[wasm_bindgen(method, js_name = set)]
    fn set(this: &CostMatrix, x: u8, y: u8, cost: u8);

    #[wasm_bindgen(method, getter = set)]
    fn get(this: &CostMatrix, x: u8, y: u8);

    #[wasm_bindgen(method, js_name = clone)]
    fn clone(this: &CostMatrix) -> CostMatrix;

    #[wasm_bindgen(method, js_name = serialize)]
    fn serialize(this: &CostMatrix) -> SerializedCostMatrix;

    #[wasm_bindgen(js_namespace = PathFinder, static_method_of = CostMatrix, js_name = deserialize)]
    fn deserialize(costMatrix: SerializedCostMatrix) -> CostMatrix;
}
