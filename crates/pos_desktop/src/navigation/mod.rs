//! Navegación: catálogo de opciones y ruta actual.

pub mod catalog;

pub use catalog::{NavAction, NavOption, NavSection, OptionGroup, OptionStatus, SectionId};

/// Pantalla que se muestra en el área de contenido.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Route {
    /// Página principal de una sección.
    Section(SectionId),
    /// Pantalla de una opción.
    Option(&'static NavOption),
    /// Acción dentro de una opción.
    Action(&'static NavOption, &'static NavAction),
    /// Catálogo de componentes (sólo en compilaciones de desarrollo).
    ComponentGallery,
}

impl Default for Route {
    fn default() -> Self {
        Self::Section(SectionId::Operations)
    }
}

impl Route {
    /// Sección cuya pestaña debe verse seleccionada.
    #[must_use]
    pub fn section(self) -> Option<SectionId> {
        match self {
            Self::Section(id) => Some(id),
            Self::Option(option) | Self::Action(option, _) => Some(catalog::section_of(option).id),
            Self::ComponentGallery => None,
        }
    }

    /// Pantalla a la que lleva «Volver» (y la tecla Escape).
    #[must_use]
    pub fn parent(self) -> Option<Self> {
        match self {
            Self::Section(_) => None,
            Self::Option(option) => Some(Self::Section(catalog::section_of(option).id)),
            Self::Action(option, _) => Some(Self::Option(option)),
            Self::ComponentGallery => Some(Self::default()),
        }
    }
}
