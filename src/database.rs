use std::error::Error;
use std::fs;

use crate::app::App;

impl App {
    pub fn save_database(&self) -> Result<(), Box<dyn Error>> {
        let toml_string = toml::to_string_pretty(&self.hex_view)?;
        let dbname = self.file_info.name.clone() + ".dz6";
        fs::write(dbname, toml_string)?;
        Ok(())
    }
    pub fn load_database(&mut self) -> Result<(), Box<dyn Error>> {
        let dbname = self.file_info.name.clone() + ".dz6";
        let data = fs::read_to_string(dbname)?;
        self.hex_view = toml::from_str(&data)?;
        Ok(())
    }
}
