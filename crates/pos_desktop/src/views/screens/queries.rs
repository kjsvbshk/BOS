//! 09 · Consultas varias: contrato, cliente y movimiento.

use chrono::NaiveDate;
use egui::{Key, RichText, Ui};
use pos_core::{application::PosStore, domain::ContractStatus};

use super::{
    Outcome, Screen, escape_pressed,
    operations::{contract_items, movement_items},
};
use crate::{
    components::{
        FormField, LookupItem, LookupOutcome, LookupState, enter_value, filter_items, info_banner,
        key_hints, lookup_dialog, single_line, suggestion_list,
    },
    theme::tokens::{color, space},
    views::{
        flow::{FlowCx, LookupSource},
        shared::{contract_card, customer_card, customer_history, movement_card},
    },
};

/// Todos los estados: la consulta muestra cualquier contrato.
const ALL_STATUSES: [ContractStatus; 7] = [
    ContractStatus::Active,
    ContractStatus::Expired,
    ContractStatus::Withdrawn,
    ContractStatus::Owned,
    ContractStatus::Layaway,
    ContractStatus::Sold,
    ContractStatus::Annulled,
];

/// Lo que pidió el usuario en un campo de consulta.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum QueryEvent {
    /// Consultar este valor (escrito, sugerido o elegido con F1).
    Submit(String),
    Back,
}

/// Campo de consulta: lo escrito filtra la lista de sugerencias; ↑ ↓ eligen,
/// Enter consulta, F1 abre la lista completa y Esc vuelve.
#[derive(Debug, Default)]
pub(super) struct QueryField {
    pub input: String,
    highlight: Option<usize>,
    lookup: Option<LookupState>,
    started: bool,
}

impl QueryField {
    pub fn ui(
        &mut self,
        ui: &mut Ui,
        label: &str,
        title: &'static str,
        items: &[LookupItem],
    ) -> Option<QueryEvent> {
        let response = FormField::new(label).show(ui, |ui| {
            ui.add(single_line(&mut self.input, ("query", label)).desired_width(280.0))
        });
        if !self.started {
            self.started = true;
            response.request_focus();
        }
        if response.changed() {
            self.highlight = None;
        }
        let searchable = !items.is_empty();
        let mut hints = vec![("Enter", "Consultar"), ("Esc", "Volver")];
        if searchable {
            hints.push(("↑ ↓", "Elegir sugerencia"));
            hints.push(("F1", "Ver todos"));
        }
        key_hints(ui, &hints);

        let mut event = None;
        if searchable
            && let Some(value) = suggestion_list(
                ui,
                items,
                &self.input,
                &mut self.highlight,
                response.has_focus(),
            )
        {
            event = Some(QueryEvent::Submit(value));
        }
        if self.lookup.is_none() {
            let (enter, escape, f1) = ui.input(|i| {
                (
                    i.key_pressed(Key::Enter),
                    i.key_pressed(Key::Escape),
                    i.key_pressed(Key::F1),
                )
            });
            let free = response.lost_focus() || !response.has_focus();
            if enter && free {
                response.request_focus();
                let value = if searchable {
                    enter_value(items, &self.input, self.highlight)
                } else {
                    self.input.trim().to_owned()
                };
                event = Some(QueryEvent::Submit(value));
            } else if escape && free {
                event = Some(QueryEvent::Back);
            } else if f1 && searchable {
                self.lookup = Some(LookupState::default());
            }
        }
        if let Some(state) = self.lookup.as_mut() {
            match lookup_dialog(ui.ctx(), title, state, items) {
                LookupOutcome::Selected(value) => {
                    self.lookup = None;
                    event = Some(QueryEvent::Submit(value));
                }
                LookupOutcome::Cancelled => self.lookup = None,
                LookupOutcome::Open => {}
            }
        }
        if let Some(QueryEvent::Submit(value)) = &event {
            self.input.clone_from(value);
            self.highlight = None;
        }
        ui.add_space(space::M);
        event
    }
}

impl QueryField {
    /// Sugerencia seleccionada o valor escrito que coincide con un elemento:
    /// se muestra su ficha antes de confirmar.
    pub fn preview(&self, items: &[LookupItem]) -> Option<String> {
        let typed = self.input.trim();
        self.highlight
            .and_then(|index| {
                filter_items(items, &self.input)
                    .get(index)
                    .map(|item| item.value.clone())
            })
            .or_else(|| {
                items
                    .iter()
                    .find(|item| item.value == typed)
                    .map(|item| item.value.clone())
            })
    }
}

