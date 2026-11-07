pub mod icons;
pub mod symbiote;
pub mod text;

use text::TextRenderer;
use tiny_skia::{Color, Paint, PathBuilder, PixmapMut, Stroke, Transform};

pub struct Renderer {
    pub text: TextRenderer,
}

impl Renderer {
    pub fn new() -> Self {
        Self { text: TextRenderer::new() }
    }

    pub fn draw_capsule(
        &mut self,
        pixmap: &mut PixmapMut,
        center_x: f32,
        top_y: f32,
        width: f32,
        height: f32,
        color: Color,
        stroke_color: Color,
    ) {
        let radius = height / 2.0;
        let left = center_x - (width / 2.0);
        let right = center_x + (width / 2.0);
        let bottom = top_y + height;

        let mut pb = PathBuilder::new();
        pb.move_to(left + radius, top_y);
        pb.line_to(right - radius, top_y);
        pb.cubic_to(right, top_y, right, bottom, right - radius, bottom);
        pb.line_to(left + radius, bottom);
        pb.cubic_to(left, bottom, left, top_y, left + radius, top_y);
        pb.close();

        if let Some(path) = pb.finish() {
            let mut paint = Paint::default();
            paint.set_color(color);
            paint.anti_alias = true;
            pixmap.fill_path(&path, &paint, tiny_skia::FillRule::Winding, Transform::identity(), None);

            let mut stroke_paint = Paint::default();
            stroke_paint.set_color(stroke_color);
            stroke_paint.anti_alias = true;
            let stroke = Stroke { width: 1.5, ..Default::default() };
            pixmap.stroke_path(&path, &stroke_paint, &stroke, Transform::identity(), None);
        }
    }

    pub fn draw_rect_panel(
        &mut self,
        pixmap: &mut PixmapMut,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        bg_color: Color,
        border_color: Color,
        corner_radius: f32,
    ) {
        let mut pb = PathBuilder::new();
        let r = corner_radius;
        pb.move_to(x + r, y);
        pb.line_to(x + w - r, y);
        pb.cubic_to(x + w, y, x + w, y + r, x + w, y + r);
        pb.line_to(x + w, y + h - r);
        pb.cubic_to(x + w, y + h, x + w - r, y + h, x + w - r, y + h);
        pb.line_to(x + r, y + h);
        pb.cubic_to(x, y + h, x, y + h - r, x, y + h - r);
        pb.line_to(x, y + r);
        pb.cubic_to(x, y, x + r, y, x + r, y);
        pb.close();

        if let Some(path) = pb.finish() {
            let mut paint = Paint::default();
            paint.set_color(bg_color);
            paint.anti_alias = true;
            pixmap.fill_path(&path, &paint, tiny_skia::FillRule::Winding, Transform::identity(), None);

            let mut stroke_paint = Paint::default();
            stroke_paint.set_color(border_color);
            stroke_paint.anti_alias = true;
            let stroke = Stroke { width: 1.5, ..Default::default() };
            pixmap.stroke_path(&path, &stroke_paint, &stroke, Transform::identity(), None);
        }
    }

    pub fn draw_visualizer_bars(&self, pixmap: &mut PixmapMut, start_x: f32, center_y: f32, time_ticks: u64) {
        let heights = [14.0, 22.0, 10.0, 18.0];
        let mut paint = Paint::default();
        paint.anti_alias = true;

        for (i, base_h) in heights.iter().enumerate() {
            let osc = (((time_ticks as f32 * 0.15) + (i as f32 * 1.5)).sin().abs()) * 12.0;
            let h = base_h + osc;
            let bx = start_x + (i as f32 * 7.0);

            if i < 2 {
                paint.set_color(Color::from_rgba8(190, 242, 100, 255));
            } else {
                paint.set_color(Color::from_rgba8(34, 211, 238, 255));
            }

            let mut pb = PathBuilder::new();
            pb.move_to(bx, center_y + (h / 2.0));
            pb.line_to(bx, center_y - (h / 2.0));
            if let Some(path) = pb.finish() {
                let stroke = Stroke { width: 3.5, ..Default::default() };
                pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
            }
        }
    }
}