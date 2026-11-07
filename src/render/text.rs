use cosmic_text::{Attrs, Buffer, FontSystem, Metrics, Shaping, SwashCache};
use tiny_skia::{Color, PixmapMut};

pub struct TextRenderer {
    font_system: FontSystem,
    swash_cache: SwashCache,
}

impl TextRenderer {
    pub fn new() -> Self {
        Self {
            font_system: FontSystem::new(),
            swash_cache: SwashCache::new(),
        }
    }

    pub fn measure_width(&mut self, text: &str, font_size: f32) -> f32 {
        let mut buffer = Buffer::new(&mut self.font_system, Metrics::new(font_size, font_size * 1.2));
        buffer.set_text(&mut self.font_system, text, Attrs::new(), Shaping::Advanced);
        buffer.shape_until_scroll(&mut self.font_system, false);

        let mut max_w = 0.0f32;
        for run in buffer.layout_runs() {
            if run.line_w > max_w {
                max_w = run.line_w;
            }
        }
        max_w
    }

    pub fn draw_text(
        &mut self,
        pixmap: &mut PixmapMut,
        text: &str,
        x: f32,
        y: f32,
        font_size: f32,
        color: Color,
    ) {
        let mut buffer = Buffer::new(&mut self.font_system, Metrics::new(font_size, font_size * 1.2));
        buffer.set_text(&mut self.font_system, text, Attrs::new(), Shaping::Advanced);
        buffer.shape_until_scroll(&mut self.font_system, false);

        let pixmap_w = pixmap.width() as i32;
        let pixmap_h = pixmap.height() as i32;
        let data = pixmap.data_mut();

        for run in buffer.layout_runs() {
            for glyph in run.glyphs.iter() {
                let physical_glyph = glyph.physical((x, y + run.line_y), 1.0);
                if let Some(image) = self.swash_cache.get_image_uncached(&mut self.font_system, physical_glyph.cache_key) {
                    let gw = image.placement.width as i32;
                    let gh = image.placement.height as i32;
                    let gx = physical_glyph.x + image.placement.left;
                    let gy = physical_glyph.y - image.placement.top;

                    for row in 0..gh {
                        for col in 0..gw {
                            let px = gx + col;
                            let py = gy + row;
                            if px >= 0 && px < pixmap_w && py >= 0 && py < pixmap_h {
                                let alpha_val = image.data[(row * gw + col) as usize];
                                if alpha_val > 0 {
                                    let idx = ((py * pixmap_w + px) * 4) as usize;
                                    let text_alpha = (color.alpha() * (alpha_val as f32 / 255.0)) * 255.0;
                                    let a_src = text_alpha as u32;
                                    let a_dst = data[idx + 3] as u32;

                                    let r_src = (color.red() * 255.0) as u32;
                                    let g_src = (color.green() * 255.0) as u32;
                                    let b_src = (color.blue() * 255.0) as u32;

                                    let out_a = a_src + a_dst * (255 - a_src) / 255;
                                    if out_a > 0 {
                                        data[idx] = ((r_src * a_src + data[idx] as u32 * (255 - a_src)) / 255).min(255) as u8;
                                        data[idx + 1] = ((g_src * a_src + data[idx + 1] as u32 * (255 - a_src)) / 255).min(255) as u8;
                                        data[idx + 2] = ((b_src * a_src + data[idx + 2] as u32 * (255 - a_src)) / 255).min(255) as u8;
                                        data[idx + 3] = out_a.min(255) as u8;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}