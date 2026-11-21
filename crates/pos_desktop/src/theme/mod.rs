//! Tema visual de la aplicación (ADR-0004): un único tema claro con la paleta
//! oficial. Se aplica una sola vez, al crear la aplicación.

pub mod fonts;
pub mod icons;
pub mod tokens;

use egui::{
    Color32, Context, CornerRadius, FontFamily, FontId, Margin, Stroke, Style, TextStyle,
    ThemePreference, Ui, Visuals, vec2,
};

use self::tokens::{color, font_size, margin, radius, size, space};

/// Nombre del estilo de texto para títulos de página.
const TITLE_STYLE: &str = "Title";

/// Estilo de texto para el título principal de una pantalla.
#[must_use]
pub fn title_style() -> TextStyle {
    TextStyle::Name(TITLE_STYLE.into())
}

/// Aplica el tema a todo el contexto. Llamar sólo desde el constructor de la app.
///
/// egui guarda un estilo para su tema claro y otro para el oscuro. La paleta es
/// una sola, así que se aplica a ambos: si algo (el sistema operativo, el
/// arnés de pruebas) activa el tema oscuro de egui, la app se ve igual.
pub fn apply(ctx: &Context) {
    fonts::install(ctx);
    ctx.set_theme(ThemePreference::Light);
    ctx.all_styles_mut(configure_style);
}

/// Ajusta el estilo de `ui` para zonas de fondo oscuro (encabezado y barra de
/// secciones): texto marfil y controles sin fondo propio. Sólo afecta a `ui`
/// y a lo que se dibuje dentro; los menús emergentes conservan el tema claro.
pub fn use_chrome_style(ui: &mut Ui) {
    let visuals = ui.visuals_mut();
    visuals.override_text_color = Some(color::TEXT_ON_DARK);
    let text = Stroke::new(1.0, color::TEXT_ON_DARK);
    visuals.widgets.noninteractive.fg_stroke = text;
    visuals.widgets.inactive = widget(
        Color32::TRANSPARENT,
        Color32::TRANSPARENT,
        Stroke::NONE,
        text,
    );
    visuals.widgets.hovered = widget(color::CHROME_HOVER, color::CHROME_HOVER, Stroke::NONE, text);
    visuals.widgets.active = widget(
        color::CHROME_HOVER,
        color::CHROME_HOVER,
        Stroke::new(1.0, color::TEXT_ON_DARK),
        text,
    );
    visuals.widgets.open = visuals.widgets.hovered;
}

fn configure_style(style: &mut Style) {
    style.visuals = visuals();
    style.text_styles = [
        (TextStyle::Small, proportional(font_size::SMALL)),
        (TextStyle::Body, proportional(font_size::BODY)),
        (TextStyle::Button, proportional(font_size::BODY)),
        (
            TextStyle::Monospace,
            FontId::new(font_size::MONOSPACE, FontFamily::Monospace),
        ),
        (TextStyle::Heading, fonts::semibold(font_size::HEADING)),
        (title_style(), fonts::bold(font_size::TITLE)),
    ]
    .into();

    let spacing = &mut style.spacing;
    spacing.item_spacing = vec2(space::S, space::S);
    spacing.button_padding = vec2(space::M, space::XS);
    spacing.interact_size.y = size::CONTROL_HEIGHT;
    spacing.window_margin = Margin::same(margin::L);
    spacing.menu_margin = Margin::same(margin::S);
    spacing.indent = space::L;

    style.animation_time = 0.15;
}

fn proportional(points: f32) -> FontId {
    FontId::new(points, FontFamily::Proportional)
}

fn widget(
    bg: Color32,
    weak_bg: Color32,
    border: Stroke,
    text: Stroke,
) -> egui::style::WidgetVisuals {
    egui::style::WidgetVisuals {
        bg_fill: bg,
        weak_bg_fill: weak_bg,
        bg_stroke: border,
        corner_radius: CornerRadius::same(radius::CONTROL),
        fg_stroke: text,
        expansion: 0.0,
    }
}

fn visuals() -> Visuals {
    let mut visuals = Visuals::light();

    visuals.panel_fill = color::SURFACE;
    visuals.window_fill = color::SURFACE;
    visuals.window_stroke = Stroke::new(1.0, color::BORDER_STRONG);
    visuals.window_corner_radius = CornerRadius::same(radius::MODAL);
    visuals.menu_corner_radius = CornerRadius::same(radius::CONTROL);

    visuals.faint_bg_color = color::SURFACE_RAISED;
    visuals.extreme_bg_color = color::SURFACE;
    visuals.text_edit_bg_color = Some(color::SURFACE);
    visuals.code_bg_color = color::SURFACE_RAISED;
    visuals.weak_text_color = Some(color::TEXT_MUTED);
    visuals.hyperlink_color = color::TEXT_MUTED;
    visuals.error_fg_color = color::ERROR;
    visuals.warn_fg_color = color::WARNING;

    // Selección: fondo beige y trazo negro de 2 px, que también marca el foco
    // de los campos de texto.
    visuals.selection.bg_fill = color::SELECTED;
    visuals.selection.stroke = Stroke::new(2.0, color::TEXT);

    let text = Stroke::new(1.0, color::TEXT);
    let border = Stroke::new(1.0, color::BORDER_STRONG);
    let border_strong = Stroke::new(1.0, color::TEXT);

    visuals.widgets.noninteractive = widget(
        color::SURFACE,
        color::SURFACE,
        Stroke::new(1.0, color::BORDER_SUBTLE),
        text,
    );
    visuals.widgets.inactive = widget(color::SURFACE, color::BEIGE, border, text);
    visuals.widgets.hovered = widget(
        color::SURFACE_RAISED,
        color::BEIGE_DEEP,
        border_strong,
        text,
    );
    // egui usa el estilo `active` también para el foco de teclado: debe
    // distinguirse claramente del estado normal.
    visuals.widgets.active = widget(
        color::OLIVE_GRAY,
        color::OLIVE_GRAY,
        border_strong,
        Stroke::new(1.0, color::TEXT_ON_DARK),
    );
    visuals.widgets.open = visuals.widgets.hovered;

    visuals
}
