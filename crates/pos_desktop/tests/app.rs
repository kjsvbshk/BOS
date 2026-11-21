//! Pruebas de navegación de la aplicación completa.

mod common;

use common::{TALL_WINDOW, app, click, meeting_day};
use egui::Key;
use egui_kittest::{Harness, kittest::Queryable};
use pos_desktop::{
    MIN_WINDOW_SIZE, PosApp,
    navigation::{Route, SectionId, catalog},
};

#[test]
fn starts_on_the_sign_in_screen() {
    let mut harness = Harness::builder()
        .with_size(egui::Vec2::from(TALL_WINDOW))
        .build_eframe(|cc| PosApp::new(cc));
    harness.run();

    harness.get_by_label("Iniciar sesión");
}

#[test]
fn choosing_a_user_opens_operations() {
    let mut harness = Harness::builder()
        .with_size(egui::Vec2::from(TALL_WINDOW))
        .build_eframe(|cc| PosApp::new(cc));
    harness.run();

    harness.key_press(Key::Num1);
    harness.run();
    harness.key_press(Key::Enter);
    harness.run();

    assert!(harness.state().signed_in());
}

#[test]
fn opens_on_operations_at_the_smallest_window_size() {
    let mut harness = Harness::builder()
        .with_size(egui::Vec2::from(MIN_WINDOW_SIZE))
        .build_eframe(|cc| PosApp::for_tests(cc, meeting_day(), "PRO"));
    harness.run();

    assert_eq!(
        harness.state().route(),
        Route::Section(SectionId::Operations)
    );
}

#[test]
fn operations_shows_a_card_for_each_of_the_eleven_options() {
    let harness = app("PRO");

    for option in catalog::section(SectionId::Operations).options() {
        harness.get_by_label(&option.numbered_title());
    }
}

#[test]
fn clicking_a_card_opens_its_option() {
    let mut harness = app("PRO");

    click(&mut harness, "01. Ingresar contratos");

    assert!(matches!(harness.state().route(), Route::Option(option) if option.number == 1));
}

#[test]
fn back_button_returns_to_the_section() {
    let mut harness = app("PRO");
    click(&mut harness, "05. Ingresar mercancía");

    harness
        .get_by_label_contains("Volver a Operaciones")
        .click();
    harness.run();

    assert_eq!(
        harness.state().route(),
        Route::Section(SectionId::Operations)
    );
}

#[test]
fn queries_offer_their_three_actions() {
    let mut harness = app("PRO");

    click(&mut harness, "09. Consultas varias");

    for action in [
        "1. Consulta de contrato",
        "2. Consulta por cliente",
        "3. Consulta de movimiento",
    ] {
        harness.get_by_label(action);
    }
}

#[test]
fn modifications_offer_their_eight_actions() {
    let mut harness = app("PRO");

    click(&mut harness, "10. Modificar, reversar y anular");

    harness.get_by_label("1. Modificar contrato");
    harness.get_by_label("8. Anular contrato");
}

#[test]
fn maintenance_tab_lists_its_options() {
    let mut harness = app("PRO");

    click(&mut harness, "Mantenimiento");

    harness.get_by_label("03. Códigos de mercancía");
}

#[test]
fn reports_tab_lists_its_options() {
    let mut harness = app("PRO");

    click(&mut harness, "Informes");

    harness.get_by_label("01. Caja diaria");
}

#[test]
fn exit_asks_for_confirmation_and_cancel_keeps_the_app_open() {
    let mut harness = app("PRO");

    harness.get_by_label_contains("Salir").click();
    harness.run();
    harness.get_by_label("Salir de la aplicación");
    click(&mut harness, "Cancelar");

    harness.get_by_label("01. Ingresar contratos");
    assert!(harness.query_by_label("Salir de la aplicación").is_none());
}
