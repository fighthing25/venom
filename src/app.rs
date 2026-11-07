use crate::addons::{get_prayer_times, Teleprompter, TrafficMonitor};
use crate::audio::AudioManager;
use crate::calendar::{get_hijri_date, get_pasaran_jawa, TodoManager, HOLIDAYS_2025};
use crate::clipboard::ClipboardHistory;
use crate::config::AppConfig;
use crate::lyrics::LyricState;
use crate::render::icons::{draw_icon, IconType};
use crate::render::symbiote::{draw_expansion_wings, draw_hover_tendrils, draw_symbiote_eyes};
use crate::render::Renderer;
use crate::sambung::SambungHpServer;
use crate::shelf::DropZoneShelf;
use std::sync::{Arc, Mutex};
use tiny_skia::{Color, PathBuilder, PixmapMut, Stroke, Transform};

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ActivePanel {
    None,
    Audio,
    MusicLyrics,
    Screenshot,
    Shelf,
    Calendar,
    SambungHp,
    Addons,
}

pub struct Spring {
    pub current: f32,
    pub target: f32,
    pub velocity: f32,
    pub stiffness: f32,
    pub damping: f32,
}

impl Spring {
    pub fn new(initial: f32) -> Self {
        Self {
            current: initial,
            target: initial,
            velocity: 0.0,
            stiffness: 240.0,
            damping: 24.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        let force = -self.stiffness * (self.current - self.target);
        let damping_force = -self.damping * self.velocity;
        let accel = force + damping_force;
        self.velocity += accel * dt;
        self.current += self.velocity * dt;
    }
}

pub struct VenomIslandApp {
    pub config: AppConfig,
    pub renderer: Renderer,
    pub width_spring: Spring,
    pub height_spring: Spring,
    pub hover_spring: Spring,
    pub expand_spring: Spring,
    pub blink_timer: f32,
    pub blink_factor: f32,
    pub cursor_x: f32,
    pub cursor_y: f32,
    pub is_hovered: bool,
    pub hovered_icon_idx: Option<usize>,
    pub active_panel: ActivePanel,
    pub lyrics: Arc<Mutex<LyricState>>,
    pub audio: AudioManager,
    pub shelf: DropZoneShelf,
    pub sambung: SambungHpServer,
    pub clipboard: ClipboardHistory,
    pub traffic: TrafficMonitor,
    pub teleprompter: Teleprompter,
    pub todos: TodoManager,
    pub tick_counter: u64,
}

impl VenomIslandApp {
    pub fn new() -> Self {
        let config = AppConfig::load();
        Self {
            config,
            renderer: Renderer::new(),
            width_spring: Spring::new(320.0),
            height_spring: Spring::new(48.0),
            hover_spring: Spring::new(0.0),
            expand_spring: Spring::new(0.0),
            blink_timer: 0.0,
            blink_factor: 1.0,
            cursor_x: 0.0,
            cursor_y: 0.0,
            is_hovered: false,
            hovered_icon_idx: None,
            active_panel: ActivePanel::None,
            lyrics: Arc::new(Mutex::new(LyricState::new())),
            audio: AudioManager::new(),
            shelf: DropZoneShelf::new(),
            sambung: SambungHpServer::new(),
            clipboard: ClipboardHistory::new(),
            traffic: TrafficMonitor::new(),
            teleprompter: Teleprompter::new(),
            todos: TodoManager::new(),
            tick_counter: 0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.tick_counter += 1;

        if let Ok(mut lrc) = self.lyrics.lock() {
            lrc.tick((dt * 1000.0) as u64);
        }

        if self.tick_counter % 60 == 0 {
            self.traffic.tick();
            self.clipboard.purge_expired_otps();
        }
        self.teleprompter.tick();

        self.blink_timer += dt;
        if self.blink_timer > 3.5 {
            self.blink_factor = (1.0 - ((self.blink_timer - 3.5) / 0.15)).abs().clamp(0.05, 1.0);
            if self.blink_timer > 3.8 {
                self.blink_timer = 0.0;
                self.blink_factor = 1.0;
            }
        }

        if self.active_panel != ActivePanel::None {
            self.expand_spring.target = 1.0;
            self.hover_spring.target = 1.0;
            self.width_spring.target = 820.0;
            self.height_spring.target = 390.0;
        } else if self.is_hovered {
            self.expand_spring.target = 0.0;
            self.hover_spring.target = 1.0;
            self.width_spring.target = 640.0;
            self.height_spring.target = 140.0;
        } else {
            self.expand_spring.target = 0.0;
            self.hover_spring.target = 0.0;

            if let Ok(lrc) = self.lyrics.lock() {
                if lrc.is_playing {
                    let text_w = self.renderer.text.measure_width(lrc.current_text(), 14.0);
                    self.width_spring.target = (text_w + 140.0).clamp(380.0, 620.0);
                } else {
                    self.width_spring.target = 320.0;
                }
            } else {
                self.width_spring.target = 320.0;
            }
            self.height_spring.target = 48.0;
        }

        self.width_spring.update(dt);
        self.height_spring.update(dt);
        self.hover_spring.update(dt);
        self.expand_spring.update(dt);
    }

    pub fn render(&mut self, pixmap: &mut PixmapMut, screen_w: f32) {
        pixmap.fill(Color::TRANSPARENT);

        let cx = screen_w / 2.0;
        let top_y = 12.0;
        let cur_w = self.width_spring.current;
        let cur_h = self.height_spring.current;
        let hover_prog = self.hover_spring.current;
        let expand_prog = self.expand_spring.current;

        if expand_prog > 0.05 {
            draw_expansion_wings(pixmap, cx, top_y, cur_w, expand_prog);
        } else if hover_prog > 0.05 {
            draw_hover_tendrils(pixmap, cx, top_y, hover_prog, cur_w);
        }

        let symbiote_ink = Color::from_rgba8(9, 9, 11, 255);
        let border_ink = Color::from_rgba8(24, 24, 27, 255);
        self.renderer.draw_capsule(pixmap, cx, top_y, cur_w, cur_h.min(54.0), symbiote_ink, border_ink);

        if expand_prog > 0.2 {
            self.render_expanded_panel(pixmap, cx, top_y, cur_w, cur_h, expand_prog);
        } else if hover_prog > 0.2 {
            self.render_hover_tendril_icons(pixmap, cx, top_y, cur_w, hover_prog);
        } else {
            let has_music = self.lyrics.lock().map(|l| l.is_playing).unwrap_or(false);
            if has_music {
                self.render_lyrics_pill(pixmap, cx, top_y);
            } else {
                let look_norm_x = ((self.cursor_x - cx) / 300.0).clamp(-1.0, 1.0);
                let look_norm_y = ((self.cursor_y - (top_y + 24.0)) / 200.0).clamp(-1.0, 1.0);
                draw_symbiote_eyes(
                    pixmap, cx, top_y + 25.0, self.blink_factor, look_norm_x, look_norm_y, false,
                );
            }
        }
    }

    fn render_lyrics_pill(&mut self, pixmap: &mut PixmapMut, cx: f32, top_y: f32) {
        let half_w = self.width_spring.current / 2.0;
        let pill_left = cx - half_w;
        let pill_right = cx + half_w;
        let center_y = top_y + 24.0;

        self.renderer.draw_visualizer_bars(pixmap, pill_left + 18.0, center_y, self.tick_counter);

        let lyric_text = if let Ok(lrc) = self.lyrics.lock() {
            lrc.current_text().to_string()
        } else {
            "Venom - Eminem".to_string()
        };

        let white = Color::from_rgba8(255, 255, 255, 255);
        let text_w = self.renderer.text.measure_width(&lyric_text, 13.0);
        let lyric_x = cx - (text_w / 2.0);
        self.renderer.text.draw_text(pixmap, &lyric_text, lyric_x, center_y - 7.0, 13.0, white);

        let mut album_paint = tiny_skia::Paint::default();
        album_paint.set_color(Color::from_rgba8(34, 211, 238, 255));
        album_paint.anti_alias = true;
        let mut pb = PathBuilder::new();
        pb.push_circle(pill_right - 24.0, center_y, 11.0);
        if let Some(path) = pb.finish() {
            pixmap.fill_path(&path, &album_paint, tiny_skia::FillRule::Winding, Transform::identity(), None);
        }
    }

    fn render_hover_tendril_icons(&mut self, pixmap: &mut PixmapMut, cx: f32, top_y: f32, width: f32, prog: f32) {
        let icon_types = [
            (IconType::Mute, "Global Audio & Mic Controller", ActivePanel::Audio),
            (IconType::MusicLrc, "Dynamic LRCLIB Music Pill", ActivePanel::MusicLyrics),
            (IconType::Screenshot, "DXGI Screenshot & Color Picker", ActivePanel::Screenshot),
            (IconType::Shelf, "Drop-Zone File Shelf", ActivePanel::Shelf),
            (IconType::Calendar, "Indonesian Calendar & To-Do", ActivePanel::Calendar),
            (IconType::SambungHp, "Sambung HP (Local Wi-Fi QR)", ActivePanel::SambungHp),
            (IconType::Addons, "Modular Add-Ons Hub", ActivePanel::Addons),
        ];

        let spacing = (width - 80.0) / (icon_types.len() - 1) as f32;
        let start_x = cx - (width / 2.0) + 40.0;
        let row_y = top_y + 36.0 + (prog * 34.0);
        let mut hovered_text = None;

        for (i, (icon, label, _panel)) in icon_types.iter().enumerate() {
            let node_x = start_x + (i as f32 * spacing);
            let dist_sq = (self.cursor_x - node_x).powi(2) + (self.cursor_y - row_y).powi(2);
            let is_this_hovered = dist_sq < 22.0 * 22.0;

            if is_this_hovered {
                self.hovered_icon_idx = Some(i);
                hovered_text = Some(*label);
            }

            let node_color = if is_this_hovered {
                Color::from_rgba8(190, 242, 100, 255)
            } else {
                Color::from_rgba8(24, 24, 27, 255)
            };

            let stroke_color = if is_this_hovered {
                Color::from_rgba8(9, 9, 11, 255)
            } else {
                Color::from_rgba8(255, 255, 255, 255)
            };

            let mut pb = PathBuilder::new();
            let radius = if is_this_hovered { 19.0 } else { 16.0 };
            pb.push_circle(node_x, row_y, radius);
            if let Some(path) = pb.finish() {
                let mut paint = tiny_skia::Paint::default();
                paint.set_color(node_color);
                paint.anti_alias = true;
                pixmap.fill_path(&path, &paint, tiny_skia::FillRule::Winding, Transform::identity(), None);

                let mut stroke_paint = tiny_skia::Paint::default();
                stroke_paint.set_color(stroke_color);
                stroke_paint.anti_alias = true;
                let stroke = Stroke { width: 1.5, ..Default::default() };
                pixmap.stroke_path(&path, &stroke_paint, &stroke, Transform::identity(), None);
            }

            let icon_stroke = if is_this_hovered {
                Color::from_rgba8(9, 9, 11, 255)
            } else {
                Color::from_rgba8(255, 255, 255, 255)
            };

            draw_icon(pixmap, match icon {
                IconType::Mute => IconType::Mute,
                IconType::MusicLrc => IconType::MusicLrc,
                IconType::Screenshot => IconType::Screenshot,
                IconType::Shelf => IconType::Shelf,
                IconType::Calendar => IconType::Calendar,
                IconType::SambungHp => IconType::SambungHp,
                IconType::Addons => IconType::Addons,
            }, node_x, row_y, icon_stroke, 1.8);
        }

        if let Some(tip) = hovered_text {
            let tip_w = self.renderer.text.measure_width(tip, 12.0) + 24.0;
            let tip_h = 28.0;
            let tip_x = cx - (tip_w / 2.0);
            let tip_y = row_y + 32.0;

            self.renderer.draw_rect_panel(
                pixmap, tip_x, tip_y, tip_w, tip_h,
                Color::from_rgba8(9, 9, 11, 255), Color::from_rgba8(190, 242, 100, 255), 4.0,
            );
            self.renderer.text.draw_text(pixmap, tip, tip_x + 12.0, tip_y + 6.0, 12.0, Color::from_rgba8(255, 255, 255, 255));
        }
    }

    fn render_expanded_panel(&mut self, pixmap: &mut PixmapMut, cx: f32, top_y: f32, w: f32, h: f32, _prog: f32) {
        let panel_x = cx - (w / 2.0);
        let panel_y = top_y + 4.0;

        let white_bg = Color::from_rgba8(255, 255, 255, 255);
        let ink_border = Color::from_rgba8(9, 9, 11, 255);
        self.renderer.draw_rect_panel(pixmap, panel_x, panel_y, w, h, white_bg, ink_border, 12.0);

        let header_h = 42.0;
        self.renderer.draw_rect_panel(pixmap, panel_x, panel_y, w, header_h, ink_border, ink_border, 12.0);

        let title = match self.active_panel {
            ActivePanel::Audio => "GLOBAL AUDIO & MIC CONTROLLER",
            ActivePanel::MusicLyrics => "DYNAMIC LRCLIB MUSIC & SYNCED LYRICS",
            ActivePanel::Screenshot => "DXGI SCREENSHOT & COLOR PICKER",
            ActivePanel::Shelf => "DROP-ZONE FILE SHELF & EXPORT",
            ActivePanel::Calendar => "INDONESIAN CALENDAR & TO-DO LIST",
            ActivePanel::SambungHp => "SAMBUNG HP // WI-FI DIRECT CONNECTOR",
            ActivePanel::Addons => "MODULAR ADD-ONS & SYSTEM HUB",
            ActivePanel::None => "VENOMISLAND",
        };
        self.renderer.text.draw_text(pixmap, title, panel_x + 20.0, panel_y + 12.0, 14.0, Color::from_rgba8(255, 255, 255, 255));

        let close_btn_x = panel_x + w - 32.0;
        let close_btn_y = panel_y + 12.0;
        self.renderer.text.draw_text(pixmap, "[X]", close_btn_x, close_btn_y, 14.0, Color::from_rgba8(190, 242, 100, 255));

        match self.active_panel {
            ActivePanel::Audio => self.render_audio_panel_body(pixmap, panel_x, panel_y + header_h + 12.0, w),
            ActivePanel::Screenshot => self.render_screenshot_panel_body(pixmap, panel_x, panel_y + header_h + 12.0, w),
            ActivePanel::Shelf => self.render_shelf_panel_body(pixmap, panel_x, panel_y + header_h + 12.0, w),
            ActivePanel::Calendar => self.render_calendar_panel_body(pixmap, panel_x, panel_y + header_h + 12.0, w),
            ActivePanel::SambungHp => self.render_sambung_panel_body(pixmap, panel_x, panel_y + header_h + 12.0, w),
            ActivePanel::Addons => self.render_addons_panel_body(pixmap, panel_x, panel_y + header_h + 12.0, w),
            _ => {}
        }
    }

    fn render_audio_panel_body(&mut self, pixmap: &mut PixmapMut, px: f32, py: f32, _w: f32) {
        let is_muted = AudioManager::is_mic_muted();
        let vol = (AudioManager::get_master_volume() * 100.0) as u32;

        let mic_bg = if is_muted {
            Color::from_rgba8(239, 68, 68, 255)
        } else {
            Color::from_rgba8(190, 242, 100, 255)
        };
        self.renderer.draw_rect_panel(pixmap, px + 24.0, py + 12.0, 360.0, 110.0, mic_bg, Color::from_rgba8(9, 9, 11, 255), 8.0);
        let mic_txt = if is_muted { "MIC: MUTED (GLOBAL)" } else { "MIC: ACTIVE (ON AIR)" };
        self.renderer.text.draw_text(pixmap, mic_txt, px + 44.0, py + 54.0, 18.0, Color::from_rgba8(9, 9, 11, 255));

        self.renderer.draw_rect_panel(pixmap, px + 400.0, py + 12.0, 360.0, 110.0, Color::from_rgba8(244, 244, 245, 255), Color::from_rgba8(9, 9, 11, 255), 8.0);
        let vol_txt = format!("AUDIO: {}% (ACTIVE)", vol);
        self.renderer.text.draw_text(pixmap, &vol_txt, px + 420.0, py + 54.0, 18.0, Color::from_rgba8(9, 9, 11, 255));

        self.renderer.draw_rect_panel(pixmap, px + 24.0, py + 140.0, 736.0, 52.0, Color::from_rgba8(255, 255, 255, 255), Color::from_rgba8(9, 9, 11, 255), 8.0);
        self.renderer.text.draw_text(pixmap, "DEVICE:  [SONY WH-1000XM5]   REALTEK HIGH DEFINITION SPEAKERS", px + 40.0, py + 158.0, 13.0, Color::from_rgba8(9, 9, 11, 255));
    }

    fn render_screenshot_panel_body(&mut self, pixmap: &mut PixmapMut, px: f32, py: f32, _w: f32) {
        let buttons = [("AREA CAPTURE", false), ("FULL SCREEN", false), ("HEX PICKER (#BEF264)", true)];
        for (i, (lbl, active)) in buttons.iter().enumerate() {
            let bx = px + 24.0 + (i as f32 * 250.0);
            let bg = if *active { Color::from_rgba8(190, 242, 100, 255) } else { Color::from_rgba8(255, 255, 255, 255) };
            self.renderer.draw_rect_panel(pixmap, bx, py + 20.0, 230.0, 130.0, bg, Color::from_rgba8(9, 9, 11, 255), 8.0);
            self.renderer.text.draw_text(pixmap, lbl, bx + 24.0, py + 75.0, 14.0, Color::from_rgba8(9, 9, 11, 255));
        }
    }

    fn render_shelf_panel_body(&mut self, pixmap: &mut PixmapMut, px: f32, py: f32, _w: f32) {
        self.renderer.draw_rect_panel(pixmap, px + 24.0, py + 10.0, 480.0, 180.0, Color::from_rgba8(255, 255, 255, 255), Color::from_rgba8(9, 9, 11, 255), 8.0);
        for (i, f) in self.shelf.files.iter().enumerate() {
            let item_y = py + 24.0 + (i as f32 * 46.0);
            let row_str = format!("{} ({})", f.name, f.size_str);
            self.renderer.text.draw_text(pixmap, &row_str, px + 44.0, item_y, 14.0, Color::from_rgba8(9, 9, 11, 255));
        }
        self.renderer.draw_rect_panel(pixmap, px + 24.0, py + 204.0, 480.0, 44.0, Color::from_rgba8(190, 242, 100, 255), Color::from_rgba8(9, 9, 11, 255), 8.0);
        self.renderer.text.draw_text(pixmap, "DRAG OUT / ZIP FILES", px + 160.0, py + 218.0, 14.0, Color::from_rgba8(9, 9, 11, 255));
    }

    fn render_sambung_panel_body(&mut self, pixmap: &mut PixmapMut, px: f32, py: f32, _w: f32) {
        self.renderer.draw_rect_panel(pixmap, px + 520.0, py + 10.0, 240.0, 240.0, Color::from_rgba8(255, 255, 255, 255), Color::from_rgba8(9, 9, 11, 255), 8.0);
        if let Ok(matrix) = self.sambung.qr_matrix.lock() {
            let cell_size = 7.0f32;
            let start_x = px + 545.0;
            let start_y = py + 25.0;
            let mut qr_paint = tiny_skia::Paint::default();
            qr_paint.set_color(Color::from_rgba8(9, 9, 11, 255));

            for (r, row) in matrix.iter().enumerate() {
                for (c, &is_dark) in row.iter().enumerate() {
                    if is_dark {
                        if let Some(rc) = tiny_skia::Rect::from_xywh(start_x + (c as f32 * cell_size), start_y + (r as f32 * cell_size), cell_size, cell_size) {
                            pixmap.fill_rect(rc, &qr_paint, Transform::identity(), None);
                        }
                    }
                }
            }
        }
        self.renderer.draw_rect_panel(pixmap, px + 40.0, py + 40.0, 440.0, 140.0, Color::from_rgba8(244, 244, 245, 255), Color::from_rgba8(9, 9, 11, 255), 8.0);
        let pin_str = format!("PIN KEAMANAN: {}", self.sambung.pin);
        self.renderer.text.draw_text(pixmap, &pin_str, px + 64.0, py + 75.0, 18.0, Color::from_rgba8(9, 9, 11, 255));
        let ip_str = format!("Akses: http://{}:7878", self.sambung.local_ip);
        self.renderer.text.draw_text(pixmap, &ip_str, px + 64.0, py + 115.0, 14.0, Color::from_rgba8(9, 9, 11, 255));
    }

    fn render_calendar_panel_body(&mut self, pixmap: &mut PixmapMut, px: f32, py: f32, _w: f32) {
        let pasaran = get_pasaran_jawa(2025, 5, 18);
        let (h_yr, h_m, h_d) = get_hijri_date(2025, 5, 18);

        let cal_header = format!("MEI 2025  |  Pasaran: {}  |  {} {} {} H", pasaran, h_d, h_m, h_yr);
        self.renderer.text.draw_text(pixmap, &cal_header, px + 40.0, py + 20.0, 15.0, Color::from_rgba8(9, 9, 11, 255));

        self.renderer.draw_rect_panel(pixmap, px + 40.0, py + 50.0, 720.0, 190.0, Color::from_rgba8(244, 244, 245, 255), Color::from_rgba8(9, 9, 11, 255), 8.0);
        self.renderer.text.draw_text(pixmap, "TANGGAL MERAH & AGENDA:", px + 56.0, py + 66.0, 13.0, Color::from_rgba8(239, 68, 68, 255));

        for (i, hol) in HOLIDAYS_2025.iter().take(4).enumerate() {
            let hol_str = format!("* {}-0{}-2025: {}", hol.day, hol.month, hol.name);
            self.renderer.text.draw_text(pixmap, &hol_str, px + 56.0, py + 95.0 + (i as f32 * 26.0), 13.0, Color::from_rgba8(9, 9, 11, 255));
        }
    }

    fn render_addons_panel_body(&mut self, pixmap: &mut PixmapMut, px: f32, py: f32, _w: f32) {
        let prayer = get_prayer_times(&self.config.active_city);
        self.renderer.draw_rect_panel(pixmap, px + 24.0, py + 10.0, 360.0, 110.0, Color::from_rgba8(244, 244, 245, 255), Color::from_rgba8(9, 9, 11, 255), 8.0);
        let pray_str = format!("SHOLAT ({}): Subuh {} | Dzuhur {} | Ashar {} | Maghrib {}", self.config.active_city, prayer.subuh, prayer.dzuhur, prayer.ashar, prayer.maghrib);
        self.renderer.text.draw_text(pixmap, &pray_str, px + 36.0, py + 50.0, 12.0, Color::from_rgba8(9, 9, 11, 255));

        self.renderer.draw_rect_panel(pixmap, px + 400.0, py + 10.0, 360.0, 110.0, Color::from_rgba8(244, 244, 245, 255), Color::from_rgba8(9, 9, 11, 255), 8.0);
        let traf_str = format!("TRAFFIC: D: {:.1} KB/s | U: {:.1} KB/s", self.traffic.download_kbps, self.traffic.upload_kbps);
        self.renderer.text.draw_text(pixmap, &traf_str, px + 420.0, py + 50.0, 13.0, Color::from_rgba8(34, 211, 238, 255));

        self.renderer.draw_rect_panel(pixmap, px + 24.0, py + 130.0, 736.0, 70.0, Color::from_rgba8(190, 242, 100, 255), Color::from_rgba8(9, 9, 11, 255), 8.0);
        self.renderer.text.draw_text(pixmap, &self.teleprompter.script, px + 40.0, py + 160.0, 14.0, Color::from_rgba8(9, 9, 11, 255));
    }

    pub fn handle_click(&mut self, x: f32, y: f32, screen_w: f32) {
        let cx = screen_w / 2.0;
        let top_y = 12.0;

        if self.active_panel != ActivePanel::None {
            let panel_x = cx - (self.width_spring.current / 2.0);
            let panel_y = top_y + 4.0;
            let close_x = panel_x + self.width_spring.current - 40.0;
            if x >= close_x && x <= close_x + 36.0 && y >= panel_y && y <= panel_y + 40.0 {
                self.active_panel = ActivePanel::None;
                return;
            }

            if self.active_panel == ActivePanel::Audio && x >= panel_x + 24.0 && x <= panel_x + 384.0 && y >= panel_y + 54.0 && y <= panel_y + 164.0 {
                AudioManager::toggle_mic_mute();
                return;
            }
        }

        if self.hover_spring.current > 0.5 && self.active_panel == ActivePanel::None {
            if let Some(idx) = self.hovered_icon_idx {
                self.active_panel = match idx {
                    0 => ActivePanel::Audio,
                    1 => ActivePanel::MusicLyrics,
                    2 => ActivePanel::Screenshot,
                    3 => ActivePanel::Shelf,
                    4 => ActivePanel::Calendar,
                    5 => ActivePanel::SambungHp,
                    6 => ActivePanel::Addons,
                    _ => ActivePanel::None,
                };
            }
        }
    }
}