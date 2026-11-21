//! Mantenimiento: clientes, usuarios, códigos y datos del negocio. Cada pantalla
//! lista lo existente y permite agregar con la captura por pasos.

use chrono::NaiveDate;
use egui::{RichText, Ui};
use pos_core::{
    application::{OpError, PosStore},
    domain::{
        CashConcept, ConceptCategory, Customer, GoldQuality, MerchandiseCode, Metal, Permission,
        Profile,
    },
};

use super::{Outcome, Screen, escape_pressed};
use crate::{
    components::{Card, EmptyState, SecondaryButton, info_banner},
    theme::{
        icons,
        tokens::{color, space},
    },
    views::{
        flow::{FlowCx, FlowSpec, StepDef, StepKind, Values},
        shared::{left, table},
    },
};

const AGAIN: &str = "¿Desea agregar otro?";

fn listing(ui: &mut Ui, title: &str, f: impl FnOnce(&mut Ui)) {
    Card::new().title(title).show(ui, f);
}

fn unique_code(exists: bool, value: &str) -> Result<(), String> {
    if exists {
        Err(OpError::DuplicateCode(value.to_owned()).to_string())
    } else {
        Ok(())
    }
}

// ------------------------------------------------------------------ Clientes

/// Registro de clientes.
pub struct Customers;

impl FlowSpec for Customers {
    fn steps(&self, _cx: &FlowCx<'_>, _values: &Values) -> Vec<StepDef> {
        vec![
            StepDef::new("id", "Cédula", StepKind::Text { optional: false }),
            StepDef::new(
                "name",
                "Nombre completo",
                StepKind::Text { optional: false },
            ),
            StepDef::new("address", "Dirección", StepKind::Text { optional: true }),
            StepDef::new("phone", "Teléfono", StepKind::Text { optional: true }),
        ]
    }

    fn check(
        &self,
        cx: &FlowCx<'_>,
        _values: &Values,
        step: &'static str,
        value: &str,
    ) -> Result<(), String> {
        if step == "id" {
            unique_code(cx.store.customer(value).is_some(), value)
        } else {
            Ok(())
        }
    }

    fn context(&self, ui: &mut Ui, cx: &FlowCx<'_>, _values: &Values) {
        listing(ui, "Clientes registrados", |ui| {
            let rows: Vec<Vec<String>> = cx
                .store
                .customers()
                .iter()
                .map(|c| vec![c.id_number.clone(), c.name.clone(), c.phone.clone()])
                .collect();
            table(
                ui,
                "customers",
                &[left("Cédula"), left("Nombre"), left("Teléfono")],
                &rows,
            );
        });
    }

    fn commit(
        &self,
        store: &mut PosStore,
        _today: NaiveDate,
        values: &Values,
    ) -> Result<String, OpError> {
        store.add_customer(Customer {
            id_number: values.get("id").to_owned(),
            name: values.get("name").to_owned(),
            address: values.get("address").to_owned(),
            phone: values.get("phone").to_owned(),
        })?;
        Ok(format!("Cliente {} registrado.", values.get("name")))
    }

    fn again_question(&self) -> &'static str {
        "¿Desea registrar otro cliente?"
    }
}

// ------------------------------------------------------------------ Usuarios

/// Usuarios y perfiles.
pub struct Users;

const PROFILE_OPTIONS: [(&str, &str); 2] = [("A", "Administrativo"), ("P", "Propietario")];

impl FlowSpec for Users {
    fn steps(&self, _cx: &FlowCx<'_>, _values: &Values) -> Vec<StepDef> {
        vec![
            StepDef::new(
                "code",
                "Código del usuario",
                StepKind::Text { optional: false },
            )
            .hint("Por ejemplo REC2."),
            StepDef::new("name", "Nombre", StepKind::Text { optional: false }),
            StepDef::new(
                "profile",
                "Perfil",
                StepKind::Choice(PROFILE_OPTIONS.to_vec()),
            ),
        ]
    }

