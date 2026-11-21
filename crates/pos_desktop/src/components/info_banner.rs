//! Aviso informativo de una línea.

use egui::{CornerRadius, Frame, Margin, RichText, Sense, Stroke, Ui, vec2};

use super::paint;
use crate::theme::{
    icons,
    tokens::{color, margin, radius, space},
};

/// Diámetro del círculo del icono.
const ICON_DIAMETER: f32 = 28.0;

/// Franja con un icono de información y un mensaje breve.
pub fn info_banner(ui: &mut Ui, message: &str) {
    Frame::new()
        .fill(color::SURFACE_RAISED)
        .stroke(Stroke::new(1.0, color::BORDER_SUBTLE))
        .corner_radius(CornerRadius::same(radius::CARD))
        .inner_margin(Margin::symmetric(margin::L, margin::M))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = space::M;
                let (rect, _) =
                    ui.allocate_exact_size(vec2(ICON_DIAMETER, ICON_DIAMETER), Sense::hover());
                paint::icon_circle(
                    ui.painter(),
                    rect.center(),
                    ICON_DIAMETER,
                    color::OLIVE_GRAY,
                    icons::INFO,
                    color::TEXT_ON_DARK,
                );
                ui.label(RichText::new(message).color(color::TEXT_MUTED));
            });
        });
}
