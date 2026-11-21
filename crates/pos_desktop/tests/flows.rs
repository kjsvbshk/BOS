//! Flujos completos recorridos sólo con teclado, con los casos de la reunión.

mod common;

use common::{app, assert_shows, clear_field, click, meeting_day, open, press, type_and_enter};
use egui::Key;
use egui_kittest::kittest::Queryable;
use pos_core::domain::{ContractStatus, Money, MovementKind};
use pos_desktop::navigation::{Route, SectionId};

#[test]
fn withdrawing_contract_26247_charges_1210000() {
    let mut harness = app("PRO");
    open(&mut harness, "3");

    type_and_enter(&mut harness, "26247");
    press(&mut harness, Key::Enter); // fecha de hoy
    press(&mut harness, Key::S); // el artículo corresponde
    assert_shows(&harness, "$1.210.000");
    press(&mut harness, Key::G);

    let view = harness.state().store().view(26247, meeting_day()).unwrap();
    assert_eq!(view.status, ContractStatus::Withdrawn);
}

#[test]
fn extending_26320_two_months_costs_56000_and_leaves_428000() {
    let mut harness = app("PRO");
    open(&mut harness, "2");

    type_and_enter(&mut harness, "26320");
    press(&mut harness, Key::N); // no modifica el plazo
    clear_field(&mut harness);
    type_and_enter(&mut harness, "2");
    press(&mut harness, Key::Enter); // fecha de hoy
    assert_shows(&harness, "$56.000");
    press(&mut harness, Key::G);

    let view = harness.state().store().view(26320, meeting_day()).unwrap();
    assert_eq!(view.payoff, Money::pesos(428_000));
}

#[test]
fn entering_a_new_contract_with_the_keyboard_registers_it() {
    let mut harness = app("PRO");
    open(&mut harness, "1");

    press(&mut harness, Key::Enter); // tipo: común
    type_and_enter(&mut harness, "26613");
    clear_field(&mut harness);
    type_and_enter(&mut harness, "3"); // plazo
    clear_field(&mut harness);
    type_and_enter(&mut harness, "7"); // porcentaje
    type_and_enter(&mut harness, "1054000001");
    type_and_enter(&mut harness, "700000");
    type_and_enter(&mut harness, "1"); // oro y joyería
    type_and_enter(&mut harness, "Anillo tipo Rolex");
    clear_field(&mut harness);
    type_and_enter(&mut harness, "Torcido");
    type_and_enter(&mut harness, "2,9");
    type_and_enter(&mut harness, "18");
    press(&mut harness, Key::Enter); // fecha de hoy
    press(&mut harness, Key::N); // no imprimir
    assert_shows(&harness, "$49.000"); // interés mensual de la reunión
    press(&mut harness, Key::G);

    let view = harness.state().store().view(26613, meeting_day()).unwrap();
    assert_eq!(
        (view.capital, view.monthly_interest),
        (Money::pesos(700_000), Money::pesos(49_000))
    );
}

#[test]
fn an_existing_contract_number_is_rejected_at_its_step() {
    let mut harness = app("PRO");
    open(&mut harness, "1");

    press(&mut harness, Key::Enter);
    type_and_enter(&mut harness, "26247");

    harness.get_by_label_contains("El contrato 26247 ya existe");
}

#[test]
fn contract_query_shows_the_642000_withdrawal_of_26148() {
    let mut harness = app("PRO");
    open(&mut harness, "9");
    open(&mut harness, "1");

    type_and_enter(&mut harness, "26148");

    assert_shows(&harness, "$642.000");
}

#[test]
fn selling_23668_for_1000000_records_a_loss_of_280000() {
    let mut harness = app("PRO");
    open(&mut harness, "6");

    press(&mut harness, Key::Enter); // venta directa
    type_and_enter(&mut harness, "23668");
    press(&mut harness, Key::Enter); // sin comprador
    type_and_enter(&mut harness, "1000000");
    press(&mut harness, Key::Enter); // fecha de hoy
    assert_shows(&harness, "$280.000");
    press(&mut harness, Key::G);

    let view = harness.state().store().view(23668, meeting_day()).unwrap();
    assert_eq!(view.status, ContractStatus::Sold);
}

#[test]
fn an_item_that_is_not_owned_cannot_be_sold() {
    let mut harness = app("PRO");
    open(&mut harness, "6");

    press(&mut harness, Key::Enter);
    type_and_enter(&mut harness, "26247");

    harness.get_by_label_contains("no está en venta");
}

#[test]
fn transferring_the_selected_expired_contract_needs_confirmation() {
    let mut harness = app("PRO");
    open(&mut harness, "4");
    let before = harness.state().store().owned(meeting_day()).len();

    press(&mut harness, Key::Enter);
    // El diálogo repite el texto del botón: se pulsa el del diálogo (el último).
    harness
        .get_all_by_label("Pasar a propios")
        .last()
        .unwrap()
        .click();
    harness.run();

    assert_eq!(
        harness.state().store().owned(meeting_day()).len(),
        before + 1
    );
}

