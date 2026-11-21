//! Catálogo de navegación: única fuente de secciones, grupos y opciones
//! (ADR-0006). Para agregar, quitar o reordenar opciones sólo se edita este
//! archivo; las vistas recorren el catálogo.
//!
//! Operaciones sigue la distribución corregida del ADR-0008 (09 Consultas
//! varias, 10 Modificaciones, 11 Otras opciones). Mantenimiento e Informes salen
//! de la reunión del 2026-11-18 (`docs/levantamiento/`).

use pos_core::domain::Permission;

use crate::theme::icons;

/// Sección principal (pestañas de la barra superior).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SectionId {
    Operations,
    Maintenance,
    Reports,
}

/// Si la opción ya tiene definido su proceso.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OptionStatus {
    /// Funciona en la demo.
    Available,
    /// Nombre conocido; su proceso todavía no está documentado.
    AwaitingSpecification,
}

/// Acción dentro de una opción (submenú de 09 y 10).
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct NavAction {
    pub id: &'static str,
    pub label: &'static str,
    pub description: &'static str,
    pub icon: &'static str,
}

/// Opción de menú dentro de una sección.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct NavOption {
    /// Identificador estable; no cambia aunque cambie el nombre.
    pub id: &'static str,
    /// Número de la opción en su sección (el del sistema original en Operaciones).
    pub number: u8,
    /// Nombre oficial completo.
    pub full_name: &'static str,
    /// Primera línea de la tarjeta.
    pub title: &'static str,
    /// Segunda línea de la tarjeta.
    pub description: &'static str,
    pub icon: &'static str,
    pub status: OptionStatus,
    /// Permiso necesario para abrirla.
    pub permission: Permission,
    /// Submenú que la opción muestra al abrirse.
    pub actions: &'static [NavAction],
}

impl NavOption {
    /// Número con dos cifras, como en el sistema original («01»).
    #[must_use]
    pub fn code(&self) -> String {
        format!("{:02}", self.number)
    }

    /// Título de tarjeta con su número («01. Ingresar contratos»).
    #[must_use]
    pub fn numbered_title(&self) -> String {
        format!("{}. {}", self.code(), self.title)
    }

    /// Nombre oficial con su número.
    #[must_use]
    pub fn numbered_full_name(&self) -> String {
        format!("{}. {}", self.code(), self.full_name)
    }
}

/// Grupo visual de opciones. Sólo agrupa: no cambia numeración ni orden.
#[derive(Debug, PartialEq, Eq)]
pub struct OptionGroup {
    pub label: &'static str,
    pub options: &'static [NavOption],
}

/// Sección principal con sus grupos de opciones.
#[derive(Debug, PartialEq, Eq)]
pub struct NavSection {
    pub id: SectionId,
    pub label: &'static str,
    pub subtitle: &'static str,
    pub icon: &'static str,
    pub groups: &'static [OptionGroup],
}

impl NavSection {
    /// Todas las opciones de la sección, en su orden.
    pub fn options(&self) -> impl Iterator<Item = &'static NavOption> + use<> {
        let groups: &'static [OptionGroup] = self.groups;
        groups.iter().flat_map(|group| group.options.iter())
    }

    /// Opción por su número.
    #[must_use]
    pub fn option_number(&self, number: u8) -> Option<&'static NavOption> {
        self.options().find(|option| option.number == number)
    }
}

const fn option(
    id: &'static str,
    number: u8,
    full_name: &'static str,
    title: &'static str,
    description: &'static str,
    icon: &'static str,
) -> NavOption {
    NavOption {
        id,
        number,
        full_name,
        title,
        description,
        icon,
        status: OptionStatus::Available,
        permission: Permission::Operate,
        actions: &[],
    }
}

const fn action(
    id: &'static str,
    label: &'static str,
    description: &'static str,
    icon: &'static str,
) -> NavAction {
    NavAction {
        id,
        label,
        description,
        icon,
    }
}

// ------------------------------------------------------------------ Operaciones

