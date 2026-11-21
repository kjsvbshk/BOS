//! Capturas para revisión visual (skill egui-ui-design, sección 3).
//!
//! Se ignora por defecto porque necesita un adaptador gráfico:
//! `cargo test -p pos_desktop --test review -- --ignored --nocapture`

use egui_kittest::{Harness, kittest::Queryable};
use pos_desktop::{INITIAL_WINDOW_SIZE, MIN_WINDOW_SIZE, PosApp};

fn harness(size: [f32; 2], zoom: f32) -> Harness<'static, PosApp> {
    let mut harness = Harness::builder()
        .with_size(egui::Vec2::from(size))
        .build_eframe(|cc| PosApp::new(cc));
    harness.ctx.set_zoom_factor(zoom);
    harness.run();
    harness
}

/// Guarda la captura en la carpeta temporal e imprime su ruta. Falla si no
/// hay adaptador gráfico o no se puede escribir el archivo.
fn save(harness: &mut Harness<'_, PosApp>, name: &str) -> Result<(), String> {
    let path = std::env::temp_dir().join(format!("pos-review-{name}.png"));
    let image = harness.render()?;
    image.save(&path).map_err(|err| err.to_string())?;
    eprintln!("{}", path.display());
    Ok(())
}

fn click(harness: &mut Harness<'_, PosApp>, label: &str) {
    harness.get_by_label(label).click();
    harness.run();
}

#[test]
#[ignore = "genera capturas PNG para revisión manual"]
fn render_operations_for_review() {
    save(&mut harness(MIN_WINDOW_SIZE, 1.0), "operaciones-minimo").unwrap();
    save(
        &mut harness(INITIAL_WINDOW_SIZE, 1.0),
        "operaciones-inicial",
    )
    .unwrap();
    save(
        &mut harness([1536.0, 1024.0], 1.0),
        "operaciones-referencia",
    )
    .unwrap();
    save(&mut harness(MIN_WINDOW_SIZE, 1.5), "operaciones-zoom-150").unwrap();
}

#[test]
#[ignore = "genera capturas PNG para revisión manual"]
fn render_screens_for_review() {
    let mut harness = harness(INITIAL_WINDOW_SIZE, 1.0);

    click(&mut harness, "09. Modificación de contratos");
    save(&mut harness, "opcion-09").unwrap();

    click(&mut harness, "Reversar");
    save(&mut harness, "accion-reversar").unwrap();

    click(&mut harness, "Mantenimiento");
    save(&mut harness, "mantenimiento").unwrap();

    harness.get_by_label_contains("Salir").click();
    harness.run();
    save(&mut harness, "salir").unwrap();
}
