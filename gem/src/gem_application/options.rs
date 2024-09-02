use std::fs;
use log::error;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
   pub screen_size: ScreenSize
}

#[derive(Deserialize, Debug)]
pub struct ScreenSize {
    pub x: i32,
    pub y: i32
}

#[derive(Debug)]
pub struct Options(Option<Config>);

impl Options {
    pub fn new() -> Self {
        Self(None)
    }

    pub fn init(&mut self, path: &str) {
        let contents = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => {
                error!(target: "gem_events", "Could not read config file {path:?}");
                // Exit the program with exit code `1`.
                panic!("Could not read config file");
            }
        };

        let data: Config = match toml::from_str(&contents) {
            Ok(d) => d,
            Err(_) => {
                error!(target: "gem_events", "Unable to load config data from {path:?}");
                // Exit the program with exit code `1`.
                panic!("Could not read config file");
            }
        };

        self.0.replace(data);
    }

    pub fn get_config(&self) -> Option<&Config> {
        self.0.as_ref()
    }
}