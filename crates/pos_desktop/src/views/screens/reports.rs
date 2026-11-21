//! Informes en pantalla (la demo no imprime).

use chrono::{Datelike, Days, NaiveDate};
use egui::{Key, Modifiers, RichText, Ui};
use pos_core::{
    application::{ExpenseOrder, PosStore},
    domain::{ContractStatus, Money, MovementKind},
};

use super::{
    Outcome, Screen, escape_pressed,
    queries::{QueryEvent, QueryField},
};
use crate::{
    components::{Card, InputMask, MaskedText, SecondaryButton, key_hints, selector},
    strings::common,
    theme::tokens::{color, space},
    views::shared::{demo_marker, facts, fmt_date, left, right, table},
};

/// Barra común: datos de demostración, salida por pantalla e «Imprimir» deshabilitado.
fn toolbar(ui: &mut Ui) {
    ui.horizontal(|ui| {
        demo_marker(ui);
        ui.label(RichText::new("Salida: pantalla").color(color::TEXT_MUTED));
        ui.add_enabled(false, SecondaryButton::new(common::PRINT))
            .on_disabled_hover_text(common::PRINT_UNAVAILABLE);
    });
    ui.add_space(space::S);
}

fn parse_date(text: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(text.trim(), "%d/%m/%Y").ok()
}

fn money_or_blank(value: Money) -> String {
    if value == Money::ZERO {
        String::new()
    } else {
        value.to_string()
    }
}

// ------------------------------------------------------------------ Caja diaria

/// Caja diaria de cualquier fecha (37:52–42:17).
#[derive(Debug, Default)]
pub struct DailyCashScreen {
    field: QueryField,
    date: Option<NaiveDate>,
    started: bool,
}

impl Screen for DailyCashScreen {
    #[expect(
        clippy::too_many_lines,
        reason = "pantalla declarativa: dividirla no la aclara"
    )]
    fn ui(&mut self, ui: &mut Ui, store: &mut PosStore, today: NaiveDate) -> Outcome {
        if !self.started {
            self.started = true;
            self.field.input = fmt_date(today);
            self.date = Some(today);
        }
        match self.field.ui(
            ui,
            "Fecha de la caja (dd/mm/aaaa)",
            "Fechas",
            InputMask::Date,
            &[],
        ) {
            Some(QueryEvent::Submit(value)) => self.date = parse_date(&value).or(self.date),
            Some(QueryEvent::Back) => return Outcome::Back,
            None => {}
        }
        let (previous, next) = ui.input_mut(|i| {
            (
                i.consume_key(Modifiers::NONE, Key::PageUp),
                i.consume_key(Modifiers::NONE, Key::PageDown),
            )
        });
        if let Some(date) = self.date {
            let moved = if previous {
                date.checked_sub_days(Days::new(1))
            } else if next {
                date.checked_add_days(Days::new(1))
            } else {
                None
            };
            if let Some(date) = moved {
                self.date = Some(date);
                self.field.input = fmt_date(date);
            }
        }
        key_hints(ui, &[("RePág / AvPág", "Día anterior / siguiente")]);
        toolbar(ui);
        let Some(date) = self.date else {
            return Outcome::Stay;
        };
        let cash = store.daily_cash(date);
        Card::new()
            .title(&format!("Caja del {}", fmt_date(date)))
            .show(ui, |ui| {
                let rows: Vec<Vec<String>> = cash
                    .lines
                    .iter()
                    .map(|l| {
                        vec![
                            l.movement.to_string(),
                            l.contract.map_or_else(String::new, |n| n.to_string()),
                            l.operation.to_owned(),
                            l.code.map_or_else(String::new, |c| c.to_string()),
                            l.detail.clone(),
                            money_or_blank(l.capital),
                            money_or_blank(l.interest),
                            money_or_blank(l.inflow),
                            money_or_blank(l.outflow),
                        ]
                    })
                    .collect();
                table(
                    ui,
                    "daily_cash",
                    &[
                        left("Mov."),
                        left("Contrato"),
                        left("Operación"),
                        left("Código"),
                        left("Detalle"),
                        right("Capital"),
                        right("Sobrecosto"),
                        right("Entrada"),
                        right("Salida"),
                    ],
                    &rows,
                );
            });
        ui.add_space(space::S);
        let s = cash.summary;
        Card::new().title("Resumen").show(ui, |ui| {
            ui.columns(2, |columns| {
                facts(
                    &mut columns[0],
                    "cash_in",
                    &[
                        ("Capital de retiros", s.withdrawn_capital.to_string()),
                        ("Sobrecosto de retiros", s.withdrawn_interest.to_string()),
                        ("Pagos de intereses", s.interest_payments.to_string()),
                        ("Abonos a capital", s.capital_payments.to_string()),
                        ("Ventas", s.sales.to_string()),
                        ("Abonos a separados", s.layaway_payments.to_string()),
                        ("Otros ingresos", s.other_income.to_string()),
                        ("Total de ingresos", s.total_in.to_string()),
                    ],
                );
                facts(
                    &mut columns[1],
                    "cash_out",
                    &[
                        ("Contratos recibidos (empeños)", s.pawned.to_string()),
                        ("Gastos y egresos", s.expenses.to_string()),
                        ("Total de egresos", s.total_out.to_string()),
                        ("Diferencia del día", s.net.to_string()),
                    ],
                );
            });
        });
        Outcome::Stay
    }
}

