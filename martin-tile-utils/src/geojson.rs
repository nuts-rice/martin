use geojson::{GeoJson, Geometry, Value};
use std::f64::consts::PI;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use crate::mvt::MvtBuilder;
use crate::{EARTH_CIRCUMFERENCE, EARTH_CIRCUMFERENCE_DEGREES, EARTH_RADIUS};
use geojson_vt_rs::{GeoJSONVT, Options, TileOptions, geojson_to_tile};
use geozero::mvt::tile::Layer;
use geozero::{ToMvt, mvt::Tile};

const MAX_EXTENT: u16 = 4096;
pub enum GeoJsonCli {}

type GeoJsonSourceResult<T> = Result<T, GeoJsonSourceError>;
pub enum GeoJsonSourceError {
    UnsupportedCharsInFilepath(PathBuf),
}

pub struct GeoJsonTilesSource {
    buffer_size: usize,
    filepath: String,
    filename: String,
}

impl GeoJsonTilesSource {
    pub fn new<P: AsRef<Path>>(filepath: P, buffer_size: usize) -> GeoJsonSourceResult<Self> {
        let path = filepath.as_ref();
        Ok(Self {
            filepath: path
                .to_str()
                .ok_or_else(|| GeoJsonSourceError::UnsupportedCharsInFilepath(path.to_path_buf()))?
                .to_string(),
            filename: path
                .file_stem()
                .unwrap_or_else(|| OsStr::new("unknown"))
                .to_string_lossy()
                .to_string(),
            buffer_size,
        })
    }

    pub async fn open(&self) -> GeoJsonSourceResult<()> {
        Ok(())
    }

    #[must_use]
    pub fn filepath(&self) -> &str {
        &self.filepath
    }

    #[must_use]
    pub fn filename(&self) -> &str {
        &self.filename
    }

    #[must_use]
    pub fn buffer_size(&self) -> usize {
        self.buffer_size
    }

    pub async fn to_mvt_source(&self, geometry: GeoJson) -> GeoJsonSourceResult<GeoJSONVT> {
        let mut mvt = MvtBuilder::new();
        let options = Options {
            max_zoom: 18,
            index_max_zoom: 5,
            index_max_points: 100000,
            generate_id: false,
            tile: TileOptions {
                tolerance: 3.,
                extent: MAX_EXTENT,
                buffer: 64,
                line_metrics: false,
            },
        };
        let mvt_base = GeoJSONVT::from_geojson(&geometry, &options);

        Ok(mvt_base)
    }

    fn fix_tile(&self, tile: &Tile) -> Tile {
        unimplemented!()
    }
}

//Only Point now
/*
pub fn geom_to_webmercator(geom: &Value, extent: u32) -> (f64, f64) {
    let x = match geom {
        Value::Point(coords) => coords[0],
        _ => return (0.0, 0.0),
    };
    let y = match geom {
        Value::Point(coords) => coords[1],
        _ => return (0.0, 0.0),
    };
    wsg84_to_webmercator(x, y);
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    fn test_roundtrip() {
        let geojson_str = r#"{
        "type": "MultiPolygon", "coordinates": [[[[40,40],[20,45],[45,30],[40,40]]],[[[35,10],[45,45],[15,40],[10,20],[35,10]],[[20,30],[35,35],[30,20],[20,30]]]]}"#;
        let geojson: GeoJson = geojson_str.parse().unwrap();
    }
}
