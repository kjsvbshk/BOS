//! Selector de un valor dentro de un conjunto fijo de opciones.

use egui::{AsIdSalt, ComboBox, Response, Ui};

/// Lista desplegable. La respuesta indica `changed()` cuando el usuario elige
/// otra opción.
///
/// `options` asocia cada valor con su texto visible. Si `current` no está entre
/// las opciones, el selector se muestra vacío.
pub fn selector<T: PartialEq + Clone>(
    ui: &mut Ui,
    id_salt: impl AsIdSalt,
    current: &mut T,
    options: &[(T, &str)],
    width: f32,
) -> Response {
    let selected_text = options
        .iter()
        .find(|(value, _)| value == current)
        .map_or("", |(_, label)| *label);

    let mut changed = false;
    let mut response = ComboBox::from_id_salt(id_salt)
        .selected_text(selected_text)
        .width(width)
        .show_ui(ui, |ui| {
            for (value, label) in options {
                if ui
                    .selectable_value(current, value.clone(), *label)
                    .changed()
                {
                    changed = true;
                }
            }
        })
        .response;

    if changed {
        response.mark_changed();
    }
    response
}