// ------------------------------------------------------------------ Gastos

/// Gastos de un período, por código o por fecha (35:46–37:52).
#[derive(Debug)]
pub struct ExpensesScreen {
    from: String,
    to: String,
    order: ExpenseOrder,
    detailed: bool,
    started: bool,
}

impl Default for ExpensesScreen {
    fn default() -> Self {
        Self {
            from: String::new(),
            to: String::new(),
            order: ExpenseOrder::ByCode,
            detailed: true,
            started: false,
        }
    }
}

const ORDER_OPTIONS: [(ExpenseOrder, &str); 2] = [
    (ExpenseOrder::ByCode, "Por código"),
    (ExpenseOrder::ByDate, "Por fecha"),
];

impl Screen for ExpensesScreen {
    #[expect(
        clippy::too_many_lines,
        reason = "pantalla declarativa: dividirla no la aclara"
    )]
    fn ui(&mut self, ui: &mut Ui, store: &mut PosStore, today: NaiveDate) -> Outcome {
        if !self.started {
            self.started = true;
            self.from = fmt_date(today.with_day(1).unwrap_or(today));
            self.to = fmt_date(today);
        }
        Card::new().title("Período").show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing.x = space::L;
                crate::components::FormField::new("Desde (dd/mm/aaaa)").show(ui, |ui| {
                    ui.add(
                        crate::components::single_line(
                            &mut MaskedText::new(&mut self.from, InputMask::Date),
                            "exp_from",
                        )
                        .desired_width(120.0),
                    )
                });
                crate::components::FormField::new("Hasta (dd/mm/aaaa)").show(ui, |ui| {
                    ui.add(
                        crate::components::single_line(
                            &mut MaskedText::new(&mut self.to, InputMask::Date),
                            "exp_to",
                        )
                        .desired_width(120.0),
                    )
                });
                crate::components::FormField::new("Orden").show(ui, |ui| {
                    selector(ui, "exp_order", &mut self.order, &ORDER_OPTIONS, 160.0)
                });
                ui.checkbox(&mut self.detailed, "Detallado");
            });
        });
        ui.add_space(space::S);
        toolbar(ui);
        let (Some(from), Some(to)) = (parse_date(&self.from), parse_date(&self.to)) else {
            ui.label(RichText::new("⚠ Escriba las fechas como dd/mm/aaaa.").color(color::ERROR));
            return if escape_pressed(ui) {
                Outcome::Back
            } else {
                Outcome::Stay
            };
        };
        let report = store.expenses(from, to, self.order);
        if self.detailed {
            Card::new().title("Detalle").show(ui, |ui| {
                let rows: Vec<Vec<String>> = report
                    .lines
                    .iter()
                    .map(|l| {
                        vec![
                            fmt_date(l.date),
                            l.code.to_string(),
                            l.concept.clone(),
                            l.description.clone(),
                            l.amount.to_string(),
                        ]
                    })
                    .collect();
                table(
                    ui,
                    "expense_lines",
                    &[
                        left("Fecha"),
                        left("Código"),
                        left("Concepto"),
                        left("Descripción"),
                        right("Valor"),
                    ],
                    &rows,
                );
            });
            ui.add_space(space::S);
        }
        Card::new().title("Totales por código").show(ui, |ui| {
            let rows: Vec<Vec<String>> = report
                .by_code
                .iter()
                .map(|c| {
                    vec![
                        c.code.to_string(),
                        c.name.clone(),
                        c.count.to_string(),
                        c.amount.to_string(),
                    ]
                })
                .collect();
            table(
                ui,
                "expense_totals",
                &[
                    left("Código"),
                    left("Concepto"),
                    right("Registros"),
                    right("Total"),
                ],
                &rows,
            );
            ui.add_space(space::S);
            ui.label(
                RichText::new(format!("Total de gastos: {}", report.total))
                    .heading()
                    .color(color::TEXT),
            );
        });
        if escape_pressed(ui) {
            Outcome::Back
        } else {
            Outcome::Stay
        }
    }
}

