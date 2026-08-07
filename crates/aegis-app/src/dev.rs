//! Development-only sample data (successor of `test_layers.rs`).
//!
//! One layer per geometry kind, so there is something to look at. Only
//! reached in debug builds (see `AegisApp::default`).

use aegis_core::{Color, FillStyle, LineStyle};
use aegis_project::{Layer, VectorSymbology};
use aegis_vector::{Feature, VectorDataset};
use geo_types::{Geometry, line_string, point, polygon};

pub fn sample_layers() -> Vec<Layer> {
    let lake = Layer::vector(
        "Lake",
        VectorDataset::from_features(vec![Feature::new(Geometry::Polygon(
            // Deliberately non-convex — which is why we outline it for now
            // instead of using egui's convex-only polygon fill.
            polygon![
                (x: -60.0, y: -20.0),
                (x: -20.0, y: -35.0),
                (x: 15.0, y: -25.0),
                (x: 30.0, y: -50.0),
                (x: 55.0, y: -40.0),
                (x: 45.0, y: -5.0),
                (x: 10.0, y: 5.0),
                (x: -15.0, y: -10.0),
                (x: -45.0, y: 5.0),
            ],
        ))]),
        VectorSymbology {
            stroke: LineStyle::new(Color::rgb(30, 100, 200), 2.0),
            // Unused until we triangulate polygon fills.
            fill: FillStyle::new(Color::rgb(180, 210, 240)),
            point_radius: 5.0,
        },
    );

    let river = Layer::vector(
        "River",
        VectorDataset::from_features(vec![Feature::new(Geometry::LineString(line_string![
            (x: -90.0, y: 80.0),
            (x: -70.0, y: 55.0),
            (x: -55.0, y: 60.0),
            (x: -40.0, y: 35.0),
            (x: -30.0, y: 10.0),
            (x: -45.0, y: 5.0), // flows into the lake
        ]))]),
        VectorSymbology {
            stroke: LineStyle::new(Color::rgb(60, 130, 220), 2.5),
            fill: FillStyle::new(Color::TRANSPARENT),
            point_radius: 5.0,
        },
    );

    let stations = Layer::vector(
        "Stations",
        VectorDataset::from_features(vec![
            Feature::new(Geometry::Point(point! { x: -70.0, y: 55.0 })),
            Feature::new(Geometry::Point(point! { x: 0.0, y: 40.0 })),
            Feature::new(Geometry::Point(point! { x: 60.0, y: -60.0 })),
            Feature::new(Geometry::Point(point! { x: 75.0, y: 30.0 })),
        ]),
        VectorSymbology {
            stroke: LineStyle::new(Color::rgb(120, 20, 20), 1.5),
            fill: FillStyle::new(Color::rgb(200, 40, 40)),
            point_radius: 5.0,
        },
    );

    // Bottom-to-top: polygon under line under points.
    vec![lake, river, stations]
}
