//! Valores del negocio sin coma flotante (ADR-0005): dinero en pesos enteros,
//! porcentajes en centésimas de punto y pesos en miligramos.

use std::{
    fmt,
    iter::Sum,
    ops::{Add, AddAssign, Neg, Sub, SubAssign},
};

/// Error al interpretar un valor escrito por el usuario.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ParseValueError {
    #[error("Escriba un valor.")]
    Empty,
    #[error("«{0}» no es un número válido.")]
    Invalid(String),
    #[error("El valor es demasiado grande.")]
    TooLarge,
}

/// Importe en pesos colombianos (sin centavos).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Money(i64);

impl Money {
    pub const ZERO: Self = Self(0);

    #[must_use]
    pub const fn pesos(value: i64) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn as_pesos(self) -> i64 {
        self.0
    }

    #[must_use]
    pub const fn is_positive(self) -> bool {
        self.0 > 0
    }

    #[must_use]
    pub const fn is_negative(self) -> bool {
        self.0 < 0
    }

    #[must_use]
    pub const fn abs(self) -> Self {
        Self(self.0.abs())
    }

    /// Multiplica por una cantidad entera (por ejemplo, meses).
    #[must_use]
    pub fn times(self, factor: u32) -> Self {
        Self(self.0.saturating_mul(i64::from(factor)))
    }

    /// Interpreta un importe escrito: «1.210.000», «$ 700000» o «700000».
    ///
    /// # Errors
    /// Si está vacío, tiene caracteres que no son dígitos o no cabe en un `i64`.
    pub fn parse(text: &str) -> Result<Self, ParseValueError> {
        let digits: String = text
            .chars()
            .filter(|c| !matches!(c, '$' | '.' | ' '))
            .collect();
        if digits.is_empty() {
            return Err(ParseValueError::Empty);
        }
        if !digits.chars().all(|c| c.is_ascii_digit()) {
            return Err(ParseValueError::Invalid(text.trim().to_owned()));
        }
        digits
            .parse::<i64>()
            .map(Self)
            .map_err(|_| ParseValueError::TooLarge)
    }
}

impl fmt::Display for Money {
    /// Formato colombiano: «$1.210.000», «-$280.000».
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sign = if self.0 < 0 { "-" } else { "" };
        write!(f, "{sign}${}", group_thousands(self.0.unsigned_abs()))
    }
}

/// Separa miles con punto: 1210000 → «1.210.000».
fn group_thousands(value: u64) -> String {
    let digits = value.to_string();
    let mut out = String::with_capacity(digits.len() + digits.len() / 3);
    for (index, digit) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index).is_multiple_of(3) {
            out.push('.');
        }
        out.push(digit);
    }
    out
}

impl Add for Money {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0.saturating_add(other.0))
    }
}

impl Sub for Money {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self(self.0.saturating_sub(other.0))
    }
}

impl AddAssign for Money {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl SubAssign for Money {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl Neg for Money {
    type Output = Self;
    fn neg(self) -> Self {
        Self(self.0.saturating_neg())
    }
}

impl Sum for Money {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, Add::add)
    }
}

/// Porcentaje mensual en centésimas de punto: 7 % = 700; 2,5 % = 250.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rate(u32);

impl Rate {
    #[must_use]
    pub const fn from_hundredths(hundredths: u32) -> Self {
        Self(hundredths)
    }

    /// Porcentaje entero: `Rate::percent(7)` = 7 %.
    #[must_use]
    pub const fn percent(percent: u32) -> Self {
        Self(percent * 100)
    }

    #[must_use]
    pub const fn hundredths(self) -> u32 {
        self.0
    }

    /// Aplica el porcentaje a un importe, redondeando al peso (mitad hacia arriba).
    #[must_use]
    pub fn of(self, amount: Money) -> Money {
        let product = i128::from(amount.as_pesos()) * i128::from(self.0);
        let rounded = (product + product.signum() * 5_000) / 10_000;
        Money::pesos(i64::try_from(rounded).unwrap_or(i64::MAX))
    }

    /// Interpreta «7», «7 %», «2,5» o «2.5».
    ///
    /// # Errors
    /// Si está vacío, no es un número o tiene más de dos decimales.
    pub fn parse(text: &str) -> Result<Self, ParseValueError> {
        parse_fixed(text.trim().trim_end_matches('%'), 2).map(Self)
    }
}

impl fmt::Display for Rate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} %", format_fixed(u64::from(self.0), 2))
    }
}

