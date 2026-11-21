//! Barra con las teclas válidas en cada momento («Enter Continuar · F1 Buscar»).

use egui::{CornerRadius, Frame, Margin, RichText, Stroke, Ui};

use crate::theme::tokens::{color, font_size, radius, space};

/// Dibuja las teclas disponibles. Cada par es (tecla, acción).
pub fn key_hints(ui: &mut Ui, hints: &[(&str, &str)]) {
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = space::S;
        for (key, action) in hints {
            Frame::new()
                .fill(color::SURFACE_RAISED)
                .stroke(Stroke::new(1.0, color::BORDER_STRONG))
                .corner_radius(CornerRadius::same(radius::CONTROL))
                .inner_margin(Margin::symmetric(6, 1))
                .show(ui, |ui| {
                    ui.label(
                        RichText::new(*key)
                            .monospace()
                            .size(font_size::SMALL)
                            .color(color::TEXT),
                    );
                });
            ui.label(
                RichText::new(*action)
                    .size(font_size::SMALL)
                    .color(color::TEXT_MUTED),
            );
            ui.add_space(space::S);
        }
    });
}
