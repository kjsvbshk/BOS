//! Pantalla de una opción o de una acción: «Volver», título y su contenido
//! (flujo, consulta, informe, submenú o aviso de pendiente).

use chrono::NaiveDate;
use egui::{RichText, Ui};
use pos_core::application::PosStore;

use super::screens::{Outcome, Screen, escape_pressed};
use crate::{
    components::{
        BadgeTone, EmptyState, OptionCard, PageHeader, SecondaryButton, StatusBadge, key_hints,
    },
    navigation::{NavAction, NavOption, OptionStatus, Route, catalog},
    strings::navigation as text,
    theme::{
        icons,
        tokens::{color, size, space},
    },
};

/// Pantalla de una opción. Devuelve la ruta elegida, si el usuario navegó.
pub fn show_option(
    ui: &mut Ui,
    option: &'static NavOption,
    screen: Option<&mut Box<dyn Screen>>,
    store: &mut PosStore,
    today: NaiveDate,
) -> Option<Route> {
    let section = catalog::section_of(option);
    let parent = Route::Section(section.id);
    let mut chosen = back_button(ui, section.label, parent);

    let mut breadcrumb = section.label.to_owned();
    if let Some(group) = catalog::group_of(option) {
        breadcrumb.push_str(text::BREADCRUMB_SEPARATOR);
        breadcrumb.push_str(group.label);
    }
    let title = option.numbered_full_name();
    PageHeader::new(option.icon, &title)
        .subtitle(&breadcrumb)
        .show(ui);

    if !store.can(option.permission) {
        EmptyState::new(text::RESTRICTED_TITLE)
            .icon(icons::LOCK)
            .description(option.permission.denied_reason())
            .show(ui);
        return chosen.or_else(|| escape_pressed(ui).then_some(parent));
    }

    if !option.actions.is_empty() {
        return chosen
            .or_else(|| actions_ui(ui, option).or_else(|| escape_pressed(ui).then_some(parent)));
    }

    if let Some(screen) = screen {
        if screen.ui(ui, store, today) == Outcome::Back {
            chosen = Some(parent);
        }
    } else {
        if option.status == OptionStatus::AwaitingSpecification {
            ui.add(StatusBadge::new(
                BadgeTone::Pending,
                text::AWAITING_SPECIFICATION,
            ));
            ui.add_space(space::L);
        }
        EmptyState::new(text::SCREEN_PENDING_TITLE)
            .icon(icons::CLIPBOARD_TEXT)
            .description(text::SCREEN_PENDING_DESCRIPTION)
            .show(ui);
        if escape_pressed(ui) {
            chosen = Some(parent);
        }
    }
    chosen
}

/// Submenú de acciones (09 y 10). Un número selecciona la acción; Enter la abre.
fn actions_ui(ui: &mut Ui, option: &'static NavOption) -> Option<Route> {
    let mut chosen = None;
    let count = option.actions.len();
    let selected =
        super::keyboard::number_selection(ui, option.id, |n| (1..=count).contains(&usize::from(n)));
    ui.label(RichText::new(text::CHOOSE_ACTION).color(color::TEXT_MUTED));
    ui.add_space(space::S);
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing = egui::vec2(space::M, space::M);
        for (index, action) in option.actions.iter().enumerate() {
            let title = format!("{}. {}", index + 1, action.label);
            let card = OptionCard::new(
                title,
                action.description,
                action.icon,
                size::OPTION_CARD_MIN_WIDTH + 40.0,
            );
            let response = ui.add(card);
            if selected.map(usize::from) == Some(index + 1) {
                response.request_focus();
            }
            if response.clicked() {
                chosen = Some(Route::Action(option, action));
            }
        }
    });
    key_hints(
        ui,
        &[
            ("1–9", text::SELECT_BY_NUMBER),
            ("Enter", text::OPEN),
            ("Esc", "Volver"),
        ],
    );
    chosen
}

/// Pantalla de una acción dentro de una opción.
pub fn show_action(
    ui: &mut Ui,
    option: &'static NavOption,
    action: &'static NavAction,
    screen: Option<&mut Box<dyn Screen>>,
    store: &mut PosStore,
    today: NaiveDate,
) -> Option<Route> {
    let parent = Route::Option(option);
    let parent_title = option.numbered_title();
    let mut chosen = back_button(ui, &parent_title, parent);
    let section = catalog::section_of(option);
    let breadcrumb = format!(
        "{}{}{}",
        section.label,
        text::BREADCRUMB_SEPARATOR,
        parent_title
    );
    PageHeader::new(action.icon, action.label)
        .subtitle(&breadcrumb)
        .show(ui);
    if !store.can(option.permission) {
        EmptyState::new(text::RESTRICTED_TITLE)
            .icon(icons::LOCK)
            .description(option.permission.denied_reason())
            .show(ui);
        return chosen.or_else(|| escape_pressed(ui).then_some(parent));
    }
    if let Some(screen) = screen
        && screen.ui(ui, store, today) == Outcome::Back
    {
        chosen = Some(parent);
    }
    chosen
}

fn back_button(ui: &mut Ui, destination: &str, route: Route) -> Option<Route> {
    let label = format!("{}  {}", icons::ARROW_LEFT, text::back_to(destination));
    let clicked = ui.add(SecondaryButton::new(label)).clicked();
    ui.add_space(space::L);
    clicked.then_some(route)
}
