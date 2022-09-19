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

//! Filters for Askama templates.

use colored::Colorize;

/// Decorates the string in bold.
pub fn bold(s: &str) -> askama::Result<String> {
    Ok(s.bold().to_string())
}

/// Decorates the string in bright black.
pub fn bright_black(s: &str) -> askama::Result<String> {
    Ok(s.bright_black().to_string())
}
