use eframe::egui;

pub enum Geometry {
    Point(egui::Pos2),
    Polyline(Vec<egui::Pos2>),
    Polygon(Vec<egui::Pos2>),
}

pub struct Feature {
    pub geometry: Geometry,
    // later: attributes: HashMap<String, String>,
}

impl Feature {
    /// Axis-aligned bounding box in world coordinates, for culling.
    pub fn bounds(&self) -> egui::Rect {
        let points: &[egui::Pos2] = match &self.geometry {
            Geometry::Point(p) => std::slice::from_ref(p),
            Geometry::Polyline(pts) | Geometry::Polygon(pts) => pts,
        };
        let mut rect = egui::Rect::NOTHING; // grows to fit whatever we feed it
        for p in points {
            rect.extend_with(*p);
        }
        rect
    }
}

pub struct Layer {
    pub name: String,
    pub visible: bool,
    pub features: Vec<Feature>,
    pub stroke: egui::Stroke,
    pub fill: egui::Color32,
}

