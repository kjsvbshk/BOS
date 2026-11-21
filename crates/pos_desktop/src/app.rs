//! Estructura principal de la aplicación: marco fijo y área de contenido.

use chrono::{Local, NaiveDate};
use egui::{CentralPanel, Frame, Margin, ScrollArea, Ui, ViewportCommand};
use pos_core::{
    application::PosStore,
    demo,
    domain::{Profile, User},
};

use crate::{
    components::{ConfirmChoice, ConfirmDialog},
    layout::{
        header::{self, HeaderAction},
        tabs::{self, TabsAction},
    },
    navigation::{Route, catalog},
    strings::navigation as text,
    theme::{self, tokens::color},
    views::{
        ComponentGallery, option_view,
        screens::{self, Screen, escape_pressed},
        section_view, sign_in,
    },
};

/// Margen del área de contenido.
const CONTENT_MARGIN: Margin = Margin::symmetric(32, 24);

/// Aplicación de escritorio. Los datos son de demostración y viven en memoria
/// (ADR-0008): las operaciones los modifican hasta cerrar la aplicación.
pub struct PosApp {
    store: PosStore,
    /// Fecha de trabajo fija (pruebas); `None` usa la fecha del equipo.
    fixed_date: Option<NaiveDate>,
    signed_in: bool,
    sign_in_choice: usize,
    route: Route,
    screen: Option<Box<dyn Screen>>,
    confirm_exit: bool,
    gallery: ComponentGallery,
}

impl PosApp {
    /// Configura el tema y crea el estado inicial con los datos de demostración.
    #[must_use]
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        theme::apply(&cc.egui_ctx);
        Self::with_store(demo_store(), None)
    }

    /// Igual que [`Self::new`], con la fecha de trabajo fija y la sesión abierta
    /// con el usuario indicado. Para pruebas y capturas reproducibles.
    #[must_use]
    pub fn for_tests(cc: &eframe::CreationContext<'_>, date: NaiveDate, user: &str) -> Self {
        theme::apply(&cc.egui_ctx);
        let mut app = Self::with_store(demo_store(), Some(date));
        app.signed_in = app.store.sign_in(user).is_ok();
        app
    }

    fn with_store(store: PosStore, fixed_date: Option<NaiveDate>) -> Self {
        Self {
            store,
            fixed_date,
            signed_in: false,
            sign_in_choice: 0,
            route: Route::default(),
            screen: None,
            confirm_exit: false,
            gallery: ComponentGallery::default(),
        }
    }

    /// Pantalla que se está mostrando.
    #[must_use]
    pub fn route(&self) -> Route {
        self.route
    }

    /// Datos de la aplicación (para pruebas).
    #[must_use]
    pub fn store(&self) -> &PosStore {
        &self.store
    }

    /// `true` si hay una sesión abierta.
    #[must_use]
    pub fn signed_in(&self) -> bool {
        self.signed_in
    }

    fn today(&self) -> NaiveDate {
        self.fixed_date.unwrap_or_else(|| Local::now().date_naive())
    }

    fn navigate(&mut self, route: Route) {
        if route != self.route {
            self.route = route;
            self.screen = screens::create(route);
        }
    }

    fn content_ui(&mut self, ui: &mut Ui) -> Option<Route> {
        let today = self.today();
        match self.route {
            Route::Section(id) => section_view::show(ui, catalog::section(id), &self.store),
            Route::Option(option) => {
                option_view::show_option(ui, option, self.screen.as_mut(), &mut self.store, today)
            }
            Route::Action(option, action) => option_view::show_action(
                ui,
                option,
                action,
                self.screen.as_mut(),
                &mut self.store,
                today,
            ),
            Route::ComponentGallery => {
                self.gallery.ui(ui);
                escape_pressed(ui).then(Route::default)
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

    fn sign_in_ui(&mut self, ui: &mut Ui) {
        CentralPanel::default()
            .frame(
                Frame::new()
                    .fill(color::SURFACE)
                    .inner_margin(CONTENT_MARGIN),
            )
            .show(ui, |ui| {
                if let Some(code) = sign_in::show(ui, &self.store, &mut self.sign_in_choice)
                    && self.store.sign_in(&code).is_ok()
                {
                    self.signed_in = true;
                    self.route = Route::default();
                    self.screen = None;
                }
            });
    }
}

/// Almacén de la demostración. Si los datos de ejemplo fallaran (no debería:
/// lo comprueban las pruebas de `pos_core`), arranca vacío.
fn demo_store() -> PosStore {
    demo::seed().unwrap_or_else(|_| {
        let owner = User {
            code: demo::OWNER_CODE.into(),
            name: String::from("Propietario"),
            profile: Profile::Owner,
        };
        PosStore::new(demo::BUSINESS_NAME, owner)
    })
}

impl eframe::App for PosApp {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        match header::show(ui, &self.store, self.signed_in) {
            Some(HeaderAction::OpenComponentGallery) => self.navigate(Route::ComponentGallery),
            Some(HeaderAction::SignOut) => self.signed_in = false,
            None => {}
        }
        if !self.signed_in {
            self.sign_in_ui(ui);
            return;
        }

        let mut next = None;
        match tabs::show(ui, self.route) {
            Some(TabsAction::Navigate(route)) => next = Some(route),
            Some(TabsAction::RequestExit) => self.confirm_exit = true,
            None => {}
        }

        let route = self.route;
        let content = CentralPanel::default()
            .frame(
                Frame::new()
                    .fill(color::SURFACE)
                    .inner_margin(CONTENT_MARGIN),
            )
            .show(ui, |ui| {
                ScrollArea::vertical()
                    .auto_shrink(false)
                    .id_salt(route)
                    .show(ui, |ui| self.content_ui(ui))
                    .inner
            })
            .inner;
        next = next.or(content);

        self.exit_dialog_ui(ui);

        if let Some(route) = next {
            self.navigate(route);
        }
    }
}
