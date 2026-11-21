//! Botones de acción.

use egui::{
    Align2, CornerRadius, Response, Role, Sense, TextStyle, TextWrapMode, Ui, Widget, WidgetInfo,
    WidgetText, vec2,
};

use super::paint;
use crate::theme::tokens::{color, radius, size};

/// La acción principal de un grupo. Debe haber como máximo una por grupo.
///
/// Es un widget propio porque el estilo general de egui marca el foco cambiando
/// el fondo, y sobre el fondo negro fijo de este botón el foco no se vería. Aquí
/// el foco se dibuja como un anillo exterior.
///
/// ```ignore
/// if ui.add(PrimaryButton::new("Guardar cambios")).clicked() { /* … */ }
/// ```
#[derive(Debug, Clone)]
pub struct PrimaryButton {
    text: String,
}

impl PrimaryButton {
    /// Crea el botón con el texto de la acción (por ejemplo «Guardar cambios»).
    #[must_use]
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

impl Widget for PrimaryButton {
    fn ui(self, ui: &mut Ui) -> Response {
        let padding = ui.spacing().button_padding;
        let galley = WidgetText::from(self.text).into_galley(
            ui,
            Some(TextWrapMode::Extend),
            f32::INFINITY,
            TextStyle::Button,
        );
        let desired = vec2(
            galley.size().x + 2.0 * padding.x,
            size::CONTROL_HEIGHT.max(galley.size().y + 2.0 * padding.y),
        );
        let (rect, response) = ui.allocate_exact_size(desired, Sense::click());
        response.widget_info(|| WidgetInfo::labeled(Role::Button, ui.is_enabled(), galley.text()));

        if ui.is_rect_visible(rect) {
            let emphasized =
                response.hovered() || response.has_focus() || response.is_pointer_button_down_on();
            let fill = if emphasized {
                color::PRIMARY_HOVER
            } else {
                color::PRIMARY
            };
            let corner = CornerRadius::same(radius::CONTROL);
            let painter = ui.painter();
            painter.rect_filled(rect, corner, fill);
            if response.has_focus() {
                paint::focus_ring(painter, rect, corner, color::TEXT);
            }
            let text_pos = Align2::CENTER_CENTER
                .anchor_size(rect.center(), galley.size())
                .min;
            painter.galley(text_pos, galley, color::TEXT_ON_DARK);
        }
        response
    }
}

/// Acción secundaria o neutral. Usa el estilo general de botón del tema.
#[derive(Debug, Clone)]
pub struct SecondaryButton {
    text: String,
}

impl SecondaryButton {
    /// Crea el botón con el texto de la acción.
    #[must_use]
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

impl Widget for SecondaryButton {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.add(egui::Button::new(self.text).min_size(vec2(0.0, size::CONTROL_HEIGHT)))
    }
}