fn error_line(ui: &mut Ui, error: Option<&String>) {
    if let Some(error) = error {
        ui.label(RichText::new(format!("⚠ {error}")).color(color::ERROR));
        ui.add_space(space::S);
    }
}

/// Consulta de contrato por número, cliente o artículo.
#[derive(Debug, Default)]
pub struct ContractQuery {
    field: QueryField,
    shown: Option<u32>,
    error: Option<String>,
}

impl Screen for ContractQuery {
    fn ui(&mut self, ui: &mut Ui, store: &mut PosStore, today: NaiveDate) -> Outcome {
        let items = contract_items(&FlowCx { store, today }, &ALL_STATUSES);
        match self.field.ui(ui, "Número de contrato", "Contratos", &items) {
            Some(QueryEvent::Submit(value)) => match value.parse::<u32>() {
                Ok(number) if store.contract(number).is_some() => {
                    self.shown = Some(number);
                    self.error = None;
                }
                _ => self.error = Some(format!("No existe el contrato «{value}».")),
            },
            Some(QueryEvent::Back) => return Outcome::Back,
            None => {}
        }
        error_line(ui, self.error.as_ref());
        let preview = self.field.preview(&items).and_then(|v| v.parse().ok());
        match preview.or(self.shown).and_then(|n| store.view(n, today)) {
            Some(view) => contract_card(ui, &view, store),
            None => info_banner(
                ui,
                "Escriba el número o el nombre del cliente, o elija un contrato de la lista.",
            ),
        }
        Outcome::Stay
    }
}

/// Consulta por cédula del cliente.
#[derive(Debug, Default)]
pub struct CustomerQuery {
    field: QueryField,
    shown: Option<String>,
    error: Option<String>,
}

impl Screen for CustomerQuery {
    fn ui(&mut self, ui: &mut Ui, store: &mut PosStore, today: NaiveDate) -> Outcome {
        let items = LookupSource::Customers.items_for(store);
        match self.field.ui(ui, "Cédula del cliente", "Clientes", &items) {
            Some(QueryEvent::Submit(value)) => {
                if store.customer(&value).is_some() {
                    self.shown = Some(value);
                    self.error = None;
                } else {
                    self.error = Some(format!("No hay un cliente con la cédula «{value}»."));
                }
            }
            Some(QueryEvent::Back) => return Outcome::Back,
            None => {}
        }
        error_line(ui, self.error.as_ref());
        let preview = self.field.preview(&items);
        match preview
            .as_deref()
            .or(self.shown.as_deref())
            .and_then(|id| store.customer(id))
        {
            Some(customer) => {
                customer_card(ui, customer);
                ui.add_space(space::S);
                customer_history(ui, &store.contracts_of(&customer.id_number, today));
            }
            None => info_banner(
                ui,
                "Escriba la cédula o el nombre, o elija un cliente de la lista.",
            ),
        }
        Outcome::Stay
    }
}

/// Consulta de un movimiento por su número.
#[derive(Debug, Default)]
pub struct MovementQuery {
    field: QueryField,
    shown: Option<u32>,
    error: Option<String>,
}

impl Screen for MovementQuery {
    fn ui(&mut self, ui: &mut Ui, store: &mut PosStore, today: NaiveDate) -> Outcome {
        let items = movement_items(&FlowCx { store, today }, |_| true);
        match self
            .field
            .ui(ui, "Número de movimiento", "Movimientos", &items)
        {
            Some(QueryEvent::Submit(value)) => match value.parse::<u32>() {
                Ok(number) if store.movement(number).is_some() => {
                    self.shown = Some(number);
                    self.error = None;
                }
                _ => self.error = Some(format!("No existe el movimiento «{value}».")),
            },
            Some(QueryEvent::Back) => return Outcome::Back,
            None => {}
        }
        error_line(ui, self.error.as_ref());
        let preview = self.field.preview(&items).and_then(|v| v.parse().ok());
        let Some(movement) = preview
            .or(self.shown)
            .and_then(|n| store.movement(n))
            .cloned()
        else {
            info_banner(ui, "Escriba el número o elija un movimiento de la lista.");
            return Outcome::Stay;
        };
        movement_card(ui, &movement, store);
        if let Some(view) = movement.contract.and_then(|n| store.view(n, today)) {
            ui.add_space(space::S);
            contract_card(ui, &view, store);
        }
        if escape_pressed(ui) {
            return Outcome::Back;
        }
        Outcome::Stay
    }
}
