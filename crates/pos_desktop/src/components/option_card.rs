//! Tarjeta de opción de menú: icono, título y descripción. Toda la tarjeta es
//! un botón.

use egui::{
    CornerRadius, Response, Role, Sense, Stroke, StrokeKind, Ui, Widget, WidgetInfo, pos2, vec2,
};

use super::paint;
use crate::theme::{
    fonts,
    tokens::{color, font_size, radius, size, space},
};

/// Tarjeta que abre una opción o una acción.
#[derive(Debug, Clone)]
pub struct OptionCard<'a> {
    title: String,
    description: &'a str,
    icon: &'a str,
    pending: bool,
    width: f32,
}

impl<'a> OptionCard<'a> {
    /// `title` es también el nombre accesible del botón.
    #[must_use]
    pub fn new(title: impl Into<String>, description: &'a str, icon: &'a str, width: f32) -> Self {
        Self {
            title: title.into(),
            description,
            icon,
            pending: false,
            width,
        }
    }

    /// Opción todavía no identificada: se muestra atenuada, pero se puede abrir.
    #[must_use]
    pub fn pending(mut self, pending: bool) -> Self {
        self.pending = pending;
        self
    }
}

impl Widget for OptionCard<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let (rect, response) =
            ui.allocate_exact_size(vec2(self.width, size::OPTION_CARD_HEIGHT), Sense::click());
        response.widget_info(|| WidgetInfo::labeled(Role::Button, ui.is_enabled(), &self.title));
        if !ui.is_rect_visible(rect) {
            return response;
        }

        let emphasized = response.hovered() || response.has_focus();
        let corner = CornerRadius::same(radius::CARD);
        let (fill, border) = if emphasized {
            (color::SURFACE_RAISED, color::BORDER_STRONG)
        } else {
            (color::SURFACE, color::BORDER_SUBTLE)
        };
        let painter = ui.painter().with_clip_rect(rect.expand(6.0));
        painter.rect(
            rect,
            corner,
            fill,
            Stroke::new(1.0, border),
            StrokeKind::Inside,
        );
        if response.has_focus() {
            paint::focus_ring(&painter, rect, corner, color::TEXT);
        }

        let diameter = size::CARD_ICON_DIAMETER;
        let icon_center = pos2(rect.left() + space::L + diameter / 2.0, rect.center().y);
        let (icon_bg, title_color) = if self.pending {
            (color::BEIGE_SOFT, color::TEXT_MUTED)
        } else {
            (color::ICON_BG, color::TEXT)
        };
        paint::icon_circle(
            &painter,
            icon_center,
            diameter,
            icon_bg,
            self.icon,
            color::TEXT,
        );

        let text_left = rect.left() + space::L + diameter + space::L;
        let text_width = (rect.right() - space::L - text_left).max(1.0);
        let title = painter.layout(
            self.title,
            fonts::semibold(font_size::HEADING),
            title_color,
            text_width,
        );
        let description = painter.layout(
            self.description.to_owned(),
            egui::FontId::proportional(font_size::BODY),
            color::TEXT_MUTED,
            text_width,
        );
        let gap = space::XS / 2.0;
        let block_height = title.size().y + gap + description.size().y;
        let top = rect.center().y - block_height / 2.0;
        let description_top = top + title.size().y + gap;
        painter.galley(pos2(text_left, top), title, color::TEXT);
        painter.galley(
            pos2(text_left, description_top),
            description,
            color::TEXT_MUTED,
        );

        response
    }
}