/// Peso en miligramos.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Weight(u32);

impl Weight {
    #[must_use]
    pub const fn milligrams(value: u32) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn as_milligrams(self) -> u32 {
        self.0
    }

    /// Interpreta gramos con hasta tres decimales: «2,9», «6.1», «125».
    ///
    /// # Errors
    /// Si está vacío, no es un número o tiene más de tres decimales.
    pub fn parse(text: &str) -> Result<Self, ParseValueError> {
        parse_fixed(text.trim().trim_end_matches('g').trim(), 3).map(Self)
    }
}

impl Add for Weight {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0.saturating_add(other.0))
    }
}

impl Sum for Weight {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::default(), Add::add)
    }
}

impl fmt::Display for Weight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} g", format_fixed(u64::from(self.0), 3))
    }
}

/// Lee un decimal con coma o punto como entero escalado: («2,5», 2) → 250.
fn parse_fixed(text: &str, decimals: u32) -> Result<u32, ParseValueError> {
    let text = text.trim();
    if text.is_empty() {
        return Err(ParseValueError::Empty);
    }
    let invalid = || ParseValueError::Invalid(text.to_owned());
    let normalized = text.replace(',', ".");
    let (whole, fraction) = normalized.split_once('.').unwrap_or((&normalized, ""));
    let fraction_len = u32::try_from(fraction.len()).map_err(|_| invalid())?;
    if whole.is_empty() && fraction.is_empty()
        || fraction_len > decimals
        || !whole
            .chars()
            .chain(fraction.chars())
            .all(|c| c.is_ascii_digit())
    {
        return Err(invalid());
    }
    let scale = 10_u32.pow(decimals);
    let whole: u32 = if whole.is_empty() {
        0
    } else {
        whole.parse().map_err(|_| ParseValueError::TooLarge)?
    };
    let fraction: u32 = if fraction.is_empty() {
        0
    } else {
        fraction.parse::<u32>().map_err(|_| invalid())? * 10_u32.pow(decimals - fraction_len)
    };
    whole
        .checked_mul(scale)
        .and_then(|w| w.checked_add(fraction))
        .ok_or(ParseValueError::TooLarge)
}

/// Escribe un entero escalado con coma decimal y sin ceros sobrantes: 2900, 3 → «2,9».
fn format_fixed(value: u64, decimals: u32) -> String {
    let scale = 10_u64.pow(decimals);
    let whole = value / scale;
    let fraction = value % scale;
    if fraction == 0 {
        return whole.to_string();
    }
    let width = usize::try_from(decimals).unwrap_or(0);
    let fraction = format!("{fraction:0width$}");
    format!("{whole},{}", fraction.trim_end_matches('0'))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn money_is_shown_with_colombian_thousands_separators() {
        assert_eq!(Money::pesos(1_210_000).to_string(), "$1.210.000");
    }

    #[test]
    fn negative_money_keeps_its_sign() {
        assert_eq!(Money::pesos(-280_000).to_string(), "-$280.000");
    }

    #[test]
    fn money_parses_formatted_text() {
        assert_eq!(Money::parse("$ 1.210.000"), Ok(Money::pesos(1_210_000)));
    }

    #[test]
    fn money_rejects_letters() {
        assert!(Money::parse("12a").is_err());
    }

    #[test]
    fn rate_applies_to_capital() {
        assert_eq!(
            Rate::percent(7).of(Money::pesos(700_000)),
            Money::pesos(49_000)
        );
    }

    #[test]
    fn rate_rounds_half_up_to_the_peso() {
        assert_eq!(
            Rate::percent(7).of(Money::pesos(350_005)),
            Money::pesos(24_500)
        );
    }

    #[test]
    fn rate_parses_decimal_comma() {
        assert_eq!(Rate::parse("2,5"), Ok(Rate::from_hundredths(250)));
    }

    #[test]
    fn rate_is_shown_without_trailing_zeros() {
        assert_eq!(Rate::from_hundredths(250).to_string(), "2,5 %");
    }

    #[test]
    fn weight_parses_grams_with_decimals() {
        assert_eq!(Weight::parse("2,9"), Ok(Weight::milligrams(2_900)));
    }

    #[test]
    fn weight_is_shown_in_grams() {
        assert_eq!(Weight::milligrams(6_100).to_string(), "6,1 g");
    }

    #[test]
    fn weight_rejects_too_many_decimals() {
        assert!(Weight::parse("1,2345").is_err());
    }
}
