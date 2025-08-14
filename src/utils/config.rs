use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub theme: String,
    pub auto_refresh: bool,
    pub confirm_deletion: bool,
    pub backup_enabled: bool,
    pub backup_interval_days: u32,
    pub log_level: String,
    pub profiles_dir: String,
    pub default_profile: Option<String>,
    pub window_geometry: WindowGeometry,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowGeometry {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub maximized: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        let mut profiles_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        profiles_dir.push("profiles");

        Self {
            theme: "light".to_string(),
            auto_refresh: true,
            confirm_deletion: true,
            backup_enabled: true,
            backup_interval_days: 7,
            log_level: "info".to_string(),
            profiles_dir: profiles_dir.to_string_lossy().to_string(),
            default_profile: None,
            window_geometry: WindowGeometry::default(),
        }
    }
}

impl Default for WindowGeometry {
    fn default() -> Self {
        Self {
            x: 100.0,
            y: 100.0,
            width: 800.0,
            height: 600.0,
            maximized: false,
        }
    }
}

impl AppConfig {
    pub fn config_dir() -> PathBuf {
        let mut path = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        path.push("config");
        
        if !path.exists() {
            std::fs::create_dir_all(&path).expect("Failed to create config directory");
        }
        
        path
    }

    pub fn config_file() -> String {
        let mut config_dir = Self::config_dir();
        config_dir.push("config.json");
        config_dir.to_string_lossy().to_string()
    }

    pub fn load() -> Self {
        let config_file = Self::config_file();
        
        if std::path::Path::new(&config_file).exists() {
            match std::fs::read_to_string(&config_file) {
                Ok(content) => match serde_json::from_str(&content) {
                    Ok(config) => {
                        log::info!("Configuration loaded from {}", config_file);
                        return config;
                    }
                    Err(e) => {
                        log::error!("Failed to parse config file: {}", e);
                    }
                },
                Err(e) => {
                    log::error!("Failed to read config file: {}", e);
                }
            }
        }

        log::info!("Using default configuration");
        Self::default()
    }
}