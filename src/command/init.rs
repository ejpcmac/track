// track - A quick-and-dirty CLI tool for tracking parcels.
// Copyright (C) 2020, 2022 Jean-Philippe Cugnet <jean-philippe@cugnet.eu>
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

use clap::Parser;
use eyre::{bail, Result};
use inquire::Text;
use thiserror::Error;

use crate::{config::Config, success};

/// Arguments for `track init`.
#[derive(Debug, Parser)]
pub struct Init {
    /// Force the init process.
    #[arg(short, long)]
    force: bool,
}

/// Usage errors of `track init`.
#[derive(Debug, Error)]
pub enum InitError {
    #[error("There is already a configuration.")]
    ExistingConfig,
}

impl super::Command for Init {
    fn run(&self) -> Result<()> {
        if !self.force && Config::load().is_ok() {
            bail!(InitError::ExistingConfig);
        }

        let api_key = Text::new("La Poste API key:").prompt()?;
        Config::new(api_key).save()?;

        success!("The configuration has been initialised.");
        Ok(())
    }
}
