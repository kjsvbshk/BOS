//! Clientes, usuarios y perfiles.

/// Cliente identificado por su cédula.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Customer {
    pub id_number: String,
    pub name: String,
    pub address: String,
    pub phone: String,
}

/// Perfil de usuario (P8: el cliente propone sólo dos).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Profile {
    /// Personal de atención: opera contratos, intereses y retiros.
    Administrative,
    /// Dueño: además pasa vencidos a propios, modifica y anula, y ve acumulados.
    Owner,
}

impl Profile {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Administrative => "Administrativo",
            Self::Owner => "Propietario",
        }
    }

    /// Si el perfil tiene el permiso. Las vistas preguntan por permisos, nunca
    /// por el nombre del perfil.
    #[must_use]
    pub const fn allows(self, permission: Permission) -> bool {
        match self {
            Self::Owner => true,
            Self::Administrative => matches!(permission, Permission::Operate),
        }
    }

    pub const ALL: [Self; 2] = [Self::Administrative, Self::Owner];
}

/// Capacidades que se pueden restringir por perfil (reunión, 54:14–55:14).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Permission {
    /// Ingresar contratos, intereses, retiros, ventas, caja y consultas.
    Operate,
    /// Pasar contratos vencidos a mercancía propia.
    TransferExpired,
    /// Modificar, reversar y anular.
    ModifyAndAnnul,
    /// Ver acumulados del negocio y auditoría.
    ViewAccumulated,
    /// Administrar usuarios y catálogos.
    Administer,
}

impl Permission {
    /// Motivo que se muestra cuando falta el permiso.
    #[must_use]
    pub const fn denied_reason(self) -> &'static str {
        match self {
            Self::Operate => "Su perfil no puede realizar operaciones.",
            Self::TransferExpired => {
                "Sólo el propietario puede pasar contratos vencidos a propios."
            }
            Self::ModifyAndAnnul => "Sólo el propietario puede modificar, reversar o anular.",
            Self::ViewAccumulated => "Sólo el propietario puede ver los acumulados del negocio.",
            Self::Administer => "Sólo el propietario puede administrar usuarios y códigos.",
        }
    }
}

/// Usuario del sistema.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub code: String,
    pub name: String,
    pub profile: Profile,
}
