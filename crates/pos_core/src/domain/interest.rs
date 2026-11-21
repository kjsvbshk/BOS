//! Fórmulas financieras **provisionales** (ADR-0008), deducidas de los ejemplos
//! de la reunión del 2026-11-18. Se validarán contra el sistema anterior.

use chrono::{Months, NaiveDate};

use super::values::{Money, Rate};

/// Interés de un mes: capital × porcentaje, redondeado al peso.
#[must_use]
pub fn monthly_interest(capital: Money, rate: Rate) -> Money {
    rate.of(capital)
}

/// Suma meses calendario (el 31-ene + 1 mes = 28/29-feb).
#[must_use]
pub fn add_months(date: NaiveDate, months: u32) -> NaiveDate {
    date.checked_add_months(Months::new(months)).unwrap_or(date)
}

/// Meses de interés causados entre la fecha del contrato y `on`: un mes
/// empezado cuenta completo y siempre hay al menos uno.
#[must_use]
pub fn months_accrued(contract_date: NaiveDate, on: NaiveDate) -> u32 {
    if on <= contract_date {
        return 1;
    }
    let mut full = 0;
    while add_months(contract_date, full + 1) <= on {
        full += 1;
    }
    let exact = add_months(contract_date, full) == on;
    (if exact { full } else { full + 1 }).max(1)
}

/// Meses que faltan por pagar.
#[must_use]
pub fn months_pending(accrued: u32, paid: u32) -> u32 {
    accrued.saturating_sub(paid)
}

/// Valor para retirar el contrato: capital + interés mensual × meses pendientes.
#[must_use]
pub fn payoff(capital: Money, rate: Rate, pending_months: u32) -> Money {
    capital + monthly_interest(capital, rate).times(pending_months)
}

/// Valor de una prórroga de `months` meses.
#[must_use]
pub fn extension_amount(capital: Money, rate: Rate, months: u32) -> Money {
    monthly_interest(capital, rate).times(months)
}

/// Fecha de vencimiento: fecha del contrato + máx(plazo, meses pagados).
#[must_use]
pub fn due_date(contract_date: NaiveDate, term_months: u32, paid_months: u32) -> NaiveDate {
    add_months(contract_date, term_months.max(paid_months))
}

/// Diferencia de una venta: positiva = utilidad, negativa = pérdida.
#[must_use]
pub fn sale_difference(price: Money, reference: Money) -> Money {
    price - reference
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn interest_of_the_new_contract_is_49000() {
        // 11:00 — 700.000 al 7 %.
        assert_eq!(
            monthly_interest(Money::pesos(700_000), Rate::percent(7)),
            Money::pesos(49_000)
        );
    }

    #[test]
    fn a_contract_consulted_on_its_first_day_accrues_one_month() {
        assert_eq!(months_accrued(date(2026, 11, 18), date(2026, 11, 18)), 1);
    }

    #[test]
    fn a_started_month_counts_as_a_full_month() {
        // 15:19 — 28-ago a 18-nov = 3 meses.
        assert_eq!(months_accrued(date(2026, 8, 28), date(2026, 11, 18)), 3);
    }

    #[test]
    fn an_exact_anniversary_does_not_start_a_new_month() {
        assert_eq!(months_accrued(date(2026, 8, 28), date(2026, 11, 28)), 3);
    }

    #[test]
    fn withdrawal_example_from_the_meeting_is_1210000() {
        // 15:40 — 1.000.000 al 7 % con 3 meses pendientes.
        assert_eq!(
            payoff(Money::pesos(1_000_000), Rate::percent(7), 3),
            Money::pesos(1_210_000)
        );
    }

    #[test]
    fn extension_of_two_months_costs_56000() {
        // 17:26 — 3 meses = 84.000; 2 meses = 56.000.
        assert_eq!(
            extension_amount(Money::pesos(400_000), Rate::percent(7), 2),
            Money::pesos(56_000)
        );
    }

    #[test]
    fn after_paying_two_of_three_months_the_payoff_is_428000() {
        // 18:16 — quedan debiendo 428.000.
        let pending = months_pending(3, 2);
        assert_eq!(
            payoff(Money::pesos(400_000), Rate::percent(7), pending),
            Money::pesos(428_000)
        );
    }

    #[test]
    fn due_date_adds_the_term() {
        // 12:34 — 28-ago, 3 meses, vence 28-nov.
        assert_eq!(due_date(date(2026, 8, 28), 3, 0), date(2026, 11, 28));
    }

    #[test]
    fn paid_months_beyond_the_term_extend_the_due_date() {
        assert_eq!(due_date(date(2026, 8, 28), 3, 4), date(2026, 12, 28));
    }

    #[test]
    fn selling_below_the_reference_is_a_loss() {
        // 27:26 — vale 1.280.000, se vende en 1.000.000.
        let difference = sale_difference(Money::pesos(1_000_000), Money::pesos(1_280_000));
        assert_eq!(difference, Money::pesos(-280_000));
    }
}
