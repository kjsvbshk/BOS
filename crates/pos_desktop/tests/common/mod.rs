//! Utilidades compartidas por las pruebas de interfaz.

#![allow(dead_code, reason = "cada archivo de pruebas usa una parte")]

use chrono::NaiveDate;
use egui::{Key, Role};
use egui_kittest::{Harness, kittest::Queryable};
use pos_desktop::{MIN_WINDOW_SIZE, PosApp};

/// Fecha de trabajo de las pruebas: el día de la reunión con el cliente.
const MEETING_DAY: NaiveDate = match NaiveDate::from_ymd_opt(2026, 11, 18) {
    Some(date) => date,
    None => panic!("fecha inválida"),
};

/// Fecha de trabajo de las pruebas.
pub fn meeting_day() -> NaiveDate {
    MEETING_DAY
}

/// Ventana alta para que el contenido quepa sin desplazarse.
pub const TALL_WINDOW: [f32; 2] = [MIN_WINDOW_SIZE[0], 1600.0];

/// Aplicación con la sesión abierta por `user` («PRO» o «ADM»).
pub fn app(user: &str) -> Harness<'static, PosApp> {
    let user = user.to_owned();
    let mut harness = Harness::builder()
        .with_size(egui::Vec2::from(TALL_WINDOW))
        .build_eframe(move |cc| PosApp::for_tests(cc, meeting_day(), &user));
    harness.run();
    harness
}

/// Hace clic en el elemento con esa etiqueta.
pub fn click(harness: &mut Harness<'_, PosApp>, label: &str) {
    harness.get_by_label(label).click();
    harness.run();
}

/// Escribe en el campo activo del flujo y pulsa Enter.
pub fn type_and_enter(harness: &mut Harness<'_, PosApp>, text: &str) {
    harness.get_by_role(Role::TextInput).type_text(text);
    harness.run();
    press(harness, Key::Enter);
}

/// Borra el contenido del campo activo (que viene con un valor propuesto).
pub fn clear_field(harness: &mut Harness<'_, PosApp>) {
    for _ in 0..12 {
        harness.key_press(Key::Backspace);
    }
    harness.run();
}

/// Pulsa una tecla y deja correr los frames.
pub fn press(harness: &mut Harness<'_, PosApp>, key: Key) {
    harness.key_press(key);
    harness.run();
}

/// Comprueba que algún texto visible contenga `text` (puede aparecer en el
/// resumen y en la ficha a la vez).
pub fn assert_shows(harness: &Harness<'_, PosApp>, text: &str) {
    assert!(
        harness.query_all_by_label_contains(text).next().is_some(),
        "no se muestra «{text}»"
    );
}

/// Abre la opción (o acción) con ese número como lo haría una persona: cada
/// cifra selecciona y Enter abre.
pub fn open(harness: &mut Harness<'_, PosApp>, number: &str) {
    for digit in number.chars() {
        let key = match digit {
            '0' => Key::Num0,
            '1' => Key::Num1,
            '2' => Key::Num2,
            '3' => Key::Num3,
            '4' => Key::Num4,
            '5' => Key::Num5,
            '6' => Key::Num6,
            '7' => Key::Num7,
            '8' => Key::Num8,
            _ => Key::Num9,
        };
        press(harness, key);
    }
    press(harness, Key::Enter);
}
