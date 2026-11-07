use tiny_skia::{Color, Paint, PathBuilder, PixmapMut, Stroke, Transform};

pub enum IconType {
    Mute,
    MusicLrc,
    Screenshot,
    Shelf,
    Calendar,
    SambungHp,
    Addons,
}

pub fn draw_icon(
    pixmap: &mut PixmapMut,
    icon: IconType,
    cx: f32,
    cy: f32,
    color: Color,
    stroke_w: f32,
) {
    let mut paint = Paint::default();
    paint.set_color(color);
    paint.anti_alias = true;

    let stroke = Stroke { width: stroke_w, ..Default::default() };

    match icon {
        IconType::Mute => {
            let mut pb = PathBuilder::new();
            pb.move_to(cx - 3.0, cy - 6.0);
            pb.line_to(cx + 3.0, cy - 6.0);
            pb.line_to(cx + 3.0, cy + 1.0);
            pb.line_to(cx - 3.0, cy + 1.0);
            pb.close();

            pb.move_to(cx - 6.0, cy);
            pb.cubic_to(cx - 6.0, cy + 6.0, cx + 6.0, cy + 6.0, cx + 6.0, cy);
            pb.move_to(cx, cy + 6.0);
            pb.line_to(cx, cy + 10.0);
            pb.move_to(cx - 4.0, cy + 10.0);
            pb.line_to(cx + 4.0, cy + 10.0);

            pb.move_to(cx - 7.0, cy - 7.0);
            pb.line_to(cx + 7.0, cy + 7.0);

            if let Some(path) = pb.finish() {
                pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
            }
        }
        IconType::MusicLrc => {
            let mut pb = PathBuilder::new();
            pb.move_to(cx + 4.0, cy - 7.0);
            pb.line_to(cx - 2.0, cy - 4.0);
            pb.line_to(cx - 2.0, cy + 4.0);
            pb.move_to(cx + 4.0, cy - 7.0);
            pb.line_to(cx + 4.0, cy + 2.0);

            if let Some(path) = pb.finish() {
                pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
            }
            let mut pb_heads = PathBuilder::new();
            pb_heads.push_circle(cx - 4.0, cy + 4.0, 3.0);
            pb_heads.push_circle(cx + 2.0, cy + 2.0, 3.0);
            if let Some(path) = pb_heads.finish() {
                pixmap.fill_path(&path, &paint, tiny_skia::FillRule::Winding, Transform::identity(), None);
            }
        }
        IconType::Screenshot => {
            let mut pb = PathBuilder::new();
            let d = 7.0;
            let c = 3.0;
            pb.move_to(cx - d, cy - d + c);
            pb.line_to(cx - d, cy - d);
            pb.line_to(cx - d + c, cy - d);

            pb.move_to(cx + d - c, cy - d);
            pb.line_to(cx + d, cy - d);
            pb.line_to(cx + d, cy - d + c);

            pb.move_to(cx + d, cy + d - c);
            pb.line_to(cx + d, cy + d);
            pb.line_to(cx + d - c, cy + d);

            pb.move_to(cx - d + c, cy + d);
            pb.line_to(cx - d, cy + d);
            pb.line_to(cx - d, cy + d - c);

            pb.move_to(cx - 1.5, cy);
            pb.line_to(cx + 1.5, cy);

            if let Some(path) = pb.finish() {
                pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
            }
        }
        IconType::Shelf => {
            let mut pb = PathBuilder::new();
            pb.move_to(cx - 8.0, cy - 5.0);
            pb.line_to(cx - 3.0, cy - 5.0);
            pb.line_to(cx - 1.0, cy - 3.0);
            pb.line_to(cx + 8.0, cy - 3.0);
            pb.line_to(cx + 8.0, cy + 6.0);
            pb.line_to(cx - 8.0, cy + 6.0);
            pb.close();
            if let Some(path) = pb.finish() {
                pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
            }
        }
        IconType::Calendar => {
            let mut pb = PathBuilder::new();
            pb.move_to(cx - 7.0, cy - 5.0);
            pb.line_to(cx + 7.0, cy - 5.0);
            pb.line_to(cx + 7.0, cy + 7.0);
            pb.line_to(cx - 7.0, cy + 7.0);
            pb.close();
            pb.move_to(cx - 7.0, cy - 1.0);
            pb.line_to(cx + 7.0, cy - 1.0);
            pb.move_to(cx - 4.0, cy - 8.0);
            pb.line_to(cx - 4.0, cy - 5.0);
            pb.move_to(cx + 4.0, cy - 8.0);
            pb.line_to(cx + 4.0, cy - 5.0);

            if let Some(path) = pb.finish() {
                pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
            }
        }
        IconType::SambungHp => {
            let mut pb = PathBuilder::new();
            pb.move_to(cx - 5.0, cy - 8.0);
            pb.line_to(cx + 5.0, cy - 8.0);
            pb.line_to(cx + 5.0, cy + 8.0);
            pb.line_to(cx - 5.0, cy + 8.0);
            pb.close();
            pb.move_to(cx - 2.0, cy - 6.0);
            pb.line_to(cx + 2.0, cy - 6.0);
            pb.move_to(cx - 1.0, cy + 6.0);
            pb.line_to(cx + 1.0, cy + 6.0);

            if let Some(path) = pb.finish() {
                pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
            }
        }
        IconType::Addons => {
            let mut pb = PathBuilder::new();
            pb.move_to(cx, cy - 7.0);
            pb.line_to(cx, cy + 7.0);
            pb.move_to(cx - 7.0, cy);
            pb.line_to(cx + 7.0, cy);

            if let Some(path) = pb.finish() {
                pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
            }
        }
    }
}