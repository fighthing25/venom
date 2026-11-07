use tiny_skia::{Color, Paint, PathBuilder, PixmapMut, Stroke, Transform};

pub fn draw_symbiote_eyes(
    pixmap: &mut PixmapMut,
    center_x: f32,
    center_y: f32,
    blink_factor: f32,
    look_x: f32,
    look_y: f32,
    is_headbang: bool,
) {
    let mut paint_white = Paint::default();
    paint_white.set_color(Color::from_rgba8(255, 255, 255, 255));
    paint_white.anti_alias = true;

    let mut paint_cyan = Paint::default();
    paint_cyan.set_color(Color::from_rgba8(34, 211, 238, 255));
    paint_cyan.anti_alias = true;

    let stroke_cyan = Stroke { width: 1.5, ..Default::default() };

    let headbang_offset = if is_headbang { 3.0 } else { 0.0 };
    let y_base = center_y + headbang_offset + (look_y * 2.0);
    let x_offset = look_x * 4.0;

    // LEFT EYE
    let mut pb_left = PathBuilder::new();
    let l_start_x = center_x - 38.0 + x_offset;
    let l_start_y = y_base + (2.0 * blink_factor);
    pb_left.move_to(l_start_x, l_start_y);
    pb_left.cubic_to(
        center_x - 28.0 + x_offset, y_base - (14.0 * blink_factor),
        center_x - 12.0 + x_offset, y_base - (10.0 * blink_factor),
        center_x - 6.0 + x_offset,  y_base - (2.0 * blink_factor),
    );
    pb_left.cubic_to(
        center_x - 18.0 + x_offset, y_base + (8.0 * blink_factor),
        center_x - 30.0 + x_offset, y_base + (6.0 * blink_factor),
        l_start_x, l_start_y,
    );
    pb_left.close();

    if let Some(path) = pb_left.finish() {
        pixmap.fill_path(&path, &paint_white, tiny_skia::FillRule::Winding, Transform::identity(), None);
        let mut pb_wire = PathBuilder::new();
        pb_wire.move_to(center_x - 32.0 + x_offset, y_base + (3.0 * blink_factor));
        pb_wire.line_to(center_x - 16.0 + x_offset, y_base + (4.0 * blink_factor));
        pb_wire.line_to(center_x - 10.0 + x_offset, y_base);
        if let Some(wire_path) = pb_wire.finish() {
            pixmap.stroke_path(&wire_path, &paint_cyan, &stroke_cyan, Transform::identity(), None);
        }
    }

    // RIGHT EYE
    let mut pb_right = PathBuilder::new();
    let r_start_x = center_x + 38.0 + x_offset;
    let r_start_y = y_base + (2.0 * blink_factor);
    pb_right.move_to(r_start_x, r_start_y);
    pb_right.cubic_to(
        center_x + 28.0 + x_offset, y_base - (14.0 * blink_factor),
        center_x + 12.0 + x_offset, y_base - (10.0 * blink_factor),
        center_x + 6.0 + x_offset,  y_base - (2.0 * blink_factor),
    );
    pb_right.cubic_to(
        center_x + 18.0 + x_offset, y_base + (8.0 * blink_factor),
        center_x + 30.0 + x_offset, y_base + (6.0 * blink_factor),
        r_start_x, r_start_y,
    );
    pb_right.close();

    if let Some(path) = pb_right.finish() {
        pixmap.fill_path(&path, &paint_white, tiny_skia::FillRule::Winding, Transform::identity(), None);
        let mut pb_wire = PathBuilder::new();
        pb_wire.move_to(center_x + 32.0 + x_offset, y_base + (3.0 * blink_factor));
        pb_wire.line_to(center_x + 16.0 + x_offset, y_base + (4.0 * blink_factor));
        pb_wire.line_to(center_x + 10.0 + x_offset, y_base);
        if let Some(wire_path) = pb_wire.finish() {
            pixmap.stroke_path(&wire_path, &paint_cyan, &stroke_cyan, Transform::identity(), None);
        }
    }
}

