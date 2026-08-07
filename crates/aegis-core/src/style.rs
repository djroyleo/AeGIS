//! Renderer-agnostic styling primitives.
//!
//! Symbology throughout `AeGIS` is expressed with these plain-data types; the
//! render layer converts them into whatever the active backend needs
//! (`egui::Color32`/`egui::Stroke` today, GPU vertex attributes later).

/// An sRGB color with straight (non-premultiplied) alpha.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const TRANSPARENT: Self = Self::rgba(0, 0, 0, 0);
    pub const BLACK: Self = Self::rgb(0, 0, 0);
    pub const WHITE: Self = Self::rgb(255, 255, 255);

    #[must_use]
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    #[must_use]
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

/// How to draw the outline of a geometry (or a line geometry itself).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LineStyle {
    pub color: Color,
    /// Width in screen pixels — strokes keep their weight at any zoom.
    pub width: f32,
}

impl LineStyle {
    #[must_use]
    pub const fn new(color: Color, width: f32) -> Self {
        Self { color, width }
    }
}

/// How to fill the interior of a polygon (or a point marker).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FillStyle {
    pub color: Color,
}

impl FillStyle {
    #[must_use]
    pub const fn new(color: Color) -> Self {
        Self { color }
    }
}