// ------------------------------------------------------------------ Acumulados

/// Acumulados por operación de un mes (44:58–48:23).
#[derive(Debug, Default)]
pub struct MonthlyScreen {
    month: Option<(i32, u32)>,
}

impl Screen for MonthlyScreen {
    #[expect(
        clippy::too_many_lines,
        reason = "pantalla declarativa: dividirla no la aclara"
    )]
    fn ui(&mut self, ui: &mut Ui, store: &mut PosStore, today: NaiveDate) -> Outcome {
        let (mut year, mut month) = self.month.unwrap_or((today.year(), today.month()));
        let mut previous = false;
        let mut next = false;
        ui.horizontal(|ui| {
            previous = ui.add(SecondaryButton::new("Mes anterior")).clicked();
            ui.label(
                RichText::new(format!("{} de {year}", month_name(month)))
                    .heading()
                    .color(color::TEXT),
            );
            next = ui.add(SecondaryButton::new("Mes siguiente")).clicked();
        });
        let (key_previous, key_next) = ui.input_mut(|i| {
            (
                i.consume_key(Modifiers::NONE, Key::ArrowLeft),
                i.consume_key(Modifiers::NONE, Key::ArrowRight),
            )
        });
        if previous || key_previous {
            (year, month) = if month == 1 {
                (year - 1, 12)
            } else {
                (year, month - 1)
            };
        }
        if next || key_next {
            (year, month) = if month == 12 {
                (year + 1, 1)
            } else {
                (year, month + 1)
            };
        }
        self.month = Some((year, month));
        key_hints(
            ui,
            &[("← →", "Mes anterior / siguiente"), ("Esc", "Volver")],
        );
        toolbar(ui);

        let a = store.monthly_accumulated(year, month);
        ui.columns(2, |columns| {
            Card::new().title("Ingresos").show(&mut columns[0], |ui| {
                facts(
                    ui,
                    "acc_in",
                    &[
                        (
                            "Capital retirado (comunes)",
                            a.withdrawn_capital_common.to_string(),
                        ),
                        (
                            "Sobrecosto de retiros (comunes)",
                            a.withdrawn_interest_common.to_string(),
                        ),
                        (
                            "Capital retirado (hipotecarios)",
                            a.withdrawn_capital_non_common.to_string(),
                        ),
                        (
                            "Sobrecosto de retiros (hipotecarios)",
                            a.withdrawn_interest_non_common.to_string(),
                        ),
                        (
                            "Pagos de intereses (prórrogas)",
                            a.interest_payments.to_string(),
                        ),
                        ("Abonos a capital", a.capital_payments.to_string()),
                        ("Salidas de inventario (ventas)", a.sales.to_string()),
                        (
                            "Abonos a mercancía separada",
                            a.layaway_payments.to_string(),
                        ),
                        ("Utilidad ocasional", a.occasional_profit.to_string()),
                        ("Otros ingresos", a.other_income.to_string()),
                        ("Total de ingresos", a.total_in.to_string()),
                    ],
                );
            });
            Card::new().title("Egresos").show(&mut columns[1], |ui| {
                facts(
                    ui,
                    "acc_out",
                    &[
                        ("Capital prestado (comunes)", a.lent_common.to_string()),
                        (
                            "Capital prestado (hipotecarios)",
                            a.lent_non_common.to_string(),
                        ),
                        ("Pérdidas y ajustes de caja", a.losses.to_string()),
                    ],
                );
                ui.add_space(space::S);
                let rows: Vec<Vec<String>> = a
                    .expenses_by_code
                    .iter()
                    .map(|c| vec![c.code.to_string(), c.name.clone(), c.amount.to_string()])
                    .collect();
                table(
                    ui,
                    "acc_expenses",
                    &[left("Código"), left("Gasto"), right("Total")],
                    &rows,
                );
                ui.add_space(space::S);
                facts(
                    ui,
                    "acc_total_out",
                    &[("Total de egresos", a.total_out.to_string())],
                );
            });
        });
        ui.add_space(space::S);
        ui.label(
            RichText::new(format!("Saldo del mes: {}", a.balance))
                .heading()
                .color(color::TEXT),
        );
        if escape_pressed(ui) {
            Outcome::Back
        } else {
            Outcome::Stay
        }
    }
}