    fn check(
        &self,
        cx: &FlowCx<'_>,
        _values: &Values,
        step: &'static str,
        value: &str,
    ) -> Result<(), String> {
        if step == "code" {
            unique_code(cx.store.users().iter().any(|u| u.code == value), value)
        } else {
            Ok(())
        }
    }

    fn context(&self, ui: &mut Ui, cx: &FlowCx<'_>, _values: &Values) {
        listing(ui, "Usuarios", |ui| {
            let rows: Vec<Vec<String>> = cx
                .store
                .users()
                .iter()
                .map(|u| vec![u.code.clone(), u.name.clone(), u.profile.label().to_owned()])
                .collect();
            table(
                ui,
                "users",
                &[left("Código"), left("Nombre"), left("Perfil")],
                &rows,
            );
        });
        ui.add_space(space::S);
        listing(ui, "Permisos por perfil", |ui| {
            let permissions = [
                (
                    Permission::Operate,
                    "Contratos, intereses, retiros, ventas, caja y consultas",
                ),
                (Permission::TransferExpired, "Pasar vencidos a propios"),
                (Permission::ModifyAndAnnul, "Modificar, reversar y anular"),
                (
                    Permission::ViewAccumulated,
                    "Acumulados, inventario de vigentes y auditoría",
                ),
                (
                    Permission::Administer,
                    "Usuarios, códigos y datos del negocio",
                ),
            ];
            let rows: Vec<Vec<String>> = permissions
                .iter()
                .map(|(permission, label)| {
                    let mark = |profile: Profile| {
                        if profile.allows(*permission) {
                            "Sí"
                        } else {
                            "No"
                        }
                        .to_owned()
                    };
                    vec![
                        (*label).to_owned(),
                        mark(Profile::Administrative),
                        mark(Profile::Owner),
                    ]
                })
                .collect();
            table(
                ui,
                "permissions",
                &[
                    left("Capacidad"),
                    left("Administrativo"),
                    left("Propietario"),
                ],
                &rows,
            );
        });
    }

    fn commit(
        &self,
        store: &mut PosStore,
        _today: NaiveDate,
        values: &Values,
    ) -> Result<String, OpError> {
        let profile = if values.get("profile") == "P" {
            Profile::Owner
        } else {
            Profile::Administrative
        };
        store.add_user(values.get("code"), values.get("name"), profile)?;
        Ok(format!("Usuario {} creado.", values.get("code")))
    }

    fn again_question(&self) -> &'static str {
        AGAIN
    }
}

// ------------------------------------------------------------------ Códigos de mercancía

/// Códigos de mercancía.
pub struct Merchandise;

const METAL_OPTIONS: [(&str, &str); 3] = [
    ("-", "Ninguno"),
    ("G", "Oro (pide quilataje)"),
    ("S", "Plata"),
];

impl FlowSpec for Merchandise {
    fn steps(&self, _cx: &FlowCx<'_>, _values: &Values) -> Vec<StepDef> {
        vec![
            StepDef::new("code", "Código", StepKind::Integer { min: 1, max: 999 }),
            StepDef::new("name", "Nombre", StepKind::Text { optional: false })
                .hint("Por ejemplo «Portátiles»."),
            StepDef::new(
                "metal",
                "¿Se pesa como metal precioso?",
                StepKind::Choice(METAL_OPTIONS.to_vec()),
            )
            .default_value("-"),
        ]
    }

    fn check(
        &self,
        cx: &FlowCx<'_>,
        _values: &Values,
        step: &'static str,
        value: &str,
    ) -> Result<(), String> {
        if step == "code" {
            unique_code(
                value
                    .parse()
                    .ok()
                    .and_then(|c| cx.store.merchandise_code(c))
                    .is_some(),
                value,
            )
        } else {
            Ok(())
        }
    }

