//! Operaciones capturadas por pasos: 01, 02, 03, 06, 07 y 08.

use chrono::NaiveDate;
use egui::{RichText, Ui};
use pos_core::{
    application::{NewContract as NewContractData, OpError, PosStore},
    domain::{
        CashDirection, ContractKind, ContractStatus, Item, Metal, Money, MovementKind, interest,
    },
};

use crate::{
    components::{Card, LookupItem, info_banner},
    theme::tokens::{color, space},
    views::{
        flow::{FlowCx, FlowSpec, LookupSource, StepDef, StepKind, Values},
        shared::{
            contract_card, customer_card, customer_history, demo_marker, fmt_date, left, right,
            table,
        },
    },
};

/// Paso «Número de contrato» con los contratos que admite la operación a la
/// vista (se filtran al escribir; F1 los muestra todos).
pub(super) fn contract_step(label: &str, items: Vec<LookupItem>) -> StepDef {
    StepDef::new(
        "contract",
        label,
        StepKind::Integer {
            min: 1,
            max: 99_999_999,
        },
    )
    .hint("Escriba el número o elíjalo de la lista. F1 busca por cliente o artículo.")
    .suggest("Contratos", items)
}

/// Contratos en alguno de los estados indicados, del más reciente al más antiguo.
pub(super) fn contract_items(cx: &FlowCx<'_>, statuses: &[ContractStatus]) -> Vec<LookupItem> {
    let mut views: Vec<_> = cx
        .store
        .all_views(cx.today)
        .filter(|v| statuses.contains(&v.status))
        .collect();
    views.sort_by_key(|v| std::cmp::Reverse(v.contract.number));
    views
        .into_iter()
        .map(|v| {
            let customer = cx
                .store
                .customer(&v.contract.customer)
                .map_or_else(String::new, |c| c.name.clone());
            LookupItem {
                value: v.contract.number.to_string(),
                label: format!("{customer} · {}", v.contract.item.description),
                detail: format!("{} · {}", v.capital, v.status.label()),
            }
        })
        .collect()
}

/// Movimientos no anulados que cumplen `keep`, del más reciente al más antiguo.
pub(super) fn movement_items(
    cx: &FlowCx<'_>,
    keep: impl Fn(&MovementKind) -> bool,
) -> Vec<LookupItem> {
    cx.store
        .movements()
        .iter()
        .rev()
        .filter(|m| !m.annulled && keep(&m.kind))
        .map(|m| {
            let subject = match (&m.kind, m.contract) {
                (MovementKind::Cash { description, .. }, _) => description.clone(),
                (_, Some(contract)) => format!("contrato {contract}"),
                _ => String::new(),
            };
            let amount = crate::views::shared::movement_amount(m)
                .map_or_else(String::new, |a| format!("{a} · "));
            LookupItem {
                value: m.number.to_string(),
                label: format!("{} · {subject}", m.kind.label()),
                detail: format!("{amount}{}", fmt_date(m.date)),
            }
        })
        .collect()
}

/// Comprueba que el contrato exista y esté en uno de los estados permitidos.
pub(super) fn check_contract(
    cx: &FlowCx<'_>,
    value: &str,
    allowed: &[ContractStatus],
    expected: &str,
) -> Result<(), String> {
    let number: u32 = value.parse().unwrap_or(0);
    let view = cx
        .store
        .view(number, cx.today)
        .ok_or_else(|| OpError::ContractNotFound(number).to_string())?;
    if allowed.contains(&view.status) {
        Ok(())
    } else {
        Err(format!(
            "El contrato {number} está en estado «{}»; se necesita un contrato {expected}.",
            view.status.label()
        ))
    }
}

/// Paso de fecha con hoy por defecto.
pub(super) fn date_step(id: &'static str, label: &str, today: NaiveDate) -> StepDef {
    StepDef::new(id, label, StepKind::Date)
        .hint("dd/mm/aaaa. Enter deja la fecha de hoy.")
        .default_value(today.format("%Y-%m-%d").to_string())
}

