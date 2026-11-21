//! Catálogo de componentes: referencia visual interna de la Fase 1.
//!
//! Muestra cada componente en sus estados con textos de ejemplo. No representa
//! ninguna operación del negocio.

use egui::{RichText, TextEdit, Ui};

use crate::{
    components::{
        BadgeTone, Card, ColumnSpec, ColumnWidth, ConfirmChoice, ConfirmDialog, DataTable,
        EmptyState, FormField, LoadingIndicator, PageHeader, PrimaryButton, SearchField,
        SecondaryButton, StatusBadge, selector, single_line,
    },
    strings::gallery as text,
    theme::{
        icons,
        tokens::{color, size, space},
    },
};

/// A partir de este ancho el formulario usa dos columnas.
const TWO_COLUMN_MIN_WIDTH: f32 = 720.0;
/// Número de elementos de ejemplo de la tabla.
const SAMPLE_ROW_COUNT: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SampleCategory {
    A,
    B,
    C,
}

const CATEGORY_OPTIONS: [(SampleCategory, &str); 3] = [
    (SampleCategory::A, text::CATEGORY_A),
    (SampleCategory::B, text::CATEGORY_B),
    (SampleCategory::C, text::CATEGORY_C),
];

#[derive(Debug, Clone)]
struct SampleRow {
    code: String,
    description: String,
    tone: BadgeTone,
}

impl SampleRow {
    fn status_text(&self) -> &'static str {
        match self.tone {
            BadgeTone::Neutral => text::STATUS_NEUTRAL,
            BadgeTone::Pending => text::STATUS_PENDING,
            BadgeTone::Success => text::STATUS_SUCCESS,
            BadgeTone::Warning => text::STATUS_WARNING,
            BadgeTone::Error => text::STATUS_ERROR,
        }
    }
}

fn sample_rows() -> Vec<SampleRow> {
    const TONES: [BadgeTone; 4] = [
        BadgeTone::Success,
        BadgeTone::Pending,
        BadgeTone::Warning,
        BadgeTone::Error,
    ];
    (1..=SAMPLE_ROW_COUNT)
        .map(|number| SampleRow {
            code: format!("EJ-{number:03}"),
            description: text::sample_description(number),
            tone: TONES[(number - 1) % TONES.len()],
        })
        .collect()
}

/// Estado del catálogo de componentes.
#[derive(Debug)]
pub struct ComponentGallery {
    name: String,
    notes: String,
    category: SampleCategory,
    search: String,
    show_errors: bool,
    disabled: bool,
    show_loading: bool,
    rows: Vec<SampleRow>,
    /// Índices de `rows` que coinciden con la búsqueda. Se recalcula sólo
    /// cuando cambian la búsqueda o las filas.
    visible: Vec<usize>,
    /// Índice en `rows` de la fila seleccionada.
    selected: Option<usize>,
    /// Índice en `rows` de la fila cuya eliminación se está confirmando.
    pending_delete: Option<usize>,
    last_event: Option<&'static str>,
}

impl Default for ComponentGallery {
    fn default() -> Self {
        let rows = sample_rows();
        let visible = (0..rows.len()).collect();
        Self {
            name: String::new(),
            notes: String::new(),
            category: SampleCategory::A,
            search: String::new(),
            show_errors: false,
            disabled: false,
            show_loading: false,
            rows,
            visible,
            selected: None,
            pending_delete: None,
            last_event: None,
        }
    }
}

impl ComponentGallery {
    /// Dibuja el catálogo dentro del área de contenido (que ya se desplaza).
    pub fn ui(&mut self, ui: &mut Ui) {
        PageHeader::new(icons::SQUARES_FOUR, text::SUBTITLE)
            .subtitle(text::INTERNAL_BADGE)
            .show(ui);
        ui.scope(|ui| {
            ui.spacing_mut().item_spacing.y = space::L;
            self.options_ui(ui);
            ui.add_enabled_ui(!self.disabled, |ui| {
                self.buttons_ui(ui);
                self.form_ui(ui);
            });
            Self::status_ui(ui);
            self.table_ui(ui);
            self.empty_and_loading_ui(ui);
            self.dialogs_ui(ui);
        });

        self.confirm_delete_ui(ui.ctx());
    }

    fn options_ui(&mut self, ui: &mut Ui) {
        ui.scope(|ui| {
            ui.set_max_width(size::PROSE_MAX_WIDTH);
            ui.label(RichText::new(text::INTRO).color(color::TEXT_MUTED));
        });
        ui.horizontal_wrapped(|ui| {
            ui.checkbox(&mut self.show_errors, text::TOGGLE_ERRORS);
            ui.checkbox(&mut self.disabled, text::TOGGLE_DISABLED);
            ui.checkbox(&mut self.show_loading, text::TOGGLE_LOADING);
        });
        if let Some(event) = self.last_event {
            ui.label(RichText::new(event).color(color::TEXT_MUTED));
        }
    }

