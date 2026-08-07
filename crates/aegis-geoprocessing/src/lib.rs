//! The geoprocessing framework of `AeGIS`.
//!
//! Modeled on the QGIS Processing / `ArcPy` pattern: every tool implements
//! [`GeoprocessingTool`] and *describes its parameters as data*
//! ([`ParamSpec`]). The GUI auto-generates tool dialogs from those specs,
//! so shipping a new tool means implementing one trait and registering it
//! in the [`ToolRegistry`] — zero GUI edits per tool.
//!
//! Algorithms themselves should lean on the `geo` crate (buffer, boolean
//! ops, area, simplification, ...) rather than reimplementing computational
//! geometry.
//!
//! Future work that belongs here: a background job runner so long-running
//! tools report progress and never block the UI thread.

use std::collections::HashMap;
use std::path::PathBuf;

use aegis_core::AegisError;
use aegis_raster::RasterDataset;
use aegis_vector::VectorDataset;

/// What kind of value a tool parameter accepts — enough information for
/// the GUI to build the right input widget.
#[derive(Debug, Clone, PartialEq)]
pub enum ParamKind {
    VectorInput,
    RasterInput,
    Number { min: Option<f64>, max: Option<f64> },
    Text,
    Bool,
    /// An output or input location on disk.
    Path,
}

/// Declarative description of one tool parameter.
#[derive(Debug, Clone, PartialEq)]
pub struct ParamSpec {
    /// Stable key used in [`ToolArgs`], e.g. `"distance"`.
    pub name: &'static str,
    /// Human-readable label for the auto-generated dialog.
    pub label: &'static str,
    pub kind: ParamKind,
    pub required: bool,
}

/// A concrete argument value, matching a [`ParamKind`].
#[derive(Debug, Clone)]
pub enum ParamValue {
    Vector(VectorDataset),
    Raster(RasterDataset),
    Number(f64),
    Text(String),
    Bool(bool),
    Path(PathBuf),
}

/// The arguments a tool is invoked with, keyed by [`ParamSpec::name`].
pub type ToolArgs = HashMap<&'static str, ParamValue>;

/// What a tool produced; each output typically becomes a new layer.
#[derive(Debug, Clone, Default)]
pub struct ToolOutput {
    pub datasets: Vec<ParamValue>,
}

/// A geoprocessing tool (buffer, clip, dissolve, raster calculator, ...).
pub trait GeoprocessingTool: Send + Sync {
    /// Stable unique id, e.g. `"vector:buffer"`.
    fn id(&self) -> &'static str;

    /// Human-readable name for menus and the tool gallery.
    fn label(&self) -> &'static str;

    /// The parameters this tool takes; the GUI builds its dialog from this.
    fn params(&self) -> Vec<ParamSpec>;

    /// Execute the tool.
    ///
    /// # Errors
    /// [`AegisError::InvalidParameter`] if `args` is missing a required
    /// parameter or a value has the wrong kind; tool-specific errors
    /// otherwise.
    fn run(&self, args: &ToolArgs) -> Result<ToolOutput, AegisError>;
}

/// The set of available tools (what a future toolbox panel enumerates).
#[derive(Default)]
pub struct ToolRegistry {
    tools: Vec<Box<dyn GeoprocessingTool>>,
}

impl ToolRegistry {
    #[must_use]
    pub const fn new() -> Self {
        Self { tools: Vec::new() }
    }

    pub fn register(&mut self, tool: Box<dyn GeoprocessingTool>) {
        self.tools.push(tool);
    }

    #[must_use]
    pub fn get(&self, id: &str) -> Option<&dyn GeoprocessingTool> {
        self.tools.iter().map(AsRef::as_ref).find(|t| t.id() == id)
    }

    /// All registered tools, in registration order.
    pub fn iter(&self) -> impl Iterator<Item = &dyn GeoprocessingTool> {
        self.tools.iter().map(AsRef::as_ref)
    }
}
