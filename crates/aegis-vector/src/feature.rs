//! A feature: one geometry plus its attribute record.

use std::collections::HashMap;

use geo_types::Rect;

use crate::geometry::{Geometry, bounds};

/// A single attribute value, aligned with the field types found in common
/// GIS formats (shapefile/DBF, `GeoPackage`, `GeoJSON` properties, ...).
#[derive(Debug, Clone, PartialEq)]
pub enum AttributeValue {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    Text(String),
}

/// One record of a vector dataset: a geometry and its attributes.
#[derive(Debug, Clone, PartialEq)]
pub struct Feature {
    pub geometry: Geometry,
    /// Attribute values keyed by field name (see [`crate::Schema`]).
    pub attributes: HashMap<String, AttributeValue>,
}

impl Feature {
    /// A feature with the given geometry and no attributes.
    #[must_use]
    pub fn new(geometry: Geometry) -> Self {
        Self {
            geometry,
            attributes: HashMap::new(),
        }
    }

    /// Axis-aligned bounding box in world coordinates, for culling.
    ///
    /// `None` if the geometry is empty.
    #[must_use]
    pub fn bounds(&self) -> Option<Rect<f64>> {
        bounds(&self.geometry)
    }
}
