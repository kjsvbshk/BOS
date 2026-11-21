//! Dibujo compartido por los widgets propios.

use egui::{
    Align2, Color32, CornerRadius, FontFamily, FontId, Painter, Pos2, Rect, Stroke, StrokeKind,
};

/// Separación entre un widget y su anillo de foco.
const FOCUS_RING_GAP: f32 = 3.0;

/// Anillo exterior de 2 px que marca el foco de teclado. Se usa en los widgets
/// cuyo fondo no cambia lo suficiente al recibir el foco.
pub fn focus_ring(painter: &Painter, rect: Rect, corner: CornerRadius, color: Color32) {
    painter.rect_stroke(
        rect.expand(FOCUS_RING_GAP),
        corner,
        Stroke::new(2.0, color),
        StrokeKind::Outside,
    );
}

/// Círculo relleno con un icono centrado.
pub fn icon_circle(
    painter: &Painter,
    center: Pos2,
    diameter: f32,
    fill: Color32,
    icon: &str,
    icon_color: Color32,
) {
    painter.circle_filled(center, diameter / 2.0, fill);
    painter.text(
        center,
        Align2::CENTER_CENTER,
        icon,
        FontId::new(diameter * 0.5, FontFamily::Proportional),
        icon_color,
    );
}
