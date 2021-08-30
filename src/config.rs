//! Tools for managing config.
use crate::action::ConfigAction;
use directories_next::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{create_dir_all, File};
use std::path::PathBuf;

const DIR_QUALIFIER: &str = "li.arty";
const DIR_ORGANIZATION: &str = "Artemis";
const DIR_APPLICATION: &str = "Shareit CLI";
const CONFIG_FILE: &str = "config.json";

#[derive(Serialize, Deserialize, Default)]
pub struct ConfigData {
    #[serde(default)]
    pub base_url: Option<String>,
    #[serde(default)]
    pub password: Option<String>,
}

pub struct Config {
    pub data: ConfigData,
    path: PathBuf,
}

impl Config {
    pub fn new() -> Result<Self, String> {
        let dirs = ProjectDirs::from(DIR_QUALIFIER, DIR_ORGANIZATION, DIR_APPLICATION)
            .ok_or("Could not find directory for config")?;
        let dir = dirs.config_dir();
        create_dir_all(&dir).map_err(|e| format!("Could not create config directory: {}", e))?;
        let path = dir.join(CONFIG_FILE);
        let data = match File::open(&path) {
            Ok(mut file) => serde_json::from_reader(&mut file)
                .map_err(|e| format!("Could not parse config file: {}", e))?,
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => ConfigData::default(),
                _ => return Err(format!("Could not open config file: {}", e)),
            },
        };
        Ok(Config { data, path })
    }

    pub fn do_action(self, action: ConfigAction) -> Result<(), String> {
        // We need to convert the config struct to a map, but is there no
        // better way to do this than a roundtrip through JSON?
        let mut data: HashMap<String, serde_json::Value> = serde_json::from_str(
            &serde_json::to_string(&self.data).expect("JSON roundtrip failed on encode."),
        )
        .expect("JSON roundtrip failed on decode.");
        match action {
            ConfigAction::List => {
                let mut output = String::new();
                for (key, value) in data.iter() {
                    output.push_str(&format!("{}: {}\n", key, value));
                }
                print!("{}", output);
            }
            ConfigAction::Get(key) => {
                let value = data
                    .get(&key)
                    .ok_or(format!("No such config option: {}", key))?;
                println!("{}", value);
            }
            ConfigAction::Set(key, value) => {
                if !data.contains_key(&key) {
                    return Err(format!("No such config option: {}", key));
                }
                data.insert(key, value.into());
                let file = File::create(&self.path)
                    .map_err(|e| format!("Could not create config file: {}", e))?;
                serde_json::to_writer(&file, &data)
                    .map_err(|e| format!("Could not write to config file: {}", e))?;
            }
        }
        Ok(())
    }
}