    fn context(&self, ui: &mut Ui, cx: &FlowCx<'_>, _values: &Values) {
        listing(ui, "Códigos de mercancía", |ui| {
            let rows: Vec<Vec<String>> = cx
                .store
                .merchandise()
                .iter()
                .map(|m| {
                    let metal = match m.metal {
                        Some(Metal::Gold) => "Oro",
                        Some(Metal::Silver) => "Plata",
                        None => "",
                    };
                    vec![m.code.to_string(), m.name.clone(), metal.to_owned()]
                })
                .collect();
            table(
                ui,
                "merchandise",
                &[left("Código"), left("Nombre"), left("Metal")],
                &rows,
            );
        });
    }

    fn commit(
        &self,
        store: &mut PosStore,
        _today: NaiveDate,
        values: &Values,
    ) -> Result<String, OpError> {
        let metal = match values.get("metal") {
            "G" => Some(Metal::Gold),
            "S" => Some(Metal::Silver),
            _ => None,
        };
        let code = u16::try_from(values.number("code")).unwrap_or(0);
        store.add_merchandise(&MerchandiseCode {
            code,
            name: values.get("name").to_owned(),
            metal,
        })?;
        Ok(format!("Código {code} creado."))
    }

    fn again_question(&self) -> &'static str {
        AGAIN
    }
}

// ------------------------------------------------------------------ Calidades de oro

/// Calidades de oro (quilatajes).
pub struct GoldQualities;

impl FlowSpec for GoldQualities {
    fn steps(&self, _cx: &FlowCx<'_>, values: &Values) -> Vec<StepDef> {
        vec![
            StepDef::new("karat", "Quilataje", StepKind::Integer { min: 1, max: 24 }),
            StepDef::new("name", "Nombre", StepKind::Text { optional: false })
                .default_value(format!("{} quilates", values.get("karat"))),
        ]
    }

    fn check(
        &self,
        cx: &FlowCx<'_>,
        _values: &Values,
        step: &'static str,
        value: &str,
    ) -> Result<(), String> {
        if step == "karat" {
            unique_code(
                cx.store
                    .gold_qualities()
                    .iter()
                    .any(|g| g.karat.to_string() == value),
                value,
            )
        } else {
            Ok(())
        }
    }

    fn context(&self, ui: &mut Ui, cx: &FlowCx<'_>, _values: &Values) {
        listing(ui, "Calidades de oro", |ui| {
            let rows: Vec<Vec<String>> = cx
                .store
                .gold_qualities()
                .iter()
                .map(|g| vec![g.karat.to_string(), g.name.clone()])
                .collect();
            table(ui, "gold", &[left("Quilates"), left("Nombre")], &rows);
        });
    }

    fn commit(
        &self,
        store: &mut PosStore,
        _today: NaiveDate,
        values: &Values,
    ) -> Result<String, OpError> {
        let karat = u8::try_from(values.number("karat")).unwrap_or(0);
        store.add_gold_quality(&GoldQuality {
            karat,
            name: values.get("name").to_owned(),
        })?;
        Ok(format!("Calidad de {karat} quilates creada."))
    }

    fn again_question(&self) -> &'static str {
        AGAIN
    }
}

// ------------------------------------------------------------------ Conceptos

/// Conceptos de caja y banco.
pub struct Concepts;

const CATEGORY_OPTIONS: [(&str, &str); 4] = [
    ("E", "Gasto"),
    ("U", "Utilidad ocasional"),
    ("P", "Pérdida / ajuste de caja"),
    ("I", "Otro ingreso"),
];

impl FlowSpec for Concepts {
    fn steps(&self, _cx: &FlowCx<'_>, _values: &Values) -> Vec<StepDef> {
        vec![
            StepDef::new("code", "Código", StepKind::Integer { min: 1, max: 999 }),
            StepDef::new("name", "Nombre", StepKind::Text { optional: false })
                .hint("Por ejemplo «Cafetería»."),
            StepDef::new(
                "category",
                "Clase de concepto",
                StepKind::Choice(CATEGORY_OPTIONS.to_vec()),
            ),
        ]
    }

