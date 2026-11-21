//! Pruebas de los componentes base, cada uno aislado.

use egui::{Key, Role};
use egui_kittest::{Harness, kittest::Queryable};
use pos_desktop::components::{
    ColumnSpec, ColumnWidth, ConfirmChoice, ConfirmDialog, DataTable, EmptyState, FormField,
    PrimaryButton, SearchField, selector,
};

mod primary_button {
    use super::*;

    #[test]
    fn reports_a_mouse_click() {
        let mut harness = Harness::new_ui_state(
            |ui, clicked: &mut bool| {
                if ui.add(PrimaryButton::new("Guardar cambios")).clicked() {
                    *clicked = true;
                }
            },
            false,
        );

        harness.get_by_label("Guardar cambios").click();
        harness.run();

        assert!(harness.state(), "el clic debería llegar al botón principal");
    }

    #[test]
    fn activates_with_enter_when_focused() {
        let mut harness = Harness::new_ui_state(
            |ui, clicked: &mut bool| {
                if ui.add(PrimaryButton::new("Guardar cambios")).clicked() {
                    *clicked = true;
                }
            },
            false,
        );

        harness.get_by_label("Guardar cambios").focus();
        harness.run();
        harness.key_press(Key::Enter);
        harness.run();

        assert!(harness.state(), "Enter con foco debería activar el botón");
    }

    #[test]
    fn ignores_clicks_while_disabled() {
        let mut harness = Harness::new_ui_state(
            |ui, clicked: &mut bool| {
                if ui
                    .add_enabled(false, PrimaryButton::new("Guardar cambios"))
                    .clicked()
                {
                    *clicked = true;
                }
            },
            false,
        );

        harness.get_by_label("Guardar cambios").click();
        harness.run();

        assert!(!harness.state(), "un botón deshabilitado no debe responder");
    }
}

mod search_field {
    use super::*;

    #[derive(Default)]
    struct SearchState {
        query: String,
        changed: bool,
    }

    fn harness(initial: &str) -> Harness<'static, SearchState> {
        Harness::new_ui_state(
            |ui, state: &mut SearchState| {
                if ui
                    .add(SearchField::new(&mut state.query, "search"))
                    .changed()
                {
                    state.changed = true;
                }
            },
            SearchState {
                query: initial.to_owned(),
                changed: false,
            },
        )
    }

    #[test]
    fn typing_updates_the_query() {
        let mut harness = harness("");

        harness.get_by_role(Role::TextInput).focus();
        harness.run();
        harness.get_by_role(Role::TextInput).type_text("EJ-002");
        harness.run();

        assert_eq!(harness.state().query, "EJ-002");
    }

    #[test]
    fn clear_button_empties_the_query_and_reports_a_change() {
        let mut harness = harness("ejemplo");

        harness.get_by_label("Limpiar").click();
        harness.run();

        assert!(
            harness.state().query.is_empty() && harness.state().changed,
            "Limpiar debe vaciar la búsqueda y marcarla como cambiada"
        );
    }
}

mod selector_widget {
    use super::*;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum Choice {
        First,
        Second,
    }

    // El selector siempre va dentro de un FormField, que le da su nombre
    // accesible; kittest comprueba que todos los controles tengan uno.
    #[test]
    fn choosing_another_option_changes_the_value() {
        let mut harness = Harness::new_ui_state(
            |ui, state: &mut (Choice, bool)| {
                let options = [(Choice::First, "Primera"), (Choice::Second, "Segunda")];
                let response = FormField::new("Opción").show(ui, |ui| {
                    selector(ui, "choice", &mut state.0, &options, 160.0)
                });
                if response.changed() {
                    state.1 = true;
                }
            },
            (Choice::First, false),
        );
        harness.run();

        harness.get_by_role(Role::ComboBox).click();
        harness.run();
        harness.get_by_label("Segunda").click();
        harness.run();

        assert_eq!(*harness.state(), (Choice::Second, true));
    }
}

