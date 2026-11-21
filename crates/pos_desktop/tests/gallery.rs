//! Pruebas del catálogo de componentes (pantalla interna de desarrollo). Se
//! abre como lo haría una persona: desde el menú del usuario.

mod common;

use common::{click, meeting_day};
use egui_kittest::{Harness, kittest::Queryable};
use pos_desktop::{PosApp, navigation::Route};

fn gallery_harness() -> Harness<'static, PosApp> {
    let mut harness = Harness::builder()
        .with_size(egui::vec2(1280.0, 2400.0))
        .build_eframe(|cc| PosApp::for_tests(cc, meeting_day(), "PRO"));
    harness.run();
    harness.get_by_label_contains("Propietario (demo)").click();
    harness.run();
    click(&mut harness, "Catálogo de componentes (desarrollo)");
    assert_eq!(harness.state().route(), Route::ComponentGallery);
    harness
}

#[test]
fn deleting_the_selected_row_asks_for_confirmation_first() {
    let mut harness = gallery_harness();

    click(&mut harness, "EJ-001");
    click(&mut harness, "Eliminar seleccionado");

    harness.get_by_label("Eliminar elemento");
    harness.get_by_label("EJ-001");
}

#[test]
fn confirming_the_dialog_removes_the_row() {
    let mut harness = gallery_harness();

    click(&mut harness, "EJ-001");
    click(&mut harness, "Eliminar seleccionado");
    click(&mut harness, "Eliminar elemento");

    harness.get_by_label("EJ-002");
    assert!(
        harness.query_by_label("EJ-001").is_none(),
        "EJ-001 debería haberse eliminado"
    );
}