#[test]
fn annulling_an_interest_payment_by_movement_number_restores_the_contract() {
    let mut harness = app("PRO");
    let movement = harness
        .state()
        .store()
        .movements_of(25971)
        .find(|m| matches!(m.kind, MovementKind::InterestPayment { .. }))
        .map(|m| m.number)
        .unwrap();
    open(&mut harness, "10");
    open(&mut harness, "5");

    type_and_enter(&mut harness, &movement.to_string());
    press(&mut harness, Key::S);
    press(&mut harness, Key::G);

    assert!(harness.state().store().movement(movement).unwrap().annulled);
}

#[test]
fn cash_expense_is_recorded_with_its_concept() {
    let mut harness = app("PRO");
    open(&mut harness, "7");
    let before = harness.state().store().movements().len();

    type_and_enter(&mut harness, "42");
    type_and_enter(&mut harness, "Tinto");
    type_and_enter(&mut harness, "8500");
    press(&mut harness, Key::Enter);
    press(&mut harness, Key::G);

    assert_eq!(harness.state().store().movements().len(), before + 1);
}

#[test]
fn escape_on_the_first_step_returns_to_operations() {
    let mut harness = app("PRO");
    open(&mut harness, "3");

    press(&mut harness, Key::Escape);

    assert_eq!(
        harness.state().route(),
        Route::Section(SectionId::Operations)
    );
}

#[test]
fn staff_cannot_open_expired_contracts() {
    let mut harness = app("ADM");

    open(&mut harness, "4");

    assert_eq!(
        harness.state().route(),
        Route::Section(SectionId::Operations)
    );
}

#[test]
fn staff_cannot_open_modifications() {
    let mut harness = app("ADM");

    open(&mut harness, "10");

    assert_eq!(
        harness.state().route(),
        Route::Section(SectionId::Operations)
    );
}

#[test]
fn monthly_accumulated_shows_the_capital_lent_in_november() {
    let mut harness = app("PRO");
    click(&mut harness, "Informes");
    open(&mut harness, "3");

    assert_shows(&harness, "$300.000");
}

#[test]
fn f1_opens_the_customer_list_and_enter_picks_the_highlighted_one() {
    let mut harness = app("PRO");
    open(&mut harness, "1");
    press(&mut harness, Key::Enter); // común
    type_and_enter(&mut harness, "26613");
    press(&mut harness, Key::Enter); // plazo propuesto
    press(&mut harness, Key::Enter); // porcentaje propuesto

    press(&mut harness, Key::F1);
    press(&mut harness, Key::Enter);

    assert_shows(&harness, "1054000001 · María Fernanda Ríos");
}

#[test]
fn pressing_a_number_several_times_never_opens_an_option() {
    let mut harness = app("PRO");

    for _ in 0..3 {
        press(&mut harness, Key::Num3);
    }

    assert_eq!(
        harness.state().route(),
        Route::Section(SectionId::Operations)
    );
}

#[test]
fn two_digits_select_option_11() {
    let mut harness = app("PRO");

    open(&mut harness, "11");

    assert!(matches!(harness.state().route(), Route::Option(option) if option.number == 11));
}

#[test]
fn amounts_show_thousands_separators_while_typing() {
    let mut harness = app("PRO");
    open(&mut harness, "1");
    press(&mut harness, Key::Enter);
    type_and_enter(&mut harness, "26613");
    press(&mut harness, Key::Enter);
    press(&mut harness, Key::Enter);
    type_and_enter(&mut harness, "1054000001");

    harness
        .get_by_role(egui::Role::TextInput)
        .type_text("1210000");
    harness.run();

    assert_eq!(
        harness
            .get_by_role(egui::Role::TextInput)
            .value()
            .as_deref(),
        Some("1.210.000")
    );
}

#[test]
fn typing_a_customer_name_and_choosing_the_suggestion_selects_the_contract() {
    let mut harness = app("PRO");
    open(&mut harness, "3");

    harness.get_by_role(egui::Role::TextInput).type_text("Ríos");
    harness.run();
    press(&mut harness, Key::ArrowDown);
    press(&mut harness, Key::Enter);

    assert_shows(&harness, "Contrato 26247");
}

#[test]
fn clicking_a_suggestion_in_the_contract_query_shows_its_card() {
    let mut harness = app("PRO");
    open(&mut harness, "9");
    open(&mut harness, "1");

    harness.get_by_label_contains("26148  ·").click();
    harness.run();

    assert_shows(&harness, "$642.000");
}

#[test]
fn the_highlighted_suggestion_is_previewed_before_confirming() {
    let mut harness = app("PRO");
    open(&mut harness, "3");

    harness.get_by_role(egui::Role::TextInput).type_text("Ríos");
    harness.run();
    press(&mut harness, Key::ArrowDown);

    // Sin pulsar Enter, la ficha del contrato seleccionado ya está a la vista.
    assert_shows(&harness, "Valor de retiro a hoy");
}
