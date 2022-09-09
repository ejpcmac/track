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

mod add;
mod all;
mod helpers;
mod info;
mod init;
mod list;
mod remove;

use clap::Parser;
use color_eyre::Result;

use self::{
    add::Add, all::All, info::Info, init::Init, list::List, remove::Remove,
};

/// A quick-and-dirty CLI tool for tracking parcels.
#[derive(Debug, Parser)]
#[clap(version, author)]
pub enum Track {
    /// Initialise the configuration.
    Init(Init),
    /// Retrieve and prints tracking info for a parcel.
    Info(Info),
    /// Print the set of tracked parcels.
    List,
    /// Add a parcel to the tracked set.
    Add(Add),
    /// Remove a parcel from the tracked set.
    Remove(Remove),
    /// Retrieve and prints tracking info for all tracked parcels.
    All,
}

trait Command {
    /// Runs the command.
    fn run(&self) -> Result<()>;
}

impl Track {
    /// Runs track.
    pub fn run() -> Result<()> {
        match Self::parse() {
            Self::Init(init) => init.run(),
            Self::Info(info) => info.run(),
            Self::List => List.run(),
            Self::Add(add) => add.run(),
            Self::Remove(remove) => remove.run(),
            Self::All => All.run(),
        }
    }
}
