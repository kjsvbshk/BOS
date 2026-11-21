//! Datos de demostración (feature `demo-data`). Nombres y cédulas inventados.
//!
//! Se cargan con las mismas operaciones del almacén, así que también prueban
//! las fórmulas con los casos de la reunión del 2026-11-18:
//! 26247 (retiro 1.210.000), 26148 (retiro 642.000 tras abonos), 26320
//! (prórroga de 2 meses = 56.000), 25880 (retiro 448.000 el 7-nov), 23668
//! (mercancía propia que vale 1.280.000 el 18-nov).

use chrono::NaiveDate;

use crate::{
    application::{NewContract, OpError, PosStore},
    domain::{
        CashConcept, ConceptCategory, ContractKind, Customer, GoldQuality, Item, MerchandiseCode,
        Metal, Money, Profile, Rate, User, Weight,
    },
};

/// Nombre del negocio en la demo.
pub const BUSINESS_NAME: &str = "Compraventa 3 Diamantes";
/// Usuario propietario de la demo.
pub const OWNER_CODE: &str = "PRO";
/// Usuario administrativo de la demo.
pub const STAFF_CODE: &str = "ADM";

fn date(year: i32, month: u32, day: u32) -> Result<NaiveDate, OpError> {
    NaiveDate::from_ymd_opt(year, month, day)
        .ok_or(OpError::InvalidValue("Fecha de demostración inválida."))
}

/// Crea el almacén con los datos de demostración. La sesión queda abierta con
/// el usuario propietario.
///
/// # Errors
/// Sólo si los datos de demostración son incoherentes (lo comprueban las pruebas).
pub fn seed() -> Result<PosStore, OpError> {
    let owner = User {
        code: OWNER_CODE.into(),
        name: "Propietario (demo)".into(),
        profile: Profile::Owner,
    };
    let mut store = PosStore::new(BUSINESS_NAME, owner);
    store.add_user(STAFF_CODE, "Recepción (demo)", Profile::Administrative)?;
    catalogs(&mut store)?;
    customers(&mut store)?;
    contracts(&mut store)?;
    cash(&mut store)?;
    Ok(store)
}

fn catalogs(store: &mut PosStore) -> Result<(), OpError> {
    let merchandise: [(u16, &str, Option<Metal>); 13] = [
        (1, "Oro y joyería", Some(Metal::Gold)),
        (2, "Platería", Some(Metal::Silver)),
        (3, "Relojes", None),
        (4, "Cámaras", None),
        (5, "Equipos de sonido", None),
        (6, "Portátiles", None),
        (8, "Bicicletas", None),
        (11, "Licuadoras", None),
        (20, "Herramientas", None),
        (25, "Motos", None),
        (30, "Máquinas de coser", None),
        (90, "Inmuebles e hipotecas", None),
        (99, "Artículo esporádico", None),
    ];
    for (code, name, metal) in merchandise {
        store.add_merchandise(&MerchandiseCode {
            code,
            name: name.into(),
            metal,
        })?;
    }
    for karat in [8, 10, 14, 16, 18, 24] {
        store.add_gold_quality(&GoldQuality {
            karat,
            name: format!("{karat} quilates"),
        })?;
    }
    let concepts: [(u16, &str, ConceptCategory); 12] = [
        (15, "Energía", ConceptCategory::Expense),
        (16, "Acueducto", ConceptCategory::Expense),
        (18, "Arrendamiento", ConceptCategory::Expense),
        (28, "Sueldos", ConceptCategory::Expense),
        (29, "Primas y vacaciones", ConceptCategory::Expense),
        (33, "Seguridad social", ConceptCategory::Expense),
        (42, "Cafetería", ConceptCategory::Expense),
        (50, "Gastos bancarios", ConceptCategory::Expense),
        (60, "Utilidad ocasional", ConceptCategory::OccasionalProfit),
        (61, "Pérdida por ajuste de caja", ConceptCategory::Loss),
        (70, "Otros ingresos", ConceptCategory::OtherIncome),
        (80, "Retiro de socios", ConceptCategory::Expense),
    ];
    for (code, name, category) in concepts {
        store.add_concept(&CashConcept {
            code,
            name: name.into(),
            category,
        })?;
    }
    Ok(())
}

