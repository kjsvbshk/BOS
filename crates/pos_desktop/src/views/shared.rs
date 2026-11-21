//! Fichas y tablas compartidas por las pantallas de operaciones, consultas e
//! informes.

use chrono::NaiveDate;
use egui::{Align, Layout, RichText, Ui};
use pos_core::{
    application::{ContractView, PosStore},
    domain::{ContractStatus, Customer, Money, Movement, MovementKind},
};

use crate::{
    components::{BadgeTone, Card, StatusBadge},
    strings::common,
    theme::tokens::{color, font_size, space},
};

/// Fecha como dd/mm/aaaa.
#[must_use]
pub fn fmt_date(date: NaiveDate) -> String {
    date.format("%d/%m/%Y").to_string()
}

/// Indicador del estado de un contrato.
#[must_use]
pub fn status_badge(status: ContractStatus) -> StatusBadge {
    let tone = match status {
        ContractStatus::Active => BadgeTone::Success,
        ContractStatus::Expired => BadgeTone::Warning,
        ContractStatus::Annulled => BadgeTone::Error,
        ContractStatus::Layaway => BadgeTone::Pending,
        ContractStatus::Withdrawn | ContractStatus::Owned | ContractStatus::Sold => {
            BadgeTone::Neutral
        }
    };
    StatusBadge::new(tone, status.label())
}

/// Marca que recuerda que los datos son de demostración.
pub fn demo_marker(ui: &mut Ui) {
    ui.add(StatusBadge::new(BadgeTone::Pending, common::DEMO_DATA));
}

/// Tabla de dos columnas «dato: valor».
pub fn facts(ui: &mut Ui, id: &str, rows: &[(&str, String)]) {
    egui::Grid::new(id)
        .num_columns(2)
        .spacing([space::L, space::XS])
        .show(ui, |ui| {
            for (label, value) in rows {
                ui.label(RichText::new(*label).color(color::TEXT_MUTED));
                ui.label(RichText::new(value).color(color::TEXT));
                ui.end_row();
            }
        });
}

/// Columna de una tabla simple.
#[derive(Debug, Clone, Copy)]
pub struct Col {
    pub title: &'static str,
    /// Importes y cantidades se alinean a la derecha.
    pub right: bool,
}

/// Columna alineada a la izquierda.
#[must_use]
pub const fn left(title: &'static str) -> Col {
    Col {
        title,
        right: false,
    }
}

/// Columna alineada a la derecha.
#[must_use]
pub const fn right(title: &'static str) -> Col {
    Col { title, right: true }
}

/// Tabla con filas alternas para listados cortos e informes.
pub fn table(ui: &mut Ui, id: &str, columns: &[Col], rows: &[Vec<String>]) {
    if rows.is_empty() {
        ui.label(RichText::new(common::NO_MATCHES).color(color::TEXT_MUTED));
        return;
    }
    egui::Grid::new(id)
        .num_columns(columns.len())
        .striped(true)
        .spacing([space::L, space::S])
        .show(ui, |ui| {
            for column in columns {
                cell(
                    ui,
                    column.right,
                    RichText::new(column.title)
                        .size(font_size::SMALL + 1.0)
                        .color(color::TEXT_MUTED),
                );
            }
            ui.end_row();
            for row in rows {
                for (column, value) in columns.iter().zip(row) {
                    cell(ui, column.right, RichText::new(value).color(color::TEXT));
                }
                ui.end_row();
            }
        });
}

fn cell(ui: &mut Ui, right: bool, text: RichText) {
    if right {
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| ui.label(text));
    } else {
        ui.label(text);
    }
}

/// Ficha del cliente.
pub fn customer_card(ui: &mut Ui, customer: &Customer) {
    Card::new().title("Cliente").show(ui, |ui| {
        facts(
            ui,
            "customer_card",
            &[
                ("Nombre", customer.name.clone()),
                ("Cédula", customer.id_number.clone()),
                ("Dirección", customer.address.clone()),
                ("Teléfono", customer.phone.clone()),
            ],
        );
    });
}

