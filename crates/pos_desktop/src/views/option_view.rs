//! Pantallas provisionales de opciones y acciones. Muestran dónde irá cada
//! proceso sin implementar ninguna regla de negocio.

use egui::{RichText, Ui};

use crate::{
    components::{BadgeTone, EmptyState, OptionCard, PageHeader, SecondaryButton, StatusBadge},
    navigation::{NavAction, NavOption, OptionStatus, Route, catalog},
    strings::navigation as text,
    theme::{
        icons,
        tokens::{color, size, space},
    },
};

/// Pantalla de una opción. Devuelve la ruta elegida, si el usuario navegó.
pub fn show_option(ui: &mut Ui, option: &'static NavOption) -> Option<Route> {
    let section = catalog::section_of(option);
    let mut chosen = back_button(ui, section.label, Route::Section(section.id));

    let mut breadcrumb = section.label.to_owned();
    if let Some(group) = catalog::group_of(option) {
        breadcrumb.push_str(text::BREADCRUMB_SEPARATOR);
        breadcrumb.push_str(group.label);
    }
    let title = option.numbered_full_name();
    PageHeader::new(option.icon, &title)
        .subtitle(&breadcrumb)
        .show(ui);
    status_badge(ui, option.status);

    if option.actions.is_empty() {
        pending_screen(ui, option.status);
        return chosen;
    }

    ui.label(RichText::new(text::CHOOSE_ACTION).color(color::TEXT_MUTED));
    ui.add_space(space::S);
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing = egui::vec2(space::M, space::M);
        for action in option.actions {
            let card = OptionCard::new(
                action.label,
                text::AWAITING_SPECIFICATION,
                action.icon,
                size::OPTION_CARD_MIN_WIDTH,
            );
            if ui.add(card).clicked() {
                chosen = Some(Route::Action(option, action));
            }
        }
    });
    chosen
}

/// Pantalla de una acción dentro de una opción.
pub fn show_action(
    ui: &mut Ui,
    option: &'static NavOption,
    action: &'static NavAction,
) -> Option<Route> {
    let parent_title = option.numbered_title();
    let chosen = back_button(ui, &parent_title, Route::Option(option));

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
    status_badge(ui, OptionStatus::AwaitingSpecification);
    pending_screen(ui, OptionStatus::AwaitingSpecification);
    chosen
}

fn back_button(ui: &mut Ui, destination: &str, route: Route) -> Option<Route> {
    let label = format!("{}  {}", icons::ARROW_LEFT, text::back_to(destination));
    let clicked = ui.add(SecondaryButton::new(label)).clicked();
    ui.add_space(space::L);
    clicked.then_some(route)
}

fn status_badge(ui: &mut Ui, status: OptionStatus) {
    let badge = match status {
        OptionStatus::AwaitingSpecification => {
            StatusBadge::new(BadgeTone::Pending, text::AWAITING_SPECIFICATION)
        }
        OptionStatus::Unidentified => StatusBadge::new(BadgeTone::Warning, text::UNIDENTIFIED),
    };
    ui.add(badge);
    ui.add_space(space::L);
}

fn pending_screen(ui: &mut Ui, status: OptionStatus) {
    let description = match status {
        OptionStatus::AwaitingSpecification => text::SCREEN_PENDING_DESCRIPTION,
        OptionStatus::Unidentified => text::UNIDENTIFIED_DESCRIPTION,
    };
    EmptyState::new(text::SCREEN_PENDING_TITLE)
        .icon(icons::CLIPBOARD_TEXT)
        .description(description)
        .show(ui);
}
