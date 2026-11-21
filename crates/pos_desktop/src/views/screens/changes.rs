//! 10 · Modificar, reversar, anular y borrar.

use chrono::NaiveDate;
use egui::Ui;
use pos_core::{
    application::{ContractChange, OpError, PosStore},
    domain::{ContractStatus, Money, MovementKind, Rate, interest},
};

use super::operations::{
    check_contract, contract_context, contract_items, contract_step, date_or_today, date_step,
    movement_items,
};
use crate::views::{
    flow::{FlowCx, FlowSpec, StepDef, StepKind, Values},
    shared::{fmt_date, movement_card},
};

const AGAIN: &str = "¿Desea realizar otra corrección?";
const NOT_ANNULLED: [ContractStatus; 6] = [
    ContractStatus::Active,
    ContractStatus::Expired,
    ContractStatus::Withdrawn,
    ContractStatus::Owned,
    ContractStatus::Layaway,
    ContractStatus::Sold,
];

// ------------------------------------------------------------------ Modificar

/// Modificar contrato: corrige un dato y lo deja en la auditoría.
pub struct Modify;

impl Modify {
    fn current(cx: &FlowCx<'_>, values: &Values) -> Option<(String, String)> {
        let contract = cx.store.contract(values.number("contract"))?;
        Some(match values.get("field") {
            "capital" => (
                contract.capital.as_pesos().to_string(),
                contract.capital.to_string(),
            ),
            "rate" => (
                contract.rate.hundredths().to_string(),
                contract.rate.to_string(),
            ),
            "term" => (
                contract.term_months.to_string(),
                format!("{} meses", contract.term_months),
            ),
            "date" => (
                contract.date.format("%Y-%m-%d").to_string(),
                fmt_date(contract.date),
            ),
            _ => (
                contract.item.description.clone(),
                contract.item.description.clone(),
            ),
        })
    }
}

impl FlowSpec for Modify {
    fn steps(&self, cx: &FlowCx<'_>, values: &Values) -> Vec<StepDef> {
        let mut steps = vec![
            contract_step(
                "Número del contrato a modificar",
                contract_items(cx, &NOT_ANNULLED),
            ),
            StepDef::new(
                "field",
                "Dato que corrige",
                StepKind::Choice(vec![
                    ("capital", "Capital"),
                    ("rate", "Porcentaje"),
                    ("term", "Plazo"),
                    ("date", "Fecha del contrato"),
                    ("description", "Descripción del artículo"),
                ]),
            ),
        ];
        let kind = match values.get("field") {
            "capital" => StepKind::Money,
            "rate" => StepKind::Rate,
            "term" => StepKind::Integer { min: 1, max: 60 },
            "date" => StepKind::Date,
            _ => StepKind::Text { optional: false },
        };
        let mut value = StepDef::new("value", "Nuevo valor", kind);
        if let Some((canonical, shown)) = Self::current(cx, values) {
            value = value
                .hint(format!("Valor actual: {shown}."))
                .default_value(canonical);
        }
        steps.push(value);
        steps
    }

    fn check(
        &self,
        cx: &FlowCx<'_>,
        _values: &Values,
        step: &'static str,
        value: &str,
    ) -> Result<(), String> {
        if step == "contract" {
            check_contract(cx, value, &NOT_ANNULLED, "no anulado")
        } else {
            Ok(())
        }
    }

    fn context(&self, ui: &mut Ui, cx: &FlowCx<'_>, values: &Values) {
        contract_context(ui, cx, values, cx.today);
    }

    fn review_extra(&self, cx: &FlowCx<'_>, values: &Values) -> Vec<(String, String)> {
        Self::current(cx, values).map_or_else(Vec::new, |(_, shown)| {
            vec![(String::from("Valor anterior"), shown)]
        })
    }

    fn commit(
        &self,
        store: &mut PosStore,
        today: NaiveDate,
        values: &Values,
    ) -> Result<String, OpError> {
        let number = values.number("contract");
        let change = match values.get("field") {
            "capital" => ContractChange::Capital(values.money("value")),
            "rate" => ContractChange::Rate(values.rate("value")),
            "term" => ContractChange::Term(values.number("value")),
            "date" => ContractChange::Date(values.date("value").unwrap_or(today)),
            _ => ContractChange::Description(values.get("value").to_owned()),
        };
        store.modify_contract(number, change, today)?;
        Ok(format!(
            "Contrato {number} modificado. El cambio quedó en el auditor interno."
        ))
    }

    fn again_question(&self) -> &'static str {
        AGAIN
    }
}

