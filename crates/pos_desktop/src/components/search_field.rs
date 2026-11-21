//! Campo de búsqueda con botón para limpiar.

use egui::{AsIdSalt, IdSalt, Response, Ui, Widget};

use super::{SecondaryButton, single_line};
use crate::{strings::common, theme::tokens::space};

/// Campo de búsqueda de una sola línea. La respuesta indica `changed()` tanto al
/// escribir como al limpiar.
///
/// El botón «Limpiar» siempre está presente (deshabilitado si no hay texto) para
/// que la disposición no salte al escribir.
pub struct SearchField<'a> {
    query: &'a mut String,
    id_salt: IdSalt,
    hint: &'a str,
    width: f32,
}

impl<'a> SearchField<'a> {
    /// `id_salt` distingue este campo de otros buscadores en la misma pantalla.
    #[must_use]
    pub fn new(query: &'a mut String, id_salt: impl AsIdSalt) -> Self {
        Self {
            query,
            id_salt: IdSalt::new(id_salt),
            hint: common::SEARCH_HINT,
            width: 280.0,
        }
    }

    /// Texto de ejemplo dentro del campo vacío.
    #[must_use]
    pub fn hint(mut self, hint: &'a str) -> Self {
        self.hint = hint;
        self
    }

    /// Ancho del campo de texto.
    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }
}

impl Widget for SearchField<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = space::S;
            let mut response = ui.add(
                single_line(self.query, self.id_salt)
                    .hint_text(self.hint)
                    .desired_width(self.width),
            );

            let has_text = !self.query.is_empty();
            let clear = ui
                .add_enabled(has_text, SecondaryButton::new(common::SEARCH_CLEAR))
                .on_disabled_hover_text(common::SEARCH_CLEAR_DISABLED);
            if clear.clicked() {
                self.query.clear();
                response.request_focus();
                response.mark_changed();
            }
            response
        })
        .inner
    }
}
