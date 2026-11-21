//! Indicador de estado. Siempre combina icono, texto y color: el color nunca
//! es la única señal.

use egui::{Color32, CornerRadius, Frame, Margin, Response, RichText, Stroke, Ui, Widget};

use crate::theme::tokens::{color, font_size, radius};

/// Tono del indicador.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BadgeTone {
    /// Información sin connotación.
    Neutral,
    /// Elemento todavía no definido o en espera.
    Pending,
    /// Resultado correcto.
    Success,
    /// Requiere atención.
    Warning,
    /// Falló o no es válido.
    Error,
}

impl BadgeTone {
    fn icon(self) -> &'static str {
        match self {
            Self::Neutral => "•",
            Self::Pending => "○",
            Self::Success => "✔",
            Self::Warning => "⚠",
            Self::Error => "✖",
        }
    }

    /// Color de texto y de fondo del tono.
    fn colors(self) -> (Color32, Color32) {
        match self {
            Self::Neutral => (color::TEXT, color::BEIGE),
            Self::Pending => (color::TEXT_MUTED, color::BEIGE_SOFT),
            Self::Success => (color::SUCCESS, color::SUCCESS_BG),
            Self::Warning => (color::WARNING, color::WARNING_BG),
            Self::Error => (color::ERROR, color::ERROR_BG),
        }
    }
}

/// Etiqueta compacta con el estado de un elemento.
#[derive(Debug, Clone)]
pub struct StatusBadge {
    tone: BadgeTone,
    text: String,
}

impl StatusBadge {
    /// Crea un indicador con su tono y su texto.
    #[must_use]
    pub fn new(tone: BadgeTone, text: impl Into<String>) -> Self {
        Self {
            tone,
            text: text.into(),
        }
    }
}

impl Widget for StatusBadge {
    fn ui(self, ui: &mut Ui) -> Response {
        let (foreground, background) = self.tone.colors();
        Frame::new()
            .fill(background)
            .stroke(Stroke::new(1.0, foreground))
            .corner_radius(CornerRadius::same(radius::BADGE))
            .inner_margin(Margin::symmetric(8, 2))
            .show(ui, |ui| {
                ui.label(
                    RichText::new(format!("{} {}", self.tone.icon(), self.text))
                        .color(foreground)
                        .size(font_size::SMALL),
                )
            })
            .inner
    }
}
