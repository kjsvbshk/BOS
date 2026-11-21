//! 04 · Contratos vencidos – Pasar al inventario de mercancía propia.

use chrono::NaiveDate;
use egui::{Key, Modifiers, RichText, Ui};
use pos_core::{
    application::{ContractView, ExpiredFilter, InterestFilter, PosStore},
    domain::Money,
};

use super::{Outcome, Screen, escape_pressed};
use crate::{
    components::{
        Card, ColumnSpec, ColumnWidth, ConfirmChoice, ConfirmDialog, DataTable, EmptyState,
        FormField, InputMask, MaskedText, PrimaryButton, SecondaryButton, key_hints, selector,
        single_line,
    },
    theme::tokens::{color, space},
    views::shared::{demo_marker, facts, fmt_date},
};

/// Estado de la pantalla de vencidos.
#[derive(Debug)]
pub struct ExpiredScreen {
    code: String,
    both_kinds: bool,
    interest: InterestFilter,
    selected: Option<u32>,
    confirm: Option<Vec<u32>>,
    message: Option<String>,
}

impl Default for ExpiredScreen {
    fn default() -> Self {
        Self {
            code: String::from("0"),
            both_kinds: true,
            interest: InterestFilter::Any,
            selected: None,
            confirm: None,
            message: None,
        }
    }
}

const KIND_OPTIONS: [(bool, &str); 2] = [(true, "Comunes y no comunes"), (false, "Sólo comunes")];
const INTEREST_OPTIONS: [(InterestFilter, &str); 3] = [
    (InterestFilter::Any, "Con y sin pagos de intereses"),
    (InterestFilter::Paid, "Con pagos de intereses"),
    (InterestFilter::Unpaid, "Sin pagos de intereses"),
];

impl ExpiredScreen {
    fn filter(&self) -> ExpiredFilter {
        ExpiredFilter {
            code: self.code.trim().parse().unwrap_or(0),
            both_kinds: self.both_kinds,
            interest: self.interest,
        }
    }

    fn filters_ui(&mut self, ui: &mut Ui) {
        Card::new().title("Filtros").show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing.x = space::XL;
                FormField::new("Código de mercancía (0 = todos)").show(ui, |ui| {
                    ui.add(
                        single_line(
                            &mut MaskedText::new(&mut self.code, InputMask::Digits { max_len: 3 }),
                            "expired_code",
                        )
                        .desired_width(90.0),
                    )
                });
                FormField::new("Tipo de contrato").show(ui, |ui| {
                    selector(
                        ui,
                        "expired_kind",
                        &mut self.both_kinds,
                        &KIND_OPTIONS,
                        220.0,
                    )
                });
                FormField::new("Intereses").show(ui, |ui| {
                    selector(
                        ui,
                        "expired_interest",
                        &mut self.interest,
                        &INTEREST_OPTIONS,
                        260.0,
                    )
                });
            });
        });
    }

    fn table_ui(&mut self, ui: &mut Ui, views: &[ContractView], store: &PosStore) {
        let columns = [
            ColumnSpec::new("Contrato", ColumnWidth::Initial(90.0)),
            ColumnSpec::new("Cliente", ColumnWidth::Initial(180.0)),
            ColumnSpec::new("Artículo", ColumnWidth::Initial(220.0)),
            ColumnSpec::new("Venció", ColumnWidth::Initial(100.0)),
            ColumnSpec::new("Capital", ColumnWidth::Initial(110.0)),
            ColumnSpec::new("Adeuda a hoy", ColumnWidth::Initial(120.0)),
            ColumnSpec::new("Pagos", ColumnWidth::Fill),
        ];
        let selected = self
            .selected
            .and_then(|n| views.iter().position(|v| v.contract.number == n));
        let empty = EmptyState::new("No hay contratos vencidos con estos filtros");
        let outcome = DataTable::new("expired_table", &columns, views.len(), empty)
            .selected(selected)
            .max_height(320.0)
            .show(ui, |row, column, ui| {
                let view = &views[row];
                let customer = store
                    .customer(&view.contract.customer)
                    .map_or_else(String::new, |c| c.name.clone());
                let text = match column {
                    0 => view.contract.number.to_string(),
                    1 => customer,
                    2 => view.contract.item.description.clone(),
                    3 => fmt_date(view.due_date),
                    4 => view.capital.to_string(),
                    5 => view.payoff.to_string(),
                    _ => format!(
                        "{} mes(es) de intereses · {} abono(s)",
                        view.months_paid,
                        view.capital_payments.len()
                    ),
                };
                ui.label(RichText::new(text).color(color::TEXT));
            });
        if let Some(row) = outcome.clicked_row {
            self.selected = views.get(row).map(|v| v.contract.number);
        }
    }
}

