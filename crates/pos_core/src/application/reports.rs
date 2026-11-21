//! Informes calculados a partir de los movimientos no anulados.

use std::collections::BTreeMap;

use chrono::{Datelike, NaiveDate};

use super::store::{ContractView, PosStore};
use crate::domain::{
    CashDirection, ConceptCategory, ContractKind, ContractStatus, Metal, Money, MovementKind,
    Weight,
};

/// Línea de la caja diaria.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CashLine {
    pub movement: u32,
    pub contract: Option<u32>,
    pub operation: &'static str,
    /// Código de mercancía o de concepto.
    pub code: Option<u16>,
    pub detail: String,
    pub capital: Money,
    pub interest: Money,
    pub inflow: Money,
    pub outflow: Money,
}

/// Totales de la caja diaria («resumen», 41:04).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CashSummary {
    pub pawned: Money,
    pub withdrawn_capital: Money,
    pub withdrawn_interest: Money,
    pub interest_payments: Money,
    pub capital_payments: Money,
    pub sales: Money,
    pub layaway_payments: Money,
    pub other_income: Money,
    pub expenses: Money,
    pub total_in: Money,
    pub total_out: Money,
    pub net: Money,
}

/// Caja de un día.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DailyCash {
    pub date: NaiveDate,
    pub lines: Vec<CashLine>,
    pub summary: CashSummary,
}

/// Línea del informe de gastos.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpenseLine {
    pub movement: u32,
    pub date: NaiveDate,
    pub code: u16,
    pub concept: String,
    pub description: String,
    pub amount: Money,
}

/// Total de un código.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeTotal {
    pub code: u16,
    pub name: String,
    pub count: usize,
    pub amount: Money,
}

/// Orden del informe de gastos (36:24).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpenseOrder {
    ByCode,
    ByDate,
}

/// Gastos de un período.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpenseReport {
    pub lines: Vec<ExpenseLine>,
    pub by_code: Vec<CodeTotal>,
    pub total: Money,
}

/// Acumulados de un mes por operación (45:09–48:23).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MonthlyAccumulated {
    pub lent_common: Money,
    pub lent_non_common: Money,
    pub withdrawn_capital_common: Money,
    pub withdrawn_interest_common: Money,
    pub withdrawn_capital_non_common: Money,
    pub withdrawn_interest_non_common: Money,
    pub interest_payments: Money,
    pub capital_payments: Money,
    pub sales: Money,
    pub layaway_payments: Money,
    pub occasional_profit: Money,
    pub other_income: Money,
    pub expenses_by_code: Vec<CodeTotal>,
    pub losses: Money,
    pub total_in: Money,
    pub total_out: Money,
    pub balance: Money,
}

/// Inventario de contratos vigentes (48:32–50:55).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ActiveInventory {
    pub by_code: Vec<CodeTotal>,
    /// Gramos de oro por quilataje.
    pub gold_by_karat: Vec<(u8, Weight)>,
    pub silver: Weight,
    pub contracts: usize,
    pub capital: Money,
    /// Intereses causados y no pagados de todos los contratos.
    pub accrued_interest: Money,
}

impl PosStore {
    fn concept_name(&self, code: u16) -> String {
        self.concept(code)
            .map_or_else(|| String::from("(sin concepto)"), |c| c.name.clone())
    }

    fn merchandise_name(&self, code: u16) -> String {
        self.merchandise_code(code)
            .map_or_else(|| String::from("(sin código)"), |m| m.name.clone())
    }

