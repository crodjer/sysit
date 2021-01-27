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

/// Get system resources overview in 40 characters
/// For usage details, try --help
#[derive(Clap)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = "Rohan Jain")]
pub struct Config {
    /// run in watch mode
    /// (will act as if running with the watch)
    #[clap(short, long)]
    pub watch: bool,

    /// run in log mode
    /// (will continuously append a row to standard output)
    #[clap(short, long)]
    pub log: bool,

    /// update interval in seconds for watch/log mode
    #[clap(short, long, default_value = "1")]
    pub interval: u64,

    /// force output to be always colorized
    #[clap(short, long)]
    pub colors: bool,

    /// force output to be never colorized
    #[clap(long)]
    pub no_colors: bool,

    /// the threshold for warm temperature
    /// (higher values will be rendered in yellow)
    #[clap(long, default_value = "55.0")]
    pub threshold_temp_warm: f32,

    /// the threshold for high temperature
    /// (higher values will be rendered in red)
    #[clap(long, default_value = "75.0")]
    pub threshold_temp_hot: f32,

    /// the threshold for medium memory usage
    /// (higher values will be rendered in yellow)
    #[clap(long, default_value = "50.0")]
    pub threshold_memory_medium: f32,

    /// the threshold for high memory usage
    /// (higher values will be rendered in red)
    #[clap(long, default_value = "80.0")]
    pub threshold_memory_high: f32,

    /// the threshold for medium cpu usage
    /// (higher values will be rendered in yellow)
    #[clap(long, default_value = "50.0")]
    pub threshold_cpu_medium: f32,

    /// the threshold for high cpu usage
    /// (higher values will be rendered in red)
    #[clap(long, default_value = "80.0")]
    pub threshold_cpu_high: f32,
}
