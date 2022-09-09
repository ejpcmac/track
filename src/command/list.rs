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

use clap::Parser;
use color_eyre::Result;
use colored::Colorize;

use crate::state::State;

#[derive(Debug, Parser)]
pub struct List;

impl super::Command for List {
    fn run(&self) -> Result<()> {
        let state = State::load()?;

        println!("\n{}\n", "--- Tracked parcels ---".bold());
        for (tracking_number, description) in state.parcels() {
            println!("{tracking_number}: {description}");
        }
        println!();

        Ok(())
    }
}
