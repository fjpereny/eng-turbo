use eframe::egui::{self, Rect};
use egui::{Color32, Pos2, Shape, Stroke};
use std::f32::consts::TAU;

/// Draw a single pie slice using a convex polygon.
fn draw_pie_slice(
    painter: &egui::Painter,
    center: Pos2,
    radius: f32,
    start_angle: f32,
    sweep_angle: f32,
    color: Color32,
) {
    let segments = 32;
    let mut points = vec![center];

    for i in 0..=segments {
        let t = start_angle + sweep_angle * (i as f32 / segments as f32);
        points.push(Pos2 {
            x: center.x + radius * t.cos(),
            y: center.y + radius * t.sin(),
        });
    }

    painter.add(Shape::convex_polygon(points, color, Stroke::NONE));
}

/// Draw a full pie chart with labeled slices.
pub fn draw_pie_chart(ui: &mut egui::Ui, radius: f32, data: &[(&str, f32, Color32)], rect: Rect) {
    let total: f32 = data.iter().map(|(_, value, _)| *value).sum();
    let mut start_angle = 0.0;

    let painter = ui.painter_at(rect);
    let center = rect.center();

    for &(label, value, color) in data {
        let sweep_angle = (value / total) * TAU;

        draw_pie_slice(&painter, center, radius, start_angle, sweep_angle, color);

        // Optional: draw label
        let mid_angle = start_angle + sweep_angle / 2.0;
        let label_pos = Pos2 {
            x: center.x + radius * 0.6 * mid_angle.cos(),
            y: center.y + radius * 0.6 * mid_angle.sin(),
        };
        painter.text(label_pos, egui::Align2::CENTER_CENTER, label, egui::FontId::new(20.0, egui::FontFamily::Proportional), Color32::BLACK);

        start_angle += sweep_angle;
    }
}