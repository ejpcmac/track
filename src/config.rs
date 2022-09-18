// track - A quick-and-dirty CLI tool for tracking parcels.
// Copyright (C) 2022 Jean-Philippe Cugnet <jean-philippe@cugnet.eu>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3 of the License.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::{fs, io, path::PathBuf};

use derive_new::new;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// The configuration for `track`.
#[derive(new, Debug, Serialize, Deserialize)]
pub struct Config {
    api_key: String,
}

/// An error that can occur when loading the config.
#[derive(Debug, Error)]
pub enum LoadError {
    #[error("impossible to locate the config file")]
    NoConfigDir(#[from] ConfigDirError),
    #[error("error while reading the config file")]
    ReadError(#[from] io::Error),
    #[error("error while parsing the config file")]
    ParseError(#[from] toml::de::Error),
}

/// An error that can occur when saving the config.
#[derive(Debug, Error)]
pub enum SaveError {
    #[error("impossible to locate the config file")]
    NoConfigDir(#[from] ConfigDirError),
    #[error("error while writing to the config file")]
    FsError(#[from] io::Error),
}

/// An error that can occur when getting the data directory.
#[derive(Debug, Error)]
pub enum ConfigDirError {
    #[error("the OS does not define a data directory")]
    NoConfigDir,
}

/// The configuration file name.
const CONFIG_FILE_NAME: &str = "config.toml";

impl Config {
    /// Loads the configuration.
    pub fn load() -> Result<Self, LoadError> {
        let config_file = config_file()?;
        let contents = fs::read_to_string(config_file)?;
        let config = toml::from_str(&contents)?;
        Ok(config)
    }

    /// Saves the configuration.
    pub fn save(&self) -> Result<(), SaveError> {
        fs::create_dir_all(config_dir()?)?;

        let config_file = config_file()?;
        let config =
            toml::to_string(self).expect("failed to serialise the config");
        fs::write(config_file, config)?;

        Ok(())
    }

    /// Gets the API key.
    pub fn api_key(&self) -> &str {
        &self.api_key
    }
}

/// Gets the config directory for `track`.
fn config_dir() -> Result<PathBuf, ConfigDirError> {
    Ok(dirs::config_dir()
        .ok_or(ConfigDirError::NoConfigDir)?
        .join(env!("CARGO_PKG_NAME")))
}

/// Gets the config file for `track`.
fn config_file() -> Result<PathBuf, ConfigDirError> {
    Ok(config_dir()?.join(CONFIG_FILE_NAME))
}
