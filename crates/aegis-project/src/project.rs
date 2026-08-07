//! The project: the ordered stack of layers shown on the map.

use aegis_core::LayerId;
use geo_types::Rect;

use crate::layer::Layer;

/// The open map document.
///
/// Future work that belongs here (not in the GUI): selection state, editing
/// sessions, undo history, project save/load, map CRS.
#[derive(Debug, Clone, Default)]
pub struct Project {
    /// Draw order = list order: index 0 is drawn first (bottom of the stack).
    pub layers: Vec<Layer>,
}

impl Project {
    #[must_use]
    pub const fn new(layers: Vec<Layer>) -> Self {
        Self { layers }
    }

    pub fn add_layer(&mut self, layer: Layer) {
        self.layers.push(layer);
    }

    /// Remove a layer by id; returns it if it was present.
    pub fn remove_layer(&mut self, id: LayerId) -> Option<Layer> {
        let index = self.layers.iter().position(|l| l.id == id)?;
        Some(self.layers.remove(index))
    }

    #[must_use]
    pub fn layer(&self, id: LayerId) -> Option<&Layer> {
        self.layers.iter().find(|l| l.id == id)
    }

    #[must_use]
    pub fn layer_mut(&mut self, id: LayerId) -> Option<&mut Layer> {
        self.layers.iter_mut().find(|l| l.id == id)
    }

    /// Bounding box of all layers' data (visible or not), in world
    /// coordinates. `None` for an empty project — the basis for a future
    /// "zoom to extent".
    #[must_use]
    pub fn bounds(&self) -> Option<Rect<f64>> {
        let mut rects = self.layers.iter().filter_map(|l| l.source.bounds());
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
