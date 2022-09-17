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

use colored::Colorize;

use crate::{client::Event, title};

/// Renders tracking info.
pub fn render(
    tracking_number: &str,
    description: Option<&str>,
    events: &[Event],
) {
    if let Some(description) = description {
        title!("\n--- {description} ({tracking_number}) ---\n");
    }

    for event in events.iter().rev() {
        let date = format!("{}:", event.date.to_rfc2822());
        println!("{} {}", date.bright_black(), event.label);
    }
}