const CUSTOMERS: [(&str, &str, &str, &str); 6] = [
    (
        "1054000001",
        "María Fernanda Ríos",
        "Calle 10 # 4-25",
        "310 555 0101",
    ),
    (
        "1054000002",
        "Jorge Iván Ospina",
        "Carrera 7 # 12-40",
        "311 555 0102",
    ),
    (
        "1054000003",
        "Luz Marina Cárdenas",
        "Calle 22 # 9-13",
        "312 555 0103",
    ),
    (
        "1054000004",
        "Andrés Felipe Muñoz",
        "Avenida Santander # 50-02",
        "313 555 0104",
    ),
    (
        "1054000005",
        "Gloria Patricia Henao",
        "Calle 65 # 23-80",
        "314 555 0105",
    ),
    (
        "1054000006",
        "Carlos Alberto Zuluaga",
        "Carrera 23 # 31-15",
        "315 555 0106",
    ),
];

fn customers(store: &mut PosStore) -> Result<(), OpError> {
    for (id_number, name, address, phone) in CUSTOMERS {
        store.add_customer(Customer {
            id_number: id_number.into(),
            name: name.into(),
            address: address.into(),
            phone: phone.into(),
        })?;
    }
    Ok(())
}

/// Contrato común de la demo.
struct Seed {
    number: u32,
    customer: usize,
    date: (i32, u32, u32),
    term: u32,
    rate: u32,
    capital: i64,
    code: u16,
    description: &'static str,
    condition: &'static str,
    weight_mg: Option<u32>,
    karat: Option<u8>,
}

impl Seed {
    fn register(&self, store: &mut PosStore, kind: ContractKind) -> Result<u32, OpError> {
        let (y, m, d) = self.date;
        let (number, _) = store.register_contract(NewContract {
            number: Some(self.number),
            kind,
            customer: CUSTOMERS[self.customer].0.into(),
            date: date(y, m, d)?,
            term_months: self.term,
            rate: Rate::percent(self.rate),
            capital: Money::pesos(self.capital),
            item: Item {
                code: self.code,
                description: self.description.into(),
                condition: self.condition.into(),
                weight: self.weight_mg.map(Weight::milligrams),
                karat: self.karat,
            },
        })?;
        Ok(number)
    }
}

