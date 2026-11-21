//! Encabezado de la aplicación: marca, fecha y hora, y usuario.

use std::time::Duration;

use chrono::{Datelike, Local};
use egui::{Frame, Layout, Margin, Panel, RichText, Ui, vec2};

use crate::{
    components::brand_mark,
    strings::header as text,
    theme::{
        self, fonts, icons,
        tokens::{color, font_size, margin, size, space},
    },
};

/// Lado del logotipo.
const LOGO_SIDE: f32 = 52.0;
/// Ancho del bloque de fecha y hora.
const CLOCK_WIDTH: f32 = 130.0;

/// Lo que el usuario pidió desde el encabezado durante este frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeaderAction {
    OpenComponentGallery,
}

/// Dibuja el encabezado como panel superior.
pub fn show(ui: &mut Ui) -> Option<HeaderAction> {
    Panel::top("app_header")
        .exact_size(size::APP_HEADER_HEIGHT)
        .show_separator_line(false)
        .frame(
            Frame::new()
                .fill(color::CHROME)
                .inner_margin(Margin::symmetric(margin::XL, 0)),
        )
        .show(ui, |ui| {
            theme::use_chrome_style(ui);
            egui::Sides::new()
                .height(ui.available_height())
                .show(ui, brand_ui, |ui| {
                    // Derecha a izquierda: el primero queda en el extremo derecho.
                    let action = user_menu_ui(ui);
                    ui.separator();
                    clock_ui(ui);
                    ui.separator();
                    action
                })
                .1
        })
        .inner
}

fn brand_ui(ui: &mut Ui) {
    ui.spacing_mut().item_spacing.x = space::L;
    brand_mark(ui, LOGO_SIDE, color::BEIGE, color::CHROME);
    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = 0.0;
        ui.add_space(space::S);
        ui.label(
            RichText::new(text::BUSINESS_NAME)
                .font(fonts::bold(font_size::BRAND))
                .extra_letter_spacing(1.5)
                .color(color::TEXT_ON_DARK),
        );
        ui.label(
            RichText::new(text::SYSTEM_NAME)
                .font(fonts::semibold(font_size::BODY))
                .extra_letter_spacing(2.5)
                .color(color::TEXT_ON_DARK_MUTED),
        );
    });
}

fn clock_ui(ui: &mut Ui) {
    let now = Local::now();
    let month = text::MONTHS[now.month0() as usize];
    let date = format!(
        "{}  {} {} {}",
        icons::CALENDAR_BLANK,
        now.day(),
        month,
        now.year()
    );
    let time = format!("{}  {}", icons::CLOCK, now.format("%H:%M:%S"));

    // Ancho fijo: un diseño vertical dentro de una fila ocuparía todo el ancho
    // libre y empujaría a los demás elementos.
    let block = vec2(CLOCK_WIDTH, ui.available_height());
    ui.allocate_ui_with_layout(block, Layout::top_down(egui::Align::Min), |ui| {
        ui.spacing_mut().item_spacing.y = space::XS;
        let row = ui.text_style_height(&egui::TextStyle::Body);
        ui.add_space(((ui.available_height() - 2.0 * row - space::XS) / 2.0).max(0.0));
        ui.label(
            RichText::new(date)
                .size(font_size::BODY)
                .color(color::TEXT_ON_DARK),
        );
        ui.label(
            RichText::new(time)
                .size(font_size::BODY)
                .color(color::TEXT_ON_DARK),
        );
    });

    // Redibuja justo al cambiar el segundo; en reposo no hay más trabajo.
    let to_next_second = 1_000 - now.timestamp_subsec_millis().min(999);
    ui.ctx()
        .request_repaint_after(Duration::from_millis(u64::from(to_next_second)));
}

fn user_menu_ui(ui: &mut Ui) -> Option<HeaderAction> {
    let label = RichText::new(format!(
        "{}  {}  {}",
        icons::USER_CIRCLE,
        text::DEMO_USER,
        icons::CARET_DOWN
    ))
    .size(font_size::BODY)
    .color(color::TEXT_ON_DARK);

    ui.menu_button(label, |ui| {
        ui.label(RichText::new(text::DEMO_SESSION).color(color::TEXT_MUTED));
        if cfg!(debug_assertions) {
            ui.separator();
            if ui.button(text::COMPONENT_GALLERY).clicked() {
                return Some(HeaderAction::OpenComponentGallery);
            }
        }
        None
    })
    .inner
    .flatten()
}
