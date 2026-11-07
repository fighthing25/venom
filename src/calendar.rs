use std::collections::HashMap;

pub struct TanggalMerah {
    pub day: u32,
    pub month: u32,
    pub name: &'static str,
}

pub const HOLIDAYS_2025: &[TanggalMerah] = &[
    TanggalMerah { day: 1, month: 1, name: "Tahun Baru 2025 Masehi" },
    TanggalMerah { day: 27, month: 1, name: "Isra Mi'raj Nabi Muhammad SAW" },
    TanggalMerah { day: 29, month: 1, name: "Tahun Baru Imlek 2576 Kongzili" },
    TanggalMerah { day: 29, month: 3, name: "Hari Suci Nyepi (Tahun Baru Saka 1947)" },
    TanggalMerah { day: 31, month: 3, name: "Idul Fitri 1446 H" },
    TanggalMerah { day: 1, month: 4, name: "Idul Fitri 1446 H" },
    TanggalMerah { day: 18, month: 4, name: "Wafat Yesus Kristus" },
    TanggalMerah { day: 20, month: 4, name: "Kebangkitan Yesus Kristus (Paskah)" },
    TanggalMerah { day: 1, month: 5, name: "Hari Buruh Internasional" },
    TanggalMerah { day: 12, month: 5, name: "Hari Raya Waisak 2569 BE" },
    TanggalMerah { day: 29, month: 5, name: "Kenaikan Yesus Kristus" },
    TanggalMerah { day: 1, month: 6, name: "Hari Lahir Pancasila" },
    TanggalMerah { day: 7, month: 6, name: "Hari Raya Idul Adha 1446 H" },
    TanggalMerah { day: 27, month: 6, name: "Tahun Baru Islam 1447 H" },
    TanggalMerah { day: 17, month: 8, name: "Hari Proklamasi Kemerdekaan RI" },
    TanggalMerah { day: 5, month: 9, name: "Maulid Nabi Muhammad SAW" },
    TanggalMerah { day: 25, month: 12, name: "Hari Raya Natal" },
];

pub fn get_pasaran_jawa(year: i32, month: u32, day: u32) -> &'static str {
    let a = (14 - month as i32) / 12;
    let y = year + 4800 - a;
    let m = month as i32 + 12 * a - 3;
    let jdn = day as i32 + (153 * m + 2) / 5 + 365 * y + y / 4 - y / 100 + y / 400 - 32045;
    match (jdn + 3) % 5 {
        0 => "Legi", 1 => "Pahing", 2 => "Pon", 3 => "Wage", 4 => "Kliwon", _ => "Legi",
    }
}

pub fn get_hijri_date(year: i32, month: u32, day: u32) -> (i32, &'static str, u32) {
    let a = (14 - month as i32) / 12;
    let y = year + 4800 - a;
    let m = month as i32 + 12 * a - 3;
    let jdn = day as i32 + (153 * m + 2) / 5 + 365 * y + y / 4 - y / 100 + y / 400 - 32045;
    let l = jdn - 1948440 + 10632;
    let n = (l - 1) / 10631;
    let l2 = l - 10631 * n + 354;
    let j = ((10985 - l2) / 5316) * ((50 * l2) / 17719) + (l2 / 5670) * ((43 * l2) / 15238);
    let l3 = l2 - ((30 - j) / 15) * ((17719 * j) / 50) - (j / 16) * ((15238 * j) / 43) + 29;
    let hijri_month = (24 * l3) / 709;
    let hijri_day = l3 - (709 * hijri_month) / 24;
    let hijri_year = 30 * n + j - 30;

    let month_names = [
        "Muharram", "Safar", "Rabi'ul Awwal", "Rabi'ul Akhir", "Jumadil Ula",
        "Jumadil Akhir", "Rajab", "Sya'ban", "Ramadhan", "Syawwal", "Dzulqa'dah", "Dzulhijjah"
    ];
    let m_idx = (hijri_month.clamp(1, 12) - 1) as usize;
    (hijri_year, month_names[m_idx], hijri_day as u32)
}

pub struct TodoManager {
    pub items: HashMap<String, Vec<String>>,
}

impl TodoManager {
    pub fn new() -> Self {
        let mut items = HashMap::new();
        items.insert("2025-05-18".to_string(), vec!["Review PR VenomIsland".to_string(), "Push to main".to_string()]);
        Self { items }
    }

    pub fn get_todos(&self, date_key: &str) -> Vec<String> {
        self.items.get(date_key).cloned().unwrap_or_default()
    }

    pub fn add_todo(&mut self, date_key: &str, task: String) {
        self.items.entry(date_key.to_string()).or_default().push(task);
    }
}