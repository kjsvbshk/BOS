//! Fuentes: Segoe UI de Windows (regular, semibold y negrita) e iconos Phosphor.
//!
//! Segoe UI se lee de la carpeta de fuentes de Windows; no se redistribuye. Si
//! no está (otro sistema, equipo sin la fuente), se usa Ubuntu Light, incluida
//! en egui. Los iconos Phosphor van dentro del ejecutable
//! (`assets/fonts/Phosphor-Fill.ttf`, licencia MIT).

use std::{path::PathBuf, sync::Arc};

use egui::{Context, FontData, FontDefinitions, FontFamily, FontId};

const PHOSPHOR: &str = "phosphor-fill";
const SEGOE_REGULAR: &str = "segoe-ui";
const SEGOE_SEMIBOLD: &str = "segoe-ui-semibold";
const SEGOE_BOLD: &str = "segoe-ui-bold";
/// Fuente proporcional incluida en egui, usada si falta Segoe UI.
const EGUI_PROPORTIONAL: &str = "Ubuntu-Light";

const SEMIBOLD_FAMILY: &str = "semibold";
const BOLD_FAMILY: &str = "bold";

/// Fuente seminegrita del tamaño indicado.
#[must_use]
pub fn semibold(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name(SEMIBOLD_FAMILY.into()))
}

/// Fuente negrita del tamaño indicado.
#[must_use]
pub fn bold(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name(BOLD_FAMILY.into()))
}

/// Instala las fuentes. Llamar una sola vez, desde el constructor de la app.
pub fn install(ctx: &Context) {
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        PHOSPHOR.to_owned(),
        Arc::new(FontData::from_static(include_bytes!(
            "../../assets/fonts/Phosphor-Fill.ttf"
        ))),
    );

    let regular = load_windows_font(&mut fonts, SEGOE_REGULAR, "segoeui.ttf");
    let semibold = load_windows_font(&mut fonts, SEGOE_SEMIBOLD, "seguisb.ttf").or(regular);
    let bold = load_windows_font(&mut fonts, SEGOE_BOLD, "segoeuib.ttf").or(semibold);

    // Respaldo común: la fuente de egui y después los iconos. Phosphor va antes
    // que las fuentes de emoji de egui porque ambas usan el área de uso privado
    // de Unicode y los iconos de la app deben ganar.
    let fallbacks = |primary: Option<&str>| -> Vec<String> {
        let mut list: Vec<String> = primary.into_iter().map(str::to_owned).collect();
        list.push(EGUI_PROPORTIONAL.to_owned());
        list.push(PHOSPHOR.to_owned());
        list
    };

    let proportional = fonts.families.entry(FontFamily::Proportional).or_default();
    let egui_emoji: Vec<String> = proportional
        .iter()
        .filter(|name| *name != EGUI_PROPORTIONAL)
        .cloned()
        .collect();
    let mut chain = fallbacks(regular);
    chain.extend(egui_emoji.iter().cloned());
    *proportional = chain;

    for (family, primary) in [(SEMIBOLD_FAMILY, semibold), (BOLD_FAMILY, bold)] {
        let mut chain = fallbacks(primary);
        chain.extend(egui_emoji.iter().cloned());
        fonts
            .families
            .insert(FontFamily::Name(family.into()), chain);
    }

    if let Some(monospace) = fonts.families.get_mut(&FontFamily::Monospace) {
        monospace.push(PHOSPHOR.to_owned());
    }

    ctx.set_fonts(fonts);
}

/// Carga una fuente de `%WINDIR%\Fonts`. Devuelve su nombre si se pudo leer.
fn load_windows_font(
    fonts: &mut FontDefinitions,
    name: &'static str,
    file: &str,
) -> Option<&'static str> {
    let windows = std::env::var_os("WINDIR")?;
    let path: PathBuf = [windows.as_os_str(), "Fonts".as_ref(), file.as_ref()]
        .iter()
        .collect();
    let bytes = std::fs::read(path).ok()?;
    fonts
        .font_data
        .insert(name.to_owned(), Arc::new(FontData::from_owned(bytes)));
    Some(name)
}