    /// Caja diaria: movimientos de caja del día (sin banco).
    #[must_use]
    pub fn daily_cash(&self, date: NaiveDate) -> DailyCash {
        let mut lines = Vec::new();
        let mut s = CashSummary::default();
        for movement in self
            .movements()
            .iter()
            .filter(|m| !m.annulled && m.date == date)
        {
            let contract = movement.contract.and_then(|n| self.contract(n));
            let item_code = contract.map(|c| c.item.code);
            let item = contract
                .map(|c| c.item.description.clone())
                .unwrap_or_default();
            let mut line = CashLine {
                movement: movement.number,
                contract: movement.contract,
                operation: movement.kind.label(),
                code: item_code,
                detail: item,
                capital: Money::ZERO,
                interest: Money::ZERO,
                inflow: Money::ZERO,
                outflow: Money::ZERO,
            };
            match &movement.kind {
                MovementKind::Pawn => {
                    let capital = contract.map_or(Money::ZERO, |c| c.capital);
                    line.capital = capital;
                    line.outflow = capital;
                    s.pawned += capital;
                }
                MovementKind::InterestPayment { amount, .. } => {
                    line.interest = *amount;
                    line.inflow = *amount;
                    s.interest_payments += *amount;
                }
                MovementKind::CapitalPayment { amount } => {
                    line.capital = *amount;
                    line.inflow = *amount;
                    s.capital_payments += *amount;
                }
                MovementKind::Withdrawal { capital, interest } => {
                    line.capital = *capital;
                    line.interest = *interest;
                    line.inflow = *capital + *interest;
                    s.withdrawn_capital += *capital;
                    s.withdrawn_interest += *interest;
                }
                MovementKind::Sale { price, .. } => {
                    line.inflow = *price;
                    s.sales += *price;
                }
                MovementKind::LayawayPayment { amount } => {
                    line.inflow = *amount;
                    s.layaway_payments += *amount;
                }
                MovementKind::Cash {
                    concept,
                    direction,
                    description,
                    amount,
                    bank: false,
                } => {
                    line.code = Some(*concept);
                    line.detail = format!("{} · {description}", self.concept_name(*concept));
                    match direction {
                        CashDirection::Income => {
                            line.inflow = *amount;
                            s.other_income += *amount;
                        }
                        CashDirection::Expense => {
                            line.outflow = *amount;
                            s.expenses += *amount;
                        }
                    }
                }
                MovementKind::TransferToOwned
                | MovementKind::Layaway { .. }
                | MovementKind::Cash { bank: true, .. } => continue,
            }
            lines.push(line);
        }
        s.total_in = s.withdrawn_capital
            + s.withdrawn_interest
            + s.interest_payments
            + s.capital_payments
            + s.sales
            + s.layaway_payments
            + s.other_income;
        s.total_out = s.pawned + s.expenses;
        s.net = s.total_in - s.total_out;
        DailyCash {
            date,
            lines,
            summary: s,
        }
    }

    /// Gastos (egresos de caja y banco) entre dos fechas, inclusive.
    #[must_use]
    pub fn expenses(&self, from: NaiveDate, to: NaiveDate, order: ExpenseOrder) -> ExpenseReport {
        let mut lines: Vec<ExpenseLine> = self
            .movements()
            .iter()
            .filter(|m| !m.annulled && m.date >= from && m.date <= to)
            .filter_map(|m| match &m.kind {
                MovementKind::Cash {
                    concept,
                    direction: CashDirection::Expense,
                    description,
                    amount,
                    ..
                } => Some(ExpenseLine {
                    movement: m.number,
                    date: m.date,
                    code: *concept,
                    concept: self.concept_name(*concept),
                    description: description.clone(),
                    amount: *amount,
                }),
                _ => None,
            })
            .collect();
        match order {
            ExpenseOrder::ByCode => lines.sort_by_key(|l| (l.code, l.date, l.movement)),
            ExpenseOrder::ByDate => lines.sort_by_key(|l| (l.date, l.movement)),
        }
        let by_code = totals_by_code(lines.iter().map(|l| (l.code, l.concept.clone(), l.amount)));
        let total = lines.iter().map(|l| l.amount).sum();
        ExpenseReport {
            lines,
            by_code,
            total,
        }
    }

