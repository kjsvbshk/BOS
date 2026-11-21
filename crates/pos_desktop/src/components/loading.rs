//! Indicador de trabajo en curso.

use egui::{Response, RichText, Spinner, Ui, Widget};

use crate::{strings::common, theme::tokens::color};

/// Animación y texto de carga.
///
/// Mientras está en pantalla la interfaz se redibuja en cada frame: mostrarlo
/// sólo mientras el trabajo está en marcha.
#[derive(Debug, Clone)]
pub struct LoadingIndicator {
    text: String,
}

impl Default for LoadingIndicator {
    fn default() -> Self {
        Self {
            text: common::LOADING.to_owned(),
        }
    }
}

impl LoadingIndicator {
    /// Indicador con un texto propio (por ejemplo «Cargando informes…»).
    #[must_use]
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

impl Widget for LoadingIndicator {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.horizontal(|ui| {
            ui.add(Spinner::new().color(color::TEXT_MUTED));
            ui.label(RichText::new(self.text).color(color::TEXT_MUTED))
        })
        .inner
    }
}
