//! Inicio de sesión simulado: se elige el usuario (y con él su perfil).

use egui::{Key, Modifiers, Ui};
use pos_core::application::PosStore;

use super::keyboard::number_selection;
use crate::{
    components::{OptionCard, PageHeader, key_hints},
    strings::sign_in as text,
    theme::{icons, tokens::space},
};

/// Ancho de cada tarjeta de usuario.
const CARD_WIDTH: f32 = 420.0;

/// Dibuja la lista de usuarios. Devuelve el código elegido.
///
/// ↑ ↓ o un número seleccionan un usuario; Enter (o un clic) entra.
pub fn show(ui: &mut Ui, store: &PosStore, highlighted: &mut usize) -> Option<String> {
    PageHeader::new(icons::USER_SWITCH, text::TITLE)
        .subtitle(text::SUBTITLE)
        .show(ui);
    let users = store.users();

    let mut target = number_selection(ui, "sign_in", |n| {
        (1..=users.len()).contains(&usize::from(n))
    })
    .map(|n| usize::from(n) - 1);
    let (down, up) = ui.input_mut(|i| {
        (
            i.consume_key(Modifiers::NONE, Key::ArrowDown),
            i.consume_key(Modifiers::NONE, Key::ArrowUp),
        )
    });
    if down {
        target = Some((*highlighted + 1).min(users.len().saturating_sub(1)));
    }
    if up {
        target = Some(highlighted.saturating_sub(1));
    }
    let nothing_focused = ui.memory(|m| m.focused().is_none());

    let mut chosen = None;
    for (index, user) in users.iter().enumerate() {
        let title = format!("{}. {} · {}", index + 1, user.name, user.code);
        let card = OptionCard::new(title, user.profile.label(), icons::USER_CIRCLE, CARD_WIDTH);
        let response = ui.add(card);
        if target == Some(index) || (target.is_none() && nothing_focused && index == *highlighted) {
            response.request_focus();
        }
        if response.has_focus() {
            *highlighted = index;
        }
        if response.clicked() {
            chosen = Some(user.code.clone());
        }
        ui.add_space(space::S);
    }
    key_hints(ui, &[("↑ ↓ / 1–9", text::MOVE), ("Enter", text::ENTER)]);
    chosen
}