    fn buttons_ui(&mut self, ui: &mut Ui) {
        Card::new().title(text::SECTION_BUTTONS).show(ui, |ui| {
            ui.horizontal(|ui| {
                let primary = ui
                    .add(PrimaryButton::new(text::PRIMARY_ACTION))
                    .on_disabled_hover_text(text::DISABLED_REASON);
                if primary.clicked() {
                    self.last_event = Some(text::SAVED_MESSAGE);
                }
                let secondary = ui
                    .add(SecondaryButton::new(text::SECONDARY_ACTION))
                    .on_disabled_hover_text(text::DISABLED_REASON);
                if secondary.clicked() {
                    self.last_event = Some(text::SECONDARY_MESSAGE);
                }
            });
        });
    }

    fn form_ui(&mut self, ui: &mut Ui) {
        Card::new().title(text::SECTION_FORM).show(ui, |ui| {
            if ui.available_width() >= TWO_COLUMN_MIN_WIDTH {
                ui.columns(2, |columns| {
                    self.name_and_category_ui(&mut columns[0]);
                    self.notes_ui(&mut columns[1]);
                });
            } else {
                self.name_and_category_ui(ui);
                self.notes_ui(ui);
            }
        });
    }

    fn name_and_category_ui(&mut self, ui: &mut Ui) {
        let name_error = self.show_errors.then_some(text::FIELD_NAME_ERROR);
        FormField::new(text::FIELD_NAME)
            .required(true)
            .help(text::FIELD_NAME_HELP)
            .error(name_error)
            .show(ui, |ui| {
                ui.add(
                    single_line(&mut self.name, "gallery_name")
                        .hint_text(text::FIELD_NAME_HINT)
                        .char_limit(60)
                        .desired_width(f32::INFINITY),
                )
            });
        ui.add_space(space::S);
        FormField::new(text::FIELD_CATEGORY).show(ui, |ui| {
            selector(
                ui,
                "gallery_category",
                &mut self.category,
                &CATEGORY_OPTIONS,
                220.0,
            )
        });
    }

    fn notes_ui(&mut self, ui: &mut Ui) {
        FormField::new(text::FIELD_NOTES)
            .help(text::FIELD_NOTES_HELP)
            .show(ui, |ui| {
                ui.add(
                    TextEdit::multiline(&mut self.notes)
                        .id_salt("gallery_notes")
                        .desired_rows(4)
                        .desired_width(f32::INFINITY),
                )
            });
    }