    /// Acumulados de un mes.
    #[must_use]
    pub fn monthly_accumulated(&self, year: i32, month: u32) -> MonthlyAccumulated {
        let mut a = MonthlyAccumulated::default();
        let mut expenses = Vec::new();
        let in_month = |date: NaiveDate| date.year() == year && date.month() == month;
        for movement in self
            .movements()
            .iter()
            .filter(|m| !m.annulled && in_month(m.date))
        {
            let kind = movement
                .contract
                .and_then(|n| self.contract(n))
                .map(|c| c.kind);
            let common = kind != Some(ContractKind::NonCommon);
            match &movement.kind {
                MovementKind::Pawn => {
                    let capital = movement
                        .contract
                        .and_then(|n| self.contract(n))
                        .map_or(Money::ZERO, |c| c.capital);
                    if common {
                        a.lent_common += capital;
                    } else {
                        a.lent_non_common += capital;
                    }
                }
                MovementKind::Withdrawal { capital, interest } => {
                    if common {
                        a.withdrawn_capital_common += *capital;
                        a.withdrawn_interest_common += *interest;
                    } else {
                        a.withdrawn_capital_non_common += *capital;
                        a.withdrawn_interest_non_common += *interest;
                    }
                }
                MovementKind::InterestPayment { amount, .. } => a.interest_payments += *amount,
                MovementKind::CapitalPayment { amount } => a.capital_payments += *amount,
                MovementKind::Sale { price, .. } => a.sales += *price,
                MovementKind::LayawayPayment { amount } => a.layaway_payments += *amount,
                MovementKind::Cash {
                    concept,
                    amount,
                    bank: false,
                    ..
                } => match self.concept(*concept).map(|c| c.category) {
                    Some(ConceptCategory::OccasionalProfit) => a.occasional_profit += *amount,
                    Some(ConceptCategory::OtherIncome) => a.other_income += *amount,
                    Some(ConceptCategory::Loss) => a.losses += *amount,
                    Some(ConceptCategory::Expense) | None => {
                        expenses.push((*concept, self.concept_name(*concept), *amount));
                    }
                },
                MovementKind::TransferToOwned
                | MovementKind::Layaway { .. }
                | MovementKind::Cash { bank: true, .. } => {}
            }
        }
        a.expenses_by_code = totals_by_code(expenses.into_iter());
        let expenses_total: Money = a.expenses_by_code.iter().map(|c| c.amount).sum();
        a.total_in = a.withdrawn_capital_common
            + a.withdrawn_interest_common
            + a.withdrawn_capital_non_common
            + a.withdrawn_interest_non_common
            + a.interest_payments
            + a.capital_payments
            + a.sales
            + a.layaway_payments
            + a.occasional_profit
            + a.other_income;
        a.total_out = a.lent_common + a.lent_non_common + expenses_total + a.losses;
        a.balance = a.total_in - a.total_out;
        a
    }

    /// Inventario de contratos vigentes (incluye los vencidos que no han
    /// pasado a propios).
    #[must_use]
    pub fn active_inventory(&self, on: NaiveDate) -> ActiveInventory {
        let views: Vec<ContractView> = self
            .all_views(on)
            .filter(|v| matches!(v.status, ContractStatus::Active | ContractStatus::Expired))
            .collect();
        let by_code = totals_by_code(views.iter().map(|v| {
            (
                v.contract.item.code,
                self.merchandise_name(v.contract.item.code),
                v.capital,
            )
        }));
        let mut gold: BTreeMap<u8, Weight> = BTreeMap::new();
        let mut silver = Weight::default();
        for view in &views {
            let metal = self
                .merchandise_code(view.contract.item.code)
                .and_then(|m| m.metal);
            let weight = view.contract.item.weight.unwrap_or_default();
            match (metal, view.contract.item.karat) {
                (Some(Metal::Gold), Some(karat)) => {
                    let entry = gold.entry(karat).or_default();
                    *entry = *entry + weight;
                }
                (Some(Metal::Silver), _) => silver = silver + weight,
                _ => {}
            }
        }
        ActiveInventory {
            by_code,
            gold_by_karat: gold.into_iter().rev().collect(),
            silver,
            contracts: views.len(),
            capital: views.iter().map(|v| v.capital).sum(),
            accrued_interest: views.iter().map(|v| v.payoff - v.capital).sum(),
        }
    }
}

/// Agrupa importes por código, en orden de código.
fn totals_by_code(items: impl Iterator<Item = (u16, String, Money)>) -> Vec<CodeTotal> {
    let mut map: BTreeMap<u16, CodeTotal> = BTreeMap::new();
    for (code, name, amount) in items {
        let entry = map.entry(code).or_insert_with(|| CodeTotal {
            code,
            name,
            count: 0,
            amount: Money::ZERO,
        });
        entry.count += 1;
        entry.amount += amount;
    }
    map.into_values().collect()
}
