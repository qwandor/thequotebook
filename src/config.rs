use eyre::{bail, Report, WrapErr};
use serde::Deserialize;
use std::{
    fs::read_to_string,
    net::SocketAddr,
    path::{Path, PathBuf},
};

/// Paths at which to look for the config file. They are searched in order, and the first one that
/// exists is used.
const CONFIG_FILENAMES: [&str; 2] = ["thequotebook.toml", "/etc/thequotebook.toml"];

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub postgres_uri: String,
    #[serde(default = "default_public_dir")]
    pub public_dir: PathBuf,
    #[serde(default = "default_bind_address")]
    pub bind_address: SocketAddr,
    pub google_client_id: String,
    #[serde(default = "default_base_url")]
    pub base_url: String,
}

impl Config {
    pub fn from_file() -> Result<Config, Report> {
        for filename in &CONFIG_FILENAMES {
            if Path::new(filename).is_file() {
                return Config::read(filename);
            }
        }
        bail!(
            "Unable to find config file in any of {:?}",
            &CONFIG_FILENAMES
        );
    }

    fn read(filename: &str) -> Result<Config, Report> {
        let config_file =
            read_to_string(filename).wrap_err_with(|| format!("Reading {}", filename))?;
        Ok(toml::from_str(&config_file)?)
    }
}

fn default_public_dir() -> PathBuf {
    Path::new("public").to_path_buf()
}

fn default_bind_address() -> SocketAddr {
    "0.0.0.0:3000".parse().unwrap()
}

fn default_base_url() -> String {
    "http://localhost:3000/".to_string()
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
