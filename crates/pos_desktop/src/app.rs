//! Estructura principal de la aplicación: marco fijo y área de contenido.

use egui::{CentralPanel, Frame, Key, Margin, ScrollArea, Ui, ViewportCommand};

use crate::{
    components::{ConfirmChoice, ConfirmDialog},
    layout::{
        header::{self, HeaderAction},
        tabs::{self, TabsAction},
    },
    navigation::{Route, catalog},
    strings::navigation as text,
    theme::{self, tokens::color},
    views::{ComponentGallery, option_view, section_view},
};

/// Margen del área de contenido.
const CONTENT_MARGIN: Margin = Margin::symmetric(32, 24);

/// Aplicación de escritorio. Durante la demo sólo navega entre pantallas;
/// no ejecuta procesos del negocio.
#[derive(Debug, Default)]
pub struct PosApp {
    route: Route,
    confirm_exit: bool,
    gallery: ComponentGallery,
}

impl PosApp {
    /// Configura el tema y las fuentes una sola vez y crea el estado inicial.
    /// Sólo lee las fuentes de Windows; sirve también para las pruebas.
    #[must_use]
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        theme::apply(&cc.egui_ctx);
        Self::default()
    }

    /// Pantalla que se está mostrando.
    #[must_use]
    pub fn route(&self) -> Route {
        self.route
    }

    fn content_ui(&mut self, ui: &mut Ui) -> Option<Route> {
        match self.route {
            Route::Section(id) => section_view::show(ui, catalog::section(id)),
            Route::Option(option) => option_view::show_option(ui, option),
            Route::Action(option, action) => option_view::show_action(ui, option, action),
            Route::ComponentGallery => {
                self.gallery.ui(ui);
                None
            }
        }
    }

    fn exit_dialog_ui(&mut self, ui: &Ui) {
        if !self.confirm_exit {
            return;
        }
        let dialog =
            ConfirmDialog::new("exit_app", text::EXIT_TITLE, text::EXIT_MESSAGE, text::EXIT);
        match dialog.show(ui.ctx()) {
            Some(ConfirmChoice::Confirm) => {
                self.confirm_exit = false;
                ui.ctx().send_viewport_cmd(ViewportCommand::Close);
            }
            Some(ConfirmChoice::Cancel) => self.confirm_exit = false,
            None => {}
        }
    }

    /// Escape vuelve a la pantalla anterior, salvo que un diálogo o un menú lo
    /// esté usando para cerrarse.
    fn back_with_escape(&self, ui: &Ui) -> Option<Route> {
        let ctx = ui.ctx();
        let busy = self.confirm_exit
            || ctx.any_popup_open()
            || ctx.memory(|memory| memory.top_modal_layer().is_some());
        if busy || !ctx.input(|input| input.key_pressed(Key::Escape)) {
            return None;
        }
        self.route.parent()
    }
}

impl eframe::App for PosApp {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        let mut next = self.back_with_escape(ui);

        if let Some(HeaderAction::OpenComponentGallery) = header::show(ui) {
            next = Some(Route::ComponentGallery);
        }
        match tabs::show(ui, self.route) {
            Some(TabsAction::Navigate(route)) => next = Some(route),
            Some(TabsAction::RequestExit) => self.confirm_exit = true,
            None => {}
        }

        let content = CentralPanel::default()
            .frame(
                Frame::new()
                    .fill(color::SURFACE)
                    .inner_margin(CONTENT_MARGIN),
            )
            .show(ui, |ui| {
                ScrollArea::vertical()
                    .auto_shrink(false)
                    .id_salt(self.route)
                    .show(ui, |ui| self.content_ui(ui))
                    .inner
            })
            .inner;
        next = next.or(content);

        self.exit_dialog_ui(ui);

        if let Some(route) = next {
            self.route = route;
        }
    }
}
