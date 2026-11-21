//! Pestaña de sección para la barra oscura superior.

use egui::{
    Align2, CornerRadius, FontId, Response, Role, Sense, Ui, Widget, WidgetInfo, pos2, vec2,
};

use super::paint;
use crate::theme::{
    fonts,
    tokens::{color, font_size, size, space},
};

/// Pestaña con icono y texto. La seleccionada toma el color del área de
/// contenido, como si se uniera a ella.
#[derive(Debug, Clone)]
pub struct SectionTab<'a> {
    label: &'a str,
    icon: &'a str,
    selected: bool,
}

impl<'a> SectionTab<'a> {
    #[must_use]
    pub fn new(label: &'a str, icon: &'a str, selected: bool) -> Self {
        Self {
            label,
            icon,
            selected,
        }
    }
}

impl Widget for SectionTab<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let text_color = if self.selected {
            color::TEXT
        } else {
            color::TEXT_ON_DARK
        };
        let font = if self.selected {
            fonts::semibold(font_size::BODY + 1.0)
        } else {
            FontId::proportional(font_size::BODY + 1.0)
        };
        let painter = ui.painter();
        let label = painter.layout_no_wrap(self.label.to_owned(), font, text_color);
        let icon_font = FontId::proportional(font_size::HEADING);
        let icon = painter.layout_no_wrap(self.icon.to_owned(), icon_font, text_color);

        let content_width = icon.size().x + space::S + label.size().x;
        let desired = vec2(content_width + 2.0 * space::XL, size::TAB_HEIGHT);
        let (rect, response) = ui.allocate_exact_size(desired, Sense::click());
        response.widget_info(|| {
            WidgetInfo::selected(Role::Tab, ui.is_enabled(), self.selected, self.label)
        });
        if !ui.is_rect_visible(rect) {
            return response;
        }

        let painter = ui.painter();
        let corner = CornerRadius {
            nw: 6,
            ne: 6,
            sw: 0,
            se: 0,
        };
        if self.selected {
            painter.rect_filled(rect, corner, color::SURFACE);
        } else if response.hovered() || response.has_focus() {
            painter.rect_filled(rect, corner, color::CHROME_HOVER);
        }
        if response.has_focus() {
            let ring = if self.selected {
                color::TEXT
            } else {
                color::TEXT_ON_DARK
            };
            paint::focus_ring(painter, rect.shrink(4.0), corner, ring);
        }

        let left = rect.center().x - content_width / 2.0;
        let icon_pos = Align2::LEFT_CENTER.anchor_size(pos2(left, rect.center().y), icon.size());
        let label_left = left + icon.size().x + space::S;
        let label_pos =
            Align2::LEFT_CENTER.anchor_size(pos2(label_left, rect.center().y), label.size());
        painter.galley(icon_pos.min, icon, text_color);
        painter.galley(label_pos.min, label, text_color);
        response
    }
}
