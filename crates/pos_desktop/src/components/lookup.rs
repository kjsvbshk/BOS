//! Consulta de catálogos y registros: lista de sugerencias bajo el campo
//! (siempre a la vista) y ventana completa con F1. Ambas se manejan con teclado.

use egui::{Context, Key, Modifiers, RichText, ScrollArea, Sense, Stroke, StrokeKind, Ui, vec2};

use super::{AppModal, single_line};
use crate::{
    strings::common,
    theme::tokens::{color, font_size, space},
};

/// Alto máximo de la lista.
const LIST_HEIGHT: f32 = 320.0;
/// Ancho de la ventana.
const DIALOG_WIDTH: f32 = 520.0;
/// Alto de cada fila.
const ROW_HEIGHT: f32 = 30.0;
/// Sugerencias visibles bajo un campo.
const MAX_SUGGESTIONS: usize = 6;

/// Elemento consultable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LookupItem {
    /// Lo que se escribe en el campo al elegirlo (cédula, código…).
    pub value: String,
    pub label: String,
    pub detail: String,
}

/// Estado de la ventana mientras está abierta.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LookupState {
    pub query: String,
    pub highlighted: usize,
}

/// Resultado de un frame de la ventana.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LookupOutcome {
    Open,
    Selected(String),
    Cancelled,
}

/// Dibuja la ventana. Flechas para moverse, Enter para elegir, Esc para cerrar.
pub fn lookup_dialog(
    ctx: &Context,
    title: &str,
    state: &mut LookupState,
    items: &[LookupItem],
) -> LookupOutcome {
    let visible = filter_items(items, &state.query);
    if state.highlighted >= visible.len() {
        state.highlighted = visible.len().saturating_sub(1);
    }

    let (down, up, enter) = ctx.input_mut(|i| {
        (
            i.consume_key(egui::Modifiers::NONE, Key::ArrowDown),
            i.consume_key(egui::Modifiers::NONE, Key::ArrowUp),
            i.key_pressed(Key::Enter),
        )
    });
    if down && state.highlighted + 1 < visible.len() {
        state.highlighted += 1;
    }
    if up {
        state.highlighted = state.highlighted.saturating_sub(1);
    }

    let outcome = AppModal::new(("lookup", title), title)
        .width(DIALOG_WIDTH)
        .show(ctx, |ui| {
            let label = ui.label(RichText::new(common::SEARCH_LABEL).color(color::TEXT_MUTED));
            let search = ui
                .add(
                    single_line(&mut state.query, "lookup_query")
                        .hint_text(common::SEARCH_HINT)
                        .desired_width(f32::INFINITY),
                )
                .labelled_by(label.id);
            search.request_focus();
            if search.changed() {
                state.highlighted = 0;
            }
            ui.add_space(space::S);
            let mut clicked = None;
            ScrollArea::vertical()
                .max_height(LIST_HEIGHT)
                .auto_shrink([false, true])
                .show(ui, |ui| {
                    if visible.is_empty() {
                        ui.label(RichText::new(common::NO_MATCHES).color(color::TEXT_MUTED));
                    }
                    for (index, item) in visible.iter().enumerate() {
                        if item_row(ui, item, index == state.highlighted, down || up).clicked() {
                            clicked = Some(item.value.clone());
                        }
                    }
                });
            ui.add_space(space::S);
            ui.label(
                RichText::new(common::LOOKUP_KEYS)
                    .size(font_size::SMALL)
                    .color(color::TEXT_MUTED),
            );
            clicked
        });

    if let Some(value) = outcome.inner {
        return LookupOutcome::Selected(value);
    }
    if enter && let Some(item) = visible.get(state.highlighted) {
        return LookupOutcome::Selected(item.value.clone());
    }
    if outcome.dismissed {
        return LookupOutcome::Cancelled;
    }
    LookupOutcome::Open
}