impl Screen for ExpiredScreen {
    #[expect(
        clippy::too_many_lines,
        reason = "pantalla declarativa: dividirla no la aclara"
    )]
    fn ui(&mut self, ui: &mut Ui, store: &mut PosStore, today: NaiveDate) -> Outcome {
        self.filters_ui(ui);
        ui.add_space(space::M);
        let views = store.expired(self.filter(), today);
        if self
            .selected
            .is_none_or(|n| !views.iter().any(|v| v.contract.number == n))
        {
            self.selected = views.first().map(|v| v.contract.number);
        }

        let capital: Money = views.iter().map(|v| v.capital).sum();
        let owed: Money = views.iter().map(|v| v.payoff).sum();
        ui.horizontal(|ui| {
            demo_marker(ui);
            facts(
                ui,
                "expired_totals",
                &[
                    ("Contratos", views.len().to_string()),
                    ("Capital prestado", capital.to_string()),
                    ("Valor adeudado", owed.to_string()),
                ],
            );
        });
        ui.add_space(space::S);
        self.table_ui(ui, &views, store);
        ui.add_space(space::S);

        let mut transfer_one = false;
        let mut transfer_all = false;
        ui.horizontal(|ui| {
            let selected = self.selected.is_some();
            transfer_one = ui
                .add_enabled(selected, PrimaryButton::new("Pasar a propios"))
                .on_disabled_hover_text("Seleccione un contrato.")
                .clicked();
            transfer_all = ui
                .add_enabled(
                    !views.is_empty(),
                    SecondaryButton::new("Pasar todos los listados (masivo)"),
                )
                .on_disabled_hover_text("No hay contratos listados.")
                .clicked();
        });
        key_hints(
            ui,
            &[
                ("↑ ↓", "Seleccionar"),
                ("Enter", "Pasar a propios"),
                ("Esc", "Volver"),
            ],
        );
        if let Some(message) = &self.message {
            ui.add_space(space::S);
            ui.label(RichText::new(message).color(color::SUCCESS));
        }

        let typing = ui.ctx().text_edit_focused();
        if self.confirm.is_none() && !typing {
            let (down, up, enter) = ui.input_mut(|i| {
                (
                    i.consume_key(Modifiers::NONE, Key::ArrowDown),
                    i.consume_key(Modifiers::NONE, Key::ArrowUp),
                    i.consume_key(Modifiers::NONE, Key::Enter),
                )
            });
            let index = self
                .selected
                .and_then(|n| views.iter().position(|v| v.contract.number == n))
                .unwrap_or(0);
            if down && index + 1 < views.len() {
                self.selected = Some(views[index + 1].contract.number);
            }
            if up && index > 0 {
                self.selected = Some(views[index - 1].contract.number);
            }
            transfer_one |= enter && self.selected.is_some();
        }
        if transfer_one {
            self.confirm = self.selected.map(|n| vec![n]);
        } else if transfer_all {
            self.confirm = Some(views.iter().map(|v| v.contract.number).collect());
        }

        if let Some(numbers) = self.confirm.clone() {
            let list = numbers
                .iter()
                .map(u32::to_string)
                .collect::<Vec<_>>()
                .join(", ");
            let message = format!(
                "Pasarán {} contrato(s) a mercancía propia y dejarán de figurar en vigentes: {list}.",
                numbers.len()
            );
            match ConfirmDialog::new(
                "confirm_transfer",
                "Pasar a mercancía propia",
                &message,
                "Pasar a propios",
            )
            .show(ui.ctx())
            {
                Some(ConfirmChoice::Confirm) => {
                    let done = numbers
                        .iter()
                        .filter(|n| store.transfer_to_owned(**n, today).is_ok())
                        .count();
                    self.message = Some(format!("{done} contrato(s) pasaron a mercancía propia."));
                    self.confirm = None;
                }
                Some(ConfirmChoice::Cancel) => self.confirm = None,
                None => {}
            }
        } else if escape_pressed(ui) {
            return Outcome::Back;
        }
        Outcome::Stay
    }
}
