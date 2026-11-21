//! Pruebas de navegación de la aplicación completa.

use egui::Key;
use egui_kittest::{Harness, kittest::Queryable};
use pos_desktop::{
    MIN_WINDOW_SIZE, PosApp,
    navigation::{Route, SectionId, catalog},
};

/// Ventana alta para que todas las tarjetas queden visibles sin desplazarse.
/// El aspecto en la ventana mínima se revisa con las capturas de `review.rs`.
const TALL_WINDOW: [f32; 2] = [MIN_WINDOW_SIZE[0], 1400.0];

fn app_harness() -> Harness<'static, PosApp> {
    let mut harness = Harness::builder()
        .with_size(egui::Vec2::from(TALL_WINDOW))
        .build_eframe(|cc| PosApp::new(cc));
    harness.run();
    harness
}

fn click(harness: &mut Harness<'_, PosApp>, label: &str) {
    harness.get_by_label(label).click();
    harness.run();
}

fn operation(number: u8) -> Option<&'static pos_desktop::navigation::NavOption> {
    catalog::section(SectionId::Operations)
        .options()
        .find(|option| option.number == number)
}

#[test]
fn opens_on_operations() {
    let harness = app_harness();

    assert_eq!(
        harness.state().route(),
        Route::Section(SectionId::Operations)
    );
}

#[test]
fn operations_shows_a_card_for_each_of_the_eleven_options() {
    let harness = app_harness();

    for option in catalog::section(SectionId::Operations).options() {
        harness.get_by_label(&option.numbered_title());
    }
}

#[test]
fn opens_on_the_smallest_window_without_errors() {
    let mut harness = Harness::builder()
        .with_size(egui::Vec2::from(MIN_WINDOW_SIZE))
        .build_eframe(|cc| PosApp::new(cc));
    harness.run();

    harness.get_by_label("01. Ingresar contratos");
}

#[test]
fn clicking_a_card_opens_its_option() {
    let mut harness = app_harness();

    click(&mut harness, "01. Ingresar contratos");

    assert_eq!(
        harness.state().route(),
        Route::Option(operation(1).unwrap())
    );
}

#[test]
fn back_button_returns_to_the_section() {
    let mut harness = app_harness();
    click(&mut harness, "01. Ingresar contratos");

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
fn escape_returns_to_the_previous_screen() {
    let mut harness = app_harness();
    click(&mut harness, "02. Prorrogar plazo");

    harness.key_press(Key::Escape);
    harness.run();

    assert_eq!(
        harness.state().route(),
        Route::Section(SectionId::Operations)
    );
}

#[test]
fn contract_modification_offers_its_four_actions() {
    let mut harness = app_harness();

    click(&mut harness, "09. Modificación de contratos");

    for action in ["Reversar", "Anular", "Modificar", "Borrar"] {
        harness.get_by_label(action);
    }
}

#[test]
fn an_action_opens_its_own_screen_and_escape_returns_to_the_option() {
    let mut harness = app_harness();
    click(&mut harness, "09. Modificación de contratos");
    click(&mut harness, "Reversar");
    assert!(
        matches!(harness.state().route(), Route::Action(_, action) if action.label == "Reversar")
    );

    harness.key_press(Key::Escape);
    harness.run();

    assert_eq!(
        harness.state().route(),
        Route::Option(operation(9).unwrap())
    );
}

#[test]
fn maintenance_tab_shows_its_pending_options_message() {
    let mut harness = app_harness();

    click(&mut harness, "Mantenimiento");

    assert_eq!(
        harness.state().route(),
        Route::Section(SectionId::Maintenance)
    );
    harness.get_by_label("Opciones pendientes de levantamiento");
}

#[test]
fn reports_tab_shows_its_pending_options_message() {
    let mut harness = app_harness();

    click(&mut harness, "Informes");

    harness.get_by_label("Opciones pendientes de levantamiento");
}

#[test]
fn exit_asks_for_confirmation_and_cancel_keeps_the_app_open() {
    let mut harness = app_harness();

    harness.get_by_label_contains("Salir").click();
    harness.run();
    harness.get_by_label("Salir de la aplicación");
    click(&mut harness, "Cancelar");

    // La pantalla sigue visible y el diálogo ya no.
    harness.get_by_label("01. Ingresar contratos");
    assert!(harness.query_by_label("Salir de la aplicación").is_none());
}
