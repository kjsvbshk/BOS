//! Selección por número en menús de tarjetas.
//!
//! Un número **sólo selecciona** la tarjeta (le da el foco); Enter la abre. Así,
//! pulsar varias veces un número nunca abre una pantalla por accidente. Dos
//! cifras seguidas (en menos de [`TWO_DIGIT_WINDOW`] segundos) forman un número
//! de dos cifras: «1» y «1» seleccionan la opción 11.

use egui::{Id, Key, Modifiers, Ui};

/// Tiempo máximo entre las dos cifras de un número.
const TWO_DIGIT_WINDOW: f64 = 1.2;

const DIGITS: [(Key, u8); 10] = [
    (Key::Num0, 0),
    (Key::Num1, 1),
    (Key::Num2, 2),
    (Key::Num3, 3),
    (Key::Num4, 4),
    (Key::Num5, 5),
    (Key::Num6, 6),
    (Key::Num7, 7),
    (Key::Num8, 8),
    (Key::Num9, 9),
];

/// Número que el usuario acaba de seleccionar en el menú `menu`, si alguno.
/// `exists` indica qué números tienen tarjeta permitida; si lo escrito no
/// corresponde a ninguna, se quita la selección. No hace nada mientras se
/// escribe en un campo de texto.
pub fn number_selection(ui: &Ui, menu: &str, exists: impl Fn(u8) -> bool) -> Option<u8> {
    let ctx = ui.ctx();
    if ctx.text_edit_focused() {
        return None;
    }
    let digit = ctx.input_mut(|input| {
        DIGITS
            .iter()
            .find(|(key, _)| input.consume_key(Modifiers::NONE, *key))
            .map(|(_, d)| *d)
    })?;
    let now = ctx.input(|input| input.time);
    let id = Id::unique(("number_selection", menu));
    let previous: Option<(u8, f64)> = ctx.data(|data| data.get_temp(id));
    let combined = previous
        .filter(|(number, time)| *number < 10 && now - time < TWO_DIGIT_WINDOW)
        .map(|(number, _)| number * 10 + digit)
        .filter(|number| exists(*number));
    let number = combined.unwrap_or(digit);
    ctx.data_mut(|data| data.insert_temp(id, (number, now)));
    if exists(number) {
        return Some(number);
    }
    // El número no lleva a ninguna opción permitida: se quita la selección
    // anterior para que Enter no abra algo que el usuario no quería.
    ctx.memory_mut(|memory| {
        if let Some(focused) = memory.focused() {
            memory.surrender_focus(focused);
        }
    });
    None
}
