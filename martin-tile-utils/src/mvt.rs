use geozero::mvt;
use mvt::tile::Feature;

pub type MvtTileData = Vec<u8>;

pub struct MvtLayer {
    pub name: String,
    pub features: Vec<Feature>,
}

pub struct MvtBuilder {
    tile: mvt::Tile,
}

impl MvtBuilder {
    pub fn new() -> Self {
        Self {
            tile: mvt::Tile::default(),
        }
    }
}

pub fn encode_geojson(geojson: &str, z: u32, x: u32, y: u32, layer_name: &str) -> MvtTileData {
    unimplemented!()
}

pub fn MoveTo(x: i32, y: i32) -> Vec<u8> {
    unimplemented!()
}
