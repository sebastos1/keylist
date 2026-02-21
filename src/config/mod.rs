mod bindings;

pub use bindings::*;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    Dark,
    Light,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_font")]
    pub font: String,
    #[serde(default = "default_font_size")]
    pub font_size: f32,
    #[serde(default = "default_theme")]
    pub theme: Theme,
    #[serde(rename = "mod", default = "default_mod_key")]
    pub mod_key: String,
    #[serde(default = "default_columns")]
    pub columns: u32,
}

fn default_font() -> String {
    "sans-serif".to_string()
}

fn default_font_size() -> f32 {
    20.0
}

fn default_theme() -> Theme {
    Theme::Dark
}

fn default_mod_key() -> String {
    "win".to_string()
}

fn default_columns() -> u32 {
    1
}

pub fn default_config_path() -> PathBuf {
    dirs::config_dir()
        .unwrap()
        .join("keylist")
        .join("config.yaml")
}

impl Default for Config {
    fn default() -> Self {
        Self {
            font: default_font(),
            font_size: default_font_size(),
            theme: default_theme(),
            mod_key: default_mod_key(),
            columns: default_columns(),
        }
    }
}

#[derive(Debug)]
pub struct Document {
    pub config: Config,
    pub categories: Vec<Category>,
}

pub fn load(path: Option<PathBuf>, mod_override: Option<String>) -> Document {
    println!("deafult config path: {:?}", default_config_path());

    let contents = if let Some(p) = path {
        match std::fs::read_to_string(&p) {
            Ok(c) => c,
            Err(_) => {
                eprintln!("no config file: {p:?}");
                std::process::exit(1);
            }
        }
    } else {
        match std::fs::read_to_string(default_config_path()) {
            Ok(c) => c,
            Err(_) => {
                return Document {
                    config: Config::default(),
                    categories: vec![],
                };
            }
        }
    };

    let mut raw: RawDocument = serde_yaml::from_str(&contents).unwrap_or_else(|e| {
        eprintln!("error: could not parse yaml: {e}");
        std::process::exit(1);
    });
    if let Some(m) = mod_override {
        raw.config.mod_key = m;
    }
    raw.into()
}
