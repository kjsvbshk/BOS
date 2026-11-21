//! Componentes visuales reutilizables. No dependen de ningún módulo de negocio:
//! reciben textos y datos, y devuelven lo que hizo el usuario.
//!
//! Reglas comunes (ver `docs/design-system.md`):
//! - Los colores salen de `theme::tokens`, nunca de valores sueltos.
//! - No usar `RichText::strong()` ni `ui.strong()`: egui toma ese color del
//!   estilo de foco, que en este tema es marfil sobre gris oliva.

mod brand_mark;
mod buttons;
mod card;
mod data_table;
mod empty_state;
mod form_field;
mod info_banner;
mod loading;
mod modal;
mod option_card;
mod page_header;
mod paint;
mod search_field;
mod section_tab;
mod selector;
mod status_badge;
mod text_input;

pub use brand_mark::brand_mark;
pub use buttons::{PrimaryButton, SecondaryButton};
pub use card::Card;
pub use data_table::{ColumnSpec, ColumnWidth, DataTable, TableOutcome};
pub use empty_state::EmptyState;
pub use form_field::FormField;
pub use info_banner::info_banner;
pub use loading::LoadingIndicator;
pub use modal::{AppModal, ConfirmChoice, ConfirmDialog, ModalOutcome};
pub use option_card::OptionCard;
pub use page_header::PageHeader;
pub use search_field::SearchField;
pub use section_tab::SectionTab;
pub use selector::selector;
pub use status_badge::{BadgeTone, StatusBadge};
pub use text_input::single_line;
