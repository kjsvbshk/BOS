//! Almacén en memoria con las operaciones del negocio (ADR-0008).
//!
//! La situación de cada contrato se deriva de sus movimientos no anulados:
//! anular o reversar marca un movimiento como anulado y deja constancia en la
//! auditoría; nunca se borra nada.

use std::collections::BTreeMap;

use chrono::NaiveDate;

use crate::domain::{
    CashConcept, Contract, ContractKind, ContractStatus, Customer, GoldQuality, Item,
    MerchandiseCode, Money, Movement, MovementKind, Permission, Profile, Rate, User, interest,
};

/// Primer número de movimiento (la reunión muestra el 8600222, 32:31).
const FIRST_MOVEMENT: u32 = 8_600_200;
/// Primer número de los contratos no comunes, que numera el sistema.
const FIRST_NON_COMMON: u32 = 900_001;

/// Error de una operación, con un mensaje que explica qué hacer.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum OpError {
    #[error("No existe el contrato {0}.")]
    ContractNotFound(u32),
    #[error("El contrato {0} ya existe. Escriba otro número.")]
    ContractExists(u32),
    #[error("No hay un cliente con la cédula {0}. Regístrelo en Mantenimiento › Clientes.")]
    CustomerNotFound(String),
    #[error("No existe el código {0}. Consulte los códigos con F1.")]
    UnknownCode(u16),
    #[error(
        "El contrato {number} está en estado «{status}»; esta operación necesita un contrato {expected}."
    )]
    WrongStatus {
        number: u32,
        status: &'static str,
        expected: &'static str,
    },
    #[error("El contrato {number} vence el {due}; todavía no se puede pasar a propios.")]
    NotExpired { number: u32, due: NaiveDate },
    #[error("No existe el movimiento {0}.")]
    MovementNotFound(u32),
    #[error("El movimiento {0} ya está anulado.")]
    MovementAlreadyAnnulled(u32),
    #[error("{0}")]
    NotAllowed(&'static str),
    #[error("{0}")]
    InvalidValue(&'static str),
    #[error("{0}")]
    PermissionDenied(&'static str),
    #[error("El código {0} ya existe.")]
    DuplicateCode(String),
}

/// Datos para registrar un contrato.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewContract {
    /// Obligatorio en los comunes; los no comunes lo recibe del sistema.
    pub number: Option<u32>,
    pub kind: ContractKind,
    pub customer: String,
    pub date: NaiveDate,
    pub term_months: u32,
    pub rate: Rate,
    pub capital: Money,
    pub item: Item,
}

/// Campo que se corrige en «Modificar contrato».
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContractChange {
    Capital(Money),
    Rate(Rate),
    Term(u32),
    Date(NaiveDate),
    Description(String),
}

/// Registro de la auditoría interna.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditEntry {
    pub date: NaiveDate,
    pub user: String,
    pub contract: Option<u32>,
    pub movement: Option<u32>,
    pub action: &'static str,
    pub detail: String,
}

/// Pago registrado en el historial de un contrato.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PaymentLine {
    pub movement: u32,
    pub date: NaiveDate,
    pub months: u32,
    pub amount: Money,
}

/// Datos de un separado.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayawayInfo {
    pub movement: u32,
    pub date: NaiveDate,
    pub price: Money,
    pub customer: Option<String>,
    pub delivery: NaiveDate,
    pub payments: Vec<PaymentLine>,
    pub paid: Money,
    pub balance: Money,
}

/// Situación de un contrato a una fecha: todo lo que muestra la consulta.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContractView {
    pub contract: Contract,
    pub status: ContractStatus,
    /// Capital después de abonos.
    pub capital: Money,
    pub monthly_interest: Money,
    pub months_accrued: u32,
    pub months_paid: u32,
    pub months_pending: u32,
    /// Capital + intereses pendientes a la fecha de la consulta.
    pub payoff: Money,
    pub due_date: NaiveDate,
    pub interest_payments: Vec<PaymentLine>,
    pub capital_payments: Vec<PaymentLine>,
    /// Fecha y total del retiro.
    pub withdrawal: Option<(NaiveDate, Money)>,
    pub owned_since: Option<NaiveDate>,
    /// Fecha, precio y valor de referencia de la venta.
    pub sale: Option<(NaiveDate, Money, Money)>,
    pub layaway: Option<LayawayInfo>,
}

impl ContractView {
    /// Intereses pagados en total.
    #[must_use]
    pub fn interest_paid(&self) -> Money {
        self.interest_payments.iter().map(|p| p.amount).sum()
    }
}

/// Filtro de «Contratos vencidos» (opción 04).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExpiredFilter {
    /// Código de mercancía; 0 = inventario general.
    pub code: u16,
    /// `true`: comunes y no comunes; `false`: sólo comunes.
    pub both_kinds: bool,
    pub interest: InterestFilter,
}

