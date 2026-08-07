//! Format drivers: how `AeGIS` reads and writes spatial file formats.

use std::path::Path;

use aegis_core::AegisError;
use aegis_raster::RasterDataset;
use aegis_vector::VectorDataset;

/// A dataset as it exists on disk: pure data, no presentation.
#[derive(Debug, Clone)]
pub enum Dataset {
    Vector(VectorDataset),
    Raster(RasterDataset),
}

/// A reader/writer for one spatial file format (the GDAL/OGR model).
///
/// Implementations are registered in a [`DriverRegistry`]; nothing outside
/// the registry needs to know which formats exist.
pub trait DatasetDriver: Send + Sync {
    /// Short unique name, e.g. `"GeoJSON"`.
    fn name(&self) -> &'static str;

    /// Whether this driver believes it can open `path` (typically by
    /// extension; drivers may also sniff file headers).
    fn can_open(&self, path: &Path) -> bool;

    /// Read the dataset at `path`.
    ///
    /// # Errors
    /// [`AegisError::Io`] on filesystem failures, or
    /// [`AegisError::InvalidData`] if the content cannot be parsed.
    fn read(&self, path: &Path) -> Result<Dataset, AegisError>;

    /// Write `dataset` to `path`, overwriting.
    ///
    /// # Errors
    /// [`AegisError::Io`] on filesystem failures, or
    /// [`AegisError::InvalidData`] if the dataset cannot be represented in
    /// this format (e.g. a raster given to a vector-only driver).
    fn write(&self, path: &Path, dataset: &Dataset) -> Result<(), AegisError>;
}

/// The set of available format drivers.
#[derive(Default)]
pub struct DriverRegistry {
    drivers: Vec<Box<dyn DatasetDriver>>,
}

impl DriverRegistry {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            drivers: Vec::new(),
        }
    }

    pub fn register(&mut self, driver: Box<dyn DatasetDriver>) {
        self.drivers.push(driver);
    }

    /// The first registered driver that can open `path`, if any.
    #[must_use]
    pub fn driver_for(&self, path: &Path) -> Option<&dyn DatasetDriver> {
        self.drivers
            .iter()
            .map(AsRef::as_ref)
            .find(|d| d.can_open(path))
    }

    /// Read `path` with the first driver that claims it.
    ///
    /// # Errors
    /// [`AegisError::UnsupportedFormat`] if no driver can open `path`;
    /// otherwise whatever the driver's `read` returns.
    pub fn read(&self, path: &Path) -> Result<Dataset, AegisError> {
        self.driver_for(path)
            .ok_or_else(|| AegisError::UnsupportedFormat {
                path: path.to_path_buf(),
            })?
            .read(path)
    }
}
