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

use std::{fs, io};

use serde::{Deserialize, Serialize};

/// The configuration for `track`.
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    api_key: String,
}

impl Config {
    /// Creates a new configuration.
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_owned(),
        }
    }

    /// Loads the configuration.
    pub fn load() -> io::Result<Self> {
        let config_file = dirs::config_dir()
            .unwrap()
            .join("track")
            .join("config.toml");
        let contents = fs::read_to_string(config_file)?;
        let config = toml::from_str(&contents)?;
        Ok(config)
    }

    /// Saves the configuration.
    pub fn save(&self) -> io::Result<()> {
        let config_dir = dirs::config_dir().unwrap().join("track");
        fs::create_dir_all(&config_dir)?;

        let config_file = config_dir.join("config.toml");
        let config = toml::to_string(self).unwrap();
        fs::write(config_file, config)?;

        Ok(())
    }

    /// Gets the API key.
    pub fn api_key(&self) -> &str {
        &self.api_key
    }
}
