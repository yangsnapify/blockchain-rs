use std::fs;
use std::path::Path;
use serde::{ Serialize, Deserialize };
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub max_transactions_per_pool: usize,
    pub cache_enabled: bool,
}

static CONFIG: Mutex<Option<Config>> = Mutex::new(None);

impl Config {
    pub fn load_config() -> Config {
        let mut config_lock = CONFIG.lock().unwrap();

        if let Some(ref config) = *config_lock {
            if config.cache_enabled {
                return config.clone();
            }
        }

        let config = Config::read_config_from_file();

        if config.cache_enabled {
            *config_lock = Some(config.clone());
        }
        config
    }

    pub fn read_config_from_file() -> Config {
        let config_path = "block_config.json";

        if !Path::new(config_path).exists() {
            panic!("Config file not found");
        }
        let config_content = fs::read_to_string(config_path).expect("Unable to read file");
        serde_json::from_str(&config_content).expect("Unable to parse config")
    }

    pub fn purge_config() {
        let mut config_lock = CONFIG.lock().unwrap();
        *config_lock = None;
        println!("Configuration cache cleared!");
    }
}
