use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppConfig {
    pub retro_sound_enabled: bool,
    pub launch_on_startup: bool,
    pub hover_sensitivity: f32,
    pub window_opacity: f32,
    pub show_kalender_jawa: bool,
    pub show_kalender_hijriah: bool,
    pub active_city: String,
    pub game_mode_blacklist: Vec<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            retro_sound_enabled: true,
            launch_on_startup: false,
            hover_sensitivity: 1.0,
            window_opacity: 1.0,
            show_kalender_jawa: true,
            show_kalender_hijriah: true,
            active_city: "Jakarta".to_string(),
            game_mode_blacklist: vec![
                "valorant.exe".to_string(),
                "cs2.exe".to_string(),
                "dota2.exe".to_string(),
            ],
        }
    }
}

impl AppConfig {
    pub fn config_path() -> PathBuf {
        let mut p = dirs_fallback();
        p.push("VenomIsland");
        let _ = fs::create_dir_all(&p);
        p.push("config.json");
        p
    }

    pub fn load() -> Self {
        let p = Self::config_path();
        if let Ok(data) = fs::read_to_string(&p) {
            if let Ok(cfg) = serde_json::from_str::<AppConfig>(&data) {
                return cfg;
            }
        }
        let def = AppConfig::default();
        def.save();
        def
    }

    pub fn save(&self) {
        let p = Self::config_path();
        if let Ok(serialized) = serde_json::to_string_pretty(self) {
            let _ = fs::write(p, serialized);
        }
    }
}

fn dirs_fallback() -> PathBuf {
    if let Ok(appdata) = std::env::var("APPDATA") {
        PathBuf::from(appdata)
    } else {
        PathBuf::from(".")
    }
}