const QUERIES: [NavAction; 3] = [
    action(
        "contract",
        "Consulta de contrato",
        "Ficha, pagos y valor de retiro",
        icons::FILE_TEXT,
    ),
    action(
        "customer",
        "Consulta por cliente",
        "Datos e historial de contratos",
        icons::USER_CIRCLE,
    ),
    action(
        "movement",
        "Consulta de movimiento",
        "Detalle por número de movimiento",
        icons::HASH,
    ),
];

const CHANGES: [NavAction; 8] = [
    action(
        "modify",
        "Modificar contrato",
        "Capital, porcentaje, plazo, fecha o artículo",
        icons::PENCIL_SIMPLE,
    ),
    action(
        "capital",
        "Abono a capital",
        "Reduce el capital del contrato",
        icons::HAND_COINS,
    ),
    action(
        "revert_transfer",
        "Revertir paso a propios",
        "El contrato vuelve a vigencia",
        icons::ARROW_U_UP_LEFT,
    ),
    action(
        "reverse_withdrawal",
        "Reversar retiro",
        "Deshace un retiro hecho por error",
        icons::ARROW_COUNTER_CLOCKWISE,
    ),
    action(
        "annul_movement",
        "Anular movimiento",
        "Por número de movimiento",
        icons::PROHIBIT,
    ),
    action(
        "annul_sale",
        "Anular venta o separado",
        "La mercancía vuelve a propios",
        icons::TAG,
    ),
    action(
        "annul_layaway_payment",
        "Anular abono a separado",
        "Por número de movimiento",
        icons::RECEIPT,
    ),
    action(
        "annul_contract",
        "Anular contrato",
        "Contrato ingresado por error",
        icons::TRASH,
    ),
];

const CONTRACTS: [NavOption; 4] = [
    option(
        "operations.01",
        1,
        "Ingresar contratos al inventario de contratos vigentes",
        "Ingresar contratos",
        "Al inventario de contratos vigentes",
        icons::FILE_PLUS,
    ),
    option(
        "operations.02",
        2,
        "Prorrogar plazo a contratos vigentes",
        "Prorrogar plazo",
        "A contratos vigentes",
        icons::CALENDAR_DOTS,
    ),
    option(
        "operations.03",
        3,
        "Retiro de contrato – Devolución a dueño original",
        "Retiro de contrato",
        "Devolución a dueño original",
        icons::HAND_COINS,
    ),
    NavOption {
        permission: Permission::TransferExpired,
        ..option(
            "operations.04",
            4,
            "Contratos vencidos – Pasar al inventario de mercancía propia",
            "Contratos vencidos",
            "Pasar al inventario de mercancía propia",
            icons::HOURGLASS_HIGH,
        )
    },
];

const INVENTORY: [NavOption; 2] = [
    NavOption {
        status: OptionStatus::AwaitingSpecification,
        ..option(
            "operations.05",
            5,
            "Ingresar mercancía al inventario de mercancía propia",
            "Ingresar mercancía",
            "Al inventario de mercancía propia",
            icons::PACKAGE,
        )
    },
    option(
        "operations.06",
        6,
        "Salida del inventario de mercancías propias",
        "Salida de mercancías",
        "Ventas, separados y abonos",
        icons::EXPORT,
    ),
];

const MOVEMENTS: [NavOption; 5] = [
    option(
        "operations.07",
        7,
        "Ingresos y/o egresos de caja",
        "Ingresos y/o egresos",
        "De caja",
        icons::CASH_REGISTER,
    ),
    option(
        "operations.08",
        8,
        "Ingresos y/o egresos de banco",
        "Ingresos y/o egresos",
        "De banco (sin uso actual)",
        icons::BANK,
    ),
    NavOption {
        actions: &QUERIES,
        ..option(
            "operations.09",
            9,
            "Consultas varias",
            "Consultas varias",
            "Contratos, clientes y movimientos",
            icons::MAGNIFYING_GLASS,
        )
    },
    NavOption {
        actions: &CHANGES,
        permission: Permission::ModifyAndAnnul,
        ..option(
            "operations.10",
            10,
            "Modificar, reversar, anular y borrar",
            "Modificar, reversar y anular",
            "Correcciones con trazabilidad",
            icons::ARROWS_CLOCKWISE,
        )
    },
    NavOption {
        status: OptionStatus::AwaitingSpecification,
        ..option(
            "operations.11",
            11,
            "Otras opciones",
            "Otras opciones",
            "Contenido pendiente de definición",
            icons::SQUARES_FOUR,
        )
    },
];

