//! Título de página: icono en círculo, título y subtítulo, con una línea
//! separadora debajo.

use egui::{RichText, Sense, Stroke, Ui, vec2};

use super::paint;
use crate::theme::{
    self,
    tokens::{color, font_size, size, space},
};

/// Encabezado de una pantalla.
#[derive(Debug, Clone, Copy)]
pub struct PageHeader<'a> {
    icon: &'a str,
    title: &'a str,
    subtitle: Option<&'a str>,
}

impl<'a> PageHeader<'a> {
    #[must_use]
    pub fn new(icon: &'a str, title: &'a str) -> Self {
        Self {
            icon,
            title,
            subtitle: None,
        }
    }

    #[must_use]
    pub fn subtitle(mut self, subtitle: &'a str) -> Self {
        self.subtitle = Some(subtitle);
        self
    }

    pub fn show(self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = space::L;
            let diameter = size::PAGE_ICON_DIAMETER;
            let (rect, _) = ui.allocate_exact_size(vec2(diameter, diameter), Sense::hover());
            paint::icon_circle(
                ui.painter(),
                rect.center(),
                diameter,
                color::PRIMARY,
                self.icon,
                color::TEXT_ON_DARK,
            );
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing.y = space::XS;
                ui.label(
                    RichText::new(self.title)
                        .text_style(theme::title_style())
                        .color(color::TEXT),
                );
                if let Some(subtitle) = self.subtitle {
                    ui.label(
                        RichText::new(subtitle)
                            .size(font_size::BODY + 1.0)
                            .color(color::TEXT_MUTED),
                    );
                }
            });
        });
        ui.add_space(space::M);
        let rect = ui.available_rect_before_wrap();
        ui.painter().hline(
            rect.x_range(),
            rect.top(),
            Stroke::new(1.0, color::BORDER_SUBTLE),
        );
        ui.add_space(space::XL);
    }
}
