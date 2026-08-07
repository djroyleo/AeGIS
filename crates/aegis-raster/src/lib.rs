//! The raster (gridded) data model of `AeGIS`.
//!
//! This crate is deliberately a *skeleton*: the types below pin down how
//! raster data flows through the architecture (IO drivers produce a
//! [`RasterDataset`], the render layer turns bands into textures,
//! geoprocessing tools consume/produce datasets) so that building out real
//! raster support never requires touching the GUI crates.
//!
//! Future work that belongs here: tiling/overviews (pyramids), block-wise
//! IO for rasters larger than memory, resampling, and band math.

use aegis_core::Crs;
use geo_types::{Coord, Rect, coord};

/// Maps pixel space (col, row) to world coordinates.
///
/// This is the standard north-up affine geotransform without rotation
/// terms: world x grows with columns, world y *shrinks* with rows (row 0 is
/// the top of the image), so `pixel_height` is typically negative.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeoTransform {
    /// World coordinates of the *outer* corner of pixel (0, 0).
    pub origin: Coord<f64>,
    /// World-units spanned by one pixel horizontally.
    pub pixel_width: f64,
    /// World-units spanned by one pixel vertically (negative for north-up).
    pub pixel_height: f64,
}

impl GeoTransform {
    /// World coordinates of the outer corner of pixel (`col`, `row`).
    #[must_use]
    pub const fn pixel_to_world(&self, col: f64, row: f64) -> Coord<f64> {
        coord! {
            x: col.mul_add(self.pixel_width, self.origin.x),
            y: row.mul_add(self.pixel_height, self.origin.y),
        }
    }
}

/// The pixel values of one band, in row-major order.
///
/// Variants mirror the sample types common in `GeoTIFF` & friends; more are
/// added as the IO layer grows real drivers.
#[derive(Debug, Clone, PartialEq)]
pub enum BandData {
    U8(Vec<u8>),
    I16(Vec<i16>),
    U16(Vec<u16>),
    I32(Vec<i32>),
    F32(Vec<f32>),
    F64(Vec<f64>),
}

/// One band of a raster: pixel values plus band-level metadata.
#[derive(Debug, Clone, PartialEq)]
pub struct Band {
    pub data: BandData,
    /// Pixel value that means "no data here", if the source declares one.
    pub nodata: Option<f64>,
}

/// A raster dataset, independent of how it is displayed.
///
/// The presentational side (which bands map to RGB, stretch, colormap,
/// opacity) lives in `aegis-project`'s raster symbology, exactly as vector
/// symbology is kept out of `aegis-vector`.
#[derive(Debug, Clone, PartialEq)]
pub struct RasterDataset {
    /// Width in pixels (number of columns).
    pub width: usize,
    /// Height in pixels (number of rows).
    pub height: usize,
    pub transform: GeoTransform,
    pub bands: Vec<Band>,
    pub crs: Crs,
}

impl RasterDataset {
    /// Bounding box of the full grid, in world coordinates.
    #[must_use]
    #[allow(
        clippy::as_conversions,
        clippy::cast_precision_loss,
        reason = "pixel counts are far below f64's 2^53 integer range"
    )]
    pub fn bounds(&self) -> Rect<f64> {
        let corner = self
            .transform
            .pixel_to_world(self.width as f64, self.height as f64);
        Rect::new(self.transform.origin, corner)
    }
}
