use js_sys::{JsString, Object};
use wasm_bindgen::prelude::*;

use super::color::Color;
use super::find::FindType;
use super::return_code::ReturnCode;
use super::structure;

#[wasm_bindgen]
extern "C" {

    /// Related javascript documentation:
    /// [RoomPosition](https://docs.screeps.com/api/#RoomPosition)
    ///
    /// Contains `X`, `Y` postion in the room with `roomName`
    #[wasm_bindgen]
    pub type RoomPosition;

    #[wasm_bindgen(constructor)]
    fn new(x: u8, y: u8, roomName: JsString) -> RoomPosition;

    /// [RoomPosition.x()](https://docs.screeps.com/api/#RoomPosition.x)
    #[wasm_bindgen(method, getter = x)]
    pub fn x(this: &RoomPosition) -> u8;

    /// [RoomPosition.x()](https://docs.screeps.com/api/#RoomPosition.y)
    #[wasm_bindgen(method, getter = y)]
    pub fn y(this: &RoomPosition) -> u8;

    /// [RoomPosition.x()](https://docs.screeps.com/api/#RoomPosition.roomName)
    #[wasm_bindgen(method, getter = roomName)]
    pub fn room_name(this: &RoomPosition) -> JsString;

    /// [RoomPosition.create_construction_site()](https://docs.screeps.com/api/#RoomPosition.createConstructionSite)
    /// Returns:
    ///   * Ok
    ///   * InvalidTarget - Can't be placed on specific location.
    ///   * Full - You have to meany construction sites. Maximum is 100 per player.
    ///   * InvalidArgs - The location is invalid.
    ///   * RclNotEnough - Room controller level insufficient.
    #[wasm_bindgen(method, js_name = createConstructionSite)]
    pub fn create_construction_site(
        this: &RoomPosition,
        structure_type: structure::StructureType,
        name: Option<&str>,
    ) -> ReturnCode;

    /// [RoomPosition.createFlag()](https://docs.screeps.com/api/#RoomPosition.createFlag)
    /// Returns:
    ///   * NameExists - Flag with same name already exists.
    ///   * InvalidArgs - The location is invalid.
    #[wasm_bindgen(method, js_name = createFlag)]
    pub fn create_flag(
        this: &RoomPosition,
        name: &str,
        color: Color,
        secondary_color: Color,
    ) -> ReturnCode;

    /// [RoomPosition.createFlag()](https://docs.screeps.com/api/#RoomPosition.createFlag)
    /// Returns:
    ///   * NameExists - Flag with same name already exists.
    ///   * InvalidArgs - The location is invalid.
    #[wasm_bindgen(method, js_name = findClosestByPath)]
    pub fn find_closest_type_by_path(this: &RoomPosition, find_type: FindType) -> Option<Object>;

    /// [RoomPosition.createFlag()](https://docs.screeps.com/api/#RoomPosition.createFlag)
    /// Returns:
    ///   * NameExists - Flag with same name already exists.
    ///   * InvalidArgs - The location is invalid.
    #[wasm_bindgen(method, js_name = findClosestByPath)]
    pub fn find_closest_objects_by_path(this: &RoomPosition, objects: FindType) -> Option<Object>;

}
