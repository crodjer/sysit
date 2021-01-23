/*
sysit | Check on the system with a quick glance!
Copyright (C) 2021 Rohan Jain

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU General Public License as published by the Free Software
Foundation, either version 3 of the License, or any later version.

This program is distributed in the hope that it will be useful, but WITHOUT
ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more
details.

You should have received a copy of the GNU General Public License along with
this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use clap::Clap;

/// Get basic system information in one line.
/// For more information use --help
#[derive(Clap)]
#[clap()]
pub struct Config {
    /// Run in watch mode. Will act as if running with the watch command.
    #[clap(short, long)]
    pub watch: bool,

    /// Run in log mode. Will continuously append a row to standard output.
    #[clap(short, long)]
    pub log: bool,

    /// Specify update interval in seconds for watch/log mode.
    #[clap(short, long, default_value = "1")]
    pub interval: u64,
}
