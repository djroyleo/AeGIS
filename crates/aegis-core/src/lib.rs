//! Shared primitives for the `AeGIS` ecosystem.
//!
//! This crate sits at the bottom of the dependency graph: every other
//! `aegis-*` crate may depend on it, and it depends on nothing but std.
//! It must never know about any GUI toolkit, geometry kind, or file format.

pub mod crs;
pub mod error;
pub mod id;
pub mod style;

pub use crs::Crs;
pub use error::AegisError;
pub use id::LayerId;
pub use style::{Color, FillStyle, LineStyle};
