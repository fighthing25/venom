use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LrcLine {
    pub time_ms: u64,
    pub text: String,
}

#[derive(Clone, Debug)]
pub struct LyricState {
    pub current_track: String,
    pub current_artist: String,
    pub lines: Vec<LrcLine>,
    pub current_line_index: usize,
    pub is_playing: bool,
    pub current_time_ms: u64,
}

impl LyricState {
    pub fn new() -> Self {
        Self {
            current_track: "Venom".to_string(),
            current_artist: "Eminem".to_string(),
            lines: vec![
                LrcLine { time_ms: 0, text: "Knock knock, let the devil in...".to_string() },
                LrcLine { time_ms: 3000, text: "Manevolent as I've ever been, head is spinnin'".to_string() },
                LrcLine { time_ms: 6000, text: "This medicine's screamin' 'L-L-L-Let us in!'".to_string() },
                LrcLine { time_ms: 9000, text: "Venom, adrenaline momentum".to_string() },
            ],
            current_line_index: 0,
            is_playing: true,
            current_time_ms: 0,
        }
    }

    pub fn tick(&mut self, dt_ms: u64) {
        if !self.is_playing || self.lines.is_empty() {
            return;
        }
        self.current_time_ms += dt_ms;
        let mut idx = 0;
        for (i, line) in self.lines.iter().enumerate() {
            if self.current_time_ms >= line.time_ms {
                idx = i;
            } else {
                break;
            }
        }
        if idx >= self.lines.len() {
            self.current_time_ms = 0;
            idx = 0;
        }
        self.current_line_index = idx;
    }

    pub fn current_text(&self) -> &str {
        if self.lines.is_empty() {
            "No Lyrics Playing"
        } else {
            &self.lines[self.current_line_index].text
        }
    }

    pub fn fetch_lyrics_async(state: Arc<Mutex<LyricState>>, track: String, artist: String) {
        thread::spawn(move || {
            let url = format!(
                "https://lrclib.net/api/get?track_name={}&artist_name={}",
                track.replace(' ', "+"),
                artist.replace(' ', "+")
            );
            if let Ok(resp) = ureq::get(&url).timeout(std::time::Duration::from_secs(4)).call() {
                if let Ok(json) = resp.into_json::<serde_json::Value>() {
                    if let Some(synced) = json.get("syncedLyrics").and_then(|v| v.as_str()) {
                        let parsed = parse_lrc(synced);
                        if !parsed.is_empty() {
                            if let Ok(mut lock) = state.lock() {
                                lock.current_track = track;
                                lock.current_artist = artist;
                                lock.lines = parsed;
                                lock.current_line_index = 0;
                                lock.current_time_ms = 0;
                            }
                        }
                    }
                }
            }
        });
    }
}

fn parse_lrc(lrc_content: &str) -> Vec<LrcLine> {
    let mut result = Vec::new();
    for line in lrc_content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') {
            if let Some(close_bracket) = trimmed.find(']') {
                let time_str = &trimmed[1..close_bracket];
                let text_str = trimmed[close_bracket + 1..].trim();
                let parts: Vec<&str> = time_str.split(':').collect();
                if parts.len() == 2 {
                    if let (Ok(m), Ok(s)) = (parts[0].parse::<u64>(), parts[1].parse::<f64>()) {
                        let total_ms = (m * 60 * 1000) + (s * 1000.0) as u64;
                        result.push(LrcLine { time_ms: total_ms, text: text_str.to_string() });
                    }
                }
            }
        }
    }
    result.sort_by_key(|l| l.time_ms);
    result
}