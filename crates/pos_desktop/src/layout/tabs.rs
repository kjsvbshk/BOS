//! Barra de secciones: Operaciones, Mantenimiento e Informes, y «Salir».

use egui::{Button, Frame, Margin, Panel, RichText, Ui, vec2};

use crate::{
    components::SectionTab,
    navigation::{Route, catalog},
    strings::navigation as text,
    theme::{
        self, icons,
        tokens::{color, font_size, margin, size, space},
    },
};

/// Lo que el usuario pidió desde la barra durante este frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabsAction {
    Navigate(Route),
    RequestExit,
}

/// Dibuja la barra de secciones como panel superior.
pub fn show(ui: &mut Ui, current: Route) -> Option<TabsAction> {
    Panel::top("section_tabs")
        .exact_size(size::TAB_HEIGHT + space::S)
        .show_separator_line(false)
        .frame(
            Frame::new()
                .fill(color::CHROME_RAISED)
                .inner_margin(Margin {
                    left: margin::XL,
                    right: margin::XL,
                    top: margin::S,
                    bottom: 0,
                }),
        )
        .show(ui, |ui| {
            theme::use_chrome_style(ui);
            let (navigate, exit) = egui::Sides::new().height(size::TAB_HEIGHT).show(
                ui,
                |ui| {
                    ui.spacing_mut().item_spacing.x = 2.0;
                    let selected = current.section();
                    let mut navigate = None;
                    for section in &catalog::SECTIONS {
                        let tab = SectionTab::new(
                            section.label,
                            section.icon,
                            selected == Some(section.id),
                        );
                        if ui.add(tab).clicked() {
                            navigate = Some(Route::Section(section.id));
                        }
                    }
                    navigate
                },
                |ui| {
                    let label = RichText::new(format!("{}  {}", icons::POWER, text::EXIT))
                        .size(font_size::BODY + 1.0)
                        .color(color::TEXT_ON_DARK);
                    ui.add(Button::new(label).min_size(vec2(0.0, size::TAB_HEIGHT - space::S)))
                        .clicked()
                },
            );
            if exit {
                Some(TabsAction::RequestExit)
            } else {
                navigate.map(TabsAction::Navigate)
            }
        })
        .inner
}