    fn check(
        &self,
        cx: &FlowCx<'_>,
        _values: &Values,
        step: &'static str,
        value: &str,
    ) -> Result<(), String> {
        if step == "code" {
            unique_code(
                value
                    .parse()
                    .ok()
                    .and_then(|c| cx.store.concept(c))
                    .is_some(),
                value,
            )
        } else {
            Ok(())
        }
    }

    fn context(&self, ui: &mut Ui, cx: &FlowCx<'_>, _values: &Values) {
        listing(ui, "Conceptos de caja y banco", |ui| {
            let rows: Vec<Vec<String>> = cx
                .store
                .concepts()
                .iter()
                .map(|c| {
                    vec![
                        c.code.to_string(),
                        c.name.clone(),
                        c.category.label().to_owned(),
                        c.category.direction().label().to_owned(),
                    ]
                })
                .collect();
            table(
                ui,
                "concepts",
                &[left("Código"), left("Nombre"), left("Clase"), left("Tipo")],
                &rows,
            );
        });
    }

    fn commit(
        &self,
        store: &mut PosStore,
        _today: NaiveDate,
        values: &Values,
    ) -> Result<String, OpError> {
        let category = match values.get("category") {
            "U" => ConceptCategory::OccasionalProfit,
            "P" => ConceptCategory::Loss,
            "I" => ConceptCategory::OtherIncome,
            _ => ConceptCategory::Expense,
        };
        let code = u16::try_from(values.number("code")).unwrap_or(0);
        store.add_concept(&CashConcept {
            code,
            name: values.get("name").to_owned(),
            category,
        })?;
        Ok(format!("Concepto {code} creado."))
    }

    fn again_question(&self) -> &'static str {
        AGAIN
    }
}

// ------------------------------------------------------------------ Datos del negocio

/// Nombre del negocio (el cliente usa el programa en tres negocios).
pub struct Business;

impl FlowSpec for Business {
    fn steps(&self, cx: &FlowCx<'_>, _values: &Values) -> Vec<StepDef> {
        vec![
            StepDef::new(
                "name",
                "Nombre del negocio",
                StepKind::Text { optional: false },
            )
            .hint("Aparece en el encabezado de la aplicación.")
            .default_value(cx.store.business_name()),
        ]
    }

    fn context(&self, ui: &mut Ui, _cx: &FlowCx<'_>, _values: &Values) {
        info_banner(
            ui,
            "Cada negocio tiene su propia instalación y sus propios datos.",
        );
    }

    fn commit(
        &self,
        store: &mut PosStore,
        _today: NaiveDate,
        values: &Values,
    ) -> Result<String, OpError> {
        store.set_business_name(values.get("name"))?;
        Ok(String::from("Nombre del negocio actualizado."))
    }

    fn again_question(&self) -> &'static str {
        "¿Desea cambiarlo de nuevo?"
    }
}

// ------------------------------------------------------------------ Copia de seguridad

/// Copia de seguridad: pantalla informativa (pregunta P9).
pub struct Backup;

impl Screen for Backup {
    fn ui(&mut self, ui: &mut Ui, _store: &mut PosStore, _today: NaiveDate) -> Outcome {
        EmptyState::new("Copia de seguridad")
            .icon(icons::HARD_DRIVES)
            .description("En el sistema anterior es un programa aparte que copia la carpeta de datos a otro disco. La demo no guarda datos, así que no hay nada que respaldar todavía.")
            .show(ui);
        ui.vertical_centered(|ui| {
            ui.add_enabled(false, SecondaryButton::new("Crear copia de seguridad"))
                .on_disabled_hover_text("Disponible cuando exista la base de datos (pregunta P9).");
            ui.add_space(space::S);
            ui.label(RichText::new("Esc para volver").color(color::TEXT_MUTED));
        });
        if escape_pressed(ui) {
            Outcome::Back
        } else {
            Outcome::Stay
        }
    }
}