/// Filtro por pagos de intereses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterestFilter {
    /// Con y sin pagos (lo que el cliente usa siempre, 19:50).
    Any,
    Paid,
    Unpaid,
}

/// Almacén de la demo.
#[derive(Debug, Clone)]
pub struct PosStore {
    business_name: String,
    users: Vec<User>,
    current_user: usize,
    customers: Vec<Customer>,
    merchandise: Vec<MerchandiseCode>,
    gold: Vec<GoldQuality>,
    concepts: Vec<CashConcept>,
    contracts: BTreeMap<u32, Contract>,
    movements: Vec<Movement>,
    audit: Vec<AuditEntry>,
    next_movement: u32,
    next_non_common: u32,
}

impl PosStore {
    /// Almacén vacío con un usuario propietario.
    #[must_use]
    pub fn new(business_name: impl Into<String>, owner: User) -> Self {
        Self {
            business_name: business_name.into(),
            users: vec![owner],
            current_user: 0,
            customers: Vec::new(),
            merchandise: Vec::new(),
            gold: Vec::new(),
            concepts: Vec::new(),
            contracts: BTreeMap::new(),
            movements: Vec::new(),
            audit: Vec::new(),
            next_movement: FIRST_MOVEMENT,
            next_non_common: FIRST_NON_COMMON,
        }
    }

    // ---------------------------------------------------------------- Consultas

    #[must_use]
    pub fn business_name(&self) -> &str {
        &self.business_name
    }

    #[must_use]
    pub fn users(&self) -> &[User] {
        &self.users
    }

    /// Usuario con la sesión abierta.
    #[must_use]
    pub fn current_user(&self) -> &User {
        &self.users[self.current_user]
    }

    #[must_use]
    pub fn can(&self, permission: Permission) -> bool {
        self.current_user().profile.allows(permission)
    }

    #[must_use]
    pub fn customers(&self) -> &[Customer] {
        &self.customers
    }

    #[must_use]
    pub fn customer(&self, id_number: &str) -> Option<&Customer> {
        self.customers
            .iter()
            .find(|c| c.id_number == id_number.trim())
    }

    #[must_use]
    pub fn merchandise(&self) -> &[MerchandiseCode] {
        &self.merchandise
    }

    #[must_use]
    pub fn merchandise_code(&self, code: u16) -> Option<&MerchandiseCode> {
        self.merchandise.iter().find(|m| m.code == code)
    }

    #[must_use]
    pub fn gold_qualities(&self) -> &[GoldQuality] {
        &self.gold
    }

    #[must_use]
    pub fn concepts(&self) -> &[CashConcept] {
        &self.concepts
    }

    #[must_use]
    pub fn concept(&self, code: u16) -> Option<&CashConcept> {
        self.concepts.iter().find(|c| c.code == code)
    }

    #[must_use]
    pub fn contract(&self, number: u32) -> Option<&Contract> {
        self.contracts.get(&number)
    }

    /// Número que recibirá el próximo contrato no común.
    #[must_use]
    pub fn next_non_common_number(&self) -> u32 {
        self.next_non_common
    }

    #[must_use]
    pub fn movement(&self, number: u32) -> Option<&Movement> {
        self.movements.iter().find(|m| m.number == number)
    }

    #[must_use]
    pub fn movements(&self) -> &[Movement] {
        &self.movements
    }

    /// Movimientos de un contrato, anulados incluidos, en orden.
    pub fn movements_of(&self, contract: u32) -> impl Iterator<Item = &Movement> {
        self.movements
            .iter()
            .filter(move |m| m.contract == Some(contract))
    }

    #[must_use]
    pub fn audit(&self) -> &[AuditEntry] {
        &self.audit
    }

    /// Situación del contrato a la fecha `on`.
    #[must_use]
    pub fn view(&self, number: u32, on: NaiveDate) -> Option<ContractView> {
        let contract = self.contracts.get(&number)?;
        Some(self.build_view(contract, on))
    }

    /// Contratos de un cliente, del más reciente al más antiguo.
    #[must_use]
    pub fn contracts_of(&self, customer: &str, on: NaiveDate) -> Vec<ContractView> {
        let mut views: Vec<ContractView> = self
            .contracts
            .values()
            .filter(|c| c.customer == customer)
            .map(|c| self.build_view(c, on))
            .collect();
        views.sort_by_key(|v| std::cmp::Reverse(v.contract.date));
        views
    }