#[expect(
    clippy::too_many_lines,
    reason = "lista plana de datos de demostración"
)]
fn contracts(store: &mut PosStore) -> Result<(), OpError> {
    let gold = |number, customer, date, term, rate, capital, description, weight_mg, karat| Seed {
        number,
        customer,
        date,
        term,
        rate,
        capital,
        code: 1,
        description,
        condition: "Bueno",
        weight_mg: Some(weight_mg),
        karat: Some(karat),
    };
    let article =
        |number, customer, date, term, rate, capital, code, description, condition| Seed {
            number,
            customer,
            date,
            term,
            rate,
            capital,
            code,
            description,
            condition,
            weight_mg: None,
            karat: None,
        };
    let common = ContractKind::Common;

    // Vigentes de la reunión.
    gold(
        26247,
        0,
        (2026, 8, 28),
        3,
        7,
        1_000_000,
        "Anillo de grado año 2006",
        6_100,
        18,
    )
    .register(store, common)?;
    let n = gold(
        26320,
        2,
        (2026, 8, 25),
        3,
        7,
        400_000,
        "Anillo de dos tiritas",
        1_400,
        18,
    );
    Seed {
        condition: "Regular",
        ..n
    }
    .register(store, common)?;
    gold(
        26580,
        4,
        (2026, 11, 7),
        3,
        10,
        300_000,
        "Pulsera esclava",
        5_200,
        14,
    )
    .register(store, common)?;
    let bike = article(
        25971,
        5,
        (2026, 9, 7),
        3,
        10,
        200_000,
        8,
        "Bicicleta todoterreno roja",
        "Bueno",
    )
    .register(store, common)?;
    store.pay_interest(bike, 1, date(2026, 11, 7)?)?;

    // 26148: intereses, abonos y retiro (12:49–14:04).
    let number = gold(
        26148,
        1,
        (2026, 8, 3),
        3,
        7,
        1_400_000,
        "Cadena tejida",
        9_800,
        18,
    )
    .register(store, common)?;
    store.pay_interest(number, 1, date(2026, 9, 2)?)?;
    store.capital_payment(number, Money::pesos(400_000), date(2026, 9, 20)?)?;
    store.pay_interest(number, 1, date(2026, 10, 3)?)?;
    store.capital_payment(number, Money::pesos(400_000), date(2026, 10, 20)?)?;
    store.withdraw(number, date(2026, 11, 3)?)?;

    // 25880: retiro del 7-nov con 98.000 de sobrecosto (38:51).
    let number = gold(
        25880,
        3,
        (2026, 7, 7),
        4,
        7,
        350_000,
        "Argollas de matrimonio",
        3_500,
        18,
    )
    .register(store, common)?;
    store.withdraw(number, date(2026, 11, 7)?)?;

    // Vencidos.
    gold(
        25890,
        0,
        (2026, 1, 20),
        6,
        10,
        200_000,
        "Cadena de gas con dije",
        4_200,
        18,
    )
    .register(store, common)?;
    article(
        25602,
        1,
        (2026, 2, 15),
        3,
        5,
        1_500_000,
        25,
        "Moto Yamaha BWS 2015",
        "Regular",
    )
    .register(store, common)?;
    article(
        25750,
        3,
        (2026, 5, 2),
        3,
        10,
        60_000,
        11,
        "Licuadora Oster",
        "Bueno",
    )
    .register(store, common)?;
    let number = article(
        25660,
        5,
        (2026, 4, 12),
        3,
        10,
        180_000,
        8,
        "Bicicleta azul",
        "Regular",
    )
    .register(store, common)?;
    store.pay_interest(number, 1, date(2026, 5, 12)?)?;
    Seed {
        code: 2,
        karat: None,
        ..gold(
            25700,
            2,
            (2026, 3, 3),
            3,
            10,
            250_000,
            "Cadena de plata",
            25_000,
            0,
        )
    }
    .register(store, common)?;

    // Mercancía propia.
    let number = gold(
        23668,
        4,
        (2026, 5, 18),
        3,
        10,
        800_000,
        "Dije de oso",
        4_900,
        18,
    )
    .register(store, common)?;
    store.transfer_to_owned(number, date(2026, 9, 1)?)?;
    let number = article(
        23986,
        1,
        (2026, 4, 5),
        2,
        10,
        120_000,
        3,
        "Reloj Casio dorado",
        "Bueno",
    )
    .register(store, common)?;
    store.transfer_to_owned(number, date(2026, 8, 1)?)?;
    let set = gold(
        24120,
        2,
        (2026, 3, 10),
        3,
        10,
        300_000,
        "Juego de cadena y aretes de plata",
        32_000,
        0,
    );
    let number = Seed {
        code: 2,
        karat: None,
        ..set
    }
    .register(store, common)?;
    store.transfer_to_owned(number, date(2026, 7, 15)?)?;
    store.layaway(
        number,
        Money::pesos(500_000),
        Some(CUSTOMERS[3].0.into()),
        date(2026, 12, 31)?,
        date(2026, 11, 2)?,
    )?;
    store.layaway_payment(number, Money::pesos(200_000), date(2026, 11, 10)?)?;

    // Contrato no común: interés de 150.000 el 7-nov (39:49).
    let mortgage = article(
        0,
        5,
        (2026, 7, 1),
        6,
        3,
        5_000_000,
        90,
        "Hipoteca casa lote barrio La Francia",
        "No aplica",
    );
    let number = mortgage.register(store, ContractKind::NonCommon)?;
    store.pay_interest(number, 1, date(2026, 11, 7)?)?;
    Ok(())
}