/// Fecha capturada, o hoy si todavía no hay.
pub(super) fn date_or_today(values: &Values, id: &str, today: NaiveDate) -> NaiveDate {
    values.date(id).unwrap_or(today)
}

/// Ficha del contrato capturado, valorada a `on`.
pub(super) fn contract_context(ui: &mut Ui, cx: &FlowCx<'_>, values: &Values, on: NaiveDate) {
    if let Some(view) = cx.store.view(values.number("contract"), on) {
        contract_card(ui, &view, cx.store);
    } else {
        info_banner(ui, "Escriba el número del contrato para ver su ficha.");
    }
}

fn not_after_today(cx: &FlowCx<'_>, value: &str) -> Result<(), String> {
    match NaiveDate::parse_from_str(value, "%Y-%m-%d") {
        Ok(date) if date > cx.today => Err(String::from("La fecha no puede ser posterior a hoy.")),
        _ => Ok(()),
    }
}

// ------------------------------------------------------------------ 01

/// 01 · Ingresar contratos al inventario de contratos vigentes.
pub struct NewContract;

impl NewContract {
    fn metal(cx: &FlowCx<'_>, values: &Values) -> Option<Metal> {
        values
            .get("code")
            .parse()
            .ok()
            .and_then(|code| cx.store.merchandise_code(code))
            .and_then(|m| m.metal)
    }
}

