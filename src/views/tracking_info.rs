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

use std::fmt::Write;

use colored::Colorize;
use derive_new::new;
use eyre::Result;

use crate::{client::Event, title};

/// A tracking info view.
#[derive(new, Debug)]
pub struct TrackingInfo<'a> {
    tracking_number: &'a str,
    description: Option<&'a str>,
    events: &'a [Event],
}

impl<'a> TrackingInfo<'a> {
    /// Renders tracking info.
    pub fn render(&self) -> Result<String> {
        let Self {
            tracking_number,
            description,
            events,
        } = self;

        let mut rendered = String::new();

        if let Some(description) = description {
            title!(
                &mut rendered,
                "\n--- {description} ({tracking_number}) ---\n"
            )?;
        } else {
            title!(&mut rendered, "\n--- {tracking_number} ---\n")?;
        }

        for event in events.iter().rev() {
            let date = format!("{}:", event.date.to_rfc2822());
            writeln!(&mut rendered, "{} {}", date.bright_black(), event.label)?;
        }

        Ok(rendered)
    }
}