    /// Todos los contratos con su situación.
    pub fn all_views(&self, on: NaiveDate) -> impl Iterator<Item = ContractView> + '_ {
        self.contracts.values().map(move |c| self.build_view(c, on))
    }

    /// Contratos vencidos según el filtro de la opción 04.
    #[must_use]
    pub fn expired(&self, filter: ExpiredFilter, on: NaiveDate) -> Vec<ContractView> {
        self.all_views(on)
            .filter(|v| v.status == ContractStatus::Expired)
            .filter(|v| filter.code == 0 || v.contract.item.code == filter.code)
            .filter(|v| filter.both_kinds || v.contract.kind == ContractKind::Common)
            .filter(|v| match filter.interest {
                InterestFilter::Any => true,
                InterestFilter::Paid => !v.interest_payments.is_empty(),
                InterestFilter::Unpaid => v.interest_payments.is_empty(),
            })
            .collect()
    }

    /// Mercancía propia (incluye la separada).
    #[must_use]
    pub fn owned(&self, on: NaiveDate) -> Vec<ContractView> {
        self.all_views(on)
            .filter(|v| matches!(v.status, ContractStatus::Owned | ContractStatus::Layaway))
            .collect()
    }

    fn active_movements(&self, contract: u32) -> impl Iterator<Item = &Movement> {
        self.movements_of(contract).filter(|m| !m.annulled)
    }

    fn build_view(&self, contract: &Contract, on: NaiveDate) -> ContractView {
        let mut capital = contract.capital;
        let mut interest_payments = Vec::new();
        let mut capital_payments = Vec::new();
        let mut withdrawal = None;
        let mut owned_since = None;
        let mut sale = None;
        let mut layaway: Option<LayawayInfo> = None;

        for movement in self.active_movements(contract.number) {
            match &movement.kind {
                MovementKind::InterestPayment { months, amount } => {
                    interest_payments.push(line(movement, *months, *amount));
                }
                MovementKind::CapitalPayment { amount } => {
                    capital -= *amount;
                    capital_payments.push(line(movement, 0, *amount));
                }
                MovementKind::Withdrawal { capital, interest } => {
                    withdrawal = Some((movement.date, *capital + *interest));
                }
                MovementKind::TransferToOwned => owned_since = Some(movement.date),
                MovementKind::Sale {
                    price, reference, ..
                } => {
                    sale = Some((movement.date, *price, *reference));
                }
                MovementKind::Layaway {
                    price,
                    customer,
                    delivery,
                } => {
                    layaway = Some(LayawayInfo {
                        movement: movement.number,
                        date: movement.date,
                        price: *price,
                        customer: customer.clone(),
                        delivery: *delivery,
                        payments: Vec::new(),
                        paid: Money::ZERO,
                        balance: *price,
                    });
                }
                MovementKind::LayawayPayment { amount } => {
                    if let Some(info) = layaway.as_mut() {
                        info.payments.push(line(movement, 0, *amount));
                        info.paid += *amount;
                        info.balance = info.price - info.paid;
                    }
                }
                MovementKind::Pawn | MovementKind::Cash { .. } => {}
            }
        }

        let months_paid: u32 = interest_payments.iter().map(|p| p.months).sum();
        let valuation_date = withdrawal.map_or(on, |(date, _)| date);
        let months_accrued = interest::months_accrued(contract.date, valuation_date);
        let months_pending = interest::months_pending(months_accrued, months_paid);
        let due_date = interest::due_date(contract.date, contract.term_months, months_paid);

        let status = if contract.annulled {
            ContractStatus::Annulled
        } else if withdrawal.is_some() {
            ContractStatus::Withdrawn
        } else if sale.is_some() || layaway.as_ref().is_some_and(|l| !l.balance.is_positive()) {
            ContractStatus::Sold
        } else if layaway.is_some() {
            ContractStatus::Layaway
        } else if owned_since.is_some() {
            ContractStatus::Owned
        } else if on > due_date {
            ContractStatus::Expired
        } else {
            ContractStatus::Active
        };

        ContractView {
            contract: contract.clone(),
            status,
            capital,
            monthly_interest: interest::monthly_interest(capital, contract.rate),
            months_accrued,
            months_paid,
            months_pending,
            payoff: interest::payoff(capital, contract.rate, months_pending),
            due_date,
            interest_payments,
            capital_payments,
            withdrawal,
            owned_since,
            sale,
            layaway,
        }
    }

    // ---------------------------------------------------------------- Operaciones

    /// 01 · Registra un contrato. Devuelve su número y el del movimiento.
    ///
    /// # Errors
    /// Si falta el número en un contrato común, el número ya existe, el cliente
    /// o el código no existen, o algún valor no es válido.
    pub fn register_contract(&mut self, new: NewContract) -> Result<(u32, u32), OpError> {
        self.require(Permission::Operate)?;
        if self.customer(&new.customer).is_none() {
            return Err(OpError::CustomerNotFound(new.customer));
        }
        if self.merchandise_code(new.item.code).is_none() {
            return Err(OpError::UnknownCode(new.item.code));
        }
        if !new.capital.is_positive() {
            return Err(OpError::InvalidValue(
                "El valor del empeño debe ser mayor que cero.",
            ));
        }
        if new.term_months == 0 {
            return Err(OpError::InvalidValue(
                "El plazo debe ser de al menos un mes.",
            ));
        }
        if new.rate.hundredths() == 0 {
            return Err(OpError::InvalidValue(
                "El porcentaje debe ser mayor que cero.",
            ));
        }
        let number = match (new.kind, new.number) {
            (ContractKind::Common, Some(number)) => number,
            (ContractKind::Common, None) => {
                return Err(OpError::InvalidValue("Escriba el número del contrato."));
            }
            (ContractKind::NonCommon, _) => self.next_non_common,
        };
        if self.contracts.contains_key(&number) {
            return Err(OpError::ContractExists(number));
        }
        if new.kind == ContractKind::NonCommon {
            self.next_non_common += 1;
        }
        self.contracts.insert(
            number,
            Contract {
                number,
                kind: new.kind,
                customer: new.customer.trim().to_owned(),
                date: new.date,
                term_months: new.term_months,
                rate: new.rate,
                capital: new.capital,
                item: new.item,
                annulled: false,
            },
        );
        let movement = self.push_movement(new.date, Some(number), MovementKind::Pawn);
        Ok((number, movement))
    }

    /// 02 · Cambia el plazo durante una prórroga.
    ///
    /// # Errors
    /// Si el contrato no existe, no está vigente o vencido, o el plazo es cero.
    pub fn change_term(
        &mut self,
        number: u32,
        term_months: u32,
        on: NaiveDate,
    ) -> Result<(), OpError> {
        self.require(Permission::Operate)?;
        self.expect_status(
            number,
            on,
            &[ContractStatus::Active, ContractStatus::Expired],
            "vigente o vencido",
        )?;
        if term_months == 0 {
            return Err(OpError::InvalidValue(
                "El plazo debe ser de al menos un mes.",
            ));
        }
        let old = self.contract_mut(number)?.term_months;
        self.contract_mut(number)?.term_months = term_months;
        self.record(
            on,
            Some(number),
            None,
            "Cambio de plazo",
            format!("{old} → {term_months} meses"),
        );
        Ok(())
    }

    /// 02 · Registra el pago de `months` meses de intereses. Devuelve el
    /// movimiento y el importe.
    ///
    /// # Errors
    /// Si el contrato no existe, no está vigente o vencido, o `months` es cero.
    pub fn pay_interest(
        &mut self,
        number: u32,
        months: u32,
        on: NaiveDate,
    ) -> Result<(u32, Money), OpError> {
        self.require(Permission::Operate)?;
        let view = self.expect_status(
            number,
            on,
            &[ContractStatus::Active, ContractStatus::Expired],
            "vigente o vencido",
        )?;
        if months == 0 {
            return Err(OpError::InvalidValue("Pague al menos un mes de intereses."));
        }
        let amount = interest::extension_amount(view.capital, view.contract.rate, months);
        let movement = self.push_movement(
            on,
            Some(number),
            MovementKind::InterestPayment { months, amount },
        );
        Ok((movement, amount))
    }

    /// 03 · Retira el contrato: el cliente paga capital e intereses pendientes.
    /// Devuelve el movimiento y el total.
    ///
    /// # Errors
    /// Si el contrato no existe o no está vigente o vencido.
    pub fn withdraw(&mut self, number: u32, on: NaiveDate) -> Result<(u32, Money), OpError> {
        self.require(Permission::Operate)?;
        let view = self.expect_status(
            number,
            on,
            &[ContractStatus::Active, ContractStatus::Expired],
            "vigente o vencido",
        )?;
        let interest = view.payoff - view.capital;
        let movement = self.push_movement(
            on,
            Some(number),
            MovementKind::Withdrawal {
                capital: view.capital,
                interest,
            },
        );
        Ok((movement, view.payoff))
    }

    /// 04 · Pasa un contrato vencido a mercancía propia.
    ///
    /// # Errors
    /// Sin permiso, si el contrato no existe o todavía no ha vencido.
    pub fn transfer_to_owned(&mut self, number: u32, on: NaiveDate) -> Result<u32, OpError> {
        self.require(Permission::TransferExpired)?;
        let view = self
            .view(number, on)
            .ok_or(OpError::ContractNotFound(number))?;
        match view.status {
            ContractStatus::Expired => {}
            ContractStatus::Active => {
                return Err(OpError::NotExpired {
                    number,
                    due: view.due_date,
                });
            }
            other => return Err(wrong_status(number, other, "vencido")),
        }
        Ok(self.push_movement(on, Some(number), MovementKind::TransferToOwned))
    }

    /// 06 · Vende mercancía propia. Devuelve el movimiento, el valor de
    /// referencia y la diferencia (negativa = pérdida).
    ///
    /// # Errors
    /// Si el artículo no es mercancía propia o el precio no es válido.
    pub fn sell(
        &mut self,
        number: u32,
        price: Money,
        buyer: Option<String>,
        on: NaiveDate,
    ) -> Result<(u32, Money, Money), OpError> {
        self.require(Permission::Operate)?;
        let view = self.expect_owned(number, on)?;
        if !price.is_positive() {
            return Err(OpError::InvalidValue(
                "El valor de venta debe ser mayor que cero.",
            ));
        }
        let reference = view.payoff;
        let movement = self.push_movement(
            on,
            Some(number),
            MovementKind::Sale {
                price,
                reference,
                buyer,
            },
        );
        Ok((
            movement,
            reference,
            interest::sale_difference(price, reference),
        ))
    }

    /// 06 · Separa mercancía propia (venta a plazos).
    ///
    /// # Errors
    /// Si el artículo no es mercancía propia, el valor no es válido o la fecha
    /// de entrega es anterior a hoy.
    pub fn layaway(
        &mut self,
        number: u32,
        price: Money,
        customer: Option<String>,
        delivery: NaiveDate,
        on: NaiveDate,
    ) -> Result<u32, OpError> {
        self.require(Permission::Operate)?;
        self.expect_owned(number, on)?;
        if !price.is_positive() {
            return Err(OpError::InvalidValue(
                "El valor del separado debe ser mayor que cero.",
            ));
        }
        if delivery < on {
            return Err(OpError::InvalidValue(
                "La fecha de entrega no puede ser anterior a la de hoy.",
            ));
        }
        Ok(self.push_movement(
            on,
            Some(number),
            MovementKind::Layaway {
                price,
                customer,
                delivery,
            },
        ))
    }

    /// 06 · Abono a una mercancía separada. Devuelve el movimiento y el saldo.
    ///
    /// # Errors
    /// Si el contrato no está separado o el abono supera el saldo.
    pub fn layaway_payment(
        &mut self,
        number: u32,
        amount: Money,
        on: NaiveDate,
    ) -> Result<(u32, Money), OpError> {
        self.require(Permission::Operate)?;
        let view = self.expect_status(number, on, &[ContractStatus::Layaway], "separado")?;
        let balance = view.layaway.map_or(Money::ZERO, |l| l.balance);
        if !amount.is_positive() {
            return Err(OpError::InvalidValue("El abono debe ser mayor que cero."));
        }
        if amount > balance {
            return Err(OpError::InvalidValue(
                "El abono supera lo que falta por pagar.",
            ));
        }
        let movement =
            self.push_movement(on, Some(number), MovementKind::LayawayPayment { amount });
        Ok((movement, balance - amount))
    }

    /// 07 / 08 · Ingreso o egreso de caja o banco.
    ///
    /// # Errors
    /// Si el concepto no existe o el valor no es válido.
    pub fn cash_movement(
        &mut self,
        concept: u16,
        description: String,
        amount: Money,
        bank: bool,
        on: NaiveDate,
    ) -> Result<u32, OpError> {
        self.require(Permission::Operate)?;
        let direction = self
            .concept(concept)
            .ok_or(OpError::UnknownCode(concept))?
            .category
            .direction();
        if !amount.is_positive() {
            return Err(OpError::InvalidValue("El valor debe ser mayor que cero."));
        }
        Ok(self.push_movement(
            on,
            None,
            MovementKind::Cash {
                concept,
                direction,
                description,
                amount,
                bank,
            },
        ))
    }

    /// 10 · Corrige un dato del contrato y lo registra en la auditoría.
    ///
    /// # Errors
    /// Sin permiso, si el contrato no existe o el valor no es válido.
    pub fn modify_contract(
        &mut self,
        number: u32,
        change: ContractChange,
        on: NaiveDate,
    ) -> Result<(), OpError> {
        self.require(Permission::ModifyAndAnnul)?;
        let contract = self.contract_mut(number)?;
        if contract.annulled {
            return Err(wrong_status(number, ContractStatus::Annulled, "no anulado"));
        }
        let (field, detail) = match change {
            ContractChange::Capital(value) => {
                if !value.is_positive() {
                    return Err(OpError::InvalidValue("El capital debe ser mayor que cero."));
                }
                let detail = format!("{} → {value}", contract.capital);
                contract.capital = value;
                ("Capital", detail)
            }
            ContractChange::Rate(value) => {
                let detail = format!("{} → {value}", contract.rate);
                contract.rate = value;
                ("Porcentaje", detail)
            }
            ContractChange::Term(value) => {
                if value == 0 {
                    return Err(OpError::InvalidValue(
                        "El plazo debe ser de al menos un mes.",
                    ));
                }
                let detail = format!("{} → {value} meses", contract.term_months);
                contract.term_months = value;
                ("Plazo", detail)
            }
            ContractChange::Date(value) => {
                let detail = format!(
                    "{} → {}",
                    contract.date.format("%d/%m/%Y"),
                    value.format("%d/%m/%Y")
                );
                contract.date = value;
                ("Fecha", detail)
            }
            ContractChange::Description(value) => {
                let detail = format!("«{}» → «{value}»", contract.item.description);
                contract.item.description = value;
                ("Descripción", detail)
            }
        };
        self.record(
            on,
            Some(number),
            None,
            "Modificación de contrato",
            format!("{field}: {detail}"),
        );
        Ok(())
    }

    /// 10 · Abono a capital. Devuelve el movimiento y el nuevo capital.
    ///
    /// # Errors
    /// Sin permiso, si el contrato no está vigente o vencido, o el abono no es
    /// menor que el capital (para saldar el contrato se usa el retiro).
    pub fn capital_payment(
        &mut self,
        number: u32,
        amount: Money,
        on: NaiveDate,
    ) -> Result<(u32, Money), OpError> {
        self.require(Permission::ModifyAndAnnul)?;
        let view = self.expect_status(
            number,
            on,
            &[ContractStatus::Active, ContractStatus::Expired],
            "vigente o vencido",
        )?;
        if !amount.is_positive() {
            return Err(OpError::InvalidValue("El abono debe ser mayor que cero."));
        }
        if amount >= view.capital {
            return Err(OpError::InvalidValue(
                "El abono debe ser menor que el capital; para saldarlo, use Retiro de contrato.",
            ));
        }
        let movement =
            self.push_movement(on, Some(number), MovementKind::CapitalPayment { amount });
        Ok((movement, view.capital - amount))
    }

    /// 10 · Anula un movimiento por su número. El contrato vuelve a la
    /// situación anterior.
    ///
    /// # Errors
    /// Sin permiso, si el movimiento no existe, ya está anulado, es un empeño
    /// (se anula el contrato) o tiene operaciones posteriores que dependen de él.
    pub fn annul_movement(&mut self, number: u32, on: NaiveDate) -> Result<(), OpError> {
        self.require(Permission::ModifyAndAnnul)?;
        let movement = self
            .movement(number)
            .ok_or(OpError::MovementNotFound(number))?
            .clone();
        if movement.annulled {
            return Err(OpError::MovementAlreadyAnnulled(number));
        }
        if let Some(contract) = movement.contract {
            self.check_dependents(&movement, contract)?;
        }
        self.annul(number);
        let detail = format!(
            "{} del {}",
            movement.kind.label(),
            movement.date.format("%d/%m/%Y")
        );
        self.record(
            on,
            movement.contract,
            Some(number),
            "Anulación de movimiento",
            detail,
        );
        Ok(())
    }

    /// 10 · Revierte el paso a propios: el contrato vuelve a vigente o vencido.
    ///
    /// # Errors
    /// Si el contrato no es mercancía propia sin vender.
    pub fn revert_transfer(&mut self, number: u32, on: NaiveDate) -> Result<(), OpError> {
        self.expect_status(
            number,
            on,
            &[ContractStatus::Owned],
            "en mercancía propia sin vender ni separar",
        )?;
        let movement = self.find_active(number, |k| matches!(k, MovementKind::TransferToOwned))?;
        self.annul_movement(movement, on)
    }

    /// 10 · Reversa un retiro hecho por error.
    ///
    /// # Errors
    /// Si el contrato no está retirado.
    pub fn reverse_withdrawal(&mut self, number: u32, on: NaiveDate) -> Result<(), OpError> {
        self.expect_status(number, on, &[ContractStatus::Withdrawn], "retirado")?;
        let movement =
            self.find_active(number, |k| matches!(k, MovementKind::Withdrawal { .. }))?;
        self.annul_movement(movement, on)
    }

    /// 10 · Anula la venta o el separado: la mercancía vuelve a propios.
    ///
    /// # Errors
    /// Si no hay venta ni separado, o el separado tiene abonos vigentes.
    pub fn annul_sale(&mut self, number: u32, on: NaiveDate) -> Result<(), OpError> {
        self.expect_status(
            number,
            on,
            &[ContractStatus::Sold, ContractStatus::Layaway],
            "vendido o separado",
        )?;
        let movement = self.find_active(number, |k| {
            matches!(k, MovementKind::Sale { .. } | MovementKind::Layaway { .. })
        })?;
        self.annul_movement(movement, on)
    }

    /// 10 · Anula un contrato mal ingresado.
    ///
    /// # Errors
    /// Sin permiso, si no existe, ya está anulado o tiene operaciones vigentes
    /// además del empeño (hay que anularlas primero).
    pub fn annul_contract(&mut self, number: u32, on: NaiveDate) -> Result<(), OpError> {
        self.require(Permission::ModifyAndAnnul)?;
        let contract = self
            .contract(number)
            .ok_or(OpError::ContractNotFound(number))?;
        if contract.annulled {
            return Err(wrong_status(number, ContractStatus::Annulled, "no anulado"));
        }
        if self
            .active_movements(number)
            .any(|m| m.kind != MovementKind::Pawn)
        {
            return Err(OpError::NotAllowed(
                "El contrato tiene pagos u operaciones vigentes. Anúlelos primero por su número de movimiento.",
            ));
        }
        let pawns: Vec<u32> = self.active_movements(number).map(|m| m.number).collect();
        for movement in pawns {
            self.annul(movement);
        }
        self.contract_mut(number)?.annulled = true;
        self.record(
            on,
            Some(number),
            None,
            "Anulación de contrato",
            String::from("Contrato registrado por error"),
        );
        Ok(())
    }

    // ---------------------------------------------------------------- Mantenimiento

    /// Cambia el usuario con la sesión abierta.
    ///
    /// # Errors
    /// Si no existe un usuario con ese código.
    pub fn sign_in(&mut self, code: &str) -> Result<(), OpError> {
        let index = self
            .users
            .iter()
            .position(|u| u.code == code)
            .ok_or(OpError::NotAllowed("No existe ese usuario."))?;
        self.current_user = index;
        Ok(())
    }

    /// # Errors
    /// Sin permiso, código vacío o repetido.
    pub fn add_user(&mut self, code: &str, name: &str, profile: Profile) -> Result<(), OpError> {
        self.require(Permission::Administer)?;
        let code = required_text(code, "Escriba el código del usuario.")?;
        if self.users.iter().any(|u| u.code == code) {
            return Err(OpError::DuplicateCode(code));
        }
        let name = required_text(name, "Escriba el nombre del usuario.")?;
        self.users.push(User {
            code,
            name,
            profile,
        });
        Ok(())
    }

    /// # Errors
    /// Cédula o nombre vacíos, o cédula repetida.
    pub fn add_customer(&mut self, customer: Customer) -> Result<(), OpError> {
        self.require(Permission::Operate)?;
        let id_number = required_text(&customer.id_number, "Escriba la cédula.")?;
        if !id_number.chars().all(|c| c.is_ascii_digit()) || !(6..=10).contains(&id_number.len()) {
            return Err(OpError::InvalidValue(
                "La cédula debe tener entre 6 y 10 dígitos.",
            ));
        }
        if self.customer(&id_number).is_some() {
            return Err(OpError::DuplicateCode(id_number));
        }
        let name = required_text(&customer.name, "Escriba el nombre del cliente.")?;
        self.customers.push(Customer {
            id_number,
            name,
            ..customer
        });
        Ok(())
    }

    /// # Errors
    /// Sin permiso, nombre vacío o código repetido.
    pub fn add_merchandise(&mut self, item: &MerchandiseCode) -> Result<(), OpError> {
        self.require(Permission::Administer)?;
        if self.merchandise_code(item.code).is_some() {
            return Err(OpError::DuplicateCode(item.code.to_string()));
        }
        let name = required_text(&item.name, "Escriba el nombre del código.")?;
        self.merchandise.push(MerchandiseCode {
            name,
            code: item.code,
            metal: item.metal,
        });
        self.merchandise.sort_by_key(|m| m.code);
        Ok(())
    }

    /// # Errors
    /// Sin permiso, nombre vacío o quilataje repetido.
    pub fn add_gold_quality(&mut self, quality: &GoldQuality) -> Result<(), OpError> {
        self.require(Permission::Administer)?;
        if self.gold.iter().any(|g| g.karat == quality.karat) {
            return Err(OpError::DuplicateCode(quality.karat.to_string()));
        }
        let name = required_text(&quality.name, "Escriba el nombre de la calidad.")?;
        self.gold.push(GoldQuality {
            name,
            karat: quality.karat,
        });
        self.gold.sort_by_key(|g| g.karat);
        Ok(())
    }

    /// # Errors
    /// Sin permiso, nombre vacío o código repetido.
    pub fn add_concept(&mut self, concept: &CashConcept) -> Result<(), OpError> {
        self.require(Permission::Administer)?;
        if self.concept(concept.code).is_some() {
            return Err(OpError::DuplicateCode(concept.code.to_string()));
        }
        let name = required_text(&concept.name, "Escriba el nombre del concepto.")?;
        self.concepts.push(CashConcept {
            name,
            code: concept.code,
            category: concept.category,
        });
        self.concepts.sort_by_key(|c| c.code);
        Ok(())
    }

    /// # Errors
    /// Sin permiso o nombre vacío.
    pub fn set_business_name(&mut self, name: &str) -> Result<(), OpError> {
        self.require(Permission::Administer)?;
        self.business_name = required_text(name, "Escriba el nombre del negocio.")?;
        Ok(())
    }

    // ---------------------------------------------------------------- Internos

    fn require(&self, permission: Permission) -> Result<(), OpError> {
        if self.can(permission) {
            Ok(())
        } else {
            Err(OpError::PermissionDenied(permission.denied_reason()))
        }
    }

    fn contract_mut(&mut self, number: u32) -> Result<&mut Contract, OpError> {
        self.contracts
            .get_mut(&number)
            .ok_or(OpError::ContractNotFound(number))
    }

    fn expect_status(
        &self,
        number: u32,
        on: NaiveDate,
        allowed: &[ContractStatus],
        expected: &'static str,
    ) -> Result<ContractView, OpError> {
        let view = self
            .view(number, on)
            .ok_or(OpError::ContractNotFound(number))?;
        if allowed.contains(&view.status) {
            Ok(view)
        } else {
            Err(wrong_status(number, view.status, expected))
        }
    }

    fn expect_owned(&self, number: u32, on: NaiveDate) -> Result<ContractView, OpError> {
        let view = self
            .view(number, on)
            .ok_or(OpError::ContractNotFound(number))?;
        if view.status == ContractStatus::Owned {
            Ok(view)
        } else {
            Err(OpError::NotAllowed(
                "Ese artículo no está en venta: no es mercancía propia disponible.",
            ))
        }
    }

    fn find_active(
        &self,
        contract: u32,
        predicate: impl Fn(&MovementKind) -> bool,
    ) -> Result<u32, OpError> {
        self.active_movements(contract)
            .filter(|m| predicate(&m.kind))
            .map(|m| m.number)
            .last()
            .ok_or(OpError::NotAllowed(
                "No se encontró la operación que se quiere deshacer.",
            ))
    }

    /// Rechaza anular un movimiento si otros posteriores dependen de él.
    fn check_dependents(&self, movement: &Movement, contract: u32) -> Result<(), OpError> {
        let later = |predicate: fn(&MovementKind) -> bool| {
            self.active_movements(contract)
                .any(|m| m.number > movement.number && predicate(&m.kind))
        };
        match movement.kind {
            MovementKind::Pawn => Err(OpError::NotAllowed(
                "Un empeño no se anula como movimiento: use «Anular contrato».",
            )),
            MovementKind::TransferToOwned
                if later(|k| {
                    matches!(k, MovementKind::Sale { .. } | MovementKind::Layaway { .. })
                }) =>
            {
                Err(OpError::NotAllowed(
                    "La mercancía ya se vendió o separó. Anule primero la venta.",
                ))
            }
            MovementKind::Layaway { .. }
                if later(|k| matches!(k, MovementKind::LayawayPayment { .. })) =>
            {
                Err(OpError::NotAllowed(
                    "El separado tiene abonos. Anule primero los abonos.",
                ))
            }
            MovementKind::InterestPayment { .. } | MovementKind::CapitalPayment { .. }
                if later(|k| {
                    matches!(
                        k,
                        MovementKind::Withdrawal { .. } | MovementKind::TransferToOwned
                    )
                }) =>
            {
                Err(OpError::NotAllowed(
                    "El contrato ya se retiró o pasó a propios. Reverse primero esa operación.",
                ))
            }
            _ => Ok(()),
        }
    }

    fn annul(&mut self, number: u32) {
        if let Some(movement) = self.movements.iter_mut().find(|m| m.number == number) {
            movement.annulled = true;
        }
    }

    fn push_movement(&mut self, date: NaiveDate, contract: Option<u32>, kind: MovementKind) -> u32 {
        let number = self.next_movement;
        self.next_movement += 1;
        let user = self.current_user().code.clone();
        self.movements.push(Movement {
            number,
            date,
            contract,
            kind,
            user,
            annulled: false,
        });
        number
    }

    fn record(
        &mut self,
        date: NaiveDate,
        contract: Option<u32>,
        movement: Option<u32>,
        action: &'static str,
        detail: String,
    ) {
        let user = self.current_user().code.clone();
        self.audit.push(AuditEntry {
            date,
            user,
            contract,
            movement,
            action,
            detail,
        });
    }
}

fn line(movement: &Movement, months: u32, amount: Money) -> PaymentLine {
    PaymentLine {
        movement: movement.number,
        date: movement.date,
        months,
        amount,
    }
}

fn wrong_status(number: u32, status: ContractStatus, expected: &'static str) -> OpError {
    OpError::WrongStatus {
        number,
        status: status.label(),
        expected,
    }
}

fn required_text(text: &str, message: &'static str) -> Result<String, OpError> {
    let text = text.trim();
    if text.is_empty() {
        Err(OpError::InvalidValue(message))
    } else {
        Ok(text.to_owned())
    }
}