impl FlowSpec for NewContract {
    #[expect(
        clippy::too_many_lines,
        reason = "pantalla declarativa: dividirla no la aclara"
    )]
    fn steps(&self, cx: &FlowCx<'_>, values: &Values) -> Vec<StepDef> {
        let common = values.get("kind") != "N";
        let metal = Self::metal(cx, values);
        let mut steps = vec![
            StepDef::new(
                "kind",
                "Tipo de contrato",
                StepKind::Choice(vec![
                    ("C", "Común"),
                    ("N", "No común (hipotecario, vehículo…)"),
                ]),
            )
            .hint("Los no comunes reciben un número consecutivo del sistema.")
            .default_value("C"),
        ];
        if common {
            steps.push(
                StepDef::new(
                    "number",
                    "Número de contrato",
                    StepKind::Integer {
                        min: 1,
                        max: 99_999_999,
                    },
                )
                .hint("El número de su libreta."),
            );
        }
        steps.extend([
            StepDef::new(
                "term",
                "Plazo en meses",
                StepKind::Integer { min: 1, max: 60 },
            )
            .hint("Habituales: 1, 2, 3 o 6.")
            .default_value("6"),
            StepDef::new("rate", "Porcentaje mensual", StepKind::Rate)
                .hint("Por ejemplo 7 o 2,5.")
                .default_value("1000"),
            StepDef::new(
                "customer",
                "Cédula del cliente",
                StepKind::Lookup {
                    source: LookupSource::Customers,
                    optional: false,
                },
            )
            .hint("F1 para consultar el listado de clientes."),
            StepDef::new("capital", "Valor del empeño", StepKind::Money),
            StepDef::new(
                "code",
                "Código de mercancía",
                StepKind::Lookup {
                    source: LookupSource::Merchandise,
                    optional: false,
                },
            )
            .hint("F1 para consultar. 99 = artículo esporádico (algo que nunca se ha empeñado)."),
            StepDef::new(
                "description",
                "Descripción del artículo",
                StepKind::Text { optional: false },
            )
            .hint(if values.get("code") == "99" {
                "Describa el artículo esporádico, por ejemplo «PlayStation 5»."
            } else {
                "Por ejemplo «Anillo tipo Rolex»."
            }),
            StepDef::new(
                "condition",
                "Estado físico",
                StepKind::Text { optional: false },
            )
            .hint("Por ejemplo: bueno, regular, torcido, reventado.")
            .default_value("Bueno"),
            StepDef::new(
                "weight",
                "Peso en gramos",
                StepKind::Weight {
                    optional: metal.is_none(),
                },
            )
            .hint(if metal.is_some() {
                "Obligatorio en oro y plata."
            } else {
                "Opcional para este código."
            }),
        ]);
        if metal == Some(Metal::Gold) {
            steps.push(
                StepDef::new(
                    "karat",
                    "Quilataje",
                    StepKind::Lookup {
                        source: LookupSource::GoldQualities,
                        optional: false,
                    },
                )
                .hint("F1 para consultar."),
            );
        }
        steps.push(
            date_step("date", "Fecha del contrato", cx.today)
                .hint("Hoy o una fecha anterior (contratos ingresados después)."),
        );
        steps.push(
            StepDef::new("print", "¿Imprimir el contrato?", StepKind::YesNo)
                .hint("La impresión todavía no está disponible en la demo.")
                .default_value("N"),
        );
        steps
    }

    fn check(
        &self,
        cx: &FlowCx<'_>,
        _values: &Values,
        step: &'static str,
        value: &str,
    ) -> Result<(), String> {
        match step {
            "number" => {
                let number: u32 = value.parse().unwrap_or(0);
                if cx.store.contract(number).is_some() {
                    Err(OpError::ContractExists(number).to_string())
                } else {
                    Ok(())
                }
            }
            "date" => not_after_today(cx, value),
            _ => Ok(()),
        }
    }

    fn context(&self, ui: &mut Ui, cx: &FlowCx<'_>, values: &Values) {
        if values.get("kind") == "N" {
            info_banner(
                ui,
                &format!(
                    "El sistema asignará el número {}.",
                    cx.store.next_non_common_number()
                ),
            );
            ui.add_space(space::S);
        }
        if let Some(customer) = cx.store.customer(values.get("customer")) {
            customer_card(ui, customer);
            ui.add_space(space::S);
            customer_history(ui, &cx.store.contracts_of(&customer.id_number, cx.today));
        } else {
            info_banner(
                ui,
                "Al escribir la cédula verá los datos del cliente y su historial.",
            );
        }
    }

    fn review_extra(&self, cx: &FlowCx<'_>, values: &Values) -> Vec<(String, String)> {
        let capital = values.money("capital");
        let rate = values.rate("rate");
        let date = date_or_today(values, "date", cx.today);
        let mut lines = vec![
            (
                String::from("Interés mensual"),
                interest::monthly_interest(capital, rate).to_string(),
            ),
            (
                String::from("Vence"),
                fmt_date(interest::due_date(date, values.number("term"), 0)),
            ),
        ];
        if values.get("kind") == "N" {
            lines.insert(
                0,
                (
                    String::from("Número asignado"),
                    cx.store.next_non_common_number().to_string(),
                ),
            );
        }
        lines
    }

    fn commit(
        &self,
        store: &mut PosStore,
        today: NaiveDate,
        values: &Values,
    ) -> Result<String, OpError> {
        let kind = if values.get("kind") == "N" {
            ContractKind::NonCommon
        } else {
            ContractKind::Common
        };
        let (number, movement) = store.register_contract(NewContractData {
            number: (kind == ContractKind::Common).then(|| values.number("number")),
            kind,
            customer: values.get("customer").to_owned(),
            date: date_or_today(values, "date", today),
            term_months: values.number("term"),
            rate: values.rate("rate"),
            capital: values.money("capital"),
            item: Item {
                code: values.get("code").parse().unwrap_or(0),
                description: values.get("description").to_owned(),
                condition: values.get("condition").to_owned(),
                weight: values.weight("weight"),
                karat: values.get("karat").parse().ok(),
            },
        })?;
        Ok(format!(
            "Contrato {number} ingresado. Movimiento {movement}."
        ))
    }

    fn again_question(&self) -> &'static str {
        "¿Desea ingresar otro contrato?"
    }
}

// ------------------------------------------------------------------ 02

/// 02 · Prorrogar plazo (pago anticipado de meses de intereses).
pub struct Extension;