/// Contratos de un cliente con su estado.
pub fn customer_history(ui: &mut Ui, views: &[ContractView]) {
    Card::new()
        .title("Historial de contratos del cliente")
        .show(ui, |ui| {
            let rows: Vec<Vec<String>> = views
                .iter()
                .map(|v| {
                    vec![
                        v.contract.number.to_string(),
                        fmt_date(v.contract.date),
                        v.contract.item.description.clone(),
                        v.capital.to_string(),
                        v.status.label().to_owned(),
                    ]
                })
                .collect();
            table(
                ui,
                "customer_history",
                &[
                    left("Contrato"),
                    left("Fecha"),
                    left("Artículo"),
                    right("Capital"),
                    left("Estado"),
                ],
                &rows,
            );
        });
}

/// Ficha completa de un contrato: cliente, condiciones, artículo, cifras e
/// historial (lo que muestra la consulta de la reunión, 10:29–14:19).
pub fn contract_card(ui: &mut Ui, view: &ContractView, store: &PosStore) {
    let contract = &view.contract;
    Card::new().show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label(
                RichText::new(format!("Contrato {}", contract.number))
                    .heading()
                    .color(color::TEXT),
            );
            ui.add(status_badge(view.status));
            ui.label(RichText::new(contract.kind.label()).color(color::TEXT_MUTED));
        });
        ui.add_space(space::XS);
        if let Some(customer) = store.customer(&contract.customer) {
            ui.label(
                RichText::new(format!(
                    "{} · C.C. {} · {}",
                    customer.name, customer.id_number, customer.phone
                ))
                .color(color::TEXT),
            );
        }
        ui.add_space(space::S);
        let code = store
            .merchandise_code(contract.item.code)
            .map_or_else(String::new, |m| format!("{} · {}", m.code, m.name));
        let mut item = vec![
            ("Código", code),
            ("Artículo", contract.item.description.clone()),
            ("Estado físico", contract.item.condition.clone()),
        ];
        if let Some(weight) = contract.item.weight {
            item.push(("Peso", weight.to_string()));
        }
        if let Some(karat) = contract.item.karat {
            item.push(("Quilataje", format!("{karat} quilates")));
        }
        ui.columns(2, |columns| {
            facts(
                &mut columns[0],
                "contract_terms",
                &[
                    ("Fecha de empeño", fmt_date(contract.date)),
                    ("Vence", fmt_date(view.due_date)),
                    ("Plazo", format!("{} meses", contract.term_months)),
                    ("Porcentaje", contract.rate.to_string()),
                    ("Capital inicial", contract.capital.to_string()),
                    ("Capital actual", view.capital.to_string()),
                ],
            );
            facts(&mut columns[1], "contract_item", &item);
        });
        ui.add_space(space::S);
        money_summary(ui, view);
        payments(ui, view);
    });
}

/// Cifras del contrato a la fecha de la consulta.
fn money_summary(ui: &mut Ui, view: &ContractView) {
    let mut rows = vec![
        ("Interés mensual", view.monthly_interest.to_string()),
        (
            "Meses causados / pagados",
            format!("{} / {}", view.months_accrued, view.months_paid),
        ),
        ("Intereses pagados", view.interest_paid().to_string()),
    ];
    match view.status {
        ContractStatus::Active | ContractStatus::Expired => {
            rows.push(("Meses pendientes", view.months_pending.to_string()));
            rows.push(("Valor de retiro a hoy", view.payoff.to_string()));
        }
        ContractStatus::Owned => rows.push(("Valor a la fecha", view.payoff.to_string())),
        _ => {}
    }
    if let Some((date, total)) = view.withdrawal {
        rows.push(("Retirado", format!("{} por {total}", fmt_date(date))));
    }
    if let Some(date) = view.owned_since {
        rows.push(("Pasó a propios", fmt_date(date)));
    }
    if let Some((date, price, reference)) = view.sale {
        rows.push((
            "Vendido",
            format!(
                "{} por {price} (valor a la fecha {reference})",
                fmt_date(date)
            ),
        ));
    }
    if let Some(layaway) = &view.layaway {
        rows.push((
            "Separado",
            format!("{} · entrega {}", layaway.price, fmt_date(layaway.delivery)),
        ));
        rows.push((
            "Abonado / saldo",
            format!("{} / {}", layaway.paid, layaway.balance),
        ));
    }
    facts(ui, "contract_money", &rows);
}

