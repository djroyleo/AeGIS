//! Vector layer painting with egui's `Painter`.

use aegis_project::VectorSymbology;
use aegis_render::{MapCamera, ScreenRect};
use aegis_vector::VectorDataset;
use eframe::egui;
use geo_types::{Coord, Geometry, LineString, Polygon, Rect, coord};

use super::{rects_overlap, to_color32, to_pos2, to_stroke};

pub fn paint_layer(
    painter: &egui::Painter,
    camera: &MapCamera,
    viewport: &ScreenRect,
    dataset: &VectorDataset,
    symbology: &VectorSymbology,
) {
    // Everything outside this world-space rect is invisible and gets
    // skipped. Expanded slightly so pixel-sized point markers straddling
    // the edge don't pop in and out.
    let margin = f64::from(symbology.point_radius) / camera.pixels_per_unit;
    let visible = camera.visible_world_rect(viewport);
    let cull_rect = Rect::new(
        coord! { x: visible.min().x - margin, y: visible.min().y - margin },
        coord! { x: visible.max().x + margin, y: visible.max().y + margin },
    );

    for feature in &dataset.features {
        // Empty geometries have no bounds and nothing to draw.
        let Some(bounds) = feature.bounds() else {
            continue;
        };
        if !rects_overlap(&cull_rect, &bounds) {
            continue; // culled: not on screen
        }
        paint_geometry(painter, camera, viewport, &feature.geometry, symbology);
    }
}

fn paint_geometry(
    painter: &egui::Painter,
    camera: &MapCamera,
    viewport: &ScreenRect,
    geometry: &Geometry<f64>,
    symbology: &VectorSymbology,
) {
    match geometry {
        Geometry::Point(p) => paint_marker(painter, camera, viewport, p.0, symbology),
        Geometry::MultiPoint(points) => {
            for p in points {
                paint_marker(painter, camera, viewport, p.0, symbology);
            }
        }
        Geometry::Line(line) => {
            let screen = vec![
                to_pos2(camera.world_to_screen(line.start, viewport)),
                to_pos2(camera.world_to_screen(line.end, viewport)),
            ];
            painter.add(egui::Shape::line(screen, to_stroke(symbology.stroke)));
        }
        Geometry::LineString(line) => {
            paint_line_string(painter, camera, viewport, line, symbology);
        }
        Geometry::MultiLineString(lines) => {
            for line in lines {
                paint_line_string(painter, camera, viewport, line, symbology);
            }
        }
        Geometry::Polygon(polygon) => {
            paint_polygon(painter, camera, viewport, polygon, symbology);
        }
        Geometry::MultiPolygon(polygons) => {
            for polygon in polygons {
                paint_polygon(painter, camera, viewport, polygon, symbology);
            }
        }
        Geometry::Rect(rect) => {
            paint_polygon(painter, camera, viewport, &rect.to_polygon(), symbology);
        }
        Geometry::Triangle(triangle) => {
            paint_polygon(painter, camera, viewport, &triangle.to_polygon(), symbology);
        }
        Geometry::GeometryCollection(collection) => {
            for g in collection {
                paint_geometry(painter, camera, viewport, g, symbology);
            }
        }
    }
}

fn paint_marker(
    painter: &egui::Painter,
    camera: &MapCamera,
    viewport: &ScreenRect,
    center: Coord<f64>,
    symbology: &VectorSymbology,
) {
    let screen = to_pos2(camera.world_to_screen(center, viewport));
    painter.circle_filled(screen, symbology.point_radius, to_color32(symbology.fill.color));
    painter.circle_stroke(screen, symbology.point_radius, to_stroke(symbology.stroke));
}

fn paint_line_string(
    painter: &egui::Painter,
    camera: &MapCamera,
    viewport: &ScreenRect,
    line: &LineString<f64>,
    symbology: &VectorSymbology,
) {
    let screen: Vec<egui::Pos2> = line
        .coords()
        .map(|c| to_pos2(camera.world_to_screen(*c, viewport)))
        .collect();
    painter.add(egui::Shape::line(screen, to_stroke(symbology.stroke)));
}

fn paint_polygon(
    painter: &egui::Painter,
    camera: &MapCamera,
    viewport: &ScreenRect,
    polygon: &Polygon<f64>,
    symbology: &VectorSymbology,
) {
    // egui can only fill *convex* polygons natively, and real GIS polygons
    // rarely are. For now: closed outlines (exterior ring + holes). Later:
    // triangulate (e.g. `earcutr`) and emit a filled Mesh using the fill
    // style — or let the wgpu pipeline take over entirely.
    paint_ring(painter, camera, viewport, polygon.exterior(), symbology);
    for hole in polygon.interiors() {
        paint_ring(painter, camera, viewport, hole, symbology);
    }
}

fn paint_ring(
    painter: &egui::Painter,
    camera: &MapCamera,
    viewport: &ScreenRect,
    ring: &LineString<f64>,
    symbology: &VectorSymbology,
) {
    let screen: Vec<egui::Pos2> = ring
        .coords()
        .map(|c| to_pos2(camera.world_to_screen(*c, viewport)))
        .collect();
    painter.add(egui::Shape::closed_line(screen, to_stroke(symbology.stroke)));
}