fn month_name(month: u32) -> &'static str {
    const NAMES: [&str; 12] = [
        "Enero",
        "Febrero",
        "Marzo",
        "Abril",
        "Mayo",
        "Junio",
        "Julio",
        "Agosto",
        "Septiembre",
        "Octubre",
        "Noviembre",
        "Diciembre",
    ];
    NAMES
        .get(month.saturating_sub(1) as usize)
        .copied()
        .unwrap_or("")
}

// ------------------------------------------------------------------ Inventario de vigentes

/// Inventario de contratos vigentes (48:32–50:55).
pub struct ActiveInventoryScreen;

impl Screen for ActiveInventoryScreen {
    fn ui(&mut self, ui: &mut Ui, store: &mut PosStore, today: NaiveDate) -> Outcome {
        toolbar(ui);
        let inventory = store.active_inventory(today);
        ui.columns(2, |columns| {
            Card::new()
                .title("Por código de mercancía")
                .show(&mut columns[0], |ui| {
                    let rows: Vec<Vec<String>> = inventory
                        .by_code
                        .iter()
                        .map(|c| {
                            vec![
                                c.code.to_string(),
                                c.name.clone(),
                                c.count.to_string(),
                                c.amount.to_string(),
                            ]
                        })
                        .collect();
                    table(
                        ui,
                        "inv_codes",
                        &[
                            left("Código"),
                            left("Mercancía"),
                            right("Contratos"),
                            right("Capital"),
                        ],
                        &rows,
                    );
                });
            Card::new()
                .title("Oro y plata")
                .show(&mut columns[1], |ui| {
                    let rows: Vec<Vec<String>> = inventory
                        .gold_by_karat
                        .iter()
                        .map(|(karat, weight)| {
                            vec![format!("Oro {karat} quilates"), weight.to_string()]
                        })
                        .collect();
                    table(ui, "inv_gold", &[left("Calidad"), right("Peso")], &rows);
                    ui.add_space(space::S);
                    facts(
                        ui,
                        "inv_silver",
                        &[("Platería", inventory.silver.to_string())],
                    );
                });
        });
        ui.add_space(space::S);
        Card::new()
            .title(&format!("Totales al {}", fmt_date(today)))
            .show(ui, |ui| {
                facts(
                    ui,
                    "inv_totals",
                    &[
                        (
                            "Contratos vigentes (incluye vencidos sin pasar a propios)",
                            inventory.contracts.to_string(),
                        ),
                        ("Capital prestado", inventory.capital.to_string()),
                        (
                            "Intereses causados por cobrar",
                            inventory.accrued_interest.to_string(),
                        ),
                    ],
                );
            });
        if escape_pressed(ui) {
            Outcome::Back
        } else {
            Outcome::Stay
        }
    }
}

