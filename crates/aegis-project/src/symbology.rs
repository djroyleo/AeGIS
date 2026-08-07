//! How a layer is drawn, expressed with `aegis-core`'s renderer-agnostic
//! style types. The render layer translates these into backend-specific
//! resources (egui shapes today, GPU buffers/textures later).

use aegis_core::{Color, FillStyle, LineStyle};

/// Single-symbol styling for a vector layer.
///
/// Future work: categorized/graduated renderers, per-geometry-kind symbols,
/// labeling — all of which extend this type (or an enum around it) without
/// touching the data crates or the panels.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VectorSymbology {
    /// Outline for polygons, stroke for lines, ring for point markers.
    pub stroke: LineStyle,
    /// Interior for polygons and point markers.
    pub fill: FillStyle,
    /// Point marker radius in screen pixels (zoom-independent).
    pub point_radius: f32,
}

impl Default for VectorSymbology {
    fn default() -> Self {
        Self {
            stroke: LineStyle::new(Color::BLACK, 1.0),
            fill: FillStyle::new(Color::rgba(128, 128, 128, 255)),
            point_radius: 5.0,
        }
    }
}

/// Styling for a raster layer (skeleton).
///
/// Future work: band → RGB mapping, contrast stretch, colormaps, hillshade.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RasterSymbology {
    /// 0.0 (invisible) ..= 1.0 (opaque).
    pub opacity: f32,
}

impl Default for RasterSymbology {
    fn default() -> Self {
        Self { opacity: 1.0 }
    }
}

/// The symbology of a layer, matching the kind of its data source.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Symbology {
    Vector(VectorSymbology),
    Raster(RasterSymbology),
}