    fn status_ui(ui: &mut Ui) {
        Card::new().title(text::SECTION_STATUS).show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.add(StatusBadge::new(BadgeTone::Neutral, text::STATUS_NEUTRAL));
                ui.add(StatusBadge::new(BadgeTone::Pending, text::STATUS_PENDING));
                ui.add(StatusBadge::new(BadgeTone::Success, text::STATUS_SUCCESS));
                ui.add(StatusBadge::new(BadgeTone::Warning, text::STATUS_WARNING));
                ui.add(StatusBadge::new(BadgeTone::Error, text::STATUS_ERROR));
            });
            ui.label(RichText::new(text::SEMANTIC_NOTE).color(color::TEXT_MUTED));
        });
    }

    fn table_ui(&mut self, ui: &mut Ui) {
        Card::new().title(text::SECTION_TABLE).show(ui, |ui| {
            let has_selection = self.selected.is_some();
            let (search_changed, (restore, delete)) = egui::Sides::new().show(
                ui,
                |ui| {
                    ui.add(SearchField::new(&mut self.search, "gallery_search"))
                        .changed()
                },
                |ui| {
                    // Derecha a izquierda: el primero queda más a la derecha.
                    let restore = ui.add(SecondaryButton::new(text::RESTORE_ROWS)).clicked();
                    let delete = ui
                        .add_enabled(has_selection, SecondaryButton::new(text::DELETE_SELECTED))
                        .on_disabled_hover_text(text::DELETE_DISABLED)
                        .clicked();
                    (restore, delete)
                },
            );
            if restore {
                self.restore_rows();
            } else if search_changed {
                self.refresh_visible();
            }
            if delete {
                self.pending_delete = self.selected;
            }

            let columns = [
                ColumnSpec::new(text::COLUMN_CODE, ColumnWidth::Initial(110.0)),
                ColumnSpec::new(text::COLUMN_DESCRIPTION, ColumnWidth::Initial(320.0)),
                ColumnSpec::new(text::COLUMN_STATUS, ColumnWidth::Fill),
            ];
            let empty = if self.rows.is_empty() {
                EmptyState::new(text::NO_ROWS_TITLE).description(text::NO_ROWS_DESCRIPTION)
            } else {
                EmptyState::new(text::NO_MATCHES_TITLE)
                    .icon("🔍")
                    .description(text::NO_MATCHES_DESCRIPTION)
            };
            let selected_visible = self
                .selected
                .and_then(|row| self.visible.iter().position(|&i| i == row));

            let rows = &self.rows;
            let visible = &self.visible;
            let outcome = DataTable::new("gallery_table", &columns, visible.len(), empty)
                .selected(selected_visible)
                .max_height(240.0)
                .show(ui, |index, column, ui| {
                    let row = &rows[visible[index]];
                    match column {
                        0 => {
                            ui.label(RichText::new(&row.code).monospace().color(color::TEXT));
                        }
                        1 => {
                            ui.label(RichText::new(&row.description).color(color::TEXT));
                        }
                        _ => {
                            ui.add(StatusBadge::new(row.tone, row.status_text()));
                        }
                    }
                });

            if let Some(index) = outcome.clicked_row {
                let row = self.visible[index];
                self.selected = (self.selected != Some(row)).then_some(row);
            }
        });
    }

    fn empty_and_loading_ui(&mut self, ui: &mut Ui) {
        Card::new().title(text::SECTION_EMPTY).show(ui, |ui| {
            let clicked = EmptyState::new(text::EMPTY_TITLE)
                .description(text::EMPTY_DESCRIPTION)
                .action(text::EMPTY_ACTION)
                .show(ui);
            if clicked {
                self.last_event = Some(text::EMPTY_ACTION_MESSAGE);
            }
            // El indicador redibuja la pantalla en cada frame: sólo se muestra a pedido.
            if self.show_loading {
                ui.add(LoadingIndicator::default());
            }
        });
    }

    fn dialogs_ui(&mut self, ui: &mut Ui) {
        Card::new().title(text::SECTION_DIALOGS).show(ui, |ui| {
            let open = ui
                .add_enabled(
                    !self.rows.is_empty(),
                    SecondaryButton::new(text::OPEN_DIALOG),
                )
                .on_disabled_hover_text(text::NO_ROWS_DESCRIPTION);
            if open.clicked() {
                self.pending_delete = Some(0);
            }
        });
    }

    fn confirm_delete_ui(&mut self, ctx: &egui::Context) {
        let Some(index) = self.pending_delete else {
            return;
        };
        let Some(row) = self.rows.get(index) else {
            self.pending_delete = None;
            return;
        };
        let message = text::confirm_message(&row.code);
        let choice = ConfirmDialog::new(
            "gallery_confirm_delete",
            text::CONFIRM_TITLE,
            &message,
            text::CONFIRM_DELETE,
        )
        .show(ctx);
        match choice {
            Some(ConfirmChoice::Confirm) => {
                self.rows.remove(index);
                self.selected = None;
                self.pending_delete = None;
                self.last_event = Some(text::DELETED_MESSAGE);
                self.refresh_visible();
            }
            Some(ConfirmChoice::Cancel) => {
                self.pending_delete = None;
                self.last_event = Some(text::CANCELLED_MESSAGE);
            }
            None => {}
        }
    }

    fn restore_rows(&mut self) {
        self.rows = sample_rows();
        self.selected = None;
        self.refresh_visible();
    }

    fn refresh_visible(&mut self) {
        let query = self.search.trim().to_lowercase();
        self.visible = self
            .rows
            .iter()
            .enumerate()
            .filter(|(_, row)| {
                query.is_empty()
                    || row.code.to_lowercase().contains(&query)
                    || row.description.to_lowercase().contains(&query)
            })
            .map(|(index, _)| index)
            .collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_keeps_only_matching_rows() {
        let mut gallery = ComponentGallery {
            search: "EJ-002".to_owned(),
            ..Default::default()
        };
        gallery.refresh_visible();
        assert_eq!(gallery.visible, vec![1]);
    }

    #[test]
    fn empty_search_shows_every_row() {
        let mut gallery = ComponentGallery {
            search: "   ".to_owned(),
            ..Default::default()
        };
        gallery.refresh_visible();
        assert_eq!(gallery.visible.len(), SAMPLE_ROW_COUNT);
    }
}
