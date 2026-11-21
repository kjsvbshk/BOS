//! Capturas para revisión visual (skill egui-ui-design, sección 3).
//!
//! Se ignora por defecto porque necesita un adaptador gráfico:
//! `cargo test -p pos_desktop --test review -- --ignored --nocapture`

mod common;

use common::{meeting_day, open, press, type_and_enter};
use egui::Key;
use egui_kittest::{Harness, kittest::Queryable};
use pos_desktop::{INITIAL_WINDOW_SIZE, MIN_WINDOW_SIZE, PosApp};

fn harness(size: [f32; 2], zoom: f32) -> Harness<'static, PosApp> {
    let mut harness = Harness::builder()
        .with_size(egui::Vec2::from(size))
        .build_eframe(|cc| PosApp::for_tests(cc, meeting_day(), "PRO"));
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

#[test]
#[ignore = "genera capturas PNG para revisión manual"]
fn render_sections_for_review() {
    let mut sign_in = Harness::builder()
        .with_size(egui::Vec2::from(INITIAL_WINDOW_SIZE))
        .build_eframe(|cc| PosApp::new(cc));
    sign_in.run();
    save(&mut sign_in, "inicio-sesion").unwrap();
    save(&mut harness(MIN_WINDOW_SIZE, 1.0), "operaciones-minimo").unwrap();
    save(&mut harness(MIN_WINDOW_SIZE, 1.5), "operaciones-zoom-150").unwrap();
    let mut h = harness(INITIAL_WINDOW_SIZE, 1.0);
    h.get_by_label("Mantenimiento").click();
    h.run();
    save(&mut h, "mantenimiento").unwrap();
}

#[test]
#[ignore = "genera capturas PNG para revisión manual"]
fn render_flows_for_review() {
    let mut h = harness([1366.0, 900.0], 1.0);
    open(&mut h, "1");
    press(&mut h, Key::Enter);
    type_and_enter(&mut h, "26613");
    press(&mut h, Key::Enter);
    press(&mut h, Key::Enter);
    press(&mut h, Key::F1);
    save(&mut h, "consulta-f1").unwrap();
    press(&mut h, Key::Enter);
    save(&mut h, "ingreso-contrato").unwrap();

    let mut h = harness([1366.0, 900.0], 1.0);
    open(&mut h, "3");
    type_and_enter(&mut h, "26247");
    press(&mut h, Key::Enter);
    press(&mut h, Key::S);
    save(&mut h, "retiro-revision").unwrap();
    press(&mut h, Key::G);
    save(&mut h, "retiro-grabado").unwrap();

    let mut h = harness([1366.0, 900.0], 1.0);
    open(&mut h, "4");
    save(&mut h, "vencidos").unwrap();

    let mut h = harness([1366.0, 900.0], 1.0);
    open(&mut h, "9");
    open(&mut h, "1");
    type_and_enter(&mut h, "26148");
    save(&mut h, "consulta-contrato").unwrap();

    let mut h = harness([1366.0, 900.0], 1.0);
    open(&mut h, "10");
    save(&mut h, "modificaciones").unwrap();
}

#[test]
#[ignore = "genera capturas PNG para revisión manual"]
fn render_reports_for_review() {
    let mut h = harness([1366.0, 1100.0], 1.0);
    h.get_by_label("Informes").click();
    h.run();
    open(&mut h, "1");
    for _ in 0..11 {
        press(&mut h, Key::PageUp);
    }
    save(&mut h, "caja-diaria").unwrap();

    let mut h = harness([1366.0, 1100.0], 1.0);
    h.get_by_label("Informes").click();
    h.run();
    open(&mut h, "3");
    save(&mut h, "acumulados").unwrap();

    let mut h = harness([1366.0, 900.0], 1.0);
    h.get_by_label("Informes").click();
    h.run();
    open(&mut h, "4");
    save(&mut h, "inventario-vigentes").unwrap();
}

#[test]
#[ignore = "genera capturas PNG para revisión manual"]
fn render_suggestions_for_review() {
    let mut h = harness([1366.0, 900.0], 1.0);
    open(&mut h, "3");
    h.get_by_role(egui::Role::TextInput).type_text("Ríos");
    h.run();
    press(&mut h, Key::ArrowDown);
    save(&mut h, "sugerencias-retiro").unwrap();

    let mut h = harness([1366.0, 900.0], 1.0);
    open(&mut h, "2");
    type_and_enter(&mut h, "26320");
    press(&mut h, Key::N);
    let mut h2 = harness([1366.0, 900.0], 1.0);
    open(&mut h2, "7");
    type_and_enter(&mut h2, "18");
    type_and_enter(&mut h2, "Arriendo");
    h2.get_by_role(egui::Role::TextInput).type_text("1200000");
    h2.run();
    save(&mut h2, "importe-miles").unwrap();
    let _ = h;
}
