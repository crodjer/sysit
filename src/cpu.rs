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

use super::colors::colorize;
use super::config::Config;
use sysinfo::{ProcessorExt, System, SystemExt};

pub fn usage(config: &Config, system: &System) -> String {
    let processor = system.global_processor_info();

    format!(
        "{}%",
        colorize(
            processor.cpu_usage(),
            config.threshold_cpu_high,
            config.threshold_cpu_medium
        )
    )
}

pub fn frequency(system: &System) -> String {
    format!("{:4} MHz", system.global_processor_info().frequency())
}
