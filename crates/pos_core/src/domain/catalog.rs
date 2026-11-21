//! Catálogos configurables desde Mantenimiento: códigos de mercancía,
//! calidades de oro y conceptos de caja.

/// Metal precioso, para el inventario por quilataje y por gramos.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Metal {
    Gold,
    Silver,
}

/// Código de mercancía (oro, relojes, bicicletas…).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MerchandiseCode {
    pub code: u16,
    pub name: String,
    /// Metal, si los artículos de este código se pesan por quilataje.
    pub metal: Option<Metal>,
}

/// Calidad de oro (quilataje).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GoldQuality {
    pub karat: u8,
    pub name: String,
}

/// Sentido de un movimiento de dinero.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CashDirection {
    Income,
    Expense,
}

impl CashDirection {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Income => "Ingreso",
            Self::Expense => "Egreso",
        }
    }
}

/// Clase de concepto de caja.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConceptCategory {
    /// Arriendo, servicios, sueldos, seguridad social…
    Expense,
    /// Ganancia de compras o ventas ocasionales.
    OccasionalProfit,
    /// Diferencias y ajustes de caja.
    Loss,
    /// Ingresos no asociados a contratos.
    OtherIncome,
}

impl ConceptCategory {
    #[must_use]
    pub const fn direction(self) -> CashDirection {
        match self {
            Self::Expense | Self::Loss => CashDirection::Expense,
            Self::OccasionalProfit | Self::OtherIncome => CashDirection::Income,
        }
    }

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Expense => "Gasto",
            Self::OccasionalProfit => "Utilidad ocasional",
            Self::Loss => "Pérdida / ajuste",
            Self::OtherIncome => "Otro ingreso",
        }
    }

    pub const ALL: [Self; 4] = [
        Self::Expense,
        Self::OccasionalProfit,
        Self::Loss,
        Self::OtherIncome,
    ];
}

/// Concepto (código) de caja o banco.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CashConcept {
    pub code: u16,
    pub name: String,
    pub category: ConceptCategory,
}
