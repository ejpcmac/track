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

use askama::Template;
use clap::Parser;
use eyre::{Result, WrapErr};

use crate::{
    client::Client, config::Config, views::tracking_info::TrackingInfo,
};

/// Arguments for `track info`.
#[derive(Debug, Parser)]
pub struct Info {
    /// The tracking number.
    tracking_number: String,
}

impl super::Command for Info {
    fn run(&self) -> Result<()> {
        let Self { tracking_number } = self;
        let config = Config::load()?;
        let client = Client::new(config.api_key())?;

        let events =
            client.get_events(tracking_number).wrap_err_with(|| {
                format!("error getting tracking info for {tracking_number}")
            })?;

        let view =
            TrackingInfo::new(tracking_number, None, &events).render()?;
        println!("{view}");

        Ok(())
    }
}