// ------------------------------------------------------------------ Mercancía propia

/// Inventario de mercancía propia (24:06–25:21).
pub struct OwnedInventoryScreen;

impl Screen for OwnedInventoryScreen {
    fn ui(&mut self, ui: &mut Ui, store: &mut PosStore, today: NaiveDate) -> Outcome {
        toolbar(ui);
        let owned = store.owned(today);
        Card::new()
            .title("Artículos de la compraventa")
            .show(ui, |ui| {
                let rows: Vec<Vec<String>> = owned
                    .iter()
                    .map(|v| {
                        let state = match (&v.layaway, v.status) {
                            (Some(l), ContractStatus::Layaway) => {
                                format!("Separado · saldo {}", l.balance)
                            }
                            _ => String::from("Disponible"),
                        };
                        vec![
                            v.contract.number.to_string(),
                            v.contract.item.description.clone(),
                            v.contract
                                .item
                                .weight
                                .map_or_else(String::new, |w| w.to_string()),
                            v.owned_since.map_or_else(String::new, fmt_date),
                            v.payoff.to_string(),
                            state,
                        ]
                    })
                    .collect();
                table(
                    ui,
                    "owned",
                    &[
                        left("Contrato"),
                        left("Artículo"),
                        right("Peso"),
                        left("Propio desde"),
                        right("Valor a la fecha"),
                        left("Situación"),
                    ],
                    &rows,
                );
            });
        if escape_pressed(ui) {
            Outcome::Back
        } else {
            Outcome::Stay
        }
    }
}

// ------------------------------------------------------------------ Auditoría

/// Auditor interno: modificaciones y anulaciones con su usuario (51:08).
pub struct AuditScreen;

impl Screen for AuditScreen {
    fn ui(&mut self, ui: &mut Ui, store: &mut PosStore, _today: NaiveDate) -> Outcome {
        toolbar(ui);
        Card::new()
            .title("Modificaciones y anulaciones")
            .show(ui, |ui| {
                let rows: Vec<Vec<String>> = store
                    .audit()
                    .iter()
                    .rev()
                    .map(|e| {
                        vec![
                            fmt_date(e.date),
                            e.user.clone(),
                            e.action.to_owned(),
                            e.contract.map_or_else(String::new, |n| n.to_string()),
                            e.movement.map_or_else(String::new, |n| n.to_string()),
                            e.detail.clone(),
                        ]
                    })
                    .collect();
                table(
                    ui,
                    "audit",
                    &[
                        left("Fecha"),
                        left("Usuario"),
                        left("Acción"),
                        left("Contrato"),
                        left("Movimiento"),
                        left("Detalle"),
                    ],
                    &rows,
                );
            });
        ui.add_space(space::S);
        Card::new().title("Movimientos anulados").show(ui, |ui| {
            let rows: Vec<Vec<String>> = store
                .movements()
                .iter()
                .filter(|m| m.annulled && !matches!(m.kind, MovementKind::Pawn))
                .map(|m| {
                    vec![
                        m.number.to_string(),
                        fmt_date(m.date),
                        m.kind.label().to_owned(),
                        m.contract.map_or_else(String::new, |n| n.to_string()),
                        m.user.clone(),
                    ]
                })
                .collect();
            table(
                ui,
                "annulled",
                &[
                    left("Movimiento"),
                    left("Fecha"),
                    left("Operación"),
                    left("Contrato"),
                    left("Registró"),
                ],
                &rows,
            );
        });
        if escape_pressed(ui) {
            Outcome::Back
        } else {
            Outcome::Stay
        }
    }
}
