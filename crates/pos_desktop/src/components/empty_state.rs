//! Estado vacío: explica por qué no hay contenido y cuál es el siguiente paso.

use egui::{RichText, Ui};

use super::PrimaryButton;
use crate::theme::tokens::{color, font_size, space};

/// Mensaje para una vista o tabla sin contenido.
#[derive(Debug, Clone, Copy)]
pub struct EmptyState<'a> {
    icon: &'a str,
    title: &'a str,
    description: Option<&'a str>,
    action: Option<&'a str>,
}

impl<'a> EmptyState<'a> {
    /// Crea el estado vacío con su título (por ejemplo «Todavía no hay elementos»).
    #[must_use]
    pub fn new(title: &'a str) -> Self {
        Self {
            icon: "🗋",
            title,
            description: None,
            action: None,
        }
    }

    /// Icono decorativo sobre el título.
    #[must_use]
    pub fn icon(mut self, icon: &'a str) -> Self {
        self.icon = icon;
        self
    }

    /// Explicación breve bajo el título.
    #[must_use]
    pub fn description(mut self, description: &'a str) -> Self {
        self.description = Some(description);
        self
    }

    /// Texto de la acción que resuelve el estado vacío.
    #[must_use]
    pub fn action(mut self, action: &'a str) -> Self {
        self.action = Some(action);
        self
    }

    /// Dibuja el estado vacío centrado. Devuelve `true` si se pulsó la acción.
    pub fn show(self, ui: &mut Ui) -> bool {
        ui.vertical_centered(|ui| {
            ui.add_space(space::L);
            ui.label(
                RichText::new(self.icon)
                    .size(font_size::TITLE * 1.5)
                    .color(color::TEXT_MUTED),
            );
            ui.label(RichText::new(self.title).heading().color(color::TEXT));
            if let Some(description) = self.description {
                ui.label(RichText::new(description).color(color::TEXT_MUTED));
            }
            let clicked = self.action.is_some_and(|action| {
                ui.add_space(space::S);
                ui.add(PrimaryButton::new(action)).clicked()
            });
            ui.add_space(space::L);
            clicked
        })
        .inner
    }
}