pub fn draw_hover_tendrils(
    pixmap: &mut PixmapMut,
    center_x: f32,
    top_y: f32,
    progress: f32,
    capsule_width: f32,
) {
    if progress <= 0.01 { return; }

    let mut paint_black = Paint::default();
    paint_black.set_color(Color::from_rgba8(9, 9, 11, 255));
    paint_black.anti_alias = true;

    let half_w = (capsule_width / 2.0) + (progress * 110.0);
    let tendril_depth = progress * 68.0;

    let mut pb = PathBuilder::new();
    pb.move_to(center_x - half_w, top_y + 12.0);
    pb.cubic_to(
        center_x - half_w - 40.0 * progress, top_y + 25.0,
        center_x - half_w - 70.0 * progress, top_y + tendril_depth + 30.0,
        center_x - half_w - 20.0 * progress, top_y + tendril_depth + 42.0,
    );
    pb.cubic_to(
        center_x - half_w - 5.0, top_y + tendril_depth + 20.0,
        center_x - half_w + 30.0, top_y + tendril_depth + 10.0,
        center_x - half_w + 60.0, top_y + 40.0,
    );
    pb.cubic_to(
        center_x - 120.0, top_y + 55.0 + (tendril_depth * 0.4),
        center_x - 40.0,  top_y + 45.0 + (tendril_depth * 0.5),
        center_x,         top_y + 50.0 + (tendril_depth * 0.4),
    );
    pb.cubic_to(
        center_x + 40.0,  top_y + 45.0 + (tendril_depth * 0.5),
        center_x + 120.0, top_y + 55.0 + (tendril_depth * 0.4),
        center_x + half_w - 60.0, top_y + 40.0,
    );
    pb.cubic_to(
        center_x + half_w - 30.0, top_y + tendril_depth + 10.0,
        center_x + half_w + 5.0,  top_y + tendril_depth + 20.0,
        center_x + half_w + 20.0 * progress, top_y + tendril_depth + 42.0,
    );
    pb.cubic_to(
        center_x + half_w + 70.0 * progress, top_y + tendril_depth + 30.0,
        center_x + half_w + 40.0 * progress, top_y + 25.0,
        center_x + half_w, top_y + 12.0,
    );
    pb.close();

    if let Some(path) = pb.finish() {
        pixmap.fill_path(&path, &paint_black, tiny_skia::FillRule::Winding, Transform::identity(), None);
    }
}

pub fn draw_expansion_wings(
    pixmap: &mut PixmapMut,
    center_x: f32,
    top_y: f32,
    panel_w: f32,
    progress: f32,
) {
    if progress <= 0.01 { return; }

    let mut paint_black = Paint::default();
    paint_black.set_color(Color::from_rgba8(9, 9, 11, 255));
    paint_black.anti_alias = true;

    let wing_span = (panel_w / 2.0) + (progress * 130.0);

    let mut pb_left = PathBuilder::new();
    pb_left.move_to(center_x - (panel_w / 2.0), top_y + 20.0);
    pb_left.cubic_to(
        center_x - wing_span * 0.7, top_y - 10.0 * progress,
        center_x - wing_span, top_y + 40.0 * progress,
        center_x - wing_span - 30.0, top_y + 80.0 * progress,
    );
    pb_left.cubic_to(
        center_x - wing_span + 10.0, top_y + 100.0,
        center_x - (panel_w / 2.0) - 40.0, top_y + 160.0,
        center_x - (panel_w / 2.0), top_y + 180.0,
    );
    pb_left.close();

    if let Some(path) = pb_left.finish() {
        pixmap.fill_path(&path, &paint_black, tiny_skia::FillRule::Winding, Transform::identity(), None);
    }

    let mut pb_right = PathBuilder::new();
    pb_right.move_to(center_x + (panel_w / 2.0), top_y + 20.0);
    pb_right.cubic_to(
        center_x + wing_span * 0.7, top_y - 10.0 * progress,
        center_x + wing_span, top_y + 40.0 * progress,
        center_x + wing_span + 30.0, top_y + 80.0 * progress,
    );
    pb_right.cubic_to(
        center_x + wing_span - 10.0, top_y + 100.0,
        center_x + (panel_w / 2.0) + 40.0, top_y + 160.0,
        center_x + (panel_w / 2.0), top_y + 180.0,
    );
    pb_right.close();

    if let Some(path) = pb_right.finish() {
        pixmap.fill_path(&path, &paint_black, tiny_skia::FillRule::Winding, Transform::identity(), None);
    }
}