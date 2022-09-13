// track - A quick-and-dirty CLI tool for tracking parcels.
// Copyright (C) 2020 Jean-Philippe Cugnet <jean-philippe@cugnet.eu>
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

use std::io::{self, Write};

use clap::Parser;
use colored::Colorize;
use eyre::Result;

use crate::config::Config;

/// Arguments for `track init`.
#[derive(Debug, Parser)]
pub struct Init {
    /// Force the init process.
    #[clap(short, long)]
    force: bool,
}

impl super::Command for Init {
    fn run(&self) -> Result<()> {
        if !self.force && Config::load().is_ok() {
            println!(
                "{}\n{}",
                "There is already a configuration.".red().bold(),
                "You can force the command by running `track init -f`.".blue()
            );
            std::process::exit(1);
        }

        let mut input = String::new();

        print!("{}", "Enter your La Poste API key: ".bold());
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;

        Config::new(input.trim()).save()?;

        println!(
            "{}",
            "The configuration has been initialised.".green().bold()
        );

        Ok(())
    }
}