fn payments(ui: &mut Ui, view: &ContractView) {
    let mut rows: Vec<Vec<String>> = view
        .interest_payments
        .iter()
        .map(|p| {
            vec![
                p.movement.to_string(),
                fmt_date(p.date),
                format!("Intereses ({} mes(es))", p.months),
                p.amount.to_string(),
            ]
        })
        .chain(view.capital_payments.iter().map(|p| {
            vec![
                p.movement.to_string(),
                fmt_date(p.date),
                String::from("Abono a capital"),
                p.amount.to_string(),
            ]
        }))
        .chain(
            view.layaway
                .iter()
                .flat_map(|l| l.payments.iter())
                .map(|p| {
                    vec![
                        p.movement.to_string(),
                        fmt_date(p.date),
                        String::from("Abono a separado"),
                        p.amount.to_string(),
                    ]
                }),
        )
        .collect();
    if rows.is_empty() {
        ui.add_space(space::XS);
        ui.label(RichText::new("Sin pagos registrados.").color(color::TEXT_MUTED));
        return;
    }
    rows.sort_by(|a, b| a[0].cmp(&b[0]));
    ui.add_space(space::S);
    ui.label(RichText::new("Pagos").color(color::TEXT_MUTED));
    table(
        ui,
        "contract_payments",
        &[
            left("Movimiento"),
            left("Fecha"),
            left("Concepto"),
            right("Valor"),
        ],
        &rows,
    );
}

/// Detalle de un movimiento (para anulaciones y consultas).
pub fn movement_card(ui: &mut Ui, movement: &Movement, store: &PosStore) {
    Card::new().show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label(
                RichText::new(format!("Movimiento {}", movement.number))
                    .heading()
                    .color(color::TEXT),
            );
            if movement.annulled {
                ui.add(StatusBadge::new(BadgeTone::Error, "Anulado"));
            }
        });
        let mut rows = vec![
            ("Operación", movement.kind.label().to_owned()),
            ("Fecha", fmt_date(movement.date)),
            ("Usuario", movement.user.clone()),
        ];
        if let Some(contract) = movement.contract {
            rows.push(("Contrato", contract.to_string()));
        }
        if let Some(amount) = movement_amount(movement) {
            rows.push(("Valor", amount.to_string()));
        }
        if let MovementKind::Cash {
            concept,
            description,
            ..
        } = &movement.kind
        {
            let name = store
                .concept(*concept)
                .map_or_else(String::new, |c| c.name.clone());
            rows.push(("Concepto", format!("{concept} · {name}")));
            rows.push(("Descripción", description.clone()));
        }
        facts(ui, "movement_card", &rows);
    });
}

/// Importe principal de un movimiento.
#[must_use]
pub fn movement_amount(movement: &Movement) -> Option<Money> {
    match &movement.kind {
        MovementKind::InterestPayment { amount, .. }
        | MovementKind::CapitalPayment { amount }
        | MovementKind::LayawayPayment { amount }
        | MovementKind::Cash { amount, .. } => Some(*amount),
        MovementKind::Withdrawal { capital, interest } => Some(*capital + *interest),
        MovementKind::Sale { price, .. } | MovementKind::Layaway { price, .. } => Some(*price),
        MovementKind::Pawn | MovementKind::TransferToOwned => None,
    }
}