// ------------------------------------------------------------------ Abono a capital

/// Abono a capital (la reunión lo registra modificando el contrato, 13:41).
pub struct CapitalPayment;

impl FlowSpec for CapitalPayment {
    fn steps(&self, cx: &FlowCx<'_>, _values: &Values) -> Vec<StepDef> {
        vec![
            contract_step(
                "Número de contrato",
                contract_items(cx, &[ContractStatus::Active, ContractStatus::Expired]),
            ),
            StepDef::new("amount", "Valor del abono", StepKind::Money)
                .hint("Debe ser menor que el capital actual."),
            date_step("date", "Fecha del abono", cx.today),
        ]
    }

    fn check(
        &self,
        cx: &FlowCx<'_>,
        values: &Values,
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
            "amount" => {
                let capital = cx
                    .store
                    .view(values.number("contract"), cx.today)
                    .map_or(Money::ZERO, |v| v.capital);
                if Money::pesos(value.parse().unwrap_or(0)) >= capital {
                    Err(format!(
                        "El abono debe ser menor que el capital ({capital}). Para saldarlo use Retiro de contrato."
                    ))
                } else {
                    Ok(())
                }
            }
            _ => Ok(()),
        }
    }

    fn context(&self, ui: &mut Ui, cx: &FlowCx<'_>, values: &Values) {
        contract_context(ui, cx, values, cx.today);
    }

    fn review_extra(&self, cx: &FlowCx<'_>, values: &Values) -> Vec<(String, String)> {
        let Some(view) = cx.store.view(values.number("contract"), cx.today) else {
            return Vec::new();
        };
        let capital = view.capital - values.money("amount");
        let rate: Rate = view.contract.rate;
        vec![
            (String::from("Capital actual"), view.capital.to_string()),
            (String::from("Nuevo capital"), capital.to_string()),
            (
                String::from("Nuevo interés mensual"),
                interest::monthly_interest(capital, rate).to_string(),
            ),
        ]
    }

    fn commit(
        &self,
        store: &mut PosStore,
        today: NaiveDate,
        values: &Values,
    ) -> Result<String, OpError> {
        let number = values.number("contract");
        let (movement, capital) = store.capital_payment(
            number,
            values.money("amount"),
            date_or_today(values, "date", today),
        )?;
        Ok(format!(
            "Abono registrado. Nuevo capital del contrato {number}: {capital}. Movimiento {movement}."
        ))
    }

    fn again_question(&self) -> &'static str {
        AGAIN
    }
}

// ------------------------------------------------------------------ Reversiones y anulaciones

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UndoKind {
    RevertTransfer,
    ReverseWithdrawal,
    AnnulMovement,
    AnnulSale,
    AnnulLayawayPayment,
    AnnulContract,
}

/// Reversiones y anulaciones: identificar, ver el efecto y confirmar.
pub struct Undo {
    kind: UndoKind,
}

impl Undo {
    /// Especificación de la acción del submenú, si es una reversión o anulación.
    #[must_use]
    pub fn for_action(id: &str) -> Option<Self> {
        let kind = match id {
            "revert_transfer" => UndoKind::RevertTransfer,
            "reverse_withdrawal" => UndoKind::ReverseWithdrawal,
            "annul_movement" => UndoKind::AnnulMovement,
            "annul_sale" => UndoKind::AnnulSale,
            "annul_layaway_payment" => UndoKind::AnnulLayawayPayment,
            "annul_contract" => UndoKind::AnnulContract,
            _ => return None,
        };
        Some(Self { kind })
    }

    /// Estados de contrato sobre los que se puede aplicar la acción.
    fn statuses(&self) -> &'static [ContractStatus] {
        match self.kind {
            UndoKind::RevertTransfer => &[ContractStatus::Owned],
            UndoKind::ReverseWithdrawal => &[ContractStatus::Withdrawn],
            UndoKind::AnnulSale => &[ContractStatus::Sold, ContractStatus::Layaway],
            _ => &NOT_ANNULLED,
        }
    }

    fn by_movement(&self) -> bool {
        matches!(
            self.kind,
            UndoKind::AnnulMovement | UndoKind::AnnulLayawayPayment
        )
    }

    fn effect(&self) -> &'static str {
        match self.kind {
            UndoKind::RevertTransfer => {
                "El contrato sale de mercancía propia y vuelve a vigente o vencido."
            }
            UndoKind::ReverseWithdrawal => {
                "El retiro se anula y el contrato vuelve a estar vigente o vencido."
            }
            UndoKind::AnnulMovement => {
                "El movimiento queda anulado y el contrato vuelve a su situación anterior."
            }
            UndoKind::AnnulSale => {
                "La venta o el separado se anula y el artículo vuelve a mercancía propia."
            }
            UndoKind::AnnulLayawayPayment => {
                "El abono se anula y el saldo del separado aumenta en ese valor."
            }
            UndoKind::AnnulContract => {
                "El contrato queda anulado y deja de figurar en los inventarios."
            }
        }
    }
}

