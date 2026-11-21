//! Valores del sistema visual. Las vistas y los componentes usan estos tokens;
//! nunca valores sueltos (ADR-0004).

/// Colores. Los cuatro primeros son la paleta oficial; los demás son mezclas de
/// ella o la propuesta de colores semánticos.
pub mod color {
    use egui::Color32;

    // Paleta oficial.
    /// Negro oliva `#11120d`.
    pub const OLIVE_BLACK: Color32 = Color32::from_rgb(0x11, 0x12, 0x0d);
    /// Gris oliva `#565449`.
    pub const OLIVE_GRAY: Color32 = Color32::from_rgb(0x56, 0x54, 0x49);
    /// Beige `#D8CFBC`.
    pub const BEIGE: Color32 = Color32::from_rgb(0xD8, 0xCF, 0xBC);
    /// Marfil `#FFFBF4`.
    pub const IVORY: Color32 = Color32::from_rgb(0xFF, 0xFB, 0xF4);

    // Mezclas de la paleta (no son colores nuevos).
    /// 50 % beige + 50 % marfil: superficies elevadas y filas alternas.
    pub const BEIGE_SOFT: Color32 = Color32::from_rgb(0xEC, 0xE5, 0xD8);
    /// 75 % beige + 25 % gris oliva: botón secundario bajo el puntero.
    pub const BEIGE_DEEP: Color32 = Color32::from_rgb(0xB8, 0xB0, 0x9F);
    /// 70 % negro oliva + 30 % gris oliva: barra de secciones.
    pub const OLIVE_DARK: Color32 = Color32::from_rgb(0x26, 0x26, 0x1F);

    // Colores semánticos: PROPUESTA pendiente de aprobación (ADR-0004).
    /// Texto e icono de error.
    pub const ERROR: Color32 = Color32::from_rgb(0x9A, 0x33, 0x24);
    /// Fondo de los avisos de error.
    pub const ERROR_BG: Color32 = Color32::from_rgb(0xF3, 0xDE, 0xD7);
    /// Texto e icono de advertencia.
    pub const WARNING: Color32 = Color32::from_rgb(0x7A, 0x5A, 0x0C);
    /// Fondo de los avisos de advertencia.
    pub const WARNING_BG: Color32 = Color32::from_rgb(0xF1, 0xE6, 0xC6);
    /// Texto e icono de éxito.
    pub const SUCCESS: Color32 = Color32::from_rgb(0x3F, 0x64, 0x30);
    /// Fondo de los avisos de éxito.
    pub const SUCCESS_BG: Color32 = Color32::from_rgb(0xDF, 0xE8, 0xD3);

    // Roles: qué color cumple cada función.
    /// Texto principal.
    pub const TEXT: Color32 = OLIVE_BLACK;
    /// Texto secundario.
    pub const TEXT_MUTED: Color32 = OLIVE_GRAY;
    /// Texto sobre fondos oscuros.
    pub const TEXT_ON_DARK: Color32 = IVORY;
    /// Fondo de la zona de contenido.
    pub const SURFACE: Color32 = IVORY;
    /// Tarjetas y superficies agrupadas.
    pub const SURFACE_RAISED: Color32 = BEIGE_SOFT;
    /// Bordes de controles (campos, tablas). Cumple 3:1 sobre marfil.
    pub const BORDER_STRONG: Color32 = OLIVE_GRAY;
    /// Separadores y bordes decorativos. No cumple 3:1 sobre marfil: nunca
    /// debe ser el único borde de un control.
    pub const BORDER_SUBTLE: Color32 = BEIGE;
    /// Fondo de la acción principal.
    pub const PRIMARY: Color32 = OLIVE_BLACK;
    /// Fondo de la acción principal bajo el puntero o con foco.
    pub const PRIMARY_HOVER: Color32 = OLIVE_GRAY;
    /// Elemento seleccionado.
    pub const SELECTED: Color32 = BEIGE;

    /// Encabezado de la aplicación (marca, fecha, usuario).
    pub const CHROME: Color32 = OLIVE_BLACK;
    /// Barra de pestañas de secciones.
    pub const CHROME_RAISED: Color32 = OLIVE_DARK;
    /// Pestaña o elemento bajo el puntero dentro de la barra oscura.
    pub const CHROME_HOVER: Color32 = OLIVE_GRAY;
    /// Texto secundario sobre la barra oscura.
    pub const TEXT_ON_DARK_MUTED: Color32 = BEIGE;
    /// Fondo de los círculos de icono sobre superficie clara.
    pub const ICON_BG: Color32 = BEIGE;
}

/// Espaciado en una retícula de 4 puntos.
pub mod space {
    pub const XS: f32 = 4.0;
    pub const S: f32 = 8.0;
    pub const M: f32 = 12.0;
    pub const L: f32 = 16.0;
    pub const XL: f32 = 24.0;
    pub const XXL: f32 = 32.0;
}

/// Los mismos pasos de espaciado, para márgenes de marcos y paneles (egui los
/// expresa en enteros).
pub mod margin {
    pub const XS: i8 = 4;
    pub const S: i8 = 8;
    pub const M: i8 = 12;
    pub const L: i8 = 16;
    pub const XL: i8 = 24;
}

