use geojson::{Geometry, Value};
use std::f64::consts::PI;
use std::ffi::OsStr;
use std::path::Path;

use crate::mvt::MvtBuilder;
use crate::{EARTH_CIRCUMFERENCE, EARTH_CIRCUMFERENCE_DEGREES, EARTH_RADIUS, wsg84_to_webmercator};
use geozero::ToMvt;
use geozero::mvt::tile::Layer;

const MAX_EXTENT: u32 = 4096;
pub enum GeoJsonCli {}

type GeoJsonSourceResult<T> = Result<T, GeoJsonSourceError>;
pub enum GeoJsonSourceError {
    UnsupportedCharsInFilepath,
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
        &self.buffer_size
    }

    pub async fn to_mvt_source(&self, geometry: Geometry) -> GeoJsonSourceResult<MvtBuilder> {
        let mut mvt = MvtBuilder::new();
        match geometry.value.type_name() {
            "Point" => {
                if let Value::Point(coords) = geometry.value {
                    let mvt_base = geometry.value.to_mvt(MAX_EXTENT, coords[0], coords[1]);
                }
            }
        }
        Ok(mvt)
    }
}

//Only Point now
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

pub fn fix_geom(geom: &Geometry<f64>) -> Geometry<f64> {}