impl FlowSpec for Undo {
    fn steps(&self, cx: &FlowCx<'_>, _values: &Values) -> Vec<StepDef> {
        let layaway_only = self.kind == UndoKind::AnnulLayawayPayment;
        let subject = if self.by_movement() {
            StepDef::new(
                "movement",
                "Número de movimiento",
                StepKind::Integer {
                    min: 1,
                    max: 99_999_999,
                },
            )
            .hint("Escriba el número o elija el movimiento de la lista.")
            .suggest(
                "Movimientos",
                movement_items(cx, |kind| {
                    if layaway_only {
                        matches!(kind, MovementKind::LayawayPayment { .. })
                    } else {
                        !matches!(kind, MovementKind::Pawn)
                    }
                }),
            )
        } else {
            contract_step("Número de contrato", contract_items(cx, self.statuses()))
        };
        vec![
            subject,
            StepDef::new("confirm", "¿Confirma la operación?", StepKind::YesNo).hint(self.effect()),
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
            "contract" => match self.kind {
                UndoKind::RevertTransfer => check_contract(
                    cx,
                    value,
                    &[ContractStatus::Owned],
                    "en mercancía propia sin vender",
                ),
                UndoKind::ReverseWithdrawal => {
                    check_contract(cx, value, &[ContractStatus::Withdrawn], "retirado")
                }
                UndoKind::AnnulSale => check_contract(
                    cx,
                    value,
                    &[ContractStatus::Sold, ContractStatus::Layaway],
                    "vendido o separado",
                ),
                _ => check_contract(cx, value, &NOT_ANNULLED, "no anulado"),
            },
            "movement" => {
                let number: u32 = value.parse().unwrap_or(0);
                let movement = cx
                    .store
                    .movement(number)
                    .ok_or_else(|| OpError::MovementNotFound(number).to_string())?;
                if movement.annulled {
                    return Err(OpError::MovementAlreadyAnnulled(number).to_string());
                }
                if self.kind == UndoKind::AnnulLayawayPayment
                    && !matches!(movement.kind, MovementKind::LayawayPayment { .. })
                {
                    return Err(format!(
                        "El movimiento {number} es «{}», no un abono a separado.",
                        movement.kind.label()
                    ));
                }
                Ok(())
            }
            "confirm" if value != "S" => Err(String::from(
                "No se hizo ningún cambio. Responda S para confirmar o Esc para volver.",
            )),
            _ => Ok(()),
        }
    }

    fn context(&self, ui: &mut Ui, cx: &FlowCx<'_>, values: &Values) {
        if self.by_movement() {
            match cx.store.movement(values.number("movement")) {
                Some(movement) => movement_card(ui, movement, cx.store),
                None => crate::components::info_banner(
                    ui,
                    "Escriba el número de movimiento para ver su detalle.",
                ),
            }
        } else {
            contract_context(ui, cx, values, cx.today);
        }
    }

    fn review_extra(&self, _cx: &FlowCx<'_>, _values: &Values) -> Vec<(String, String)> {
        vec![(String::from("Efecto"), self.effect().to_owned())]
    }

    fn commit(
        &self,
        store: &mut PosStore,
        today: NaiveDate,
        values: &Values,
    ) -> Result<String, OpError> {
        let contract = values.number("contract");
        let movement = values.number("movement");
        match self.kind {
            UndoKind::RevertTransfer => store.revert_transfer(contract, today)?,
            UndoKind::ReverseWithdrawal => store.reverse_withdrawal(contract, today)?,
            UndoKind::AnnulMovement | UndoKind::AnnulLayawayPayment => {
                store.annul_movement(movement, today)?;
            }
            UndoKind::AnnulSale => store.annul_sale(contract, today)?,
            UndoKind::AnnulContract => store.annul_contract(contract, today)?,
        }
        let subject = if self.by_movement() {
            format!("movimiento {movement}")
        } else {
            format!("contrato {contract}")
        };
        Ok(format!(
            "Operación aplicada sobre el {subject}. Quedó registrada en el auditor interno."
        ))
    }

    fn again_question(&self) -> &'static str {
        AGAIN
    }
}
