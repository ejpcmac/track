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

#![doc = include_str!("../README.md")]
#![warn(rust_2018_idioms)]
#![warn(clippy::redundant_pub_crate)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::use_self)]
#![deny(missing_docs)]
#![deny(unused_must_use)]
#![forbid(unsafe_code)]

mod client;
mod command;
mod config;
mod state;
mod views;

#[doc(hidden)]
pub use command::Track;
