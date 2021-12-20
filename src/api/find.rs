use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use wasm_bindgen::JsCast;
use super::room_position::RoomPosition;
use super::creep::Creep;
use super::source::Source;
use super::structure::spawn::Spawn;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive)]
#[repr(i8)]
/// Type corresponding to `FIND_*` constants. This type is usually used by find
/// functions.
///
/// See [Screeps::Constants](https://docs.screeps.com/api/#Constants) for more
/// details.
pub enum FindType {
    ExitTop = 1,
    ExitRight = 3,
    ExitBottom = 5,
    ExitLeft = 7,
    Exit = 10,

    Creeps = 101,
    MyCreeps = 102,
    HostileCreeps = 103,

    SourcesActive = 104,
    Sources = 105,

    DroppedResources = 106,

    Structures = 107,
    MyStructures = 108,
    HostileStructures = 109,

    Flags = 110,

    MySpawns = 112,
    HostileSpawns = 113,

    ConstructionSites = 111,
    MyConstructionSites = 114,
    HostileConstructionSites = 115,

    Minerals = 116,

    Nukes = 117,

    Tombstones = 118,

    PowerCreeps = 119,
    MyPowerCreeps = 120,
    HostilePowerCreeps = 121,

    Deposits = 122,

    Ruins = 123,
}

impl wasm_bindgen::convert::IntoWasmAbi for FindType {
    type Abi = i32;

    #[inline]
    fn into_abi(self) -> Self::Abi {
        (self as i32).into_abi()
    }
}

impl wasm_bindgen::convert::FromWasmAbi for FindType {
    type Abi = i32;

    #[inline]
    unsafe fn from_abi(js: i32) -> Self {
        Self::from_i32(js).unwrap()
    }
}

impl wasm_bindgen::describe::WasmDescribe for FindType {
    fn describe() {
        wasm_bindgen::describe::inform(wasm_bindgen::describe::I32)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive)]
#[repr(u8)]
pub enum FindExits {
    ExitTop = 1,
    ExitRight = 3,
    ExitBottom = 5,
    ExitLeft = 7,
    Exit = 10,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive)]
#[repr(u8)]
pub enum FindCreeps {
    Creeps = 101,
    MyCreeps = 102,
    HostileCreeps = 103,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive)]
#[repr(u8)]
pub enum FindSources {
    SourcesActive = 104,
    Sources = 105,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive)]
#[repr(u8)]
pub enum FindResources {
    DroppedResources = 106,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive)]
#[repr(u8)]
pub enum FindStructures {
    Structures = 107,
    MyStructures = 108,
    HostileStructures = 109,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive)]
#[repr(u8)]
pub enum FindFlags {
    Flags = 110,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive)]
#[repr(u8)]
pub enum FindSpawns {
    MySpawns = 112,
    HostileSpawns = 113,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive)]
#[repr(u8)]
pub enum FindConstructionSites {
    ConstructionSites = 111,
    MyConstructionSites = 114,
    HostileConstructionSites = 115,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive)]
#[repr(u8)]
pub enum FindMinerals {
    Minerals = 116,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive)]
#[repr(u8)]
pub enum FindNukes {
    Nukes = 117,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive)]
#[repr(u8)]
pub enum FindTombstones {
    Tombstones = 118,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive)]
#[repr(u8)]
pub enum FindPowerCreeps {
    PowerCreeps = 119,
    MyPowerCreeps = 120,
    HostilePowerCreeps = 121,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive)]
#[repr(u8)]
pub enum FindDeposits {
    Deposits = 122,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive)]
#[repr(u8)]
pub enum FindRuins {
    Ruins = 123,
}

pub trait ToFind {
    type OutputType: JsCast;
    fn to_find(self) -> u8;
}

impl ToFind for FindExits {
    type OutputType = RoomPosition;

    fn to_find(self) -> u8 {
        self as u8
    }
}

impl ToFind for FindCreeps {
    type OutputType = Creep;

    fn to_find(self) -> u8 {
        self as u8
    }
}

impl ToFind for FindSources {
    type OutputType = Source;

    fn to_find(self) -> u8 {
        self as u8
    }
}

// TODO....
// impl ToFind for FindResources {
//     type OutputType = ;
//
//     fn to_find(self) -> u8 {
//         self as u8
//     }
// }
//
// TODO....
// impl ToFind for FindStructures {
//     type OutputType = ;
//
//     fn to_find(self) -> u8 {
//         self as u8
//     }
// }
//
// TODO....
// impl ToFind for FindFlags {
//     type OutputType = ;
//
//     fn to_find(self) -> u8 {
//         self as u8
//     }
// }
//
impl ToFind for FindSpawns {
    type OutputType = Spawn;

    fn to_find(self) -> u8 {
        self as u8
    }
}

// TODO....
// impl ToFind for FindConstructionSites {
//     type OutputType = ;
//
//     fn to_find(self) -> u8 {
//         self as u8
//     }
// }
//
// TODO....
// impl ToFind for FindMinerals {
//     type OutputType = ;
//
//     fn to_find(self) -> u8 {
//         self as u8
//     }
// }
//
// TODO....
// impl ToFind for FindNukes {
//     type OutputType = ;
//
//     fn to_find(self) -> u8 {
//         self as u8
//     }
// }
//
// TODO....
// impl ToFind for FindTombstones {
//     type OutputType = ;
//
//     fn to_find(self) -> u8 {
//         self as u8
//     }
// }
//
// TODO....
// impl ToFind for FindPowerCreeps {
//     type OutputType = ;
//
//     fn to_find(self) -> u8 {
//         self as u8
//     }
// }
//
// TODO....
// impl ToFind for FindDeposits {
//     type OutputType = ;
//
//     fn to_find(self) -> u8 {
//         self as u8
//     }
// }
//
// TODO....
// impl ToFind for FindRuins {
//     type OutputType = ;
//
//     fn to_find(self) -> u8 {
//         self as u8
//     }
// }
//