/// Movimiento de caja de la demo: fecha, concepto, descripción y valor.
type CashSeed = ((i32, u32, u32), u16, &'static str, i64);

fn cash(store: &mut PosStore) -> Result<(), OpError> {
    let movements: [CashSeed; 8] = [
        ((2026, 10, 31), 28, "Nómina de octubre", 2_400_000),
        ((2026, 11, 3), 42, "Tinto y galletas", 8_500),
        ((2026, 11, 5), 33, "Seguridad social de octubre", 300_000),
        ((2026, 11, 7), 28, "Adelanto de sueldo", 200_000),
        ((2026, 11, 7), 15, "Factura de energía", 185_000),
        ((2026, 11, 10), 18, "Arriendo de noviembre", 1_200_000),
        (
            (2026, 11, 12),
            60,
            "Compra y venta inmediata de un anillo",
            50_000,
        ),
        ((2026, 11, 15), 61, "Diferencia al cuadrar la caja", 12_000),
    ];
    for ((y, m, d), concept, description, amount) in movements {
        store.cash_movement(
            concept,
            description.into(),
            Money::pesos(amount),
            false,
            date(y, m, d)?,
        )?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        application::{ExpiredFilter, InterestFilter},
        domain::ContractStatus,
    };

    fn store() -> PosStore {
        seed().unwrap()
    }

    fn on(y: i32, m: u32, d: u32) -> NaiveDate {
        date(y, m, d).unwrap()
    }

    #[test]
    fn seed_loads_without_errors() {
        assert!(seed().is_ok());
    }

    #[test]
    fn contract_26247_withdraws_for_1210000_on_november_18() {
        let view = store().view(26247, on(2026, 11, 18)).unwrap();
        assert_eq!(view.payoff, Money::pesos(1_210_000));
    }

    #[test]
    fn contract_26148_was_withdrawn_for_642000() {
        let view = store().view(26148, on(2026, 11, 18)).unwrap();
        assert_eq!(
            view.withdrawal.map(|(_, total)| total),
            Some(Money::pesos(642_000))
        );
    }

    #[test]
    fn contract_26148_first_interest_payment_was_98000() {
        let view = store().view(26148, on(2026, 11, 18)).unwrap();
        assert_eq!(view.interest_payments[0].amount, Money::pesos(98_000));
    }

    #[test]
    fn contract_26148_second_interest_payment_was_70000() {
        let view = store().view(26148, on(2026, 11, 18)).unwrap();
        assert_eq!(view.interest_payments[1].amount, Money::pesos(70_000));
    }

    #[test]
    fn extension_of_26320_costs_56000_and_leaves_428000() {
        let mut store = store();
        let (_, amount) = store.pay_interest(26320, 2, on(2026, 11, 18)).unwrap();
        let view = store.view(26320, on(2026, 11, 18)).unwrap();
        assert_eq!(
            (amount, view.payoff),
            (Money::pesos(56_000), Money::pesos(428_000))
        );
    }

    #[test]
    fn owned_23668_is_worth_1280000_and_selling_for_1000000_loses_280000() {
        let mut store = store();
        let (_, reference, difference) = store
            .sell(23668, Money::pesos(1_000_000), None, on(2026, 11, 18))
            .unwrap();
        assert_eq!(
            (reference, difference),
            (Money::pesos(1_280_000), Money::pesos(-280_000))
        );
    }

    #[test]
    fn daily_cash_of_november_7_includes_the_448000_withdrawal() {
        let cash = store().daily_cash(on(2026, 11, 7));
        let withdrawal = cash
            .lines
            .iter()
            .find(|l| l.contract == Some(25880))
            .unwrap();
        assert_eq!(withdrawal.inflow, Money::pesos(448_000));
    }

    #[test]
    fn non_common_contract_paid_150000_interest() {
        let view = store().view(900_001, on(2026, 11, 18)).unwrap();
        assert_eq!(view.interest_paid(), Money::pesos(150_000));
    }

    #[test]
    fn expired_list_with_any_interest_has_five_contracts() {
        let filter = ExpiredFilter {
            code: 0,
            both_kinds: true,
            interest: InterestFilter::Any,
        };
        assert_eq!(store().expired(filter, on(2026, 11, 18)).len(), 5);
    }

    #[test]
    fn expired_list_filtered_by_paid_interest_has_the_blue_bicycle() {
        let filter = ExpiredFilter {
            code: 0,
            both_kinds: true,
            interest: InterestFilter::Paid,
        };
        let numbers: Vec<u32> = store()
            .expired(filter, on(2026, 11, 18))
            .iter()
            .map(|v| v.contract.number)
            .collect();
        assert_eq!(numbers, vec![25660]);
    }

    #[test]
    fn transferring_an_expired_contract_makes_it_owned() {
        let mut store = store();
        store.transfer_to_owned(25890, on(2026, 11, 18)).unwrap();
        assert_eq!(
            store.view(25890, on(2026, 11, 18)).unwrap().status,
            ContractStatus::Owned
        );
    }

    #[test]
    fn an_active_contract_cannot_pass_to_owned() {
        assert!(store().transfer_to_owned(26247, on(2026, 11, 18)).is_err());
    }

    #[test]
    fn annulling_an_interest_payment_restores_the_previous_payoff() {
        let mut store = store();
        let (movement, _) = store.pay_interest(26320, 2, on(2026, 11, 18)).unwrap();
        store.annul_movement(movement, on(2026, 11, 18)).unwrap();
        assert_eq!(
            store.view(26320, on(2026, 11, 18)).unwrap().payoff,
            Money::pesos(484_000)
        );
    }

    #[test]
    fn reversing_a_withdrawal_makes_the_contract_active_again() {
        let mut store = store();
        store.withdraw(26247, on(2026, 11, 18)).unwrap();
        store.reverse_withdrawal(26247, on(2026, 11, 18)).unwrap();
        assert_eq!(
            store.view(26247, on(2026, 11, 18)).unwrap().status,
            ContractStatus::Active
        );
    }

    #[test]
    fn annulling_a_sale_returns_the_item_to_owned() {
        let mut store = store();
        store
            .sell(23668, Money::pesos(1_000_000), None, on(2026, 11, 18))
            .unwrap();
        store.annul_sale(23668, on(2026, 11, 18)).unwrap();
        assert_eq!(
            store.view(23668, on(2026, 11, 18)).unwrap().status,
            ContractStatus::Owned
        );
    }

    #[test]
    fn a_layaway_with_payments_cannot_be_annulled_directly() {
        assert!(store().annul_sale(24120, on(2026, 11, 18)).is_err());
    }

    #[test]
    fn layaway_balance_is_price_minus_payments() {
        let view = store().view(24120, on(2026, 11, 18)).unwrap();
        assert_eq!(view.layaway.unwrap().balance, Money::pesos(300_000));
    }

    #[test]
    fn staff_cannot_transfer_expired_contracts() {
        let mut store = store();
        store.sign_in(STAFF_CODE).unwrap();
        assert!(store.transfer_to_owned(25890, on(2026, 11, 18)).is_err());
    }

    #[test]
    fn modifying_a_contract_is_audited() {
        let mut store = store();
        store
            .modify_contract(
                26247,
                crate::application::ContractChange::Capital(Money::pesos(500_000)),
                on(2026, 11, 18),
            )
            .unwrap();
        assert_eq!(
            store.audit().last().unwrap().action,
            "Modificación de contrato"
        );
    }

    #[test]
    fn a_contract_with_payments_cannot_be_annulled() {
        assert!(store().annul_contract(26148, on(2026, 11, 18)).is_err());
    }

    #[test]
    fn monthly_accumulated_separates_common_and_non_common_capital() {
        let month = store().monthly_accumulated(2026, 7);
        assert_eq!(
            (month.lent_common, month.lent_non_common),
            (Money::pesos(350_000), Money::pesos(5_000_000))
        );
    }

    #[test]
    fn active_inventory_counts_gold_by_karat() {
        let inventory = store().active_inventory(on(2026, 11, 18));
        assert!(
            inventory
                .gold_by_karat
                .iter()
                .any(|(karat, _)| *karat == 18)
        );
    }
}