impl FlowSpec for Extension {
    fn steps(&self, cx: &FlowCx<'_>, values: &Values) -> Vec<StepDef> {
        let current_term = cx
            .store
            .contract(values.number("contract"))
            .map_or(0, |c| c.term_months);
        let mut steps = vec![
            contract_step(
                "Número de contrato",
                contract_items(cx, &[ContractStatus::Active, ContractStatus::Expired]),
            ),
            StepDef::new("change_term", "¿Modifica el plazo?", StepKind::YesNo)
                .hint(format!("Plazo actual: {current_term} meses."))
                .default_value("N"),
        ];
        if values.yes("change_term") {
            steps.push(StepDef::new(
                "new_term",
                "Nuevo plazo en meses",
                StepKind::Integer { min: 1, max: 60 },
            ));
        }
        let term = if values.yes("change_term") && values.has("new_term") {
            values.number("new_term")
        } else {
            current_term
        };
        steps.push(
            StepDef::new(
                "months",
                "Meses de intereses que paga",
                StepKind::Integer { min: 1, max: 24 },
            )
            .hint("Puede pagar menos meses que el plazo completo.")
            .default_value(term.max(1).to_string()),
        );
        steps.push(date_step("date", "Fecha del pago", cx.today));
        steps
    }

    fn check(
        &self,
        cx: &FlowCx<'_>,
        _values: &Values,
        step: &'static str,
        value: &str,
    ) -> Result<(), String> {
        match step {
            "contract" => check_contract(
                cx,
                value,
                &[ContractStatus::Active, ContractStatus::Expired],
                "vigente o vencido",
            ),
            "date" => not_after_today(cx, value),
            _ => Ok(()),
        }
    }

    fn context(&self, ui: &mut Ui, cx: &FlowCx<'_>, values: &Values) {
        if let Some(view) = cx.store.view(values.number("contract"), cx.today) {
            let term = view.contract.term_months;
            let full = interest::extension_amount(view.capital, view.contract.rate, term);
            info_banner(ui, &format!("Plazo completo: {term} meses = {full}."));
            ui.add_space(space::S);
        }
        contract_context(ui, cx, values, cx.today);
    }

    fn review_extra(&self, cx: &FlowCx<'_>, values: &Values) -> Vec<(String, String)> {
        let Some(view) = cx.store.view(values.number("contract"), cx.today) else {
            return Vec::new();
        };
        let months = values.number("months");
        let term = if values.yes("change_term") {
            values.number("new_term")
        } else {
            view.contract.term_months
        };
        let amount = interest::extension_amount(view.capital, view.contract.rate, months);
        let due = interest::due_date(view.contract.date, term, view.months_paid + months);
        vec![
            (String::from("Valor a pagar"), amount.to_string()),
            (String::from("Nuevo vencimiento"), fmt_date(due)),
        ]
    }

    fn commit(
        &self,
        store: &mut PosStore,
        today: NaiveDate,
        values: &Values,
    ) -> Result<String, OpError> {
        let number = values.number("contract");
        let date = date_or_today(values, "date", today);
        if values.yes("change_term") {
            store.change_term(number, values.number("new_term"), date)?;
        }
        let (movement, amount) = store.pay_interest(number, values.number("months"), date)?;
        Ok(format!(
            "Prórroga del contrato {number}: {} mes(es) por {amount}. Movimiento {movement}.",
            values.number("months")
        ))
    }

    fn again_question(&self) -> &'static str {
        "¿Desea registrar otra prórroga?"
    }
}

// ------------------------------------------------------------------ 03

/// 03 · Retiro de contrato – Devolución a dueño original.
pub struct Withdrawal;

impl FlowSpec for Withdrawal {
    fn steps(&self, cx: &FlowCx<'_>, values: &Values) -> Vec<StepDef> {
        let item = cx.store.contract(values.number("contract")).map(|c| {
            let weight = c.item.weight.map_or_else(String::new, |w| format!(", {w}"));
            format!("{}{weight}", c.item.description)
        });
        vec![
            contract_step(
                "Número del contrato a retirar",
                contract_items(cx, &[ContractStatus::Active, ContractStatus::Expired]),
            ),
            date_step("date", "Fecha del retiro", cx.today),
            StepDef::new(
                "verified",
                "¿El artículo corresponde al contrato?",
                StepKind::YesNo,
            )
            .hint(format!(
                "Verifique físicamente: {}.",
                item.unwrap_or_default()
            )),
        ]
    }

