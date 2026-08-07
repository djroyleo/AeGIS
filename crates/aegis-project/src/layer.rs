//! A layer: one dataset plus everything presentational about it.

use aegis_core::LayerId;
use aegis_raster::RasterDataset;
use aegis_vector::VectorDataset;
use geo_types::Rect;

use crate::symbology::{RasterSymbology, Symbology, VectorSymbology};

/// The data behind a layer.
///
/// This is the seam new data types plug into: adding e.g. point clouds or
/// TINs means a new variant here plus a renderer arm — nothing else.
#[derive(Debug, Clone)]
pub enum LayerSource {
    Vector(VectorDataset),
    Raster(RasterDataset),
}

impl LayerSource {
    /// Bounding box of the underlying data, in world coordinates.
    #[must_use]
    pub fn bounds(&self) -> Option<Rect<f64>> {
        match self {
            Self::Vector(v) => v.bounds(),
            Self::Raster(r) => Some(r.bounds()),
        }
    }
}

/// One entry in the Contents panel: a dataset plus how it is displayed.
#[derive(Debug, Clone)]
pub struct Layer {
    pub id: LayerId,
    pub name: String,
    pub visible: bool,
    pub source: LayerSource,
    pub symbology: Symbology,
}

impl Layer {
    /// A visible vector layer with the given symbology.
    #[must_use]
    pub fn vector(
        name: impl Into<String>,
        dataset: VectorDataset,
        symbology: VectorSymbology,
    ) -> Self {
        Self {
            id: LayerId::next(),
            name: name.into(),
            visible: true,
            source: LayerSource::Vector(dataset),
            symbology: Symbology::Vector(symbology),
        }
    }

    /// A visible raster layer with the given symbology.
    #[must_use]
    pub fn raster(
        name: impl Into<String>,
        dataset: RasterDataset,
        symbology: RasterSymbology,
    ) -> Self {
        Self {
            id: LayerId::next(),
            name: name.into(),
            visible: true,
            source: LayerSource::Raster(dataset),
            symbology: Symbology::Raster(symbology),
        }
    }
}
