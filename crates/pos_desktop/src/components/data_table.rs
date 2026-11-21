//! Tabla de datos con encabezado, filas alternas, selección y estado vacío.

use egui::{
    Align, AsIdSalt, CornerRadius, Frame, IdSalt, Layout, Margin, RichText, Sense, Stroke, Ui,
};
use egui_extras::{Column, TableBuilder};

use super::EmptyState;
use crate::theme::tokens::{color, font_size, margin, radius, size};

/// Ancho del indicador de fila seleccionada.
const MARKER_WIDTH: f32 = 18.0;

/// Cómo se reparte el ancho de una columna.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColumnWidth {
    /// Ancho fijo.
    Fixed(f32),
    /// Ancho inicial que el usuario puede cambiar.
    Initial(f32),
    /// Ocupa el espacio restante.
    Fill,
}

/// Definición de una columna.
#[derive(Debug, Clone, Copy)]
pub struct ColumnSpec<'a> {
    pub title: &'a str,
    pub width: ColumnWidth,
}

impl<'a> ColumnSpec<'a> {
    #[must_use]
    pub fn new(title: &'a str, width: ColumnWidth) -> Self {
        Self { title, width }
    }
}

/// Resultado de dibujar la tabla durante un frame.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TableOutcome {
    /// Fila sobre la que se hizo clic.
    pub clicked_row: Option<usize>,
    /// `true` si se pulsó la acción del estado vacío.
    pub empty_action_clicked: bool,
}

/// Tabla con filas virtualizadas: sólo se dibujan las filas visibles, así que
/// admite listas largas.
///
/// La fila seleccionada se marca con fondo beige **y** con el símbolo «▶», para
/// que la selección no dependa sólo del color.
pub struct DataTable<'a> {
    id_salt: IdSalt,
    columns: &'a [ColumnSpec<'a>],
    row_count: usize,
    selected: Option<usize>,
    max_height: f32,
    empty: EmptyState<'a>,
}

impl<'a> DataTable<'a> {
    /// `id_salt` debe ser único entre las tablas de la pantalla.
    #[must_use]
    pub fn new(
        id_salt: impl AsIdSalt,
        columns: &'a [ColumnSpec<'a>],
        row_count: usize,
        empty: EmptyState<'a>,
    ) -> Self {
        Self {
            id_salt: IdSalt::new(id_salt),
            columns,
            row_count,
            selected: None,
            max_height: 400.0,
            empty,
        }
    }

    /// Alto máximo del cuerpo; con más filas aparece la barra de desplazamiento.
    #[must_use]
    pub fn max_height(mut self, max_height: f32) -> Self {
        self.max_height = max_height;
        self
    }

    /// Fila seleccionada, si hay una.
    #[must_use]
    pub fn selected(mut self, selected: Option<usize>) -> Self {
        self.selected = selected;
        self
    }

    /// Dibuja la tabla. `cell(fila, columna, ui)` dibuja el contenido de cada celda.
    pub fn show(self, ui: &mut Ui, cell: impl FnMut(usize, usize, &mut Ui)) -> TableOutcome {
        // Fondo marfil propio: dentro de una tarjeta beige, las filas alternas
        // y la selección necesitan contrastar con la superficie.
        Frame::new()
            .fill(color::SURFACE)
            .stroke(Stroke::new(1.0, color::BORDER_SUBTLE))
            .corner_radius(CornerRadius::same(radius::CARD))
            .inner_margin(Margin::same(margin::XS))
            .show(ui, |ui| {
                // Las filas no heredan el espaciado vertical de la página.
                ui.spacing_mut().item_spacing.y = 0.0;
                self.table_ui(ui, cell)
            })
            .inner
    }

    fn table_ui(self, ui: &mut Ui, mut cell: impl FnMut(usize, usize, &mut Ui)) -> TableOutcome {
        let mut outcome = TableOutcome::default();

        let mut builder = TableBuilder::new(ui)
            .id_salt(self.id_salt)
            .striped(true)
            .resizable(true)
            .sense(Sense::click())
            .max_scroll_height(self.max_height)
            .auto_shrink([false, true])
            .cell_layout(Layout::left_to_right(Align::Center))
            .column(Column::exact(MARKER_WIDTH));
        for spec in self.columns {
            let column = match spec.width {
                ColumnWidth::Fixed(width) => Column::exact(width),
                ColumnWidth::Initial(width) => Column::initial(width).at_least(60.0),
                ColumnWidth::Fill => Column::remainder().at_least(80.0),
            };
            builder = builder.column(column.clip(true));
        }

        let table = builder.header(size::TABLE_HEADER_HEIGHT, |mut header| {
            header.col(|ui| underline_header(ui));
            for spec in self.columns {
                header.col(|ui| {
                    ui.label(RichText::new(spec.title).color(color::TEXT));
                    underline_header(ui);
                });
            }
        });

        if self.row_count == 0 {
            table.body(|_| {});
            outcome.empty_action_clicked = self.empty.show(ui);
            return outcome;
        }

        let column_count = self.columns.len();
        table.body(|body| {
            body.rows(size::TABLE_ROW_HEIGHT, self.row_count, |mut row| {
                let index = row.index();
                let is_selected = self.selected == Some(index);
                row.set_selected(is_selected);
                row.col(|ui| {
                    if is_selected {
                        ui.label(RichText::new("▶").size(font_size::SMALL).color(color::TEXT));
                    }
                });
                for column in 0..column_count {
                    row.col(|ui| {
                        // Una etiqueta seleccionable captura el clic y la fila no lo recibiría.
                        ui.style_mut().interaction.selectable_labels = false;
                        cell(index, column, ui);
                    });
                }
                if row.response().clicked() {
                    outcome.clicked_row = Some(index);
                }
            });
        });
        outcome
    }
}

/// Línea inferior del encabezado, en el color de borde de controles.
fn underline_header(ui: &Ui) {
    let rect = ui.max_rect();
    // Un punto por encima del borde: justo en el borde queda fuera del recorte.
    let y = rect.bottom() - 1.0;
    ui.painter()
        .hline(rect.x_range(), y, Stroke::new(1.0, color::BORDER_STRONG));
}