    fn check(
        &self,
        cx: &FlowCx<'_>,
        _values: &Values,
        step: &'static str,
        value: &str,
    ) -> Result<(), String> {
        match step {
            "contract" => check_contract(
                cx,
                value,
                &[ContractStatus::Active, ContractStatus::Expired],
                "vigente o vencido",
            ),
            "date" => not_after_today(cx, value),
            "verified" if value != "S" => Err(String::from(
                "Verifique el artículo antes de entregarlo. Responda S cuando corresponda.",
            )),
            _ => Ok(()),
        }
    }

    fn context(&self, ui: &mut Ui, cx: &FlowCx<'_>, values: &Values) {
        contract_context(ui, cx, values, date_or_today(values, "date", cx.today));
    }

    fn review_extra(&self, cx: &FlowCx<'_>, values: &Values) -> Vec<(String, String)> {
        let on = date_or_today(values, "date", cx.today);
        let Some(view) = cx.store.view(values.number("contract"), on) else {
            return Vec::new();
        };
        vec![
            (String::from("Capital"), view.capital.to_string()),
            (
                format!("Intereses ({} meses)", view.months_pending),
                (view.payoff - view.capital).to_string(),
            ),
            (String::from("Total a pagar"), view.payoff.to_string()),
        ]
    }

    fn commit(
        &self,
        store: &mut PosStore,
        today: NaiveDate,
        values: &Values,
    ) -> Result<String, OpError> {
        let number = values.number("contract");
        let (movement, total) = store.withdraw(number, date_or_today(values, "date", today))?;
        Ok(format!(
            "Contrato {number} retirado por {total}. Movimiento {movement}."
        ))
    }

    fn again_question(&self) -> &'static str {
        "¿Desea retirar otro contrato?"
    }
}

// ------------------------------------------------------------------ 06

/// 06 · Salida del inventario de mercancías propias.
pub struct OwnedExit;

impl FlowSpec for OwnedExit {
    fn steps(&self, cx: &FlowCx<'_>, values: &Values) -> Vec<StepDef> {
        let mut steps = vec![
            StepDef::new(
                "mode",
                "Tipo de salida",
                StepKind::Choice(vec![
                    ("V", "Venta directa"),
                    ("S", "Separado (venta a plazos)"),
                    ("A", "Abono a mercancía separada"),
                ]),
            )
            .default_value("V"),
            contract_step(
                "Número de contrato del artículo",
                if values.get("mode") == "A" {
                    contract_items(cx, &[ContractStatus::Layaway])
                } else {
                    contract_items(cx, &[ContractStatus::Owned])
                },
            ),
        ];
        match values.get("mode") {
            "S" => steps.extend([
                StepDef::new(
                    "buyer",
                    "Cédula de quien separa",
                    StepKind::Lookup {
                        source: LookupSource::Customers,
                        optional: false,
                    },
                )
                .hint("F1 para consultar."),
                StepDef::new("price", "Valor del separado", StepKind::Money),
                StepDef::new("delivery", "Fecha estimada de entrega", StepKind::Date)
                    .hint("dd/mm/aaaa"),
                date_step("date", "Fecha de hoy", cx.today),
            ]),
            "A" => steps.extend([
                StepDef::new("amount", "Valor del abono", StepKind::Money),
                date_step("date", "Fecha del abono", cx.today),
            ]),
            _ => steps.extend([
                StepDef::new(
                    "buyer",
                    "Cédula del comprador",
                    StepKind::Lookup {
                        source: LookupSource::Customers,
                        optional: true,
                    },
                )
                .hint("Opcional. F1 para consultar."),
                StepDef::new("price", "Valor de venta", StepKind::Money),
                date_step("date", "Fecha de la venta", cx.today),
            ]),
        }
        steps
    }

