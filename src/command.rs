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
use eyre::Result;

use crate::{config, error, hint};

use self::{
    add::Add,
    all::All,
    info::Info,
    init::{Init, InitError},
    list::List,
    remove::{Remove, RemoveError},
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
    List(List),
    /// Add a parcel to the tracked set.
    Add(Add),
    /// Remove a parcel from the tracked set.
    Remove(Remove),
    /// Retrieve and prints tracking info for all tracked parcels.
    All(All),
}

trait Command {
    /// Runs the command.
    fn run(&self) -> Result<()>;
}

impl Track {
    /// Runs track.
    pub fn run() -> Result<()> {
        let result = match Self::parse() {
            Self::Init(init) => init.run(),
            Self::Info(info) => info.run(),
            Self::List(list) => list.run(),
            Self::Add(add) => add.run(),
            Self::Remove(remove) => remove.run(),
            Self::All(all) => all.run(),
        };

        match result {
            Err(e) => handle_errors(e),
            Ok(()) => Ok(()),
        }
    }
}

fn handle_errors(e: color_eyre::Report) -> Result<()> {
    if e.downcast_ref::<config::LoadError>().is_some() {
        error!("The configuration is absent or invalid.");
        hint!("You can create a configuration by running `track init`.");
        std::process::exit(1);
    } else if let Some(e) = e.downcast_ref::<InitError>() {
        match e {
            InitError::ExistingConfig => {
                error!("{e}");
                hint!("You can force the command by running `track init -f`.");
            }
        }
        std::process::exit(1);
    } else if let Some(e) = e.downcast_ref::<RemoveError>() {
        error!("{e}");
        std::process::exit(1);
    } else {
        Err(e)
    }
}
