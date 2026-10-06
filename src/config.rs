use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

const CONFIG_PATH: &str = "config.json";

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Corner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
    None,
    ShowDesktop,
    TaskView,
    LockScreen,
    OpenApplication(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotCorner {
    pub enabled: bool,
    pub action: Action,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub top_left: HotCorner,
    pub top_right: HotCorner,
    pub bottom_left: HotCorner,
    pub bottom_right: HotCorner,

    pub corner_size: u32,
    pub activation_delay_ms: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            top_left: HotCorner {
                enabled: true,
                action: Action::ShowDesktop,
            },

            top_right: HotCorner {
                enabled: true,
                action: Action::TaskView,
            },

            bottom_left: HotCorner {
                enabled: false,
                action: Action::LockScreen,
            },

            bottom_right: HotCorner {
                enabled: false,
                action: Action::None,
            },

            corner_size: 10,
            activation_delay_ms: 300,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        if !Path::new(CONFIG_PATH).exists() {
            let config = Self::default();

            config.save()?;

            return Ok(config);
        }

        let content = fs::read_to_string(CONFIG_PATH)?;

        let config = serde_json::from_str(&content)?;

        Ok(config)
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;

        fs::write(CONFIG_PATH, json)?;

        Ok(())
    }
}
