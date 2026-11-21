//! Diálogos modales: bloquean el resto de la ventana hasta que se cierran.

use egui::{Align, AsIdSalt, Context, Frame, Id, Layout, Margin, RichText, Ui};

use super::{PrimaryButton, SecondaryButton};
use crate::{
    strings::common,
    theme::tokens::{color, margin, size, space},
};

/// Resultado de mostrar un [`AppModal`] durante un frame.
#[derive(Debug)]
pub struct ModalOutcome<R> {
    /// Lo que devolvió el contenido.
    pub inner: R,
    /// `true` si el usuario pulsó Escape o hizo clic fuera del diálogo.
    pub dismissed: bool,
}

/// Ventana modal con título. Mientras el estado de la vista diga que está
/// abierta, hay que llamar a [`AppModal::show`] en cada frame.
#[derive(Debug, Clone, Copy)]
pub struct AppModal<'a> {
    id: Id,
    title: &'a str,
    width: f32,
}

impl<'a> AppModal<'a> {
    /// `id_source` debe ser único en toda la aplicación.
    #[must_use]
    pub fn new(id_source: impl AsIdSalt, title: &'a str) -> Self {
        Self {
            id: Id::unique(id_source),
            title,
            width: size::MODAL_WIDTH,
        }
    }

    /// Ancho del contenido del diálogo.
    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Dibuja el diálogo y su contenido.
    pub fn show<R>(
        self,
        ctx: &Context,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> ModalOutcome<R> {
        let frame = Frame::popup(&ctx.global_style()).inner_margin(Margin::same(margin::XL));
        let response = egui::Modal::new(self.id).frame(frame).show(ctx, |ui| {
            ui.set_width(self.width);
            ui.label(RichText::new(self.title).heading().color(color::TEXT));
            ui.add_space(space::S);
            add_contents(ui)
        });
        ModalOutcome {
            dismissed: response.should_close(),
            inner: response.inner,
        }
    }
}

/// Respuesta del usuario a un [`ConfirmDialog`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfirmChoice {
    Confirm,
    Cancel,
}

/// Diálogo que pide confirmar una acción. El botón de confirmación nombra la
/// acción («Eliminar elemento»), nunca «Aceptar» o «Sí».
///
/// Orden de botones de Windows: la acción a la izquierda y «Cancelar» a la
/// derecha. Escape o un clic fuera del diálogo equivalen a cancelar.
#[derive(Debug, Clone, Copy)]
pub struct ConfirmDialog<'a> {
    modal: AppModal<'a>,
    message: &'a str,
    confirm_label: &'a str,
}

impl<'a> ConfirmDialog<'a> {
    /// Crea el diálogo con su título, el mensaje y el texto de la acción.
    #[must_use]
    pub fn new(
        id_source: impl AsIdSalt,
        title: &'a str,
        message: &'a str,
        confirm_label: &'a str,
    ) -> Self {
        Self {
            modal: AppModal::new(id_source, title),
            message,
            confirm_label,
        }
    }

    /// Dibuja el diálogo. Devuelve la elección del usuario en el frame en que
    /// la toma; `None` mientras no haya decidido.
    #[must_use]
    pub fn show(self, ctx: &Context) -> Option<ConfirmChoice> {
        let outcome = self.modal.show(ctx, |ui| {
            ui.label(RichText::new(self.message).color(color::TEXT));
            ui.add_space(space::L);
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                // En un diseño de derecha a izquierda el primer botón queda a la derecha.
                let cancel = ui.add(SecondaryButton::new(common::CANCEL));
                let confirm = ui.add(PrimaryButton::new(self.confirm_label));
                if confirm.clicked() {
                    Some(ConfirmChoice::Confirm)
                } else if cancel.clicked() {
                    Some(ConfirmChoice::Cancel)
                } else {
                    None
                }
            })
            .inner
        });
        outcome
            .inner
            .or(outcome.dismissed.then_some(ConfirmChoice::Cancel))
    }
}