    fn check(
        &self,
        cx: &FlowCx<'_>,
        values: &Values,
        step: &'static str,
        value: &str,
    ) -> Result<(), String> {
        match step {
            "contract" if values.get("mode") == "A" => {
                check_contract(cx, value, &[ContractStatus::Layaway], "separado")
            }
            "contract" => {
                let number: u32 = value.parse().unwrap_or(0);
                match cx.store.view(number, cx.today) {
                    Some(view) if view.status == ContractStatus::Owned => Ok(()),
                    Some(_) => Err(format!(
                        "El {number} no está en venta: no es mercancía propia disponible."
                    )),
                    None => Err(OpError::ContractNotFound(number).to_string()),
                }
            }
            "delivery" => match NaiveDate::parse_from_str(value, "%Y-%m-%d") {
                Ok(date) if date < cx.today => Err(String::from(
                    "La fecha de entrega no puede ser anterior a hoy.",
                )),
                _ => Ok(()),
            },
            "date" => not_after_today(cx, value),
            _ => Ok(()),
        }
    }

    fn context(&self, ui: &mut Ui, cx: &FlowCx<'_>, values: &Values) {
        contract_context(ui, cx, values, date_or_today(values, "date", cx.today));
    }

    fn review_extra(&self, cx: &FlowCx<'_>, values: &Values) -> Vec<(String, String)> {
        let on = date_or_today(values, "date", cx.today);
        let Some(view) = cx.store.view(values.number("contract"), on) else {
            return Vec::new();
        };
        match values.get("mode") {
            "A" => {
                let balance = view.layaway.map_or_else(Default::default, |l| l.balance);
                let amount = values.money("amount");
                vec![
                    (String::from("Saldo actual"), balance.to_string()),
                    (
                        String::from("Saldo después del abono"),
                        (balance - amount).to_string(),
                    ),
                ]
            }
            "S" => vec![(
                String::from("Valor del separado"),
                values.money("price").to_string(),
            )],
            _ => {
                let price = values.money("price");
                let difference = interest::sale_difference(price, view.payoff);
                let label = if difference.is_negative() {
                    "Pérdida"
                } else {
                    "Utilidad"
                };
                vec![
                    (String::from("Valor a la fecha"), view.payoff.to_string()),
                    (String::from("Valor de venta"), price.to_string()),
                    (String::from(label), difference.abs().to_string()),
                ]
            }
        }
    }

    fn commit(
        &self,
        store: &mut PosStore,
        today: NaiveDate,
        values: &Values,
    ) -> Result<String, OpError> {
        let number = values.number("contract");
        let date = date_or_today(values, "date", today);
        let buyer = values.has("buyer").then(|| values.get("buyer").to_owned());
        match values.get("mode") {
            "S" => {
                let delivery = values.date("delivery").unwrap_or(date);
                let movement =
                    store.layaway(number, values.money("price"), buyer, delivery, date)?;
                Ok(format!(
                    "Artículo {number} separado. Movimiento {movement}."
                ))
            }
            "A" => {
                let (movement, balance) =
                    store.layaway_payment(number, values.money("amount"), date)?;
                Ok(format!(
                    "Abono registrado. Saldo pendiente: {balance}. Movimiento {movement}."
                ))
            }
            _ => {
                let (movement, _, difference) =
                    store.sell(number, values.money("price"), buyer, date)?;
                let result = if difference.is_negative() {
                    "pérdida"
                } else {
                    "utilidad"
                };
                Ok(format!(
                    "Venta del {number} grabada con {result} de {}. Movimiento {movement}.",
                    difference.abs()
                ))
            }
        }
    }

    fn again_question(&self) -> &'static str {
        "¿Desea registrar otra salida?"
    }
}

// ------------------------------------------------------------------ 07 / 08