const OPERATION_GROUPS: [OptionGroup; 3] = [
    OptionGroup {
        label: "Contratos",
        options: &CONTRACTS,
    },
    OptionGroup {
        label: "Inventario",
        options: &INVENTORY,
    },
    OptionGroup {
        label: "Movimientos y consultas",
        options: &MOVEMENTS,
    },
];

// ------------------------------------------------------------------ Mantenimiento

const PEOPLE: [NavOption; 2] = [
    option(
        "maintenance.01",
        1,
        "Clientes",
        "Clientes",
        "Registro y consulta de clientes",
        icons::USERS,
    ),
    NavOption {
        permission: Permission::Administer,
        ..option(
            "maintenance.02",
            2,
            "Usuarios y perfiles",
            "Usuarios y perfiles",
            "Códigos de empleados y permisos",
            icons::IDENTIFICATION_CARD,
        )
    },
];

const CODES: [NavOption; 3] = [
    NavOption {
        permission: Permission::Administer,
        ..option(
            "maintenance.03",
            3,
            "Códigos de mercancía",
            "Códigos de mercancía",
            "Categorías de artículos",
            icons::TAG,
        )
    },
    NavOption {
        permission: Permission::Administer,
        ..option(
            "maintenance.04",
            4,
            "Códigos de calidad de oro",
            "Calidades de oro",
            "Quilatajes",
            icons::DIAMOND,
        )
    },
    NavOption {
        permission: Permission::Administer,
        ..option(
            "maintenance.05",
            5,
            "Códigos de caja y banco",
            "Conceptos de caja",
            "Gastos, utilidades y ajustes",
            icons::LIST_BULLETS,
        )
    },
];

const SYSTEM: [NavOption; 2] = [
    NavOption {
        permission: Permission::Administer,
        ..option(
            "maintenance.06",
            6,
            "Datos del negocio",
            "Datos del negocio",
            "Nombre de la compraventa",
            icons::STOREFRONT,
        )
    },
    option(
        "maintenance.07",
        7,
        "Copia de seguridad",
        "Copia de seguridad",
        "Respaldo de la información",
        icons::HARD_DRIVES,
    ),
];

const MAINTENANCE_GROUPS: [OptionGroup; 3] = [
    OptionGroup {
        label: "Personas",
        options: &PEOPLE,
    },
    OptionGroup {
        label: "Códigos",
        options: &CODES,
    },
    OptionGroup {
        label: "Sistema",
        options: &SYSTEM,
    },
];

// ------------------------------------------------------------------ Informes

const CASH_REPORTS: [NavOption; 2] = [
    option(
        "reports.01",
        1,
        "Caja diaria",
        "Caja diaria",
        "Movimientos y resumen de un día",
        icons::CASH_REGISTER,
    ),
    option(
        "reports.02",
        2,
        "Gastos del período",
        "Gastos del período",
        "Por código o por fecha",
        icons::RECEIPT,
    ),
];

const ACCUMULATED: [NavOption; 2] = [
    NavOption {
        permission: Permission::ViewAccumulated,
        ..option(
            "reports.03",
            3,
            "Acumulados por operación",
            "Acumulados del mes",
            "Comunes e hipotecarios por separado",
            icons::CHART_BAR,
        )
    },
    NavOption {
        permission: Permission::ViewAccumulated,
        ..option(
            "reports.04",
            4,
            "Inventario de contratos vigentes",
            "Inventario de vigentes",
            "Por código, oro por quilataje y plata",
            icons::STACK,
        )
    },
];

