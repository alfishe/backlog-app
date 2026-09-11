use parking_lot::RwLock;
use std::path::PathBuf;

pub struct AppState {
    port: u16,
    data_dir: RwLock<Option<PathBuf>>,
}

impl AppState {
    pub fn new(port: u16) -> Self {
        Self {
            port,
            data_dir: RwLock::new(None),
        }
    }

    pub fn server_port(&self) -> u16 {
        self.port
    }

    pub fn data_dir(&self) -> Option<PathBuf> {
        self.data_dir.read().clone()
    }

    pub fn set_data_dir(&self, path: &str) -> Result<(), std::io::Error> {
        let path = PathBuf::from(path);
        if !path.exists() {
            std::fs::create_dir_all(&path)?;
        }
        *self.data_dir.write() = Some(path);
        Ok(())
    }

    pub fn backlog_path(&self) -> Option<PathBuf> {
        self.data_dir().map(|p| p.join("backlog.md"))
    }

    pub fn archive_path(&self) -> Option<PathBuf> {
        self.data_dir().map(|p| p.join("archive.md"))
    }

    pub fn backup_dir(&self) -> Option<PathBuf> {
        self.data_dir().map(|p| p.join("backups"))
    }
}
