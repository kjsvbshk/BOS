//! Aplicación de escritorio del POS de empeños (egui/eframe).
//!
//! Sólo contiene presentación: tema, componentes y vistas. Las reglas de
//! negocio viven en `pos_core` (ADR-0001).

pub mod app;
pub mod components;
pub mod layout;
pub mod navigation;
pub mod strings;
pub mod theme;
pub mod views;

pub use app::PosApp;

/// Identificador de la aplicación. eframe lo usa para la carpeta de datos:
/// no cambiarlo después de publicar la aplicación.
pub const APP_ID: &str = "pos_pawnshop";

/// Tamaño inicial de la ventana.
pub const INITIAL_WINDOW_SIZE: [f32; 2] = [1366.0, 768.0];
/// Tamaño mínimo de la ventana.
pub const MIN_WINDOW_SIZE: [f32; 2] = [1280.0, 720.0];

/// Opciones de la ventana nativa.
#[must_use]
pub fn native_options() -> eframe::NativeOptions {
    eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title(strings::APP_TITLE)
            .with_app_id(APP_ID)
            .with_inner_size(INITIAL_WINDOW_SIZE)
            .with_min_inner_size(MIN_WINDOW_SIZE),
        // OpenGL por compatibilidad con equipos modestos y escritorio remoto (ADR-0003).
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    }
}

/// Abre la ventana principal y ejecuta la aplicación hasta que se cierre.
///
/// # Errors
/// Devuelve un error si no se puede crear la ventana o el contexto gráfico.
pub fn run() -> eframe::Result {
    eframe::run_native(
        APP_ID,
        native_options(),
        Box::new(|cc| Ok(Box::new(PosApp::new(cc)))),
    )
}
