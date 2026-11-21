//! Contratos de empeño y sus movimientos.

use chrono::NaiveDate;

use super::{
    catalog::CashDirection,
    values::{Money, Rate, Weight},
};

/// Tipo de contrato.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContractKind {
    /// Número digitado por el usuario desde su libreta.
    Common,
    /// Hipotecarios, vehículos y otros: número consecutivo del sistema.
    NonCommon,
}

impl ContractKind {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Common => "Común",
            Self::NonCommon => "No común (hipotecario)",
        }
    }
}

/// Artículo que garantiza el contrato.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Item {
    pub code: u16,
    pub description: String,
    pub condition: String,
    pub weight: Option<Weight>,
    pub karat: Option<u8>,
}

/// Datos con los que se registró el contrato. Su situación (capital vigente,
/// estado, intereses) se deriva de los movimientos.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Contract {
    pub number: u32,
    pub kind: ContractKind,
    pub customer: String,
    pub date: NaiveDate,
    pub term_months: u32,
    pub rate: Rate,
    pub capital: Money,
    pub item: Item,
    pub annulled: bool,
}

/// Qué ocurrió en un movimiento.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MovementKind {
    /// Registro del contrato: sale el capital prestado.
    Pawn,
    /// Pago de intereses (prórroga).
    InterestPayment { months: u32, amount: Money },
    /// Abono que reduce el capital.
    CapitalPayment { amount: Money },
    /// Retiro: el cliente paga capital e intereses y recupera el artículo.
    Withdrawal { capital: Money, interest: Money },
    /// El contrato vencido pasa a mercancía propia.
    TransferToOwned,
    /// Venta directa de mercancía propia.
    Sale {
        price: Money,
        reference: Money,
        buyer: Option<String>,
    },
    /// Venta a plazos (separado).
    Layaway {
        price: Money,
        customer: Option<String>,
        delivery: NaiveDate,
    },
    /// Abono a una mercancía separada.
    LayawayPayment { amount: Money },
    /// Ingreso o egreso de caja o banco.
    Cash {
        concept: u16,
        direction: CashDirection,
        description: String,
        amount: Money,
        bank: bool,
    },
}

impl MovementKind {
    /// Nombre de la operación para listados e informes.
    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            Self::Pawn => "Empeño",
            Self::InterestPayment { .. } => "Pago de intereses",
            Self::CapitalPayment { .. } => "Abono a capital",
            Self::Withdrawal { .. } => "Retiro",
            Self::TransferToOwned => "Paso a propios",
            Self::Sale { .. } => "Venta",
            Self::Layaway { .. } => "Separado",
            Self::LayawayPayment { .. } => "Abono a separado",
            Self::Cash {
                direction: CashDirection::Income,
                bank: false,
                ..
            } => "Ingreso de caja",
            Self::Cash {
                direction: CashDirection::Expense,
                bank: false,
                ..
            } => "Egreso de caja",
            Self::Cash {
                direction: CashDirection::Income,
                bank: true,
                ..
            } => "Ingreso de banco",
            Self::Cash {
                direction: CashDirection::Expense,
                bank: true,
                ..
            } => "Egreso de banco",
        }
    }
}

/// Operación registrada, con su número de movimiento.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Movement {
    pub number: u32,
    pub date: NaiveDate,
    pub contract: Option<u32>,
    pub kind: MovementKind,
    pub user: String,
    pub annulled: bool,
}

/// Situación de un contrato.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContractStatus {
    Active,
    Expired,
    Withdrawn,
    Owned,
    Layaway,
    Sold,
    Annulled,
}

impl ContractStatus {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Active => "Vigente",
            Self::Expired => "Vencido",
            Self::Withdrawn => "Retirado",
            Self::Owned => "Mercancía propia",
            Self::Layaway => "Separado",
            Self::Sold => "Vendido",
            Self::Annulled => "Anulado",
        }
    }
}
