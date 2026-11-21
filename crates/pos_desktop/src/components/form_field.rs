//! Campo de formulario: etiqueta, control, ayuda y error con una disposición fija.

use egui::{Response, RichText, Ui};

use crate::{
    strings,
    theme::tokens::{color, font_size, space},
};

/// Envuelve un control de entrada con su etiqueta y sus mensajes.
///
/// ```ignore
/// FormField::new("Nombre")
///     .required(true)
///     .error(name_error.as_deref())
///     .show(ui, |ui| ui.text_edit_singleline(&mut self.name));
/// ```
#[derive(Debug, Clone, Copy)]
pub struct FormField<'a> {
    label: &'a str,
    required: bool,
    help: Option<&'a str>,
    error: Option<&'a str>,
}

impl<'a> FormField<'a> {
    /// Crea un campo con su etiqueta visible.
    #[must_use]
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            required: false,
            help: None,
            error: None,
        }
    }

    /// Marca el campo como obligatorio.
    #[must_use]
    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    /// Texto de ayuda bajo el control. Se oculta mientras haya un error.
    #[must_use]
    pub fn help(mut self, help: &'a str) -> Self {
        self.help = Some(help);
        self
    }

    /// Mensaje de error de validación; `None` si el valor es correcto.
    #[must_use]
    pub fn error(mut self, error: Option<&'a str>) -> Self {
        self.error = error;
        self
    }

    /// Dibuja el campo. `add_input` agrega el control y devuelve su respuesta,
    /// que queda asociada a la etiqueta para los lectores de pantalla.
    pub fn show(self, ui: &mut Ui, add_input: impl FnOnce(&mut Ui) -> Response) -> Response {
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = space::XS;

            let label = ui
                .horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = space::XS;
                    let label = ui.label(RichText::new(self.label).color(color::TEXT));
                    if self.required {
                        ui.label(RichText::new(strings::REQUIRED_MARK).color(color::TEXT_MUTED))
                            .on_hover_text(strings::REQUIRED_HINT);
                    }
                    label
                })
                .inner;

            let response = add_input(ui).labelled_by(label.id);

            if let Some(error) = self.error {
                ui.label(
                    RichText::new(format!("⚠ {error}"))
                        .color(ui.visuals().error_fg_color)
                        .size(font_size::SMALL),
                );
            } else if let Some(help) = self.help {
                ui.label(
                    RichText::new(help)
                        .color(color::TEXT_MUTED)
                        .size(font_size::SMALL),
                );
            }
            response
        })
        .inner
    }
}
