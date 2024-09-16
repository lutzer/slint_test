use std::{fs::File, io::{Error, Read, Write}};

use serde::{Deserialize, Serialize};
use ui::AppState;

use crate::ui;

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub window_x: i32,
    pub window_y: i32,
    pub state: AppState
}

impl Settings {
    pub fn save(self: &Settings, path: &str) -> std::io::Result<()> {
        // Serialize the settings to a JSON string
        let json = serde_json::to_string_pretty(self)?;
        
        // Write the JSON string to a file
        let mut file = File::create(path)?;
        file.write_all(json.as_bytes())?;
        
        Ok(())
    }

    pub fn load(path: &str) -> Result<Settings, Error> {
        // Read the file content
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        
        // Deserialize the JSON string to a Settings struct
        let settings = serde_json::from_str(&contents)?;
        return Ok(settings);
    }

    pub fn default() -> Settings {
        return Settings{
            window_x: 100,
            window_y: 100,
            state: AppState {
                counter: 0,
                counter_opacity: 1.0
            }
        }
    }
}