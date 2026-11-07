use std::collections::VecDeque;
use windows::Win32::NetworkManagement::IpHelper::{GetIfTable2, FreeMibTable, MIB_IF_TABLE2};

pub struct PrayerSchedule {
    pub imsak: &'static str,
    pub subuh: &'static str,
    pub terbit: &'static str,
    pub dzuhur: &'static str,
    pub ashar: &'static str,
    pub maghrib: &'static str,
    pub isya: &'static str,
}

pub fn get_prayer_times(city: &str) -> PrayerSchedule {
    match city {
        "Surabaya" => PrayerSchedule {
            imsak: "04:05", subuh: "04:15", terbit: "05:31", dzuhur: "11:32",
            ashar: "14:52", maghrib: "17:27", isya: "18:39"
        },
        "Bandung" => PrayerSchedule {
            imsak: "04:24", subuh: "04:34", terbit: "05:49", dzuhur: "11:51",
            ashar: "15:12", maghrib: "17:47", isya: "18:59"
        },
        _ => PrayerSchedule {
            imsak: "04:25", subuh: "04:35", terbit: "05:51", dzuhur: "11:52",
            ashar: "15:13", maghrib: "17:48", isya: "19:00"
        },
    }
}

pub struct TrafficMonitor {
    pub download_kbps: f32,
    pub upload_kbps: f32,
    pub history_down: VecDeque<f32>,
    last_in_octets: u64,
    last_out_octets: u64,
}

impl TrafficMonitor {
    pub fn new() -> Self {
        Self {
            download_kbps: 0.0,
            upload_kbps: 0.0,
            history_down: VecDeque::from(vec![0.0; 24]),
            last_in_octets: 0,
            last_out_octets: 0,
        }
    }

    pub fn tick(&mut self) {
        unsafe {
            let mut table_ptr: *mut MIB_IF_TABLE2 = std::ptr::null_mut();
            if GetIfTable2(&mut table_ptr).is_ok() && !table_ptr.is_null() {
                let table = &*table_ptr;
                let num_entries = table.NumEntries as usize;
                let rows = std::slice::from_raw_parts(table.Table.as_ptr(), num_entries);

                let mut total_in = 0u64;
                let mut total_out = 0u64;

                for row in rows {
                    if row.InterfaceAndOperStatusFlags.HardwareInterface() && row.OperStatus.0 == 1 {
                        total_in += row.InOctets;
                        total_out += row.OutOctets;
                    }
                }

                if self.last_in_octets > 0 && total_in >= self.last_in_octets {
                    let diff_in = total_in - self.last_in_octets;
                    let diff_out = total_out.saturating_sub(self.last_out_octets);
                    self.download_kbps = (diff_in as f32 * 8.0) / 1024.0;
                    self.upload_kbps = (diff_out as f32 * 8.0) / 1024.0;
                }

                self.last_in_octets = total_in;
                self.last_out_octets = total_out;

                if self.history_down.len() >= 24 {
                    self.history_down.pop_front();
                }
                self.history_down.push_back(self.download_kbps);

                FreeMibTable(table_ptr as *const _);
            }
        }
    }
}

pub struct Teleprompter {
    pub script: String,
    pub scroll_y: f32,
    pub speed: f32,
    pub is_active: bool,
}

impl Teleprompter {
    pub fn new() -> Self {
        Self {
            script: "Halo semuanya, selamat datang di update mingguan VenomIsland Pure Flat OS!".to_string(),
            scroll_y: 0.0,
            speed: 1.2,
            is_active: false,
        }
    }

    pub fn tick(&mut self) {
        if self.is_active {
            self.scroll_y += self.speed;
        }
    }
}