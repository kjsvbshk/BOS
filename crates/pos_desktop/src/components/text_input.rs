//! Campos de texto con las medidas del sistema visual y máscaras de entrada.
//!
//! Una máscara decide carácter por carácter qué se puede escribir: lo que no
//! corresponde (una letra en una cédula, una segunda coma en un peso) no llega
//! a entrar, tampoco al pegar con Ctrl+V. La validación completa del formato
//! se hace al confirmar.

use std::ops::Range;

use egui::{Align2, AsIdSalt, TextBuffer, TextEdit, text::CharIndex, vec2};

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

/// Qué se puede escribir en un campo.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMask {
    /// Cualquier texto, con largo máximo.
    Free { max_len: usize },
    /// Sólo dígitos.
    Digits { max_len: usize },
    /// Importe: dígitos y los puntos de miles que pone el propio campo.
    Money,
    /// Decimal con coma o punto: «2,9», «7.5».
    Decimal { decimals: usize },
    /// Fecha dd/mm/aaaa (también con guiones).
    Date,
    /// Teléfono: dígitos y espacios.
    Phone,
    /// Nombre de persona: letras, espacios, punto, guion y apóstrofo.
    Name,
    /// Código alfanumérico sin espacios.
    Code { max_len: usize },
}

/// Largo máximo de los nombres.
const NAME_MAX_LEN: usize = 60;
/// Cifras máximas de un importe (billones de pesos).
const MONEY_MAX_DIGITS: usize = 12;
/// Cifras enteras máximas de un decimal.
const DECIMAL_MAX_WHOLE: usize = 6;
/// Cifras máximas de un teléfono.
const PHONE_MAX_DIGITS: usize = 10;

impl InputMask {
    /// `true` si `text` es un contenido admisible mientras se escribe (puede
    /// estar incompleto, pero no tener caracteres o formas imposibles).
    #[must_use]
    pub fn accepts(self, text: &str) -> bool {
        let len = text.chars().count();
        match self {
            Self::Free { max_len } => len <= max_len,
            Self::Digits { max_len } => len <= max_len && text.chars().all(|c| c.is_ascii_digit()),
            Self::Money => {
                text.chars().all(|c| c.is_ascii_digit() || c == '.')
                    && text.chars().filter(char::is_ascii_digit).count() <= MONEY_MAX_DIGITS
            }
            Self::Decimal { decimals } => {
                let separators = text.chars().filter(|c| matches!(c, ',' | '.')).count();
                let (whole, fraction) = text.split_once([',', '.']).unwrap_or((text, ""));
                separators <= 1
                    && text
                        .chars()
                        .all(|c| c.is_ascii_digit() || matches!(c, ',' | '.'))
                    && whole.len() <= DECIMAL_MAX_WHOLE
                    && fraction.len() <= decimals
            }
            Self::Date => {
                len <= 10
                    && text
                        .chars()
                        .all(|c| c.is_ascii_digit() || matches!(c, '/' | '-'))
                    && text.chars().filter(|c| matches!(c, '/' | '-')).count() <= 2
            }
            Self::Phone => {
                len <= PHONE_MAX_DIGITS + 4
                    && text.chars().all(|c| c.is_ascii_digit() || c == ' ')
                    && text.chars().filter(char::is_ascii_digit).count() <= PHONE_MAX_DIGITS
            }
            Self::Name => {
                len <= NAME_MAX_LEN
                    && !text.starts_with(' ')
                    && !text.contains("  ")
                    && text
                        .chars()
                        .all(|c| c.is_alphabetic() || matches!(c, ' ' | '.' | '-' | '\''))
            }
            Self::Code { max_len } => {
                len <= max_len && text.chars().all(|c| c.is_ascii_alphanumeric())
            }
        }
    }
}

/// Texto que sólo admite lo que permite su máscara.
pub struct MaskedText<'a> {
    text: &'a mut String,
    mask: InputMask,
}

impl<'a> MaskedText<'a> {
    pub fn new(text: &'a mut String, mask: InputMask) -> Self {
        Self { text, mask }
    }
}

/// Marca de tipo para [`TextBuffer::type_id`].
struct MaskedTextType;

impl TextBuffer for MaskedText<'_> {
    fn is_mutable(&self) -> bool {
        true
    }

    fn as_str(&self) -> &str {
        self.text.as_str()
    }

    /// Inserta carácter por carácter sólo lo que la máscara admite.
    fn insert_text(&mut self, text: &str, char_index: CharIndex) -> usize {
        let mut index = char_index.0;
        let mut inserted = 0;
        for ch in text.chars() {
            let mut candidate: String = self.text.chars().take(index).collect();
            candidate.push(ch);
            candidate.extend(self.text.chars().skip(index));
            if self.mask.accepts(&candidate) {
                *self.text = candidate;
                index += 1;
                inserted += 1;
            }
        }
        inserted
    }

    fn delete_char_range(&mut self, char_range: Range<CharIndex>) {
        TextBuffer::delete_char_range(self.text, char_range);
    }

    fn type_id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<MaskedTextType>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn typed(mask: InputMask, input: &str) -> String {
        let mut text = String::new();
        let mut buffer = MaskedText::new(&mut text, mask);
        buffer.insert_text(input, CharIndex(0));
        text
    }

    #[test]
    fn id_numbers_drop_letters() {
        assert_eq!(typed(InputMask::Digits { max_len: 10 }, "10a54b0"), "10540");
    }

    #[test]
    fn id_numbers_stop_at_their_maximum_length() {
        assert_eq!(
            typed(InputMask::Digits { max_len: 10 }, "123456789012"),
            "1234567890"
        );
    }

    #[test]
    fn weights_accept_one_decimal_separator() {
        assert_eq!(typed(InputMask::Decimal { decimals: 3 }, "2,9,5g"), "2,95");
    }

    #[test]
    fn weights_limit_their_decimals() {
        assert_eq!(
            typed(InputMask::Decimal { decimals: 3 }, "1.23456"),
            "1.234"
        );
    }

    #[test]
    fn phones_keep_digits_and_spaces() {
        assert_eq!(typed(InputMask::Phone, "310-555 01a01"), "310555 0101");
    }

    #[test]
    fn names_drop_digits() {
        assert_eq!(typed(InputMask::Name, "María 3 Ríos"), "María Ríos");
    }

    #[test]
    fn dates_accept_digits_and_slashes() {
        assert_eq!(typed(InputMask::Date, "18/11/2026x"), "18/11/2026");
    }

    #[test]
    fn codes_drop_spaces_and_symbols() {
        assert_eq!(typed(InputMask::Code { max_len: 8 }, "REC 2-b"), "REC2b");
    }
}