const CONTROL: [NavOption; 2] = [
    option(
        "reports.05",
        5,
        "Inventario de mercancía propia",
        "Mercancía propia",
        "Artículos de la compraventa",
        icons::PACKAGE,
    ),
    NavOption {
        permission: Permission::ViewAccumulated,
        ..option(
            "reports.06",
            6,
            "Auditor interno",
            "Auditor interno",
            "Modificaciones y anulaciones",
            icons::SHIELD_CHECK,
        )
    },
];

const REPORT_GROUPS: [OptionGroup; 3] = [
    OptionGroup {
        label: "Caja",
        options: &CASH_REPORTS,
    },
    OptionGroup {
        label: "Acumulados",
        options: &ACCUMULATED,
    },
    OptionGroup {
        label: "Inventarios y control",
        options: &CONTROL,
    },
];

/// Las tres secciones principales, en el orden de la barra superior.
pub static SECTIONS: [NavSection; 3] = [
    NavSection {
        id: SectionId::Operations,
        label: "Operaciones",
        subtitle: "Gestión de contratos, mercancías y movimientos",
        icon: icons::GEAR,
        groups: &OPERATION_GROUPS,
    },
    NavSection {
        id: SectionId::Maintenance,
        label: "Mantenimiento",
        subtitle: "Administración y configuración del sistema",
        icon: icons::WRENCH,
        groups: &MAINTENANCE_GROUPS,
    },
    NavSection {
        id: SectionId::Reports,
        label: "Informes",
        subtitle: "Consultas, reportes y análisis de información",
        icon: icons::CHART_BAR,
        groups: &REPORT_GROUPS,
    },
];

/// Sección por identificador.
#[must_use]
pub fn section(id: SectionId) -> &'static NavSection {
    // El arreglo contiene las tres variantes de `SectionId` (comprobado en pruebas).
    SECTIONS
        .iter()
        .find(|section| section.id == id)
        .unwrap_or(&SECTIONS[0])
}

/// Sección a la que pertenece una opción.
#[must_use]
pub fn section_of(option: &NavOption) -> &'static NavSection {
    SECTIONS
        .iter()
        .find(|section| section.options().any(|candidate| candidate.id == option.id))
        .unwrap_or(&SECTIONS[0])
}

/// Grupo al que pertenece una opción.
#[must_use]
pub fn group_of(option: &NavOption) -> Option<&'static OptionGroup> {
    SECTIONS
        .iter()
        .flat_map(|section| section.groups.iter())
        .find(|group| {
            group
                .options
                .iter()
                .any(|candidate| candidate.id == option.id)
        })
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn operations_has_eleven_options_numbered_in_order() {
        let numbers: Vec<u8> = section(SectionId::Operations)
            .options()
            .map(|o| o.number)
            .collect();
        assert_eq!(numbers, (1..=11).collect::<Vec<u8>>());
    }

    #[test]
    fn option_09_is_queries_and_10_is_modifications() {
        let operations = section(SectionId::Operations);
        let titles = (
            operations.option_number(9).map(|o| o.title),
            operations.option_number(10).map(|o| o.full_name),
        );
        assert_eq!(
            titles,
            (
                Some("Consultas varias"),
                Some("Modificar, reversar, anular y borrar")
            )
        );
    }

    #[test]
    fn option_ids_are_unique() {
        let ids: Vec<&str> = SECTIONS
            .iter()
            .flat_map(NavSection::options)
            .map(|o| o.id)
            .collect();
        let unique: HashSet<&str> = ids.iter().copied().collect();
        assert_eq!(unique.len(), ids.len());
    }

    #[test]
    fn every_section_id_has_a_section() {
        for id in [
            SectionId::Operations,
            SectionId::Maintenance,
            SectionId::Reports,
        ] {
            assert_eq!(section(id).id, id);
        }
    }

    #[test]
    fn modifications_offer_the_eight_actions_described_in_the_meeting() {
        let option = section(SectionId::Operations).option_number(10).unwrap();
        assert_eq!(option.actions.len(), 8);
    }

    #[test]
    fn expired_contracts_need_the_transfer_permission() {
        let option = section(SectionId::Operations).option_number(4).unwrap();
        assert_eq!(option.permission, Permission::TransferExpired);
    }
}
