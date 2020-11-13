extern crate inflector;
extern crate version_compare;

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use thiserror::Error;
use version_compare::Version;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    version: String,
    pub listen_address: String,
    pub drop_invalid_packets: bool,
    // pub velocity: bool,
    // pub velocity_secret: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            version: built_info::PKG_VERSION.to_string(),
            listen_address: "127.0.0.1:25565".to_string(),
            drop_invalid_packets: false,
            // velocity: false,
            // velocity_secret: thread_rng().sample_iter(&Alphanumeric).take(16).collect(),
        }
    }
}

impl Config {
    // TODO: reloadable config
    pub fn load() -> Self {
        let path = Path::new("shiryuu.toml");

        if path.exists() {
            toml::from_str(&fs::read_to_string(path).unwrap()).unwrap()
        } else {
            let config = Config::default();
            fs::write(path, toml::to_string_pretty(&config).unwrap()).unwrap();

            config
        }
    }

    pub fn validate(&self) -> Result<Vec<&str>, ConfigError> {
        // TODO: implement custom type for warnings
        let mut warnings: Vec<&'static str> = vec![];

        let config_version = Version::from(self.version.as_str()).unwrap();
        let version = Version::from(built_info::PKG_VERSION).unwrap();

        if version.part(0).unwrap() != config_version.part(0).unwrap() {
            return Err(ConfigError::MajorVersionMismatch());
        } else if version.part(1).unwrap() != config_version.part(1).unwrap() {
            warnings.push("Detected a minor version mismatch. A config update is encouraged.");
        }

        Ok(warnings)
    }
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Detected a major version mismatch! Please fix your config.")]
    MajorVersionMismatch(),
}

lazy_static! {
    pub static ref CONFIG: Config = Config::load();
}

#[allow(dead_code)]
pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));

    pub fn get_pretty_name() -> String {
        inflector::cases::titlecase::to_title_case(PKG_NAME)
    }
}
