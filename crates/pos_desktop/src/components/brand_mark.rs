//! Logotipo: un diamante dibujado con vectores (se ve nítido a cualquier zoom).

use egui::{Color32, Pos2, Rect, Sense, Shape, Stroke, Ui, pos2, vec2};

/// Dibuja el diamante en un cuadrado del lado indicado.
pub fn brand_mark(ui: &mut Ui, side: f32, fill: Color32, facets: Color32) {
    let (rect, _) = ui.allocate_exact_size(vec2(side, side), Sense::hover());
    if !ui.is_rect_visible(rect) {
        return;
    }
    let at = |x: f32, y: f32| point(rect, x, y);
    let outline = vec![
        at(0.22, 0.14),
        at(0.78, 0.14),
        at(1.0, 0.40),
        at(0.5, 0.94),
        at(0.0, 0.40),
    ];
    let painter = ui.painter();
    painter.add(Shape::convex_polygon(outline, fill, Stroke::NONE));

    let stroke = Stroke::new((side / 32.0).max(1.0), facets);
    let lines = [
        // Cintura.
        (at(0.0, 0.40), at(1.0, 0.40)),
        // Corona.
        (at(0.22, 0.14), at(0.33, 0.40)),
        (at(0.5, 0.14), at(0.33, 0.40)),
        (at(0.5, 0.14), at(0.67, 0.40)),
        (at(0.78, 0.14), at(0.67, 0.40)),
        // Pabellón.
        (at(0.33, 0.40), at(0.5, 0.94)),
        (at(0.67, 0.40), at(0.5, 0.94)),
    ];
    for (from, to) in lines {
        painter.line_segment([from, to], stroke);
    }
}

fn point(rect: Rect, x: f32, y: f32) -> Pos2 {
    pos2(
        rect.left() + x * rect.width(),
        rect.top() + y * rect.height(),
    )
}