fn item_row(ui: &mut Ui, item: &LookupItem, highlighted: bool, moved: bool) -> egui::Response {
    let width = ui.available_width();
    let (rect, response) = ui.allocate_exact_size(vec2(width, ROW_HEIGHT), Sense::click());
    let text = format!("{}  ·  {}", item.value, item.label);
    response.widget_info(|| {
        egui::WidgetInfo::selected(egui::Role::ListBoxOption, true, highlighted, &text)
    });
    if highlighted {
        ui.painter().rect_filled(rect, 4.0, color::SELECTED);
        ui.painter()
            .rect_stroke(rect, 4.0, Stroke::new(1.0, color::TEXT), StrokeKind::Inside);
        if moved {
            response.scroll_to_me(None);
        }
    } else if response.hovered() {
        ui.painter().rect_filled(rect, 4.0, color::SURFACE_RAISED);
    }
    let left = rect.left_center() + vec2(space::S, 0.0);
    ui.painter().text(
        left,
        egui::Align2::LEFT_CENTER,
        text,
        egui::FontId::proportional(font_size::BODY),
        color::TEXT,
    );
    if !item.detail.is_empty() {
        let right = rect.right_center() - vec2(space::S, 0.0);
        ui.painter().text(
            right,
            egui::Align2::RIGHT_CENTER,
            &item.detail,
            egui::FontId::proportional(font_size::SMALL),
            color::TEXT_MUTED,
        );
    }
    response
}

/// Elementos que contienen el texto buscado (en valor, nombre o detalle).
#[must_use]
pub fn filter_items<'a>(items: &'a [LookupItem], query: &str) -> Vec<&'a LookupItem> {
    let query = query.trim().to_lowercase();
    items
        .iter()
        .filter(|item| {
            query.is_empty()
                || item.value.to_lowercase().contains(&query)
                || item.label.to_lowercase().contains(&query)
                || item.detail.to_lowercase().contains(&query)
        })
        .collect()
}

/// Lista de sugerencias bajo un campo de texto.
///
/// Con el campo activo, ↑ ↓ mueven la selección; un clic elige directamente.
/// Devuelve el valor elegido con el ratón.
pub fn suggestion_list(
    ui: &mut Ui,
    items: &[LookupItem],
    query: &str,
    highlighted: &mut Option<usize>,
    keys_active: bool,
) -> Option<String> {
    let visible = filter_items(items, query);
    let shown = visible.len().min(MAX_SUGGESTIONS);
    if *highlighted >= Some(shown) {
        *highlighted = None;
    }
    let (down, up) = if keys_active {
        ui.input_mut(|i| {
            (
                i.consume_key(Modifiers::NONE, Key::ArrowDown),
                i.consume_key(Modifiers::NONE, Key::ArrowUp),
            )
        })
    } else {
        (false, false)
    };
    if down && shown > 0 {
        *highlighted = Some(highlighted.map_or(0, |i| (i + 1).min(shown - 1)));
    }
    if up {
        *highlighted = highlighted.and_then(|i| i.checked_sub(1));
    }

    ui.add_space(space::XS);
    let header = if visible.is_empty() {
        String::from(common::NO_MATCHES)
    } else {
        format!(
            "{} {shown} de {} · {}",
            common::SUGGESTIONS,
            visible.len(),
            common::SUGGESTION_KEYS
        )
    };
    ui.label(
        RichText::new(header)
            .size(font_size::SMALL)
            .color(color::TEXT_MUTED),
    );
    let mut clicked = None;
    for (index, item) in visible.iter().take(shown).enumerate() {
        if item_row(ui, item, *highlighted == Some(index), false).clicked() {
            clicked = Some(item.value.clone());
        }
    }
    clicked
}

/// Valor que se envía al pulsar Enter en un campo con sugerencias: lo escrito
/// si coincide exactamente con un elemento; si no, la sugerencia seleccionada;
/// si no hay ninguna, lo escrito.
#[must_use]
pub fn enter_value(items: &[LookupItem], query: &str, highlighted: Option<usize>) -> String {
    let typed = query.trim();
    if items.iter().any(|item| item.value == typed) {
        return typed.to_owned();
    }
    highlighted
        .and_then(|index| {
            filter_items(items, query)
                .get(index)
                .map(|item| item.value.clone())
        })
        .unwrap_or_else(|| typed.to_owned())
}
