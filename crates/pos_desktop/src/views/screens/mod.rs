//! Pantallas de las opciones y su registro por ruta.

mod changes;
mod expired;
mod maintenance;
mod operations;
mod queries;
mod reports;

use chrono::NaiveDate;
use egui::{Key, Modifiers, Ui};
use pos_core::application::PosStore;

use super::flow::FlowScreen;
use crate::navigation::Route;

/// Lo que pide una pantalla al terminar el frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Stay,
    /// Volver a la pantalla anterior (Esc en el primer paso, «No continuar»).
    Back,
}

/// Pantalla con estado propio dentro del área de contenido.
pub trait Screen {
    fn ui(&mut self, ui: &mut Ui, store: &mut PosStore, today: NaiveDate) -> Outcome;
}

/// Crea la pantalla de una ruta. `None` si la ruta no tiene pantalla propia
/// (secciones, submenús y opciones pendientes de especificación).
#[must_use]
pub fn create(route: Route) -> Option<Box<dyn Screen>> {
    match route {
        Route::Option(option) => match option.id {
            "operations.01" => Some(Box::new(FlowScreen::new(operations::NewContract))),
            "operations.02" => Some(Box::new(FlowScreen::new(operations::Extension))),
            "operations.03" => Some(Box::new(FlowScreen::new(operations::Withdrawal))),
            "operations.04" => Some(Box::new(expired::ExpiredScreen::default())),
            "operations.06" => Some(Box::new(FlowScreen::new(operations::OwnedExit))),
            "operations.07" => Some(Box::new(FlowScreen::new(operations::CashEntry {
                bank: false,
            }))),
            "operations.08" => Some(Box::new(FlowScreen::new(operations::CashEntry {
                bank: true,
            }))),
            "maintenance.01" => Some(Box::new(FlowScreen::new(maintenance::Customers))),
            "maintenance.02" => Some(Box::new(FlowScreen::new(maintenance::Users))),
            "maintenance.03" => Some(Box::new(FlowScreen::new(maintenance::Merchandise))),
            "maintenance.04" => Some(Box::new(FlowScreen::new(maintenance::GoldQualities))),
            "maintenance.05" => Some(Box::new(FlowScreen::new(maintenance::Concepts))),
            "maintenance.06" => Some(Box::new(FlowScreen::new(maintenance::Business))),
            "maintenance.07" => Some(Box::new(maintenance::Backup)),
            "reports.01" => Some(Box::new(reports::DailyCashScreen::default())),
            "reports.02" => Some(Box::new(reports::ExpensesScreen::default())),
            "reports.03" => Some(Box::new(reports::MonthlyScreen::default())),
            "reports.04" => Some(Box::new(reports::ActiveInventoryScreen)),
            "reports.05" => Some(Box::new(reports::OwnedInventoryScreen)),
            "reports.06" => Some(Box::new(reports::AuditScreen)),
            _ => None,
        },
        Route::Action(option, action) => match (option.id, action.id) {
            ("operations.09", "contract") => Some(Box::new(queries::ContractQuery::default())),
            ("operations.09", "customer") => Some(Box::new(queries::CustomerQuery::default())),
            ("operations.09", "movement") => Some(Box::new(queries::MovementQuery::default())),
            ("operations.10", "modify") => Some(Box::new(FlowScreen::new(changes::Modify))),
            ("operations.10", "capital") => {
                Some(Box::new(FlowScreen::new(changes::CapitalPayment)))
            }
            ("operations.10", id) => changes::Undo::for_action(id)
                .map(|spec| Box::new(FlowScreen::new(spec)) as Box<dyn Screen>),
            _ => None,
        },
        Route::Section(_) | Route::ComponentGallery => None,
    }
}

/// Esc pulsado y libre (sin diálogos ni menús abiertos). Lo consume.
#[must_use]
pub fn escape_pressed(ui: &Ui) -> bool {
    let ctx = ui.ctx();
    let busy = ctx.any_popup_open() || ctx.memory(|memory| memory.top_modal_layer().is_some());
    !busy && ctx.input_mut(|input| input.consume_key(Modifiers::NONE, Key::Escape))
}
