//! Página principal de una sección: sus opciones agrupadas en tarjetas o, si
//! todavía no tiene opciones, un estado vacío.

use egui::{RichText, Ui};

use crate::{
    components::{EmptyState, OptionCard, PageHeader, info_banner},
    navigation::{NavSection, OptionStatus, Route},
    strings::navigation as text,
    theme::{
        icons,
        tokens::{color, font_size, size, space},
    },
};

/// Separación entre tarjetas.
const CARD_GAP: f32 = space::M;
/// Máximo de columnas de tarjetas.
const MAX_COLUMNS: u8 = 3;

/// Dibuja la sección. Devuelve la ruta elegida, si el usuario abrió una opción.
pub fn show(ui: &mut Ui, section: &'static NavSection) -> Option<Route> {
    PageHeader::new(section.icon, section.label)
        .subtitle(section.subtitle)
        .show(ui);

    if section.groups.is_empty() {
        let description = text::section_pending_description(section.label);
        EmptyState::new(text::SECTION_PENDING_TITLE)
            .icon(icons::CLIPBOARD_TEXT)
            .description(&description)
            .show(ui);
        return None;
    }

    let columns = column_count(ui.available_width());
    let card_width = card_width(ui.available_width(), columns);
    let mut chosen = None;

    for group in section.groups {
        ui.label(
            RichText::new(group.label.to_uppercase())
                .size(font_size::SMALL + 1.0)
                .extra_letter_spacing(1.2)
                .color(color::TEXT_MUTED),
        );
        ui.add_space(space::XS);
        for row in group.options.chunks(usize::from(columns)) {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = CARD_GAP;
                for option in row {
                    let card = OptionCard::new(
                        option.numbered_title(),
                        option.description,
                        option.icon,
                        card_width,
                    )
                    .pending(option.status == OptionStatus::Unidentified);
                    if ui.add(card).clicked() {
                        chosen = Some(Route::Option(option));
                    }
                }
            });
            ui.add_space(CARD_GAP - ui.spacing().item_spacing.y);
        }
        ui.add_space(space::M);
    }

    info_banner(ui, text::SELECT_OPTION_HINT);
    chosen
}

/// Columnas que caben en el ancho disponible (entre 1 y 3).
fn column_count(available: f32) -> u8 {
    (1..=MAX_COLUMNS)
        .rev()
        .find(|&columns| card_width(available, columns) >= size::OPTION_CARD_MIN_WIDTH)
        .unwrap_or(1)
}

fn card_width(available: f32, columns: u8) -> f32 {
    let gaps = CARD_GAP * f32::from(columns.saturating_sub(1));
    ((available - gaps) / f32::from(columns.max(1))).floor()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wide_windows_use_three_columns() {
        assert_eq!(column_count(1200.0), 3);
    }

    #[test]
    fn medium_windows_use_two_columns() {
        assert_eq!(column_count(700.0), 2);
    }

    #[test]
    fn narrow_windows_use_one_column() {
        assert_eq!(column_count(400.0), 1);
    }
}
