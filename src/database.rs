use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use crate::app::App;

impl App {
    pub fn save_database(&self) -> Result<(), Box<dyn Error>> {
        // do not save a database if there's nothing to be saved
        if self.hex_view.bookmarks.is_empty() && self.hex_view.comment_name_list.is_empty() {
            return Ok(());
        }

        let toml_string = toml::to_string_pretty(&self.hex_view)?;
        let target_dir: &Path = Path::new(&self.file_info.path)
            .parent()
            .unwrap_or(Path::new("."));
        let cwd_db = format!("{}.dz6", self.file_info.name);
        let target_db: PathBuf = target_dir.join(&cwd_db);

        // try target's path or else current directory
        fs::write(&target_db, &toml_string).or_else(|_| fs::write(&cwd_db, &toml_string))?;

        Ok(())
    }
    pub fn load_database(&mut self) -> Result<(), Box<dyn Error>> {
        let target_dir: &Path = Path::new(&self.file_info.path)
            .parent()
            .unwrap_or(Path::new("."));
        let cwd_db = format!("{}.dz6", self.file_info.name);
        let target_db: PathBuf = target_dir.join(&cwd_db);
        let data = fs::read_to_string(&cwd_db).or_else(|_| fs::read_to_string(&target_db))?;

        self.hex_view = toml::from_str(&data)?;
        Ok(())
    }
}
