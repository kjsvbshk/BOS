//! Campos de texto con las medidas del sistema visual.

use egui::{Align2, AsIdSalt, TextBuffer, TextEdit, vec2};

use crate::theme::tokens::size;

/// Campo de texto de una línea con el alto de los demás controles y el texto
/// centrado verticalmente. Devuelve el `TextEdit` para seguir configurándolo
/// (texto de ayuda, ancho, límite de caracteres…).
pub fn single_line(text: &mut dyn TextBuffer, id_salt: impl AsIdSalt) -> TextEdit<'_> {
    TextEdit::singleline(text)
        .id_salt(id_salt)
        .align(Align2::LEFT_CENTER)
        .min_size(vec2(0.0, size::CONTROL_HEIGHT))
}