mod form_field {
    use super::*;

    #[test]
    fn shows_the_error_instead_of_the_help_text() {
        let mut harness = Harness::new_ui_state(
            |ui, value: &mut String| {
                FormField::new("Nombre")
                    .help("Texto de ayuda")
                    .error(Some("Escriba un nombre."))
                    .show(ui, |ui| ui.text_edit_singleline(value));
            },
            String::new(),
        );
        harness.run();

        harness.get_by_label_contains("Escriba un nombre.");
        assert!(
            harness.query_by_label("Texto de ayuda").is_none(),
            "la ayuda debe ocultarse mientras hay un error"
        );
    }
}

mod confirm_dialog {
    use super::*;

    fn harness() -> Harness<'static, Option<ConfirmChoice>> {
        Harness::new_ui_state(
            |ui, choice: &mut Option<ConfirmChoice>| {
                let dialog =
                    ConfirmDialog::new("test_confirm", "Eliminar", "¿Seguro?", "Eliminar elemento");
                if let Some(answer) = dialog.show(ui.ctx()) {
                    *choice = Some(answer);
                }
            },
            None,
        )
    }

    #[test]
    fn returns_confirm_when_the_action_is_clicked() {
        let mut harness = harness();
        harness.run();

        harness.get_by_label("Eliminar elemento").click();
        harness.run();

        assert_eq!(*harness.state(), Some(ConfirmChoice::Confirm));
    }

    #[test]
    fn returns_cancel_when_cancel_is_clicked() {
        let mut harness = harness();
        harness.run();

        harness.get_by_label("Cancelar").click();
        harness.run();

        assert_eq!(*harness.state(), Some(ConfirmChoice::Cancel));
    }

    #[test]
    fn escape_counts_as_cancel() {
        let mut harness = harness();
        harness.run();

        harness.key_press(Key::Escape);
        harness.run();

        assert_eq!(*harness.state(), Some(ConfirmChoice::Cancel));
    }
}

mod empty_state {
    use super::*;

    #[test]
    fn reports_a_click_on_its_action() {
        let mut harness = Harness::new_ui_state(
            |ui, clicked: &mut bool| {
                if EmptyState::new("Sin elementos")
                    .action("Crear elemento")
                    .show(ui)
                {
                    *clicked = true;
                }
            },
            false,
        );

        harness.get_by_label("Crear elemento").click();
        harness.run();

        assert!(harness.state());
    }
}

mod data_table {
    use super::*;

    #[test]
    fn reports_the_clicked_row() {
        let mut harness =
            Harness::builder()
                .with_size(egui::vec2(600.0, 300.0))
                .build_ui_state(
                    |ui, clicked: &mut Option<usize>| {
                        let columns = [ColumnSpec::new("Nombre", ColumnWidth::Fill)];
                        let outcome =
                            DataTable::new("test_table", &columns, 3, EmptyState::new("Vacía"))
                                .show(ui, |row, _, ui| {
                                    ui.label(format!("Fila {}", row + 1));
                                });
                        if outcome.clicked_row.is_some() {
                            *clicked = outcome.clicked_row;
                        }
                    },
                    None,
                );
        harness.run();

        harness.get_by_label("Fila 2").click();
        harness.run();

        assert_eq!(*harness.state(), Some(1));
    }

    #[test]
    fn shows_the_empty_state_when_there_are_no_rows() {
        let mut harness = Harness::builder()
            .with_size(egui::vec2(600.0, 300.0))
            .build_ui(|ui| {
                let columns = [ColumnSpec::new("Nombre", ColumnWidth::Fill)];
                DataTable::new("test_table", &columns, 0, EmptyState::new("Sin resultados"))
                    .show(ui, |_, _, _| {});
            });
        harness.run();

        harness.get_by_label("Nombre");
        harness.get_by_label("Sin resultados");
    }
}
