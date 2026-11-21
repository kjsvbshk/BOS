//! Catálogo de navegación: única fuente de secciones, grupos y opciones
//! (ADR-0006). Para agregar, quitar o reordenar opciones sólo se edita este
//! archivo; las vistas recorren el catálogo.
//!
//! Los nombres oficiales (`full_name`) vienen de la distribución aprobada el
//! 2026-11-21. En las tarjetas se muestran partidos en `title` y `description`,
//! que juntos repiten el nombre oficial sin agregar información.

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
    /// Nombre conocido; su proceso todavía no está documentado.
    AwaitingSpecification,
    /// Todavía no se sabe qué opción del sistema anterior ocupa este lugar.
    Unidentified,
}

/// Acción dentro de una opción (por ejemplo, «Reversar» en la opción 09).
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct NavAction {
    pub id: &'static str,
    pub label: &'static str,
    pub icon: &'static str,
}

/// Opción de menú dentro de una sección.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct NavOption {
    /// Identificador estable; no cambia aunque cambie el nombre.
    pub id: &'static str,
    /// Número de la opción en el sistema original.
    pub number: u8,
    /// Nombre oficial completo.
    pub full_name: &'static str,
    /// Primera línea de la tarjeta.
    pub title: &'static str,
    /// Segunda línea de la tarjeta.
    pub description: &'static str,
    pub icon: &'static str,
    pub status: OptionStatus,
    /// Acciones que la opción muestra al abrirse.
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
    /// Vacío mientras las opciones estén pendientes de levantamiento.
    pub groups: &'static [OptionGroup],
}

impl NavSection {
    /// Todas las opciones de la sección, en su orden original.
    pub fn options(&self) -> impl Iterator<Item = &'static NavOption> + use<> {
        let groups: &'static [OptionGroup] = self.groups;
        groups.iter().flat_map(|group| group.options.iter())
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
        status: OptionStatus::AwaitingSpecification,
        actions: &[],
    }
}

const CONTRACT_CHANGES: [NavAction; 4] = [
    NavAction {
        id: "reverse",
        label: "Reversar",
        icon: icons::ARROW_COUNTER_CLOCKWISE,
    },
    NavAction {
        id: "cancel",
        label: "Anular",
        icon: icons::PROHIBIT,
    },
    NavAction {
        id: "modify",
        label: "Modificar",
        icon: icons::PENCIL_SIMPLE,
    },
    NavAction {
        id: "delete",
        label: "Borrar",
        icon: icons::TRASH,
    },
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
    option(
        "operations.04",
        4,
        "Contratos vencidos – Pasar al inventario de mercancía propia",
        "Contratos vencidos",
        "Pasar al inventario de mercancía propia",
        icons::HOURGLASS_HIGH,
    ),
];

const INVENTORY: [NavOption; 2] = [
    option(
        "operations.05",
        5,
        "Ingresar mercancía al inventario de mercancía propia",
        "Ingresar mercancía",
        "Al inventario de mercancía propia",
        icons::PACKAGE,
    ),
    option(
        "operations.06",
        6,
        "Salida del inventario de mercancías propias",
        "Salida de mercancías",
        "Del inventario de mercancías propias",
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
        "De banco",
        icons::BANK,
    ),
    NavOption {
        actions: &CONTRACT_CHANGES,
        ..option(
            "operations.09",
            9,
            "Modificación de contratos",
            "Modificación de contratos",
            "Reversar, anular, modificar o borrar",
            icons::ARROWS_CLOCKWISE,
        )
    },
    option(
        "operations.10",
        10,
        "Otras opciones",
        "Otras opciones",
        "Contenido pendiente de definición",
        icons::SQUARES_FOUR,
    ),
    NavOption {
        status: OptionStatus::Unidentified,
        ..option(
            "operations.11",
            11,
            "Pendiente de identificar",
            "Pendiente de identificar",
            "Opción del sistema anterior por confirmar",
            icons::QUESTION,
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
        groups: &[],
    },
    NavSection {
        id: SectionId::Reports,
        label: "Informes",
        subtitle: "Consultas, reportes y análisis de información",
        icon: icons::CHART_BAR,
        groups: &[],
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
    fn maintenance_and_reports_have_no_options_yet() {
        assert_eq!(section(SectionId::Maintenance).options().count(), 0);
        assert_eq!(section(SectionId::Reports).options().count(), 0);
    }

    #[test]
    fn contract_modification_offers_its_four_actions() {
        let option = section(SectionId::Operations)
            .options()
            .find(|o| o.number == 9)
            .unwrap();
        let labels: Vec<&str> = option.actions.iter().map(|a| a.label).collect();
        assert_eq!(labels, ["Reversar", "Anular", "Modificar", "Borrar"]);
    }
}
