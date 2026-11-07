use std::time::Instant;

#[derive(Clone, Debug)]
pub struct ClipboardItem {
    pub text: String,
    pub is_otp: bool,
    pub created_at: Instant,
}

pub struct ClipboardHistory {
    pub items: Vec<ClipboardItem>,
    pub max_items: usize,
}

impl ClipboardHistory {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            max_items: 20,
        }
    }

    pub fn push(&mut self, text: String) {
        if text.trim().is_empty() {
            return;
        }
        let is_otp = text.len() >= 4 && text.len() <= 8 && text.chars().all(|c| c.is_ascii_digit());
        self.items.retain(|i| i.text != text);
        self.items.insert(0, ClipboardItem {
            text,
            is_otp,
            created_at: Instant::now(),
        });
        if self.items.len() > self.max_items {
            self.items.truncate(self.max_items);
        }
    }

    pub fn purge_expired_otps(&mut self) {
        self.items.retain(|item| {
            if item.is_otp {
                item.created_at.elapsed().as_secs() < 60
            } else {
                true
            }
        });
    }
}