use eyre::{bail, Report, WrapErr};
use serde::{Deserialize, Deserializer};
use std::{
    fs::read_to_string,
    net::SocketAddr,
    path::{Path, PathBuf},
    time::Duration,
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
    // TODO: Should this be base64? Or generated on startup?
    pub secret: String,
    #[serde(
        default = "default_session_duration",
        deserialize_with = "de_duration_seconds",
        rename = "session_duration_seconds"
    )]
    pub session_duration: Duration,
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

    pub fn absolute_url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }
}

fn default_public_dir() -> PathBuf {
    Path::new("public").to_path_buf()
}

fn default_bind_address() -> SocketAddr {
    "0.0.0.0:3000".parse().unwrap()
}

fn default_base_url() -> String {
    "http://localhost:3000".to_string()
}

fn default_session_duration() -> Duration {
    // 30 days
    Duration::from_secs(30 * 24 * 60 * 60)
}

pub fn de_duration_seconds<'de, D: Deserializer<'de>>(d: D) -> Result<Duration, D::Error> {
    let seconds = u64::deserialize(d)?;
    Ok(Duration::from_secs(seconds))
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
