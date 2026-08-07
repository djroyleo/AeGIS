//! Folder connections: the model behind the Catalog panel.
//!
//! Skeleton for now: the types establish how the Catalog panel will talk
//! to the filesystem without doing any IO in GUI code. Real directory
//! walking (and caching/watching) lands here.

use std::path::PathBuf;

/// A user-added root directory to browse for spatial data
/// (`ArcGIS`'s "folder connection" / QGIS's browser entry).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FolderConnection {
    pub path: PathBuf,
}

impl FolderConnection {
    #[must_use]
    pub const fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

/// One row in the catalog tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CatalogEntry {
    /// A subdirectory that can be expanded.
    Folder(PathBuf),
    /// A file some registered driver can open (`driver` is its name).
    Dataset { path: PathBuf, driver: String },
    /// A file no driver understands (shown greyed out, if at all).
    Unrecognized(PathBuf),
}
