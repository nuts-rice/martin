use geozero::mvt;
use std::collections::HashMap;

pub type MvtTileData = Vec<u8>;

pub struct MvtBuilder {
    tiles: HashMap<u64, mvt::Tile>,
}

impl MvtBuilder {
    pub fn new() -> Self {
        Self {
            tiles: HashMap::new(),
        }
    }
}

pub fn encode_geojson(geojson: &str, z: u32, x: u32, y: u32, layer_name: &str) -> MvtTileData {
    unimplemented!()
}

pub fn MoveTo(x: i32, y: i32) -> Vec<u8> {
    unimplemented!()
}