/// Medidas de controles.
pub mod size {
    /// Alto mínimo de botones, campos y selectores.
    pub const CONTROL_HEIGHT: f32 = 32.0;
    /// Alto de una fila de tabla.
    pub const TABLE_ROW_HEIGHT: f32 = 30.0;
    /// Alto del encabezado de tabla.
    pub const TABLE_HEADER_HEIGHT: f32 = 32.0;
    /// Ancho de lectura cómodo para textos largos.
    pub const PROSE_MAX_WIDTH: f32 = 560.0;
    /// Ancho de los diálogos.
    pub const MODAL_WIDTH: f32 = 420.0;
    /// Alto del encabezado de la aplicación.
    pub const APP_HEADER_HEIGHT: f32 = 76.0;
    /// Alto de las pestañas de sección.
    pub const TAB_HEIGHT: f32 = 48.0;
    /// Alto de una tarjeta de opción.
    pub const OPTION_CARD_HEIGHT: f32 = 88.0;
    /// Diámetro del círculo de icono de las tarjetas.
    pub const CARD_ICON_DIAMETER: f32 = 52.0;
    /// Diámetro del círculo de icono del título de página.
    pub const PAGE_ICON_DIAMETER: f32 = 60.0;
    /// Ancho mínimo de una tarjeta antes de pasar a menos columnas.
    pub const OPTION_CARD_MIN_WIDTH: f32 = 300.0;
}

/// Radios de esquina.
pub mod radius {
    pub const CONTROL: u8 = 4;
    pub const CARD: u8 = 6;
    pub const MODAL: u8 = 8;
    pub const BADGE: u8 = 10;
}

/// Tamaños de texto en puntos: base 15, según la referencia visual aprobada.
pub mod font_size {
    pub const SMALL: f32 = 12.0;
    pub const BODY: f32 = 15.0;
    pub const MONOSPACE: f32 = 13.0;
    pub const HEADING: f32 = 18.0;
    pub const TITLE: f32 = 26.0;
    /// Nombre del negocio en el encabezado.
    pub const BRAND: f32 = 24.0;
    /// Iconos de tarjetas y pestañas.
    pub const ICON: f32 = 26.0;
}

#[cfg(test)]
mod tests {
    use egui::Color32;

    use super::color::*;

    /// Contraste WCAG 2.2 entre dos colores.
    fn contrast(a: Color32, b: Color32) -> f32 {
        fn channel(value: u8) -> f32 {
            let c = f32::from(value) / 255.0;
            if c <= 0.039_28 {
                c / 12.92
            } else {
                ((c + 0.055) / 1.055).powf(2.4)
            }
        }
        fn luminance(color: Color32) -> f32 {
            0.2126 * channel(color.r()) + 0.7152 * channel(color.g()) + 0.0722 * channel(color.b())
        }
        let (la, lb) = (luminance(a), luminance(b));
        (la.max(lb) + 0.05) / (la.min(lb) + 0.05)
    }

    const TEXT_MIN: f32 = 4.5;
    const COMPONENT_MIN: f32 = 3.0;

    #[test]
    fn text_pairs_meet_wcag_aa() {
        let pairs = [
            ("texto sobre superficie", TEXT, SURFACE),
            ("texto secundario sobre superficie", TEXT_MUTED, SURFACE),
            ("texto sobre superficie elevada", TEXT, SURFACE_RAISED),
            (
                "texto secundario sobre superficie elevada",
                TEXT_MUTED,
                SURFACE_RAISED,
            ),
            ("texto sobre selección", TEXT, SELECTED),
            ("texto secundario sobre beige", TEXT_MUTED, BEIGE),
            ("texto sobre botón secundario con puntero", TEXT, BEIGE_DEEP),
            ("texto de acción principal", TEXT_ON_DARK, PRIMARY),
            (
                "texto de acción principal con puntero",
                TEXT_ON_DARK,
                PRIMARY_HOVER,
            ),
            ("texto sobre encabezado", TEXT_ON_DARK, CHROME),
            (
                "texto secundario sobre encabezado",
                TEXT_ON_DARK_MUTED,
                CHROME,
            ),
            (
                "texto sobre barra de secciones",
                TEXT_ON_DARK,
                CHROME_RAISED,
            ),
            (
                "texto sobre pestaña con puntero",
                TEXT_ON_DARK,
                CHROME_HOVER,
            ),
            ("icono sobre su círculo", TEXT, ICON_BG),
            ("error sobre superficie", ERROR, SURFACE),
            ("error sobre su fondo", ERROR, ERROR_BG),
            ("advertencia sobre superficie", WARNING, SURFACE),
            ("advertencia sobre su fondo", WARNING, WARNING_BG),
            ("éxito sobre superficie", SUCCESS, SURFACE),
            ("éxito sobre su fondo", SUCCESS, SUCCESS_BG),
        ];
        let failing: Vec<String> = pairs
            .iter()
            .filter(|(_, fg, bg)| contrast(*fg, *bg) < TEXT_MIN)
            .map(|(name, fg, bg)| format!("{name}: {:.2}:1", contrast(*fg, *bg)))
            .collect();
        assert!(
            failing.is_empty(),
            "pares de texto por debajo de 4.5:1: {failing:?}"
        );
    }

    #[test]
    fn control_borders_meet_wcag_aa() {
        let pairs = [
            ("borde de control sobre superficie", BORDER_STRONG, SURFACE),
            (
                "borde de control sobre superficie elevada",
                BORDER_STRONG,
                SURFACE_RAISED,
            ),
            ("acción principal sobre superficie", PRIMARY, SURFACE),
        ];
        let failing: Vec<String> = pairs
            .iter()
            .filter(|(_, fg, bg)| contrast(*fg, *bg) < COMPONENT_MIN)
            .map(|(name, fg, bg)| format!("{name}: {:.2}:1", contrast(*fg, *bg)))
            .collect();
        assert!(failing.is_empty(), "bordes por debajo de 3:1: {failing:?}");
    }

    #[test]
    fn subtle_border_is_not_enough_as_a_control_border() {
        // Documenta la regla del ADR-0004: el beige sobre marfil no delimita un control.
        assert!(contrast(BORDER_SUBTLE, SURFACE) < COMPONENT_MIN);
    }
}
