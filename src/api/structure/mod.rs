use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};

pub mod spawn;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum StructureType {
    Spawn,
    Extension,
    Road,
    Wall,
    Rampart,
    KeeperLair,
    Portal,
    Controller,
    Link,
    Storage,
    Tower,
    Observer,
    PowerBank,
    PowerSpawn,
    Extractor,
    Lab,
    Terminal,
    Container,
    Nuker,
    Factory,
    InvaderCore,
}

impl wasm_bindgen::convert::IntoWasmAbi for StructureType {
    type Abi = <String as IntoWasmAbi>::Abi;

    #[inline]
    fn into_abi(self) -> Self::Abi {
        match self {
            StructureType::Spawn => "spawn".into_abi(),
            StructureType::Extension => "extension".into_abi(),
            StructureType::Road => "road".into_abi(),
            StructureType::Wall => "constructedWall".into_abi(),
            StructureType::Rampart => "rampart".into_abi(),
            StructureType::KeeperLair => "keeperLair".into_abi(),
            StructureType::Portal => "portal".into_abi(),
            StructureType::Controller => "controller".into_abi(),
            StructureType::Link => "link".into_abi(),
            StructureType::Storage => "storage".into_abi(),
            StructureType::Tower => "tower".into_abi(),
            StructureType::Observer => "observer".into_abi(),
            StructureType::PowerBank => "powerBank".into_abi(),
            StructureType::PowerSpawn => "powerSpawn".into_abi(),
            StructureType::Extractor => "extractor".into_abi(),
            StructureType::Lab => "lab".into_abi(),
            StructureType::Terminal => "terminal".into_abi(),
            StructureType::Container => "container".into_abi(),
            StructureType::Nuker => "nuker".into_abi(),
            StructureType::Factory => "factory".into_abi(),
            StructureType::InvaderCore => "invaderCore".into_abi(),
        }
    }
}

impl wasm_bindgen::convert::FromWasmAbi for StructureType {
    type Abi = <String as FromWasmAbi>::Abi;

    #[inline]
    unsafe fn from_abi(js: <String as FromWasmAbi>::Abi) -> Self {
        match String::from_abi(js).as_str() {
            "spawn" => StructureType::Spawn,
            "extension" => StructureType::Extension,
            "road" => StructureType::Road,
            "constructedWall" => StructureType::Wall,
            "rampart" => StructureType::Rampart,
            "keeperLair" => StructureType::KeeperLair,
            "portal" => StructureType::Portal,
            "controller" => StructureType::Controller,
            "link" => StructureType::Link,
            "storage" => StructureType::Storage,
            "tower" => StructureType::Tower,
            "observer" => StructureType::Observer,
            "powerBank" => StructureType::PowerBank,
            "powerSpawn" => StructureType::PowerSpawn,
            "extractor" => StructureType::Extractor,
            "lab" => StructureType::Lab,
            "terminal" => StructureType::Terminal,
            "container" => StructureType::Container,
            "nuker" => StructureType::Nuker,
            "factory" => StructureType::Factory,
            "invaderCore" => StructureType::InvaderCore,
            _ => panic!("Unexpected ABI value in StructureType FFI"),
        }
    }
}

impl wasm_bindgen::describe::WasmDescribe for StructureType {
    #[inline]
    fn describe() {
        String::describe()
    }
}
