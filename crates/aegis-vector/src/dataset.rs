//! A vector dataset: a homogeneous collection of features with a schema.

use aegis_core::Crs;
use geo_types::Rect;

use crate::feature::Feature;

/// The declared type of an attribute field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FieldKind {
    Bool,
    Int,
    Float,
    Text,
}

/// One column of the attribute table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldDef {
    pub name: String,
    pub kind: FieldKind,
}

/// The attribute schema of a dataset (the columns of its attribute table).
pub type Schema = Vec<FieldDef>;

/// A vector dataset, independent of how it is displayed.
///
/// Everything presentational (name shown in the Contents panel, visibility,
/// symbology) lives on `aegis-project`'s `Layer`, which *references* a
/// dataset. IO drivers read and write datasets; geoprocessing tools consume
/// and produce them.
#[derive(Debug, Clone, Default)]
pub struct VectorDataset {
    pub features: Vec<Feature>,
    pub schema: Schema,
    pub crs: Crs,
}

impl VectorDataset {
    /// A dataset from bare features, with an empty schema.
    #[must_use]
    pub const fn from_features(features: Vec<Feature>) -> Self {
        Self {
            features,
            schema: Schema::new(),
            crs: Crs::Unknown,
        }
    }

    /// Bounding box of all features, in world coordinates.
    ///
    /// `None` if the dataset is empty (or contains only empty geometries).
    #[must_use]
    pub fn bounds(&self) -> Option<Rect<f64>> {
        let mut rects = self.features.iter().filter_map(Feature::bounds);
        let first = rects.next()?;
        Some(rects.fold(first, |acc, r| {
            Rect::new(
                geo_types::coord! {
                    x: acc.min().x.min(r.min().x),
                    y: acc.min().y.min(r.min().y),
                },
                geo_types::coord! {
                    x: acc.max().x.max(r.max().x),
                    y: acc.max().y.max(r.max().y),
                },
            )
        }))
    }
}