/// 07 y 08 · Ingresos y/o egresos de caja o banco.
pub struct CashEntry {
    pub bank: bool,
}

impl FlowSpec for CashEntry {
    fn steps(&self, cx: &FlowCx<'_>, _values: &Values) -> Vec<StepDef> {
        vec![
            StepDef::new(
                "concept",
                "Código del concepto",
                StepKind::Lookup {
                    source: LookupSource::Concepts,
                    optional: false,
                },
            )
            .hint("F1 para consultar: gastos, utilidades, pérdidas y otros ingresos."),
            StepDef::new(
                "description",
                "Descripción",
                StepKind::Text { optional: false },
            )
            .hint("Por ejemplo «Arriendo de noviembre»."),
            StepDef::new("amount", "Valor", StepKind::Money),
            date_step("date", "Fecha", cx.today),
        ]
    }

    fn check(
        &self,
        cx: &FlowCx<'_>,
        _values: &Values,
        step: &'static str,
        value: &str,
    ) -> Result<(), String> {
        if step == "date" {
            not_after_today(cx, value)
        } else {
            Ok(())
        }
    }

    fn context(&self, ui: &mut Ui, cx: &FlowCx<'_>, values: &Values) {
        if self.bank {
            info_banner(
                ui,
                "El cliente indicó que nunca ha usado bancos; se conserva por fidelidad al sistema anterior.",
            );
            ui.add_space(space::S);
        }
        if let Some(concept) = values
            .get("concept")
            .parse()
            .ok()
            .and_then(|code| cx.store.concept(code))
        {
            ui.label(
                RichText::new(format!(
                    "{} · {}",
                    concept.category.label(),
                    concept.category.direction().label()
                ))
                .color(color::TEXT_MUTED),
            );
            ui.add_space(space::S);
        }
        Card::new()
            .title(if self.bank {
                "Últimos movimientos de banco"
            } else {
                "Últimos movimientos de caja"
            })
            .show(ui, |ui| {
                demo_marker(ui);
                let rows: Vec<Vec<String>> = cx
                    .store
                    .movements()
                    .iter()
                    .rev()
                    .filter(|m| {
                        matches!(m.kind, MovementKind::Cash { bank, .. } if bank == self.bank)
                            && !m.annulled
                    })
                    .take(8)
                    .map(|m| {
                        let (concept, direction, description, amount) = match &m.kind {
                            MovementKind::Cash {
                                concept,
                                direction,
                                description,
                                amount,
                                ..
                            } => (*concept, *direction, description.clone(), *amount),
                            _ => (0, CashDirection::Income, String::new(), Money::default()),
                        };
                        vec![
                            fmt_date(m.date),
                            concept.to_string(),
                            description,
                            direction.label().to_owned(),
                            amount.to_string(),
                        ]
                    })
                    .collect();
                table(
                    ui,
                    "recent_cash",
                    &[
                        left("Fecha"),
                        left("Código"),
                        left("Descripción"),
                        left("Tipo"),
                        right("Valor"),
                    ],
                    &rows,
                );
            });
    }

    fn review_extra(&self, cx: &FlowCx<'_>, values: &Values) -> Vec<(String, String)> {
        let direction = values
            .get("concept")
            .parse()
            .ok()
            .and_then(|code| cx.store.concept(code))
            .map(|c| c.category.direction());
        direction.map_or_else(Vec::new, |d| {
            vec![(String::from("Tipo de movimiento"), d.label().to_owned())]
        })
    }

    fn commit(
        &self,
        store: &mut PosStore,
        today: NaiveDate,
        values: &Values,
    ) -> Result<String, OpError> {
        let movement = store.cash_movement(
            values.get("concept").parse().unwrap_or(0),
            values.get("description").to_owned(),
            values.money("amount"),
            self.bank,
            date_or_today(values, "date", today),
        )?;
        Ok(format!("Movimiento {movement} grabado."))
    }

    fn again_question(&self) -> &'static str {
        "¿Desea registrar otro movimiento?"
    }
}
