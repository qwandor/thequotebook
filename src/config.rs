use eyre::{Report, WrapErr};
use serde::Deserialize;
use std::fs::read_to_string;

const CONFIG_FILENAME: &str = "thequotebook.toml";

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub postgres_uri: String,
}

impl Config {
    pub fn from_file() -> Result<Config, Report> {
        Config::read(CONFIG_FILENAME)
    }

    fn read(filename: &str) -> Result<Config, Report> {
        let config_file =
            read_to_string(filename).wrap_err_with(|| format!("Reading {}", filename))?;
        Ok(toml::from_str(&config_file)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Parsing the example config file should not give any errors.
    #[test]
    fn example_config() {
        Config::read("thequotebook.example.toml").unwrap();
    }

    /// Parsing an empty config file should fail.
    #[test]
    fn empty_config() {
        toml::from_str::<Config>("").unwrap_err();
    }
}
