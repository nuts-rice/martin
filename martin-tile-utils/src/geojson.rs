use geojson::{GeoJson, Geometry, Value};
use std::f64::consts::PI;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use crate::mvt::MvtBuilder;
use crate::{EARTH_CIRCUMFERENCE, EARTH_CIRCUMFERENCE_DEGREES, EARTH_RADIUS};
use geojson_vt_rs::{GeoJSONVT, Options, TileOptions, geojson_to_tile};
use geozero::mvt::tile::Layer;
use geozero::{ToJson, ToMvt, mvt::Tile};

const MAX_EXTENT: u16 = 4096;
pub enum GeoJsonCli {}

type GeoJsonSourceResult<T> = Result<T, GeoJsonSourceError>;

#[derive(Debug)]
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

    async fn test_roundtrip() {
        let expected_geo: geo_types::Geometry<f64> =
            geo_types::Point::new(960000.0, 6002729.0).into();
        let expected_mvt = expected_geo
            .to_mvt(256, 958826.08, 5987771.04, 978393.96, 6007338.92)
            .unwrap();
        let expected_geojson = expected_mvt.to_json().unwrap();
        let geojson_str = r#"{
        "type": "Point", "coordinates": [15, 61]}"#;
        let geojson: GeoJson = geojson_str.parse().unwrap();
        let gjt = GeoJsonTilesSource::new("test.geojson", 256).unwrap();
        let actual_mvt = gjt.to_mvt_source(geojson).await.unwrap();
    }
}
