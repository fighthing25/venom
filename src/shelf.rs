use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct ShelfFile {
    pub path: PathBuf,
    pub name: String,
    pub size_str: String,
    pub extension: String,
}

pub struct DropZoneShelf {
    pub files: Vec<ShelfFile>,
}

impl DropZoneShelf {
    pub fn new() -> Self {
        Self {
            files: vec![
                ShelfFile {
                    path: PathBuf::from("Desain_Final.pdf"),
                    name: "Desain_Final.pdf".to_string(),
                    size_str: "2.4 MB".to_string(),
                    extension: "pdf".to_string(),
                },
                ShelfFile {
                    path: PathBuf::from("Capture_01.png"),
                    name: "Capture_01.png".to_string(),
                    size_str: "1.1 MB".to_string(),
                    extension: "png".to_string(),
                },
            ],
        }
    }

    pub fn add_file(&mut self, path: PathBuf) {
        let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
        let ext = path.extension().unwrap_or_default().to_string_lossy().to_string();
        let size = if let Ok(meta) = std::fs::metadata(&path) {
            let bytes = meta.len();
            if bytes > 1024 * 1024 {
                format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
            } else {
                format!("{:.0} KB", bytes as f64 / 1024.0)
            }
        } else {
            "0 KB".to_string()
        };

        self.files.push(ShelfFile { path, name, size_str: size, extension: ext });
    }

    pub fn clear(&mut self) {
        self.files.clear();
    }
}