//! Tarjeta: superficie que agrupa contenido relacionado.

use egui::{CornerRadius, Frame, InnerResponse, Margin, RichText, Stroke, Ui};

use crate::theme::tokens::{color, margin, radius, space};

/// Agrupa contenido relacionado sobre una superficie beige. Usar sólo para
/// grupos reales: si todo va en tarjetas, nada se lee como grupo.
#[derive(Debug, Clone, Copy, Default)]
pub struct Card<'a> {
    title: Option<&'a str>,
}

impl<'a> Card<'a> {
    /// Tarjeta sin título.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Título de la tarjeta.
    #[must_use]
    pub fn title(mut self, title: &'a str) -> Self {
        self.title = Some(title);
        self
    }

    /// Dibuja la tarjeta ocupando todo el ancho disponible.
    pub fn show<R>(self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
        Frame::new()
            .fill(color::SURFACE_RAISED)
            .stroke(Stroke::new(1.0, color::BORDER_SUBTLE))
            .corner_radius(CornerRadius::same(radius::CARD))
            .inner_margin(Margin::same(margin::L))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                if let Some(title) = self.title {
                    ui.label(RichText::new(title).heading().color(color::TEXT));
                    ui.add_space(space::XS);
                }
                add_contents(ui)
            })
    }
}
